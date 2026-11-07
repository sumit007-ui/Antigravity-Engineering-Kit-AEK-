//! Read-only environment diagnostics engine with PASS, WARN, FAIL, SKIP statuses.

use aek_platform::detection::{detect_arch, detect_os, find_executable_in_path};
use aek_platform::paths::PlatformPaths;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Standard status reported for an environment diagnostic check.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE", tag = "status")]
pub enum DiagnosticStatus {
    /// Check satisfied requirements without issues.
    Pass,
    /// Non-critical advisory or missing optional capability.
    Warn { message: String },
    /// Critical requirement missing or broken.
    Fail {
        reason: String,
        remediation: Option<String>,
    },
    /// Check skipped because parent dependency or platform is not applicable.
    Skip { reason: String },
}

/// An individual diagnostic check item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticCheck {
    /// Identifier or title of check.
    pub name: String,
    /// Logical category (Toolchain, Platform, Runtime, Configuration, Antigravity).
    pub category: String,
    /// Evaluation outcome.
    pub status: DiagnosticStatus,
    /// Contextual details, version strings, or filesystem paths.
    pub details: Option<String>,
}

impl DiagnosticCheck {
    /// Creates a new `DiagnosticCheck`.
    #[must_use]
    pub fn new(
        name: impl Into<String>,
        category: impl Into<String>,
        status: DiagnosticStatus,
        details: Option<String>,
    ) -> Self {
        Self {
            name: name.into(),
            category: category.into(),
            status,
            details,
        }
    }
}

/// Diagnostic report containing all evaluated checks.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DiagnosticReport {
    /// Host operating system.
    pub os: String,
    /// CPU target architecture.
    pub arch: String,
    /// Evaluated diagnostic checks.
    pub checks: Vec<DiagnosticCheck>,
}

impl DiagnosticReport {
    /// Creates a new empty `DiagnosticReport` populated with host OS and architecture.
    #[must_use]
    pub fn new() -> Self {
        Self {
            os: detect_os().to_string(),
            arch: detect_arch(),
            checks: Vec::new(),
        }
    }

    /// Adds a check to the report.
    pub fn add_check(&mut self, check: DiagnosticCheck) {
        self.checks.push(check);
    }

    /// True if no checks have reported a `FAIL` status.
    #[must_use]
    pub fn is_healthy(&self) -> bool {
        !self
            .checks
            .iter()
            .any(|c| matches!(c.status, DiagnosticStatus::Fail { .. }))
    }

    /// Count of checks that passed.
    #[must_use]
    pub fn pass_count(&self) -> usize {
        self.checks
            .iter()
            .filter(|c| matches!(c.status, DiagnosticStatus::Pass))
            .count()
    }

    /// Count of checks with warnings.
    #[must_use]
    pub fn warn_count(&self) -> usize {
        self.checks
            .iter()
            .filter(|c| matches!(c.status, DiagnosticStatus::Warn { .. }))
            .count()
    }

    /// Count of failed checks.
    #[must_use]
    pub fn fail_count(&self) -> usize {
        self.checks
            .iter()
            .filter(|c| matches!(c.status, DiagnosticStatus::Fail { .. }))
            .count()
    }

    /// Count of skipped checks.
    #[must_use]
    pub fn skip_count(&self) -> usize {
        self.checks
            .iter()
            .filter(|c| matches!(c.status, DiagnosticStatus::Skip { .. }))
            .count()
    }

    /// Total checks evaluated.
    #[must_use]
    pub fn total_count(&self) -> usize {
        self.checks.len()
    }
}

/// Diagnostic runner orchestrating all environment checks.
pub struct DiagnosticsRunner;

