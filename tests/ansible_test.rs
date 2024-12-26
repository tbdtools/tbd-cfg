use anyhow::Result;
use assert_fs::prelude::*;
use tbd_cfg::ansible::AnsibleWrapper;

#[tokio::test]
async fn test_ansible_validation() -> Result<()> {
    let temp = assert_fs::TempDir::new()?;

    // Create test playbook
    let playbook_content = r#"---
- hosts: localhost
  tasks:
    - name: Echo test
      debug:
        msg: "Test message"
"#;

    let playbook = temp.child("test.yml");
    playbook.write_str(playbook_content)?;

    // Create test inventory
    let inventory_content = "localhost ansible_connection=local";
    let inventory = temp.child("inventory");
    inventory.write_str(inventory_content)?;

    let wrapper = AnsibleWrapper::new();

    // Test playbook validation
    wrapper
        .validate_playbook(playbook.path(), inventory.path())
        .await?;

    Ok(())
}
