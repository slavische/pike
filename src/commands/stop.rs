use log::info;
use std::fs::{self};
use std::io::{self, BufRead};
use std::path::Path;
use anyhow::{bail, Context, Result};  
use std::process::Command;

pub fn cmd(data_dir: &Path) -> Result<()> {
    let instances_path = data_dir.join("cluster");
    let dirs = fs::read_dir(instances_path)?;

    info!(
        "Stopping picodata cluster, data folder: {}",
        data_dir.to_str().unwrap()
    );

    // Iterate through instance folders and
    // search for "pid" file. After the pid
    // is known - kill the instance
    for current_dir in dirs {
        let instance_dir = current_dir?.path();

        if !instance_dir.is_dir() {
            bail!("{} is not a directory",
                instance_dir.to_str().unwrap_or("<undefined>"));
        }
        let Some(folder_name) = instance_dir.file_name() else {
            continue;
        };

        if !folder_name.to_str().unwrap().starts_with("i_") {
            continue;
        }

        let pid_file_path = instance_dir.join("pid");
        if !pid_file_path.exists() {
            bail!("PID file does not exist in folder: {}",
                instance_dir.display());
        }

        let pid = read_pid_from_file(&pid_file_path).context("couldn't read the PID file")?;
        
        kill_process_by_pid(pid).context(format!("error killing proccess with PID {}: ", pid))?;
    }

    Ok(())
}

fn read_pid_from_file(pid_file_path: &Path) -> Result<u32> {
    let file = fs::File::open(pid_file_path)?;

    let mut lines = io::BufReader::new(file).lines();
    let pid_line = lines.next().context("PID file is empty")??;

    let pid = pid_line.trim().parse::<u32>().context(
        format!("error parsing PID from file {}",
                    pid_file_path.display())
    )?;

    Ok(pid)
}

fn kill_process_by_pid(pid: u32) -> Result<()> {
    let status = Command::new("kill")
        .args(["-9", &pid.to_string()])
        .status()?;

    if !status.success() {  
        bail!("failed to kill process with PID: {}", pid)
    } 
    
    info!("successfully killed process with PID: {}", pid);

    Ok(())
}
