//! System diagnostics and prerequisite health checking.

use crate::system::{PlatformInfo, execute_safe};
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Categorization of diagnostic checks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DoctorCategory {
    /// Development toolchains (rustc, cargo, compilers).
    Toolchain,
    /// Version control tools (git).
    VersionControl,
    /// Operating system and architecture.
    Platform,
    /// Google Antigravity integration and paths.
    Antigravity,
    /// AEK and workspace configuration.
    Configuration,
    /// Optional runtimes (Node.js, Python).
    Runtime,
}

/// Status of an individual diagnostic check.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum CheckStatus {
    /// Check passed successfully.
    Ok,
    /// Check passed with non-critical advice or warning.
    Warning { message: String },
    /// Check failed; requires remediation.
    Failed {
        reason: String,
        remediation: Option<String>,
    },
}

/// An individual diagnostic check item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DoctorCheck {
    /// Name or title of the check.
    pub name: String,
    /// Functional category.
    pub category: DoctorCategory,
    /// Evaluation outcome.
    pub status: CheckStatus,
    /// Additional context, version strings, or paths.
    pub details: Option<String>,
}

impl DoctorCheck {
    /// Creates a new `DoctorCheck`.
    #[must_use]
    pub fn new(
        name: impl Into<String>,
        category: DoctorCategory,
        status: CheckStatus,
        details: Option<String>,
    ) -> Self {
        Self {
            name: name.into(),
            category,
            status,
            details,
        }
    }
}

/// Aggregate report of all executed diagnostic checks.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DoctorReport {
    /// List of evaluated checks.
    pub checks: Vec<DoctorCheck>,
    /// Host platform information.
    pub platform: PlatformInfo,
}

impl Default for DoctorReport {
    fn default() -> Self {
        Self::new()
    }
}

impl DoctorReport {
    /// Creates a new empty `DoctorReport` with current host platform details.
    #[must_use]
    pub fn new() -> Self {
        Self {
            checks: Vec::new(),
            platform: PlatformInfo::current(),
        }
    }

    /// Adds a check to the report.
    pub fn add_check(&mut self, check: DoctorCheck) {
        self.checks.push(check);
    }

    /// Returns true if no checks have failed.
    #[must_use]
    pub fn is_healthy(&self) -> bool {
        !self
            .checks
            .iter()
            .any(|c| matches!(c.status, CheckStatus::Failed { .. }))
    }

    /// Count of checks that passed with `Ok`.
    #[must_use]
    pub fn ok_count(&self) -> usize {
        self.checks
            .iter()
            .filter(|c| matches!(c.status, CheckStatus::Ok))
            .count()
    }

    /// Count of checks with `Warning`.
    #[must_use]
    pub fn warning_count(&self) -> usize {
        self.checks
            .iter()
            .filter(|c| matches!(c.status, CheckStatus::Warning { .. }))
            .count()
    }

    /// Count of checks with `Failed`.
    #[must_use]
    pub fn failed_count(&self) -> usize {
        self.checks
            .iter()
            .filter(|c| matches!(c.status, CheckStatus::Failed { .. }))
            .count()
    }

    /// Total count of evaluated checks.
    #[must_use]
    pub fn total_count(&self) -> usize {
        self.checks.len()
    }
}

/// Helper for detecting installed command-line tools.
pub struct ToolDetector;

impl ToolDetector {
    /// Checks for the availability of an executable by running a version flag.
    #[must_use]
    pub fn detect_tool(binary: &str, args: &[&str], category: DoctorCategory) -> DoctorCheck {
        match execute_safe(binary, args) {
            Ok(res) if res.success => {
                let first_line = res.stdout.lines().next().unwrap_or("Installed").to_string();
                DoctorCheck::new(binary, category, CheckStatus::Ok, Some(first_line))
            }
            Ok(res) => DoctorCheck::new(
                binary,
                category,
                CheckStatus::Failed {
                    reason: format!(
                        "Command exited with error code {:?}: {}",
                        res.status_code, res.stderr
                    ),
                    remediation: Some(format!(
                        "Verify that '{binary}' is correctly installed and accessible in PATH."
                    )),
                },
                None,
            ),
            Err(_) => DoctorCheck::new(
                binary,
                category,
                CheckStatus::Failed {
                    reason: format!("Executable '{binary}' was not found in PATH."),
                    remediation: Some(format!(
                        "Install '{binary}' and ensure it is on your system PATH."
                    )),
                },
                None,
            ),
        }
    }

