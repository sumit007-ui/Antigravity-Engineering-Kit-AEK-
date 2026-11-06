//! GitHub MCP server integration.

use aek_mcp::config::{McpServerConfig, McpTransportConfig, PermissionProfile};
use std::collections::HashMap;

/// Returns standard configuration for GitHub MCP server using npx.
#[must_use]
pub fn github_config() -> McpServerConfig {
    let mut env = HashMap::new();
    env.insert(
        "GITHUB_PERSONAL_ACCESS_TOKEN".to_string(),
        "${GITHUB_TOKEN}".to_string(),
    );

    McpServerConfig {
        id: "github".to_string(),
        name: "GitHub MCP Server".to_string(),
        description: "Access GitHub repositories, issues, pull requests, and code searches"
            .to_string(),
        transport: McpTransportConfig::Stdio {
            command: "npx".to_string(),
            args: vec![
                "-y".to_string(),
                "@modelcontextprotocol/server-github".to_string(),
            ],
            env,
            working_dir: None,
        },
        enabled: true,
        timeout_seconds: 30,
        requires_auth: true,
        auth_env_vars: vec!["GITHUB_TOKEN".to_string()],
        required_dependencies: vec!["npx".to_string(), "node".to_string()],
        permissions: PermissionProfile {
            allow_read_only: true,
            allow_filesystem_write: false,
            allow_network_write: false, // Requires explicit user approval for mutations
            allow_sensitive: false,
            allowed_paths: Vec::new(),
        },
        version: "1.0.0".to_string(),
    }
}
