//! Trivy vulnerability and misconfiguration scanner integration.

use crate::errors::SecurityError;
use crate::findings::{ScannerKind, SecurityFinding, Severity};
use aek_core::execution::SafeCommand;
use aek_platform::find_executable_in_path;
use serde_json::Value;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::Duration;
use tracing::warn;

/// Adapter executing Trivy filesystem and dependency vulnerability scans.
#[derive(Debug, Default, Clone)]
pub struct TrivyAdapter;

impl TrivyAdapter {
    /// Creates a new `TrivyAdapter`.
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Detects the Trivy CLI executable on system PATH.
    #[must_use]
    pub fn detect(&self) -> Option<PathBuf> {
        find_executable_in_path("trivy")
            .or_else(|| find_executable_in_path("trivy.exe"))
            .or_else(|| find_executable_in_path("trivy.cmd"))
    }

    /// Parses Trivy's JSON report output into normalized `SecurityFinding`s.
    pub fn parse_output(&self, json_str: &str) -> Result<Vec<SecurityFinding>, SecurityError> {
        let val: Value = serde_json::from_str(json_str)
            .map_err(|e| SecurityError::OutputParse(format!("Invalid Trivy JSON: {e}")))?;

        let mut findings = Vec::new();
        let empty_vec = Vec::new();
        let results = val["Results"].as_array().unwrap_or(&empty_vec);

        for res in results {
            let target = res["Target"].as_str().unwrap_or("").to_string();

            // 1. Process Vulnerabilities
            if let Some(vulns) = res["Vulnerabilities"].as_array() {
                for vuln in vulns {
                    let vuln_id = vuln["VulnerabilityID"]
                        .as_str()
                        .unwrap_or("UNKNOWN-VULN")
                        .to_string();
                    let pkg_name = vuln["PkgName"].as_str().unwrap_or("").to_string();
                    let installed_ver = vuln["InstalledVersion"].as_str().unwrap_or("").to_string();
                    let fixed_ver = vuln["FixedVersion"].as_str().unwrap_or("").to_string();
                    let title = vuln["Title"].as_str().unwrap_or(&vuln_id).to_string();
                    let desc = vuln["Description"].as_str().unwrap_or(&title).to_string();
                    let primary_url = vuln["PrimaryURL"].as_str().map(|u| u.to_string());
                    let raw_severity = vuln["Severity"].as_str().unwrap_or("UNKNOWN");
                    let severity = Severity::from_str_loose(raw_severity);

                    let mut metadata = HashMap::new();
                    metadata.insert("target".to_string(), Value::String(target.clone()));
                    if !installed_ver.is_empty() {
                        metadata.insert(
                            "installed_version".to_string(),
                            Value::String(installed_ver),
                        );
                    }
                    if !fixed_ver.is_empty() {
                        metadata.insert("fixed_version".to_string(), Value::String(fixed_ver));
                    }
                    if let Some(ref url) = primary_url {
                        metadata.insert("url".to_string(), Value::String(url.clone()));
                    }

                    findings.push(SecurityFinding {
                        id: format!("trivy-{vuln_id}-{pkg_name}"),
                        scanner: ScannerKind::Trivy,
                        rule_id: vuln_id.clone(),
                        severity,
                        category: "dependency-vulnerability".to_string(),
                        description: desc,
                        file_path: if target.is_empty() {
                            None
                        } else {
                            Some(PathBuf::from(&target))
                        },
                        line_number: None,
                        dependency: if pkg_name.is_empty() {
                            None
                        } else {
                            Some(pkg_name)
                        },
                        advisory_id: Some(vuln_id),
                        remediation: primary_url,
                        confidence: None,
                        metadata,
                    });
                }
            }

            // 2. Process Misconfigurations
            if let Some(misconfigs) = res["Misconfigurations"].as_array() {
                for mis in misconfigs {
                    let id = mis["ID"].as_str().unwrap_or("MISCONFIG").to_string();
                    let title = mis["Title"].as_str().unwrap_or(&id).to_string();
                    let desc = mis["Description"].as_str().unwrap_or(&title).to_string();
                    let raw_severity = mis["Severity"].as_str().unwrap_or("MEDIUM");
                    let severity = Severity::from_str_loose(raw_severity);

                    findings.push(SecurityFinding {
                        id: format!("trivy-misconfig-{id}"),
                        scanner: ScannerKind::Trivy,
                        rule_id: id,
                        severity,
                        category: "misconfiguration".to_string(),
                        description: desc,
                        file_path: if target.is_empty() {
                            None
                        } else {
                            Some(PathBuf::from(&target))
                        },
                        line_number: None,
                        dependency: None,
                        advisory_id: None,
                        remediation: None,
                        confidence: None,
                        metadata: HashMap::new(),
                    });
                }
            }
        }

        Ok(findings)
    }

    /// Executes Trivy scan against filesystem directory.
    ///
    /// Returns `Err(SecurityError::ScannerNotFound)` when the Trivy binary is absent.
    /// Callers that want graceful degradation should call [`detect`](Self::detect) first
    /// and skip this method when it returns `None`.
    pub async fn scan(
        &self,
        target_dir: &Path,
        timeout_secs: u64,
    ) -> Result<Vec<SecurityFinding>, SecurityError> {
        let bin = match self.detect() {
            Some(b) => b,
            None => {
                return Err(SecurityError::ScannerNotAvailable("trivy".to_string()));
            }
        };

        let mut cmd = SafeCommand::new(bin.to_string_lossy());
        cmd = cmd.arg("fs");
        cmd = cmd.arg("--format=json");
        cmd = cmd.arg("--quiet");
        cmd = cmd.arg(".");
        cmd = cmd.current_dir(target_dir);
        cmd = cmd.timeout(Duration::from_secs(timeout_secs));

        let res = cmd
            .execute()
            .await
            .map_err(|e| SecurityError::Execution(e.to_string()))?;

        if !res.stdout.trim().is_empty() {
            self.parse_output(&res.stdout)
        } else if !res.success {
            warn!("Trivy scan failed with stderr: {}", res.stderr);
            Err(SecurityError::Execution(res.stderr))
        } else {
            Ok(Vec::new())
        }
    }
}
