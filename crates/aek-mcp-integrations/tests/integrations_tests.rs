// Integration tests for standard MCP integrations

use aek_mcp_integrations::standard_integrations_registry;
use std::path::PathBuf;

#[test]
fn test_all_five_standard_integrations_are_valid() {
    let scratch_dir = PathBuf::from("./scratch");
    let registry = standard_integrations_registry(&scratch_dir);

    assert_eq!(registry.list().len(), 5);

    // 1. Context7
    let context7 = registry.get("context7").expect("context7");
    assert!(context7.validate().is_ok());
    assert!(!context7.requires_auth);

    // 2. GitHub
    let github = registry.get("github").expect("github");
    assert!(github.validate().is_ok());
    assert!(github.requires_auth);
    assert_eq!(github.auth_env_vars, vec!["GITHUB_TOKEN"]);

    // 3. Playwright
    let playwright = registry.get("playwright").expect("playwright");
    assert!(playwright.validate().is_ok());
    assert_eq!(playwright.timeout_seconds, 45);

    // 4. Sequential Thinking
    let seq = registry
        .get("sequential-thinking")
        .expect("sequential-thinking");
    assert!(seq.validate().is_ok());

    // 5. Filesystem
    let fs = registry.get("filesystem").expect("filesystem");
    assert!(fs.validate().is_ok());
    assert!(fs.permissions.allow_filesystem_write);
    assert!(!fs.permissions.allow_sensitive);
}