    /// Checks for an optional tool, reporting a warning instead of a failure if missing.
    #[must_use]
    pub fn detect_optional_tool(
        binary: &str,
        args: &[&str],
        category: DoctorCategory,
        purpose: &str,
    ) -> DoctorCheck {
        match execute_safe(binary, args) {
            Ok(res) if res.success => {
                let first_line = res.stdout.lines().next().unwrap_or("Installed").to_string();
                DoctorCheck::new(binary, category, CheckStatus::Ok, Some(first_line))
            }
            _ => DoctorCheck::new(
                binary,
                category,
                CheckStatus::Warning {
                    message: format!(
                        "'{binary}' is not installed ({purpose}). Some advanced features may be unavailable."
                    ),
                },
                None,
            ),
        }
    }
}

/// Detector for Antigravity configuration and workspace paths.
pub struct AntigravityDetector;

impl AntigravityDetector {
    /// Inspects global and workspace Antigravity configurations.
    #[must_use]
    pub fn check_environment(workspace_root: Option<&Path>) -> Vec<DoctorCheck> {
        let mut checks = Vec::new();

        // 1. Global Gemini / Antigravity path
        let global_dir = directories::BaseDirs::new()
            .map(|d| d.home_dir().join(".gemini"))
            .unwrap_or_else(|| std::path::PathBuf::from(".gemini"));

        if global_dir.exists() {
            let config_dir = global_dir.join("config");
            let mcp_config = config_dir.join("mcp_config.json");
            let mut details = format!("Global directory found at: {}", global_dir.display());
            if mcp_config.exists() {
                details.push_str(" (mcp_config.json present)");
            }
            checks.push(DoctorCheck::new(
                "Antigravity Global Config",
                DoctorCategory::Antigravity,
                CheckStatus::Ok,
                Some(details),
            ));
        } else {
            checks.push(DoctorCheck::new(
                "Antigravity Global Config",
                DoctorCategory::Antigravity,
                CheckStatus::Warning {
                    message: format!(
                        "Global Antigravity directory '{}' was not found. Global customizations will not be shared.",
                        global_dir.display()
                    ),
                },
                None,
            ));
        }

        // 2. Workspace root customizations
        if let Some(root) = workspace_root {
            let agents_dir = root.join(".agents");
            let agents_md = root.join("AGENTS.md");
            let gemini_md = root.join("GEMINI.md");

            if agents_dir.exists() || agents_md.exists() || gemini_md.exists() {
                let mut found = Vec::new();
                if agents_dir.exists() {
                    found.push(".agents/");
                }
                if agents_md.exists() {
                    found.push("AGENTS.md");
                }
                if gemini_md.exists() {
                    found.push("GEMINI.md");
                }
                checks.push(DoctorCheck::new(
                    "Antigravity Workspace",
                    DoctorCategory::Antigravity,
                    CheckStatus::Ok,
                    Some(format!("Customizations detected: {}", found.join(", "))),
                ));
            } else {
                checks.push(DoctorCheck::new(
                    "Antigravity Workspace",
                    DoctorCategory::Antigravity,
                    CheckStatus::Warning {
                        message:
                            "No workspace customizations (.agents/, AGENTS.md, GEMINI.md) found."
                                .to_string(),
                    },
                    None,
                ));
            }
        }

        checks
    }
}

/// Orchestrates full diagnostic execution.
pub struct DoctorRunner;

impl DoctorRunner {
    /// Executes all standard AEK diagnostic checks.
    #[must_use]
    pub fn run_all(workspace_root: Option<&Path>) -> DoctorReport {
        let mut report = DoctorReport::new();

        // 1. Toolchain
        report.add_check(ToolDetector::detect_tool(
            "rustc",
            &["--version"],
            DoctorCategory::Toolchain,
        ));
        report.add_check(ToolDetector::detect_tool(
            "cargo",
            &["--version"],
            DoctorCategory::Toolchain,
        ));

        // 2. Version Control
        report.add_check(ToolDetector::detect_tool(
            "git",
            &["--version"],
            DoctorCategory::VersionControl,
        ));

        // 3. Platform
        let plat = &report.platform;
        report.add_check(DoctorCheck::new(
            "Operating System & Target",
            DoctorCategory::Platform,
            CheckStatus::Ok,
            Some(format!(
                "{}-{} ({} logical cores)",
                plat.os, plat.arch, plat.logical_cores
            )),
        ));

        // 4. Antigravity Environment
        for check in AntigravityDetector::check_environment(workspace_root) {
            report.add_check(check);
        }

        // 5. Optional Runtimes
        report.add_check(ToolDetector::detect_optional_tool(
            "node",
            &["--version"],
            DoctorCategory::Runtime,
            "required for Promptfoo LLM testing",
        ));
        report.add_check(ToolDetector::detect_optional_tool(
            "python",
            &["--version"],
            DoctorCategory::Runtime,
            "required for DSPy programmatic prompts",
        ));

        report
    }
}
