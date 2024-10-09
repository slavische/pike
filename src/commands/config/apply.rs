use serde::Deserialize;
use serde_yaml::Value;
use std::{
    collections::HashMap,
    fs,
    io::Write,
    path::Path,
    process::{Command, Stdio},
};

fn apply_service_config(
    plugin_name: &str,
    plugin_version: &str,
    service_name: &str,
    config: HashMap<String, Value>,
    admin_socket: &Path,
) {
    let mut queries: Vec<String> = Vec::new();

    for (key, value) in &config {
        let value = serde_json::to_string(&value).unwrap();
        queries.push(format!(
            r#"ALTER PLUGIN "{plugin_name}" {plugin_version} SET {service_name}.{key}='{value}';"#
        ));
    }

    for query in queries {
        let mut picodata_admin = Command::new("picodata")
            .arg("admin")
            .arg(admin_socket.to_str().unwrap())
            .stdout(Stdio::null())
            .stdin(Stdio::piped())
            .spawn()
            .unwrap();

        {
            let picodata_stdin = picodata_admin.stdin.as_mut().unwrap();
            picodata_stdin.write_all(query.as_bytes()).unwrap();
        }
    }
}

#[derive(Debug, Deserialize)]
struct Package {
    name: String,
    version: String,
}

#[derive(Debug, Deserialize)]
struct CargoManifest {
    package: Package,
}

pub fn cmd(config_path: &Path, data_dir: &Path) {
    let admin_socket = data_dir.join("cluster").join("i_1").join("admin.sock");
    let cargo_manifest: &CargoManifest =
        &toml::from_str(&fs::read_to_string("Cargo.toml").unwrap()).unwrap();
    let config: HashMap<String, HashMap<String, Value>> =
        serde_yaml::from_str(&fs::read_to_string(config_path).unwrap()).unwrap();
    for (service_name, service_config) in config {
        apply_service_config(
            &cargo_manifest.package.name,
            &cargo_manifest.package.version,
            &service_name,
            service_config,
            &admin_socket,
        )
    }
}
