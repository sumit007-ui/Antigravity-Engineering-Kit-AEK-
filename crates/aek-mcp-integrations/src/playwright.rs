//! Microsoft Playwright browser automation MCP server integration.

use aek_mcp::config::{McpServerConfig, McpTransportConfig, PermissionProfile};
use std::collections::HashMap;

/// Returns standard configuration for Playwright browser automation MCP server.
#[must_use]
pub fn playwright_config() -> McpServerConfig {
    McpServerConfig {
        id: "playwright".to_string(),
        name: "Playwright Browser Automation".to_string(),
        description: "Automate browser interactions, take screenshots, and inspect DOM structures"
            .to_string(),
        transport: McpTransportConfig::Stdio {
            command: "npx".to_string(),
            args: vec![
                "-y".to_string(),
                "@playwright/mcp@latest".to_string(),
                "--headless".to_string(),
            ],
            env: HashMap::new(),
            working_dir: None,
        },
        enabled: true,
        timeout_seconds: 45,
        requires_auth: false,
        auth_env_vars: Vec::new(),
        required_dependencies: vec!["npx".to_string(), "node".to_string()],
        permissions: PermissionProfile {
            allow_read_only: true,
            allow_filesystem_write: false,
            allow_network_write: false,
            allow_sensitive: false,
            allowed_paths: Vec::new(),
        },
        version: "1.0.0".to_string(),
    }
}
