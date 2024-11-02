use clap::{Parser, Subcommand};
use std::{env, path::PathBuf};

mod commands;

/// A helper utility to work with Picodata plugins.
#[derive(Parser)]
#[command(
    bin_name = "cargo pike",
    version,
    about,
    disable_help_subcommand = true
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Run a Picodata cluster
    Run {
        #[arg(short, long, value_name = "TOPOLOGY", default_value = "topology.toml")]
        topology: PathBuf,
        #[arg(short, long, value_name = "DATA_DIR", default_value = "./tmp")]
        data_dir: PathBuf,
        /// Disable the automatic installation of plugins
        #[arg(long)]
        disable_install_plugins: bool,
        /// Base http port for picodata instances
        #[arg(short, long, default_value = "8000")]
        base_http_ports: i32,
        // TODO: add demon flag, if true then set output logs to file and release stdin
    },
    /// Remove all data files of previous cluster run
    Clean {
        #[arg(short, long, value_name = "DATA_DIR", default_value = "./tmp")]
        data_dir: PathBuf,
    },
    /// Helpers for work with plugins
    Plugin {
        #[command(subcommand)]
        command: Plugin,
    },
    /// Helpers for work with config of services
    Config {
        #[command(subcommand)]
        command: Config,
    },
}

#[derive(Subcommand)]
enum Plugin {
    /// Pack your plugin into a distributable bundle
    Pack,
    /// Create a new Picodata plugin
    New {
        #[arg(value_name = "path")]
        path: PathBuf,
        /// Disable the automatic git initialization
        #[arg(long)]
        without_git: bool,
    },
    /// Create a new Picodata plugin in an existing directory
    Init {
        /// Disable the automatic git initialization
        #[arg(long)]
        without_git: bool,
    },
}

#[derive(Subcommand)]
enum Config {
    /// Apply services config on Picodata cluster started by the Run command
    Apply {
        /// Path to config of the plugin
        #[arg(
            short,
            long,
            value_name = "CONFIG",
            default_value = "plugin_config.yaml"
        )]
        config_path: PathBuf,
        /// Path to data directory of the cluster
        #[arg(short, long, value_name = "DATA_DIR", default_value = "./tmp")]
        data_dir: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse_from(env::args().skip(1));

    match &cli.command {
        Command::Run {
            topology,
            data_dir,
            disable_install_plugins,
            base_http_ports,
        } => commands::run::cmd(
            topology,
            data_dir,
            !disable_install_plugins,
            base_http_ports,
        )
        .unwrap(),
        Command::Clean { data_dir } => commands::clean::cmd(data_dir),
        Command::Plugin { command } => match command {
            Plugin::Pack => commands::plugin::pack::cmd(),
            Plugin::New { path, without_git } => {
                commands::plugin::new::cmd(Some(path), !without_git)
            }
            Plugin::Init { without_git } => commands::plugin::new::cmd(None, !without_git),
        },
        Command::Config { command } => match command {
            Config::Apply {
                config_path,
                data_dir,
            } => commands::config::apply::cmd(config_path, data_dir),
        },
    }
}
