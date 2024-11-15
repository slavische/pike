use colored::*;
use ctrlc;
use lazy_static::lazy_static;
use serde::Deserialize;
use std::io::Write;
use std::path::Path;
use std::process::{exit, Child, Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::{collections::HashMap, error::Error, fs, path::PathBuf};

const PLUGINS_DIR: &str = "target/debug";

#[derive(Debug, Deserialize)]
struct Service {
    name: String,
    plugin: String,
}

#[derive(Debug, Deserialize)]
struct Tier {
    instances: u8,
    replication_factor: u8,
    services: Option<Vec<Service>>,
}

#[derive(Debug, Deserialize)]
struct Topology {
    tiers: HashMap<String, Tier>,
}

lazy_static! {
    static ref PICODATA_PROCESSES: Arc<Mutex<Vec<Child>>> = Arc::new(Mutex::new(Vec::new()));
}

fn enable_plugins(topology: &Topology, data_dir: &Path, picodata_path: &PathBuf) {
    let plugins_dir = Path::new(PLUGINS_DIR);
    let mut plugins: HashMap<String, String> = HashMap::new();
    for tier in topology.tiers.values() {
        let Some(services) = &tier.services else {
            continue;
        };
        for service in services {
            plugins.entry(service.plugin.clone()).or_insert_with(|| {
                let plugin_dir = plugins_dir.join(service.plugin.clone());

                if !plugin_dir.exists() {
                    eprintln!(
                        "{} {} {}",
                        "[-] Directory ".red(),
                        plugin_dir.to_str().unwrap(),
                        "does not exist".red()
                    );
                    shutdown();
                }
                let mut versions: Vec<_> = fs::read_dir(plugin_dir)
                    .unwrap()
                    .map(|r| r.unwrap())
                    .collect();
                versions.sort_by_key(|dir| dir.path());
                versions
                    .last()
                    .unwrap()
                    .file_name()
                    .to_str()
                    .unwrap()
                    .to_string()
            });
        }
    }

    let mut queries: Vec<String> = Vec::new();

    for (plugin, version) in &plugins {
        queries.push(format!(r#"CREATE PLUGIN "{plugin}" {version};"#));
        queries.push(format!(r#"ALTER PLUGIN "{plugin}" MIGRATE TO {version};"#));
        queries.push(format!(r#"ALTER PLUGIN "{plugin}" {version} ENABLE;"#));
    }

    for (tier_name, tier) in &topology.tiers {
        let Some(services) = &tier.services else {
            continue;
        };
        for service in services {
            let plugin_name = &service.plugin;
            let plugin_version = plugins.get(plugin_name).unwrap();
            let service_name = &service.name;
            queries.push(format!(r#"ALTER PLUGIN "{plugin_name}" {plugin_version} ADD SERVICE "{service_name}" TO TIER "{tier_name}";"#));
        }
    }

    let admin_soket = data_dir.join("cluster").join("i_1").join("admin.sock");

    for query in queries {
        let mut picodata_admin = Command::new(picodata_path)
            .arg("admin")
            .arg(admin_soket.to_str().unwrap())
            .stdin(Stdio::piped())
            .spawn()
            .unwrap();

        {
            let picodata_stdin = picodata_admin.stdin.as_mut().unwrap();
            picodata_stdin.write_all(query.as_bytes()).unwrap();
        }
        thread::sleep(Duration::from_secs(3));
    }
}

fn shutdown() {
    let mut processes = PICODATA_PROCESSES.lock().unwrap();
    for mut process in processes.drain(..) {
        let _ = process.kill();
    }
    exit(0);
}

pub fn cmd(
    topology_path: &PathBuf,
    data_dir: &Path,
    is_plugins_instalation_enabled: bool,
    base_http_ports: &i32,
    picodata_path: &PathBuf,
) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all(data_dir).unwrap();

    {
        ctrlc::set_handler(move || {
            println!("{}", "\nReceived Ctrl+C. Shutting down ...".green());

            shutdown();
        })
        .expect(&"Error setting Ctrl+C handler".red());
    }

    let topology: &Topology = &toml::from_str(&fs::read_to_string(topology_path)?)?;

    let first_instance_bin_port = 3001;
    let mut instance_id = 0;
    for (tier_name, tier) in &topology.tiers {
        for _ in 0..(tier.instances * tier.replication_factor) {
            instance_id += 1;
            let bin_port = 3000 + instance_id;
            let http_port = base_http_ports + instance_id;
            let instance_data_dir = data_dir.join("cluster").join(format!("i_{}", instance_id));

            // TODO: make it as child processes with catch output and redirect it to main
            // output
            let process = Command::new(picodata_path)
                .args([
                    "run",
                    "--data-dir",
                    instance_data_dir.to_str().ok_or("Invalid data dir path")?,
                    "--plugin-dir",
                    PLUGINS_DIR,
                    "--listen",
                    &format!("127.0.0.1:{}", bin_port),
                    "--peer",
                    &format!("127.0.0.1:{}", first_instance_bin_port),
                    "--init-replication-factor",
                    &tier.replication_factor.to_string(),
                    "--http-listen",
                    &format!("127.0.0.1:{}", http_port),
                    "--tier",
                    tier_name,
                ])
                .spawn()
                .expect(&"Failed to execute process".red());
            // TODO: parse output and wait next line
            // main/116/governor_loop I> handling instance state change, current_state: Online(1), instance_id: i1
            thread::sleep(Duration::from_secs(5));
            PICODATA_PROCESSES.lock().unwrap().push(process);
        }
    }

    if is_plugins_instalation_enabled {
        enable_plugins(topology, data_dir, picodata_path);
    }

    loop {
        thread::sleep(std::time::Duration::from_millis(100));
    }
}
