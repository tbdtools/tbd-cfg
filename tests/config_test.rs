use anyhow::Result;
use assert_fs::prelude::*;
use std::path::PathBuf;
use tbd_cfg::config::{ConfigProvider, ConfigSpec};

#[test]
fn test_config_parsing() -> Result<()> {
    let temp = assert_fs::TempDir::new()?;

    let config_content = r#"
name = "web-config"
target = "dev"

[provider]
type = "ansible"
playbook = "playbook.yml"
inventory = "inventory.yml"

[variables]
port = 80
domain = "example.com"
"#;

    let config_file = temp.child("config.toml");
    config_file.write_str(config_content)?;

    let content = std::fs::read_to_string(config_file.path())?;
    let config: ConfigSpec = toml::from_str(&content)?;

    assert_eq!(config.name, "web-config");
    assert_eq!(config.target, "dev");

    if let ConfigProvider::Ansible {
        playbook,
        inventory,
    } = config.provider
    {
        assert_eq!(playbook, PathBuf::from("playbook.yml"));
        assert_eq!(inventory, PathBuf::from("inventory.yml"));
    } else {
        panic!("Wrong provider type parsed");
    }

    Ok(())
}
