use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "task-manager", version, about = "Terminal görev yöneticisi")]
pub struct Cli {
    // path to the JSON data file
    #[arg(long, short, global = true, default_value = "tasks.json")]
    pub file: PathBuf,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Add { title: String },
    List,
    Done { id: u64 },
    Delete { id: u64 },
}
