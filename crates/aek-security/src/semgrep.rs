//! Semgrep static analysis security scanner integration.

use crate::errors::SecurityError;
use crate::findings::{ScannerKind, SecurityFinding, Severity};
use aek_core::execution::SafeCommand;
use aek_platform::find_executable_in_path;
use serde_json::Value;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::Duration;
use tracing::{info, warn};

/// Adapter executing Semgrep scans and parsing findings.
#[derive(Debug, Default, Clone)]
pub struct SemgrepAdapter;

impl SemgrepAdapter {
    /// Creates a new `SemgrepAdapter`.
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Detects the Semgrep CLI executable on system PATH.
    #[must_use]
    pub fn detect(&self) -> Option<PathBuf> {
        find_executable_in_path("semgrep")
            .or_else(|| find_executable_in_path("semgrep.exe"))
            .or_else(|| find_executable_in_path("semgrep.cmd"))
    }

    /// Parses Semgrep's JSON output into normalized `SecurityFinding`s.
    pub fn parse_output(&self, json_str: &str) -> Result<Vec<SecurityFinding>, SecurityError> {
        let val: Value = serde_json::from_str(json_str)
            .map_err(|e| SecurityError::OutputParse(format!("Invalid Semgrep JSON: {e}")))?;

        let results = val["results"].as_array().ok_or_else(|| {
            SecurityError::OutputParse("Missing 'results' array in Semgrep JSON".to_string())
        })?;

        let mut findings = Vec::new();
        for (idx, item) in results.iter().enumerate() {
            let rule_id = item["check_id"]
                .as_str()
                .unwrap_or("unknown-rule")
                .to_string();
            let path_str = item["path"].as_str().unwrap_or("").to_string();
            let line = item["start"]["line"].as_u64().map(|l| l as usize);
            let message = item["extra"]["message"].as_str().unwrap_or("").to_string();
            let raw_severity = item["extra"]["severity"].as_str().unwrap_or("INFO");
            let severity = Severity::from_str_loose(raw_severity);

            let mut metadata = HashMap::new();
            if let Some(extra_map) = item["extra"]["metadata"].as_object() {
                for (k, v) in extra_map {
                    metadata.insert(k.clone(), v.clone());
                }
            }

            findings.push(SecurityFinding {
                id: format!("semgrep-{idx}-{rule_id}"),
                scanner: ScannerKind::Semgrep,
                rule_id,
                severity,
                category: "static-analysis".to_string(),
                description: message,
                file_path: if path_str.is_empty() {
                    None
                } else {
                    Some(PathBuf::from(path_str))
                },
                line_number: line,
                dependency: None,
                advisory_id: None,
                remediation: None,
                confidence: None,
                metadata,
            });
        }

        Ok(findings)
    }

    /// Executes Semgrep scan against target directory.
    pub async fn scan(
        &self,
        target_dir: &Path,
        config: Option<&str>,
        timeout_secs: u64,
    ) -> Result<Vec<SecurityFinding>, SecurityError> {
        let bin = match self.detect() {
            Some(b) => b,
            None => {
                info!("Semgrep not installed on host system; returning empty scan results");
                return Ok(Vec::new());
            }
        };

        let mut cmd = SafeCommand::new(bin.to_string_lossy());
        cmd = cmd.arg("scan");
        cmd = cmd.arg("--json");
        cmd = cmd.arg("--quiet");
        cmd = cmd.arg("--no-git-ignore");

        if let Some(cfg) = config {
            cmd = cmd.arg(format!("--config={cfg}"));
        } else {
            cmd = cmd.arg("--config=auto");
        }

        cmd = cmd.current_dir(target_dir);
        cmd = cmd.timeout(Duration::from_secs(timeout_secs));

        let res = cmd
            .execute()
            .await
            .map_err(|e| SecurityError::Execution(e.to_string()))?;

        if !res.stdout.trim().is_empty() {
            self.parse_output(&res.stdout)
        } else if !res.success {
            warn!("Semgrep scan failed with stderr: {}", res.stderr);
            Err(SecurityError::Execution(res.stderr))
        } else {
            Ok(Vec::new())
        }
    }
}
