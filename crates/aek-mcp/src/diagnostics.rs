//! Diagnostics and health check runner for MCP servers.

use crate::config::{McpServerConfig, McpTransportConfig};
use aek_platform::detection::find_executable_in_path;
use serde::{Deserialize, Serialize};

/// Outcome status for an MCP diagnostics check.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum McpCheckStatus {
    Pass,
    Warn,
    Fail,
}

impl std::fmt::Display for McpCheckStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pass => write!(f, "PASS"),
            Self::Warn => write!(f, "WARN"),
            Self::Fail => write!(f, "FAIL"),
        }
    }
}

/// An individual diagnostics check report item.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct McpCheckItem {
    pub name: String,
    pub status: McpCheckStatus,
    pub message: String,
}

/// Complete diagnostic report for an MCP server.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServerDiagnosticReport {
    pub server_id: String,
    pub status: McpCheckStatus,
    pub checks: Vec<McpCheckItem>,
}

/// Diagnostics engine for auditing MCP server readiness and environment health.
pub struct McpDoctor;

impl McpDoctor {
    /// Audits an MCP server configuration against system toolchains and credentials.
    #[must_use]
    pub fn audit_server(config: &McpServerConfig) -> ServerDiagnosticReport {
        let mut checks = Vec::new();

        // 1. Configuration validity check
        match config.validate() {
            Ok(()) => checks.push(McpCheckItem {
                name: "Configuration Schema".to_string(),
                status: McpCheckStatus::Pass,
                message: "Valid server configuration".to_string(),
            }),
            Err(e) => checks.push(McpCheckItem {
                name: "Configuration Schema".to_string(),
                status: McpCheckStatus::Fail,
                message: e.to_string(),
            }),
        }

        // 2. Transport & Executable check
        match &config.transport {
            McpTransportConfig::Stdio { command, .. } => {
                let resolved = find_executable_in_path(command);
                if resolved.is_some() {
                    checks.push(McpCheckItem {
                        name: "Executable Presence".to_string(),
                        status: McpCheckStatus::Pass,
                        message: format!("Command '{command}' found on system PATH"),
                    });
                } else {
                    checks.push(McpCheckItem {
                        name: "Executable Presence".to_string(),
                        status: McpCheckStatus::Fail,
                        message: format!(
                            "Command '{command}' not found on PATH. Server cannot launch."
                        ),
                    });
                }
            }
            McpTransportConfig::Http { url, .. } => {
                let is_secure = url.starts_with("https://");
                checks.push(McpCheckItem {
                    name: "HTTP Transport".to_string(),
                    status: if is_secure {
                        McpCheckStatus::Pass
                    } else {
                        McpCheckStatus::Warn
                    },
                    message: if is_secure {
                        format!("Secure endpoint: {url}")
                    } else {
                        format!("Insecure HTTP endpoint: {url}")
                    },
                });
            }
        }

        // 3. Required toolchain dependencies check
        for dep in &config.required_dependencies {
            if find_executable_in_path(dep).is_some() {
                checks.push(McpCheckItem {
                    name: format!("Dependency: {dep}"),
                    status: McpCheckStatus::Pass,
                    message: format!("Dependency '{dep}' available"),
                });
            } else {
                checks.push(McpCheckItem {
                    name: format!("Dependency: {dep}"),
                    status: McpCheckStatus::Warn,
                    message: format!("Dependency '{dep}' not found on PATH"),
                });
            }
        }

        // 4. Authentication credentials check
        if config.requires_auth {
            let mut missing_vars = Vec::new();
            for var in &config.auth_env_vars {
                if std::env::var(var).is_err() {
                    missing_vars.push(var.clone());
                }
            }

            if missing_vars.is_empty() {
                checks.push(McpCheckItem {
                    name: "Authentication Credentials".to_string(),
                    status: McpCheckStatus::Pass,
                    message: "Required authentication environment variables are set".to_string(),
                });
            } else {
                checks.push(McpCheckItem {
                    name: "Authentication Credentials".to_string(),
                    status: McpCheckStatus::Warn,
                    message: format!(
                        "Missing auth environment variables: {}",
                        missing_vars.join(", ")
                    ),
                });
            }
        }

        // Compute overall server status
        let overall_status = if checks.iter().any(|c| c.status == McpCheckStatus::Fail) {
            McpCheckStatus::Fail
        } else if checks.iter().any(|c| c.status == McpCheckStatus::Warn) {
            McpCheckStatus::Warn
        } else {
            McpCheckStatus::Pass
        };

        ServerDiagnosticReport {
            server_id: config.id.clone(),
            status: overall_status,
            checks,
        }
    }
}
