use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about)]
pub struct Cli {
    /// Command to run
    #[command(subcommand)]
    pub command: Command,

    /// Output directory
    #[arg(short, long)]
    pub output: Option<PathBuf>,
}

#[derive(Subcommand)]
pub enum Command {
    /// Download issues
    Download { issues: String },
}
