// tbd-cfg/src/cli/apply.rs
use crate::config::{ConfigManager, ConfigSpec};
use anyhow::Result;
use std::path::Path;
use tbd_iac::state::StateManager;

pub async fn handle_apply(config_path: &Path, target: &str, vars: Option<&str>) -> Result<()> {
    let mut config = load_config(config_path)?;

    // Validate target matches configuration
    if config.target != target {
        anyhow::bail!(
            "Target mismatch: config specifies '{}' but '{}' was requested",
            config.target,
            target
        );
    }

    // Merge extra vars with config vars
    if let Some(obj) = config.variables.as_object_mut() {
        if let Some(extra) = parse_extra_vars(vars)?.as_object() {
            obj.extend(extra.clone());
        }
    }

    let manager = ConfigManager::new(StateManager::new());
    manager.apply_config(config).await?;

    Ok(())
}

fn load_config(path: &Path) -> Result<ConfigSpec> {
    let content = std::fs::read_to_string(path)?;
    let config: ConfigSpec = toml::from_str(&content)?;
    Ok(config)
}

fn parse_extra_vars(vars: Option<&str>) -> Result<serde_json::Value> {
    match vars {
        Some(v) => Ok(serde_json::from_str(v)?),
        None => Ok(serde_json::Value::Object(serde_json::Map::new())),
    }
}
