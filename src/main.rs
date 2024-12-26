use anyhow::Result;
use clap::Parser;
use tbd_cfg::cli::{Cli, Commands};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Parse command line arguments
    let cli = Cli::parse();

    // Handle commands
    match cli.command {
        Commands::Apply {
            config,
            target,
            vars,
        } => {
            tbd_cfg::cli::apply::handle_apply(&config, &target, vars.as_deref()).await?;
        }
        Commands::Validate { config } => {
            tbd_cfg::cli::validate::handle_validate(&config).await?;
        }
        Commands::Import {
            playbook,
            inventory,
            name,
        } => {
            tbd_cfg::cli::import::handle_import(&playbook, &inventory, &name).await?;
        }
    }

    Ok(())
}
