// Integration tests for Antigravity MCP adapter export and merging

use aek_agents::adapter::AgentAdapter;
use aek_agents::antigravity::AntigravityAdapter;
use aek_mcp::config::{McpServerConfig, McpTransportConfig, PermissionProfile};
use tempfile::tempdir;

#[test]
fn test_antigravity_mcp_export_and_merge() {
    let dir = tempdir().expect("tempdir");
    let target_mcp_json = dir.path().join("mcp_config.json");

    // Existing Antigravity config with an existing user server
    std::fs::write(
        &target_mcp_json,
        r#"{
  "mcpServers": {
    "my-custom-server": {
      "command": "python",
      "args": ["-m", "custom_mcp"]
    }
  }
}"#,
    )
    .expect("write initial json");

    let adapter = AntigravityAdapter::new();

    let new_servers = vec![
        McpServerConfig {
            id: "context7".to_string(),
            name: "Context7".to_string(),
            description: "Docs".to_string(),
            transport: McpTransportConfig::Http {
                url: "https://mcp.context7.com/mcp".to_string(),
                headers: std::collections::HashMap::new(),
            },
            enabled: true,
            timeout_seconds: 20,
            requires_auth: false,
            auth_env_vars: vec![],
            required_dependencies: vec![],
            permissions: PermissionProfile::default(),
            version: "1.0.0".to_string(),
        },
        McpServerConfig {
            id: "github".to_string(),
            name: "GitHub".to_string(),
            description: "GitHub workflows".to_string(),
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
            required_dependencies: vec![],
            permissions: PermissionProfile::default(),
            version: "1.0.0".to_string(),
        },
    ];

    // Dry-run merge
    let dry_report = adapter
        .merge_mcp_config(&new_servers, Some(&target_mcp_json), true)
        .expect("dry run merge");
    assert!(dry_report.dry_run);
    assert_eq!(dry_report.total_servers, 3); // 1 existing + 2 new
    assert_eq!(dry_report.merged_count, 2);

    // Actual merge
    let act_report = adapter
        .merge_mcp_config(&new_servers, Some(&target_mcp_json), false)
        .expect("actual merge");
    assert!(!act_report.dry_run);
    assert!(act_report.backup_file.is_some());

    // Verify resulting JSON on disk preserves existing server and adds new ones
    let result_content = std::fs::read_to_string(&target_mcp_json).expect("read merged json");
    let parsed: serde_json::Value = serde_json::from_str(&result_content).expect("parse json");
    let mcp_servers = parsed.get("mcpServers").unwrap().as_object().unwrap();

    assert!(mcp_servers.contains_key("my-custom-server"));
    assert!(mcp_servers.contains_key("context7"));
    assert!(mcp_servers.contains_key("github"));

    // Verify format matches Antigravity expectations
    let context7_entry = mcp_servers.get("context7").unwrap();
    assert_eq!(
        context7_entry.get("url").unwrap().as_str().unwrap(),
        "https://mcp.context7.com/mcp"
    );

    let github_entry = mcp_servers.get("github").unwrap();
    assert_eq!(
        github_entry.get("command").unwrap().as_str().unwrap(),
        "npx"
    );
}
