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
        // TODO: add demon flag, if true then set output logs to file and release stdin
    },
    /// Helpers for work with plugins
    Plugin {
        #[command(subcommand)]
        command: Plugin,
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
    },
    /// Create a new Picodata plugin in an existing directory
    Init,
}

fn main() {
    let cli = Cli::parse_from(env::args().skip(1));

    match &cli.command {
        Command::Run { topology, data_dir } => commands::run::cmd(topology, data_dir).unwrap(),
        Command::Plugin { command } => match command {
            Plugin::Pack => commands::pack::cmd(),
            Plugin::New { path } => commands::new::cmd(Some(path)),
            Plugin::Init => commands::new::cmd(None),
        },
    }
}