impl DiagnosticsRunner {
    /// Runs all system, toolchain, runtime, and configuration checks.
    #[must_use]
    pub fn run_all(
        workspace_dir: Option<&Path>,
        config_is_valid: Option<bool>,
    ) -> DiagnosticReport {
        let mut report = DiagnosticReport::new();

        // 1. Platform & OS
        report.add_check(DiagnosticCheck::new(
            "Operating System",
            "Platform",
            DiagnosticStatus::Pass,
            Some(report.os.clone()),
        ));
        report.add_check(DiagnosticCheck::new(
            "CPU Architecture",
            "Platform",
            DiagnosticStatus::Pass,
            Some(report.arch.clone()),
        ));

        // 2. Mandatory Core Toolchains (Rust, Cargo, Git)
        report.add_check(Self::check_tool(
            "rustc",
            &["--version"],
            "Core Toolchain",
            true,
            "Install Rust via rustup (https://rustup.rs)",
        ));
        report.add_check(Self::check_tool(
            "cargo",
            &["--version"],
            "Core Toolchain",
            true,
            "Install Cargo via rustup (https://rustup.rs)",
        ));
        report.add_check(Self::check_tool(
            "git",
            &["--version"],
            "Core Toolchain",
            true,
            "Install Git (https://git-scm.com)",
        ));

        // 3. Optional Runtimes (Node.js, Python)
        report.add_check(Self::check_tool(
            "node",
            &["--version"],
            "Optional Runtimes",
            false,
            "Optional: Install Node.js for Promptfoo evaluation (https://nodejs.org)",
        ));
        report.add_check(Self::check_tool(
            "python",
            &["--version"],
            "Optional Runtimes",
            false,
            "Optional: Install Python 3.10+ for DSPy programmatic prompts (https://python.org)",
        ));

        // 4. AEK Configuration Check
        let paths = PlatformPaths::resolve();
        let config_exists = paths.config_file.exists();
        if config_exists {
            let valid = config_is_valid.unwrap_or(true);
            if valid {
                report.add_check(DiagnosticCheck::new(
                    "AEK Configuration",
                    "Configuration",
                    DiagnosticStatus::Pass,
                    Some(format!("Valid at {}", paths.config_file.display())),
                ));
            } else {
                report.add_check(DiagnosticCheck::new(
                    "AEK Configuration",
                    "Configuration",
                    DiagnosticStatus::Fail {
                        reason: "Configuration file contains syntax or schema errors".to_string(),
                        remediation: Some(
                            "Run `aek config validate` to view and fix errors".to_string(),
                        ),
                    },
                    Some(paths.config_file.display().to_string()),
                ));
            }
        } else {
            report.add_check(DiagnosticCheck::new(
                "AEK Configuration",
                "Configuration",
                DiagnosticStatus::Warn {
                    message: format!(
                        "Configuration file does not exist at '{}'",
                        paths.config_file.display()
                    ),
                },
                Some("Run `aek init` to initialize default configuration".to_string()),
            ));
        }

        // 5. Antigravity Reliable Detection
        report.add_check(Self::check_antigravity(workspace_dir, &paths));

        report
    }

    fn check_tool(
        binary: &str,
        args: &[&str],
        category: &str,
        mandatory: bool,
        remediation_help: &str,
    ) -> DiagnosticCheck {
        if find_executable_in_path(binary).is_none() {
            if mandatory {
                return DiagnosticCheck::new(
                    binary,
                    category,
                    DiagnosticStatus::Fail {
                        reason: format!("Mandatory binary '{binary}' was not found in PATH"),
                        remediation: Some(remediation_help.to_string()),
                    },
                    None,
                );
            }
            return DiagnosticCheck::new(
                binary,
                category,
                DiagnosticStatus::Warn {
                    message: format!("Optional binary '{binary}' was not found in PATH"),
                },
                Some(remediation_help.to_string()),
            );
        }

        // Execute safe version check with bounded timeout
        match crate::system::execute_safe(binary, args) {
            Ok(output) if output.success => {
                let first_line = output
                    .stdout
                    .lines()
                    .next()
                    .unwrap_or("Installed")
                    .trim()
                    .to_string();
                DiagnosticCheck::new(binary, category, DiagnosticStatus::Pass, Some(first_line))
            }
            _ => {
                if mandatory {
                    DiagnosticCheck::new(
                        binary,
                        category,
                        DiagnosticStatus::Fail {
                            reason: format!(
                                "'{binary}' was found in PATH but failed execution check"
                            ),
                            remediation: Some(remediation_help.to_string()),
                        },
                        None,
                    )
                } else {
                    DiagnosticCheck::new(
                        binary,
                        category,
                        DiagnosticStatus::Warn {
                            message: format!("Optional '{binary}' failed execution check"),
                        },
                        None,
                    )
                }
            }
        }
    }

    fn check_antigravity(workspace_dir: Option<&Path>, paths: &PlatformPaths) -> DiagnosticCheck {
        // Reliable check: Do not infer solely from empty directory
        let mut verified_markers = Vec::new();

        // Check if `agy` CLI binary exists in PATH
        if let Some(agy_path) = find_executable_in_path("agy") {
            verified_markers.push(format!("CLI: {}", agy_path.display()));
        }

        // Check global directory for genuine config files
        if let Some(ref dir) = paths.antigravity_dir {
            let mcp_cfg = dir.join("config").join("mcp_config.json");
            let brain_dir = dir.join("brain");
            if mcp_cfg.is_file() {
                verified_markers.push(format!("MCP Config: {}", mcp_cfg.display()));
            } else if brain_dir.is_dir() {
                verified_markers.push(format!("Platform Directory: {}", dir.display()));
            }
        }

        // Check workspace for active customization rules
        if let Some(ws) = workspace_dir {
            let agents_dir = ws.join(".agents");
            let gemini_md = ws.join("GEMINI.md");
            if agents_dir.is_dir() {
                verified_markers.push(format!("Workspace: {}", agents_dir.display()));
            } else if gemini_md.is_file() {
                verified_markers.push(format!("Workspace Rule: {}", gemini_md.display()));
            }
        }

        if !verified_markers.is_empty() {
            DiagnosticCheck::new(
                "Google Antigravity",
                "Environment",
                DiagnosticStatus::Pass,
                Some(verified_markers.join(", ")),
            )
        } else {
            DiagnosticCheck::new(
                "Google Antigravity",
                "Environment",
                DiagnosticStatus::Skip {
                    reason:
                        "No Antigravity installations, MCP servers, or agent rules detected on host"
                            .to_string(),
                },
                None,
            )
        }
    }
}
