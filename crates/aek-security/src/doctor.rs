//! Security toolchain diagnostics and environment audit.

use aek_core::execution::SafeCommand;
use aek_platform::find_executable_in_path;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Diagnostic check result for an individual security tool.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCheckResult {
    pub name: String,
    pub installed: bool,
    pub path: Option<String>,
    pub version: Option<String>,
    pub required_for: String,
}

/// Aggregated doctor report auditing all security toolchains.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityDoctorReport {
    pub tools: Vec<ToolCheckResult>,
    pub all_installed: bool,
}

/// Runner auditing security scanners and dependency audit tools.
#[derive(Debug, Default, Clone)]
pub struct SecurityDoctor;

impl SecurityDoctor {
    /// Creates a new `SecurityDoctor`.
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Audits all security tools asynchronously.
    pub async fn audit(&self) -> SecurityDoctorReport {
        let mut tools = Vec::new();

        // 1. Semgrep
        tools.push(
            Self::check_tool("semgrep", "--version", "Static code analysis & SAST rules").await,
        );

        // 2. Trivy
        tools.push(
            Self::check_tool(
                "trivy",
                "--version",
                "Filesystem & container vulnerability scanning",
            )
            .await,
        );

        // 3. cargo-audit
        tools.push(
            Self::check_tool(
                "cargo-audit",
                "--version",
                "Rust crate vulnerability auditing",
            )
            .await,
        );

        // 4. govulncheck
        tools.push(
            Self::check_tool(
                "govulncheck",
                "-version",
                "Go module vulnerability auditing",
            )
            .await,
        );

        // 5. pip-audit
        tools.push(
            Self::check_tool(
                "pip-audit",
                "--version",
                "Python package vulnerability auditing",
            )
            .await,
        );

        // 6. npm
        tools.push(
            Self::check_tool(
                "npm",
                "--version",
                "JavaScript / TypeScript dependency audit",
            )
            .await,
        );

        let all_installed = tools.iter().all(|t| t.installed);
        SecurityDoctorReport {
            tools,
            all_installed,
        }
    }

    async fn check_tool(bin_name: &str, ver_flag: &str, required_for: &str) -> ToolCheckResult {
        let exe = find_executable_in_path(bin_name)
            .or_else(|| find_executable_in_path(&format!("{bin_name}.exe")))
            .or_else(|| find_executable_in_path(&format!("{bin_name}.cmd")));

        if let Some(path) = exe {
            let path_str = path.to_string_lossy().to_string();
            let mut cmd = SafeCommand::new(&path_str);
            cmd = cmd.arg(ver_flag);
            cmd = cmd.timeout(Duration::from_secs(5));

            let version = match cmd.execute().await {
                Ok(res) if res.success => {
                    Some(res.stdout.lines().next().unwrap_or("").trim().to_string())
                }
                _ => None,
            };

            ToolCheckResult {
                name: bin_name.to_string(),
                installed: true,
                path: Some(path_str),
                version,
                required_for: required_for.to_string(),
            }
        } else {
            ToolCheckResult {
                name: bin_name.to_string(),
                installed: false,
                path: None,
                version: None,
                required_for: required_for.to_string(),
            }
        }
    }
}
