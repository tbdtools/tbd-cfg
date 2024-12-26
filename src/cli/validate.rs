use crate::ansible::AnsibleWrapper;
use crate::config::{ConfigProvider, ConfigSpec};
use anyhow::Result;
use std::path::Path;

pub async fn handle_validate(config_path: &Path) -> Result<()> {
    // Load and parse configuration
    let content = std::fs::read_to_string(config_path)?;
    let config: ConfigSpec = toml::from_str(&content)?;

    // Validate based on provider type
    match config.provider {
        ConfigProvider::Ansible {
            playbook,
            inventory,
        } => {
            let wrapper = AnsibleWrapper::new();
            wrapper.validate_playbook(&playbook, &inventory).await?;
        }
        ConfigProvider::Shell { script } => {
            // Validate script exists and is executable
            if !script.exists() {
                anyhow::bail!("Script file does not exist: {}", script.display());
            }

            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let metadata = std::fs::metadata(&script)?;
                if metadata.permissions().mode() & 0o111 == 0 {
                    anyhow::bail!("Script file is not executable: {}", script.display());
                }
            }
        }
    }

    Ok(())
}
