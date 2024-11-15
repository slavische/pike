use colored::*;
use std::{fs, path::Path};

pub fn cmd(data_dir: &Path) {
    println!("{}", "Clearing cluster data directory".green());
    let _ = fs::remove_dir_all(data_dir);
}
