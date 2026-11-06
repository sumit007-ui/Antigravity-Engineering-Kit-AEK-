//! Context7 documentation search integration.

use aek_mcp::config::{McpServerConfig, McpTransportConfig, PermissionProfile};

/// Returns the standard default configuration for Upstash Context7 documentation MCP server.
#[must_use]
pub fn context7_config() -> McpServerConfig {
    McpServerConfig {
        id: "context7".to_string(),
        name: "Context7 Library Documentation".to_string(),
        description: "Fetch up-to-date documentation for open-source frameworks and libraries"
            .to_string(),
        transport: McpTransportConfig::Http {
            url: "https://mcp.context7.com/mcp".to_string(),
            headers: std::collections::HashMap::new(),
        },
        enabled: true,
        timeout_seconds: 20,
        requires_auth: false,
        auth_env_vars: Vec::new(),
        required_dependencies: Vec::new(),
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
