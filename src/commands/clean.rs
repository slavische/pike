use log::info;
use std::{fs, path::Path};

pub fn cmd(data_dir: &Path) {
    info!("{}", "Clearing cluster data directory");
    let _ = fs::remove_dir_all(data_dir);
}
