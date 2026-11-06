//! Server configuration models and validation.

use crate::errors::McpError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Transport specification for connecting to an MCP server.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum McpTransportConfig {
    /// Local subprocess communicating over stdin/stdout.
    Stdio {
        command: String,
        #[serde(default)]
        args: Vec<String>,
        #[serde(default)]
        env: HashMap<String, String>,
        #[serde(default)]
        working_dir: Option<PathBuf>,
    },
    /// Streamable HTTP transport (e.g. SSE / JSON-RPC).
    Http {
        url: String,
        #[serde(default)]
        headers: HashMap<String, String>,
    },
}

/// Permission policy governing an MCP server's execution boundaries.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PermissionProfile {
    #[serde(default = "default_true")]
    pub allow_read_only: bool,
    #[serde(default)]
    pub allow_filesystem_write: bool,
    #[serde(default)]
    pub allow_network_write: bool,
    #[serde(default)]
    pub allow_sensitive: bool,
    #[serde(default)]
    pub allowed_paths: Vec<PathBuf>,
}

fn default_true() -> bool {
    true
}

impl Default for PermissionProfile {
    fn default() -> Self {
        Self {
            allow_read_only: true,
            allow_filesystem_write: false,
            allow_network_write: false,
            allow_sensitive: false,
            allowed_paths: Vec::new(),
        }
    }
}

/// Typed configuration definition for a registered MCP server.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct McpServerConfig {
    /// Unique lowercase identifier (e.g. `github`, `context7`).
    pub id: String,
    /// Human-readable display name.
    pub name: String,
    /// Detailed description of provided capabilities.
    pub description: String,
    /// Transport mechanism (stdio or http).
    pub transport: McpTransportConfig,
    /// Whether the server is active.
    #[serde(default = "default_true")]
    pub enabled: bool,
    /// Default timeout for protocol requests in seconds.
    #[serde(default = "default_timeout")]
    pub timeout_seconds: u64,
    /// Whether server requires authentication credentials.
    #[serde(default)]
    pub requires_auth: bool,
    /// Names of environment variables expected for authentication.
    #[serde(default)]
    pub auth_env_vars: Vec<String>,
    /// Toolchain or binary dependencies required to launch (e.g. `["node", "npx"]`).
    #[serde(default)]
    pub required_dependencies: Vec<String>,
    /// Configured permissions boundary.
    #[serde(default)]
    pub permissions: PermissionProfile,
    /// Server bundle or config version.
    #[serde(default = "default_version")]
    pub version: String,
}

fn default_timeout() -> u64 {
    30
}

fn default_version() -> String {
    "1.0.0".to_string()
}

impl McpServerConfig {
    /// Validates the configuration against security and formatting constraints.
    pub fn validate(&self) -> Result<(), McpError> {
        // 1. Identifier validation
        if self.id.is_empty()
            || !self
                .id
                .chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
        {
            return Err(McpError::InvalidConfiguration(
                self.id.clone(),
                "Server ID must be non-empty, lowercase alphanumeric with hyphens".to_string(),
            ));
        }

        // 2. Transport validation
        match &self.transport {
            McpTransportConfig::Stdio { command, args, .. } => {
                if command.trim().is_empty() {
                    return Err(McpError::InvalidConfiguration(
                        self.id.clone(),
                        "Stdio command must not be empty".to_string(),
                    ));
                }
                for arg in args {
                    if arg.contains("&&") || arg.contains("||") || arg.contains(';') {
                        return Err(McpError::InvalidConfiguration(
                            self.id.clone(),
                            format!("Illegal shell punctuation in argument '{arg}'"),
                        ));
                    }
                }
            }
            McpTransportConfig::Http { url, .. } => {
                if !url.starts_with("http://") && !url.starts_with("https://") {
                    return Err(McpError::InvalidConfiguration(
                        self.id.clone(),
                        "HTTP transport URL must begin with http:// or https://".to_string(),
                    ));
                }
            }
        }

        // 3. Timeout validation
        if self.timeout_seconds == 0 {
            return Err(McpError::InvalidConfiguration(
                self.id.clone(),
                "Timeout seconds must be greater than 0".to_string(),
            ));
        }

        Ok(())
    }
}

