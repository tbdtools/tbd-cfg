// tbd-cfg/src/cli/import.rs
use crate::config::{ConfigProvider, ConfigSpec};
use anyhow::Result;
use std::path::{Path, PathBuf};

pub async fn handle_import(playbook: &Path, inventory: &Path, name: &str) -> Result<()> {
    // Validate input files exist
    if !playbook.exists() {
        anyhow::bail!("Playbook file does not exist: {}", playbook.display());
    }
    if !inventory.exists() {
        anyhow::bail!("Inventory file does not exist: {}", inventory.display());
    }

    // Create config spec
    let config = ConfigSpec {
        name: name.to_string(),
        target: "dev".to_string(), // Default target
        provider: ConfigProvider::Ansible {
            playbook: playbook.to_path_buf(),
            inventory: inventory.to_path_buf(),
        },
        variables: serde_json::Value::Object(serde_json::Map::new()),
    };

    // Write configuration file
    let output_path = PathBuf::from(format!("{}.toml", name));
    let content = toml::to_string_pretty(&config)?;
    std::fs::write(&output_path, content)?;

    println!("Created configuration file: {}", output_path.display());
    Ok(())
}
