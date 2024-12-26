pub mod apply;
pub mod import;
pub mod validate;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Apply configuration to target systems
    Apply {
        /// Path to configuration file
        #[arg(short, long)]
        config: PathBuf,

        /// Target environment (dev, staging, prod)
        #[arg(short, long)]
        target: String,

        /// Additional variables in JSON format
        #[arg(short, long)]
        vars: Option<String>,
    },

    /// Validate configuration
    Validate {
        /// Path to configuration file
        #[arg(short, long)]
        config: PathBuf,
    },

    /// Import existing Ansible playbooks
    Import {
        /// Path to Ansible playbook
        #[arg(short, long)]
        playbook: PathBuf,

        /// Path to inventory file
        #[arg(short, long)]
        inventory: PathBuf,

        /// Output configuration name
        #[arg(short, long)]
        name: String,
    },
}
