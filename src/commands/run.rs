use serde::Deserialize;
use std::path::Path;
use std::process::Command;
use std::thread;
use std::time::Duration;
use std::{collections::HashMap, error::Error, fs, path::PathBuf};

#[derive(Debug, Deserialize)]
struct Tier {
    instances: u8,
    replication_factor: u8,
}

#[derive(Debug, Deserialize)]
struct Topology {
    tiers: HashMap<String, Tier>,
}

pub fn cmd(topology_path: &PathBuf, data_dir: &Path) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all(data_dir).unwrap();

    let topology: &Topology = &toml::from_str(&fs::read_to_string(topology_path)?)?;
    dbg!(topology);

    let first_instance_bin_port = 3001;
    let mut instance_id = 0;
    for (tier_name, tier) in &topology.tiers {
        for _ in 0..(tier.instances * tier.replication_factor) {
            instance_id += 1;
            let bin_port = 3000 + instance_id;
            let http_port = 8000 + instance_id;
            let instance_data_dir = data_dir.join("cluster").join(format!("i_{}", instance_id));

            // TODO: make it as child processes with catch output and redirect it to main
            // output
            Command::new("picodata")
                .args([
                    "run",
                    "--data-dir",
                    instance_data_dir.to_str().ok_or("Invalid data dir path")?,
                    "--plugin-dir",
                    "target/debug",
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
                .expect("failed to execute process");

            // TODO: parse output and wait next line
            // main/116/governor_loop I> handling instance state change, current_state: Online(1), instance_id: i1
            thread::sleep(Duration::from_secs(5));
        }
    }

    // TODO: wait all child processes
    thread::sleep(Duration::from_secs(88888));

    // TODO: stop all child processes if ctrl+c

    Ok(())
}
