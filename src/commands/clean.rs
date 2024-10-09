use std::{fs, path::Path};

pub fn cmd(data_dir: &Path) {
    let _ = fs::remove_dir_all(data_dir);
}
