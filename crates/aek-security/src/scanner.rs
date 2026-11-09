//! Unified security scanning orchestrator combining Semgrep, Trivy, and dependency audits.

use crate::dependencies::DependencyAuditor;
use crate::errors::SecurityError;
use crate::findings::{SecurityFinding, Severity};
use crate::policy::SecurityPolicy;
use crate::semgrep::SemgrepAdapter;
use crate::trivy::TrivyAdapter;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Aggregated outcome of a security scan run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityScanReport {
    pub target_dir: PathBuf,
    pub total_findings: usize,
    pub critical_count: usize,
    pub high_count: usize,
    pub medium_count: usize,
    pub low_count: usize,
    pub policy_violated: bool,
    pub findings: Vec<SecurityFinding>,
    /// Scanners that were requested but could not run (binary not found).
    #[serde(default)]
    pub skipped_scanners: Vec<String>,
    /// True when at least one requested scanner was skipped.
    #[serde(default)]
    pub incomplete: bool,
}

/// Orchestrator coordinating all security scanners and applying security policies.
#[derive(Debug, Default, Clone)]
pub struct SecurityScanner {
    pub policy: SecurityPolicy,
    pub semgrep: SemgrepAdapter,
    pub trivy: TrivyAdapter,
    pub dependencies: DependencyAuditor,
}

impl SecurityScanner {
    /// Creates a new `SecurityScanner` with default policy.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new `SecurityScanner` with a custom `SecurityPolicy`.
    #[must_use]
    pub fn with_policy(policy: SecurityPolicy) -> Self {
        Self {
            policy,
            semgrep: SemgrepAdapter::new(),
            trivy: TrivyAdapter::new(),
            dependencies: DependencyAuditor::new(),
        }
    }

    /// Executes configured scanners against target directory and produces a `SecurityScanReport`.
    ///
    /// When a requested scanner binary is absent the scanner is recorded in
    /// `skipped_scanners` and `incomplete` is set.  Unless
    /// `policy.allow_missing_scanners` is `true`, an incomplete scan sets
    /// `policy_violated = true`.
    pub async fn scan(
        &self,
        target_dir: &Path,
        run_semgrep: bool,
        run_trivy: bool,
        run_dependencies: bool,
        timeout_secs: u64,
    ) -> Result<SecurityScanReport, SecurityError> {
        let mut all_findings = Vec::new();
        let mut skipped_scanners = Vec::new();

        if run_semgrep {
            match self.semgrep.detect() {
                Some(_) => {
                    let mut findings = self.semgrep.scan(target_dir, None, timeout_secs).await?;
                    all_findings.append(&mut findings);
                }
                None => {
                    skipped_scanners.push("semgrep".to_string());
                }
            }
        }

        if run_trivy {
            match self.trivy.detect() {
                Some(_) => {
                    let mut findings = self.trivy.scan(target_dir, timeout_secs).await?;
                    all_findings.append(&mut findings);
                }
                None => {
                    skipped_scanners.push("trivy".to_string());
                }
            }
        }

        if run_dependencies {
            let mut findings = self.dependencies.audit(target_dir, timeout_secs).await?;
            all_findings.append(&mut findings);
        }

        let incomplete = !skipped_scanners.is_empty();

        let mut critical_count = 0;
        let mut high_count = 0;
        let mut medium_count = 0;
        let mut low_count = 0;
        let mut policy_violated = false;

        for f in &all_findings {
            match f.severity {
                Severity::Critical => critical_count += 1,
                Severity::High => high_count += 1,
                Severity::Medium => medium_count += 1,
                Severity::Low => low_count += 1,
                _ => {}
            }

            if self.policy.violates_policy(f) {
                policy_violated = true;
            }
        }

        // Fail-closed: an incomplete scan is a policy violation unless explicitly opted out.
        if incomplete && !self.policy.allow_missing_scanners {
            policy_violated = true;
        }

        Ok(SecurityScanReport {
            target_dir: target_dir.to_path_buf(),
            total_findings: all_findings.len(),
            critical_count,
            high_count,
            medium_count,
            low_count,
            policy_violated,
            findings: all_findings,
            skipped_scanners,
            incomplete,
        })
    }
}
