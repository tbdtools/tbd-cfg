use crate::ansible::AnsibleWrapper;
use crate::state::StateAdapter;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::path::PathBuf;
use tbd_iac::state::StateManager as IacStateManager;

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigSpec {
    pub name: String,
    pub target: String,
    pub provider: ConfigProvider,
    pub variables: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ConfigProvider {
    Ansible {
        playbook: PathBuf,
        inventory: PathBuf,
    },
    Shell {
        script: PathBuf,
    },
}

pub struct ConfigManager {
    state: StateAdapter,
    ansible_wrapper: AnsibleWrapper,
}

impl ConfigManager {
    pub fn new(state: IacStateManager) -> Self {
        Self {
            state: StateAdapter::new(state),
            ansible_wrapper: AnsibleWrapper::new(),
        }
    }

    pub async fn apply_config(&self, spec: ConfigSpec) -> Result<()> {
        // Create the state representation
        let state_value = json!({
            "name": spec.name,
            "target": spec.target,
            "provider": serde_json::to_value(&spec.provider)?,
            "variables": spec.variables,
        });

        // Record in state before applying
        self.state
            .add_config_state(spec.name.clone(), state_value)
            .await?;

        // Apply the configuration based on provider type
        match &spec.provider {
            ConfigProvider::Ansible {
                playbook,
                inventory,
            } => {
                self.ansible_wrapper
                    .run_playbook(playbook, inventory, spec.variables)
                    .await
            }
            ConfigProvider::Shell { script: _ } => {
                // Shell script execution implementation
                todo!("Implement shell script execution")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_config_manager() -> Result<()> {
        let state = IacStateManager::new();
        let manager = ConfigManager::new(state);

        let spec = ConfigSpec {
            name: "test-config".to_string(),
            target: "dev".to_string(),
            provider: ConfigProvider::Ansible {
                playbook: PathBuf::from("playbook.yml"),
                inventory: PathBuf::from("inventory.yml"),
            },
            variables: json!({
                "test": "value"
            }),
        };

        // We expect this to fail since files don't exist, but it should fail
        // after state management, not during
        let result = manager.apply_config(spec).await;
        assert!(result.is_err());

        Ok(())
    }
}
