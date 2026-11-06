//! Controlled Filesystem MCP server integration.

use aek_mcp::config::{McpServerConfig, McpTransportConfig, PermissionProfile};
use std::collections::HashMap;
use std::path::Path;

/// Returns standard configuration for Filesystem MCP server restricted to a specific allowed path.
#[must_use]
pub fn filesystem_config(allowed_directory: impl AsRef<Path>) -> McpServerConfig {
    let dir_buf = allowed_directory.as_ref().to_path_buf();
    let dir_str = dir_buf.to_string_lossy().to_string();

    McpServerConfig {
        id: "filesystem".to_string(),
        name: "Restricted Filesystem MCP".to_string(),
        description:
            "Controlled read and write access strictly limited to designated workspace directories"
                .to_string(),
        transport: McpTransportConfig::Stdio {
            command: "npx".to_string(),
            args: vec![
                "-y".to_string(),
                "@modelcontextprotocol/server-filesystem".to_string(),
                dir_str,
            ],
            env: HashMap::new(),
            working_dir: Some(dir_buf.clone()),
        },
        enabled: true,
        timeout_seconds: 30,
        requires_auth: false,
        auth_env_vars: Vec::new(),
        required_dependencies: vec!["npx".to_string(), "node".to_string()],
        permissions: PermissionProfile {
            allow_read_only: true,
            allow_filesystem_write: true,
            allow_network_write: false,
            allow_sensitive: false,
            allowed_paths: vec![dir_buf],
        },
        version: "1.0.0".to_string(),
    }
}
