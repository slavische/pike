use anyhow::{Context, Result};
use log::info;
use std::{fs, path::Path};

pub fn cmd(data_dir: &Path) -> Result<()> {
    info!("Clearing cluster data directory");
    fs::remove_dir_all(data_dir)
        .context(format!("failed to remove directory {}", data_dir.display()))?;

    Ok(())
}
