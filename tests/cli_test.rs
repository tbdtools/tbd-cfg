// tbd-cfg/tests/cli_test.rs
use anyhow::Result;
use assert_fs::prelude::*;
use clap::Parser;
use tbd_cfg::cli::{self, Cli};

#[tokio::test]
async fn test_validate_config() -> Result<()> {
    let temp = assert_fs::TempDir::new()?;

    // Create test playbook and inventory files
    let playbook_content = r#"---
- hosts: localhost
  tasks:
    - name: Echo test
      debug:
        msg: "Test message"
"#;

    let playbook = temp.child("playbook.yml");
    playbook.write_str(playbook_content)?;

    let inventory_content = "localhost ansible_connection=local";
    let inventory = temp.child("inventory.yml");
    inventory.write_str(inventory_content)?;

    // Create test config with proper provider enum structure
    let config_content = format!(
        r#"name = "test-config"
target = "dev"
provider = {{ type = "ansible", playbook = "{}", inventory = "{}" }}
variables = {{ test_var = "test_value" }}"#,
        playbook.path().display(),
        inventory.path().display()
    );

    let config_file = temp.child("test-config.toml");
    config_file.write_str(&config_content)?;

    let args = vec![
        "tbd-cfg",
        "validate",
        "-c",
        config_file.path().to_str().unwrap(),
    ];
    let cli = Cli::parse_from(args);

    match cli.command {
        cli::Commands::Validate { config } => {
            cli::validate::handle_validate(&config).await?;
        }
        _ => panic!("Wrong command parsed"),
    }

    Ok(())
}

#[test]
fn test_cli_parsing() -> Result<()> {
    // Test apply command
    let apply_args = vec![
        "tbd-cfg",
        "apply",
        "-c",
        "config.toml",
        "-t",
        "dev",
        "--vars",
        r#"{"key":"value"}"#,
    ];
    let cli = Cli::parse_from(apply_args);
    match cli.command {
        cli::Commands::Apply {
            config,
            target,
            vars,
        } => {
            assert_eq!(config.to_str().unwrap(), "config.toml");
            assert_eq!(target, "dev");
            assert_eq!(vars.unwrap(), r#"{"key":"value"}"#);
        }
        _ => panic!("Wrong command parsed"),
    }

    // Test validate command
    let validate_args = vec!["tbd-cfg", "validate", "-c", "config.toml"];
    let cli = Cli::parse_from(validate_args);
    match cli.command {
        cli::Commands::Validate { config } => {
            assert_eq!(config.to_str().unwrap(), "config.toml");
        }
        _ => panic!("Wrong command parsed"),
    }

    Ok(())
}
