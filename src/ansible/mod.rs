use anyhow::{Context, Result};
use serde_json::Value;
use std::path::{Path, PathBuf};
use tokio::fs;
use tokio::process::Command;

pub struct AnsibleWrapper {
    working_dir: PathBuf,
}

impl Default for AnsibleWrapper {
    fn default() -> Self {
        Self::new()
    }
}

impl AnsibleWrapper {
    pub fn new() -> Self {
        Self {
            working_dir: PathBuf::from("/tmp/tbd-cfg"),
        }
    }

    pub async fn run_playbook(&self, playbook: &Path, inventory: &Path, vars: Value) -> Result<()> {
        // Create temporary working directory if it doesn't exist
        fs::create_dir_all(&self.working_dir)
            .await
            .context("Failed to create working directory")?;

        // Write variables to temporary file
        let vars_file = self.working_dir.join("vars.json");
        fs::write(&vars_file, serde_json::to_string_pretty(&vars)?)
            .await
            .context("Failed to write variables file")?;

        // Build ansible-playbook command
        let mut command = Command::new("ansible-playbook");

        command
            .arg("-i")
            .arg(inventory)
            .arg("--extra-vars")
            .arg(format!("@{}", vars_file.display()))
            .arg(playbook);

        // Run the command
        let output = command
            .output()
            .await
            .context("Failed to execute ansible-playbook")?;

        // Check for success and handle output
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let stdout = String::from_utf8_lossy(&output.stdout);

            anyhow::bail!(
                "Ansible playbook failed:\nSTDOUT:\n{}\nSTDERR:\n{}",
                stdout,
                stderr
            );
        }

        // Clean up temporary files
        fs::remove_file(&vars_file)
            .await
            .context("Failed to clean up variables file")?;

        Ok(())
    }

    pub async fn check_ansible_installed() -> Result<()> {
        let output = Command::new("ansible-playbook")
            .arg("--version")
            .output()
            .await
            .context("Failed to check ansible-playbook installation")?;

        if !output.status.success() {
            anyhow::bail!("ansible-playbook is not installed or not accessible");
        }

        Ok(())
    }

    pub async fn validate_playbook(&self, playbook: &Path, inventory: &Path) -> Result<()> {
        let output = Command::new("ansible-playbook")
            .arg("--syntax-check")
            .arg("-i")
            .arg(inventory)
            .arg(playbook)
            .output()
            .await
            .context("Failed to validate playbook")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Playbook validation failed: {}", stderr);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_fs::prelude::*;
    use tokio;

    #[tokio::test]
    async fn test_validate_playbook() {
        let temp = assert_fs::TempDir::new().unwrap();

        // Create a simple test playbook
        let playbook_content = r#"---
- hosts: localhost
  tasks:
    - name: Echo test
      debug:
        msg: "Test message"
"#;
        let playbook = temp.child("test.yml");
        playbook.write_str(playbook_content).unwrap();

        // Create a simple inventory
        let inventory_content = "localhost ansible_connection=local";
        let inventory = temp.child("inventory");
        inventory.write_str(inventory_content).unwrap();

        let wrapper = AnsibleWrapper::new();
        let result = wrapper
            .validate_playbook(playbook.path(), inventory.path())
            .await;

        assert!(result.is_ok());
    }
}
