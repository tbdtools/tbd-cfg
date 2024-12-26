// tbd-cfg/src/state/mod.rs
use anyhow::Result;
use serde_json::Value;
use tbd_iac::state::{Resource, StateManager as IacStateManager};

pub struct StateAdapter {
    state: IacStateManager,
}

impl StateAdapter {
    pub fn new(state: IacStateManager) -> Self {
        Self { state }
    }

    pub async fn add_config_state(&self, id: String, state: Value) -> Result<()> {
        let resource = Resource::new(id, "tbd-cfg", state);
        self.state.add_resource(resource).await
    }

    pub async fn get_config(&self, id: &str) -> Option<Value> {
        self.state.get_resource(id).await.map(|r| r.state().clone())
    }

    pub async fn list_configs(&self) -> Vec<String> {
        let resources = self.state.list_resources().await;
        resources
            .into_iter()
            .filter(|r| r.provider() == "tbd-cfg")
            .map(|r| r.id().to_string())
            .collect()
    }

    pub async fn remove_config(&self, id: &str) -> Result<()> {
        self.state.remove_resource(id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn test_state_adapter() -> Result<()> {
        let state = IacStateManager::new();
        let adapter = StateAdapter::new(state);

        // Add a config
        let test_state = json!({
            "name": "test",
            "target": "dev",
            "variables": {
                "key": "value"
            }
        });

        adapter
            .add_config_state("test-config".to_string(), test_state.clone())
            .await?;

        // Verify we can retrieve it
        let retrieved = adapter.get_config("test-config").await;
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap(), test_state);

        // List configs
        let configs = adapter.list_configs().await;
        assert_eq!(configs.len(), 1);
        assert_eq!(configs[0], "test-config");

        // Remove config
        adapter.remove_config("test-config").await?;
        assert!(adapter.get_config("test-config").await.is_none());

        Ok(())
    }
}
