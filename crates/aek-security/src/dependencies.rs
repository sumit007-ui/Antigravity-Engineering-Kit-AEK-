//! Ecosystem-specific dependency vulnerability auditing.

use crate::errors::SecurityError;
use crate::findings::{ScannerKind, SecurityFinding, Severity};
use aek_core::execution::SafeCommand;
use aek_platform::find_executable_in_path;
use serde_json::Value;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::Duration;
use tracing::info;

/// Auditor executing dependency vulnerability checks across lockfiles.
#[derive(Debug, Default, Clone)]
pub struct DependencyAuditor;

impl DependencyAuditor {
    /// Creates a new `DependencyAuditor`.
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Verifies that target directory is safe from path traversal.
    pub fn validate_path(&self, path: &Path) -> Result<(), SecurityError> {
        let path_str = path.to_string_lossy();
        if path_str.contains("../") || path_str.contains("..\\") {
            return Err(SecurityError::PathTraversal(path.to_path_buf()));
        }
        Ok(())
    }

    /// Parses output from `cargo audit --json`.
    pub fn parse_cargo_audit(&self, json_str: &str) -> Result<Vec<SecurityFinding>, SecurityError> {
        let val: Value = serde_json::from_str(json_str)
            .map_err(|e| SecurityError::OutputParse(format!("Invalid cargo-audit JSON: {e}")))?;

        let mut findings = Vec::new();
        if let Some(list) = val["vulnerabilities"]["list"].as_array() {
            for (idx, item) in list.iter().enumerate() {
                let advisory_id = item["advisory"]["id"]
                    .as_str()
                    .unwrap_or("RUSTSEC-UNKNOWN")
                    .to_string();
                let pkg_name = item["package"]["name"].as_str().unwrap_or("").to_string();
                let pkg_ver = item["package"]["version"]
                    .as_str()
                    .unwrap_or("")
                    .to_string();
                let title = item["advisory"]["title"].as_str().unwrap_or("").to_string();
                let url = item["advisory"]["url"].as_str().map(|u| u.to_string());
                let raw_sev = item["advisory"]["severity"].as_str().unwrap_or("HIGH");
                let severity = Severity::from_str_loose(raw_sev);

                let mut metadata = HashMap::new();
                if !pkg_ver.is_empty() {
                    metadata.insert("version".to_string(), Value::String(pkg_ver));
                }
                if let Some(ref u) = url {
                    metadata.insert("url".to_string(), Value::String(u.clone()));
                }

                findings.push(SecurityFinding {
                    id: format!("cargo-audit-{idx}-{advisory_id}"),
                    scanner: ScannerKind::CargoAudit,
                    rule_id: advisory_id.clone(),
                    severity,
                    category: "dependency-vulnerability".to_string(),
                    description: title,
                    file_path: Some(PathBuf::from("Cargo.lock")),
                    line_number: None,
                    dependency: if pkg_name.is_empty() {
                        None
                    } else {
                        Some(pkg_name)
                    },
                    advisory_id: Some(advisory_id),
                    remediation: url,
                    confidence: None,
                    metadata,
                });
            }
        }

        Ok(findings)
    }

    /// Audits a project directory for dependency vulnerabilities across supported ecosystems.
    pub async fn audit(
        &self,
        target_dir: &Path,
        timeout_secs: u64,
    ) -> Result<Vec<SecurityFinding>, SecurityError> {
        self.validate_path(target_dir)?;
        let mut findings = Vec::new();

        // 1. Rust Ecosystem (Cargo.lock)
        let cargo_lock = target_dir.join("Cargo.lock");
        if cargo_lock.is_file() {
            if let Some(cargo_bin) = find_executable_in_path("cargo") {
                let mut cmd = SafeCommand::new(cargo_bin.to_string_lossy());
                cmd = cmd.arg("audit").arg("--json");
                cmd = cmd.current_dir(target_dir);
                cmd = cmd.timeout(Duration::from_secs(timeout_secs));

                match cmd.execute().await {
                    Ok(res) if !res.stdout.trim().is_empty() => {
                        if let Ok(mut parsed) = self.parse_cargo_audit(&res.stdout) {
                            findings.append(&mut parsed);
                        }
                    }
                    Ok(_) => {}
                    Err(err) => {
                        info!("cargo-audit not installed or execution skipped: {err}");
                    }
                }
            }
        }

        Ok(findings)
    }

    /// Synchronous boundary validation for testing and paths.
    pub fn audit_directory(&self, target_dir: &Path) -> Result<(), SecurityError> {
        self.validate_path(target_dir)
    }
}
