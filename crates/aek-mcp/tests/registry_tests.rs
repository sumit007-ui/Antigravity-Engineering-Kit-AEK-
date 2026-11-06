// Registry and configuration tests for AEK MCP

use aek_mcp::config::{McpServerConfig, McpTransportConfig, PermissionProfile};
use aek_mcp::registry::McpServerRegistry;
use tempfile::tempdir;

#[test]
fn test_mcp_config_serialization_and_validation() {
    let config = McpServerConfig {
        id: "github".to_string(),
        name: "GitHub MCP Server".to_string(),
        description: "GitHub workflows and repository tools".to_string(),
        transport: McpTransportConfig::Stdio {
            command: "npx".to_string(),
            args: vec![
                "-y".to_string(),
                "@modelcontextprotocol/server-github".to_string(),
            ],
            env: std::collections::HashMap::from([(
                "GITHUB_PERSONAL_ACCESS_TOKEN".to_string(),
                "${GITHUB_TOKEN}".to_string(),
            )]),
            working_dir: None,
        },
        enabled: true,
        timeout_seconds: 30,
        requires_auth: true,
        auth_env_vars: vec!["GITHUB_TOKEN".to_string()],
        required_dependencies: vec!["npx".to_string()],
        permissions: PermissionProfile::default(),
        version: "1.0.0".to_string(),
    };

    assert!(config.validate().is_ok());

    let toml_str = toml::to_string(&config).expect("serialize to toml");
    let deserialized: McpServerConfig = toml::from_str(&toml_str).expect("deserialize from toml");
    assert_eq!(deserialized.id, "github");
    assert_eq!(deserialized.name, "GitHub MCP Server");
}

#[test]
fn test_registry_registration_and_persistence() {
    let dir = tempdir().expect("tempdir");
    let registry_file = dir.path().join("registry.toml");

    let mut registry = McpServerRegistry::new();
    let config = McpServerConfig {
        id: "context7".to_string(),
        name: "Context7".to_string(),
        description: "Documentation indexing".to_string(),
        transport: McpTransportConfig::Http {
            url: "https://mcp.context7.com/mcp".to_string(),
            headers: std::collections::HashMap::new(),
        },
        enabled: true,
        timeout_seconds: 15,
        requires_auth: false,
        auth_env_vars: vec![],
        required_dependencies: vec![],
        permissions: PermissionProfile::default(),
        version: "1.0.0".to_string(),
    };

    assert!(registry.register(config.clone()).is_ok());
    assert_eq!(registry.list().len(), 1);

    // Save and reload
    assert!(registry.save_to_file(&registry_file).is_ok());

    let reloaded = McpServerRegistry::load_from_file(&registry_file).expect("load registry");
    assert_eq!(reloaded.list().len(), 1);
    assert_eq!(reloaded.get("context7").unwrap().id, "context7");
}
