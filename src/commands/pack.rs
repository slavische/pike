use flate2::write::GzEncoder;
use flate2::Compression;
use serde::Deserialize;
use std::fs::File;
use std::path::Path;
use std::{env, fs};
use tar::Builder;

#[derive(Deserialize)]
struct PackageInfo {
    name: String,
    version: String,
}

#[derive(Deserialize)]
struct CargoManifest {
    package: PackageInfo,
}

#[cfg(target_os = "linux")]
const LIB_EXT: &str = "so";

#[cfg(target_os = "macos")]
const LIB_EXT: &str = "dylib";

pub fn cmd() {
    let root_dir = env::current_dir().unwrap();
    let release_dir = Path::new(&root_dir).join("target").join("release");

    if !release_dir.exists() {
        panic!("Please run `cargo build --release` first")
    }

    let cargo_manifest: CargoManifest =
        toml::from_str(&fs::read_to_string(root_dir.join("Cargo.toml")).unwrap()).unwrap();

    let normalized_package_name = cargo_manifest.package.name.replace("-", "_");

    let compressed_file = File::create(format!(
        "target/{}-{}.tar.gz",
        &normalized_package_name, cargo_manifest.package.version
    ))
    .unwrap();
    let mut encoder = GzEncoder::new(compressed_file, Compression::best());

    {
        let mut tarball = Builder::new(&mut encoder);

        let lib_name = format!("lib{}.{}", normalized_package_name, LIB_EXT);
        let mut lib_file = File::open(release_dir.join(&lib_name)).unwrap();
        tarball.append_file(lib_name, &mut lib_file).unwrap();

        let mut manifest_file = File::open(release_dir.join("manifest.yaml")).unwrap();
        tarball
            .append_file("manifest.yaml", &mut manifest_file)
            .unwrap();

        tarball
            .append_dir_all("migrations", release_dir.join("migrations"))
            .unwrap();
    }

    encoder.finish().unwrap();
}
