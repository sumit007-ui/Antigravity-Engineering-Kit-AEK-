//! Security policy, failure thresholds, and finding exclusions.

use crate::findings::{SecurityFinding, Severity};
use serde::{Deserialize, Serialize};

/// Policy defining security failure gates and rule exclusions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityPolicy {
    pub fail_on_critical: bool,
    pub fail_on_high: bool,
    pub fail_on_medium: bool,
    pub fail_on_low: bool,
    pub ignored_rules: Vec<String>,
    pub ignored_paths: Vec<String>,
    /// When true, a scan that completes with some scanners missing/skipped is NOT
    /// treated as a policy violation.  Default false (fail-closed).
    #[serde(default)]
    pub allow_missing_scanners: bool,
}

impl Default for SecurityPolicy {
    fn default() -> Self {
        Self {
            fail_on_critical: true,
            fail_on_high: true,
            fail_on_medium: false,
            fail_on_low: false,
            ignored_rules: Vec::new(),
            ignored_paths: Vec::new(),
            allow_missing_scanners: false,
        }
    }
}

impl SecurityPolicy {
    /// Determines whether a security finding violates this policy.
    #[must_use]
    pub fn violates_policy(&self, finding: &SecurityFinding) -> bool {
        // Check rule exclusions
        if self.ignored_rules.iter().any(|r| r == &finding.rule_id) {
            return false;
        }

        // Check path exclusions
        if let Some(file_path) = &finding.file_path {
            let path_str = file_path.to_string_lossy();
            if self.ignored_paths.iter().any(|pattern| {
                if pattern.ends_with("/*") {
                    let prefix = &pattern[..pattern.len() - 2];
                    path_str.starts_with(prefix)
                } else {
                    path_str == *pattern
                }
            }) {
                return false;
            }
        }

        // Check severity gates
        match finding.severity {
            Severity::Critical => self.fail_on_critical,
            Severity::High => self.fail_on_high,
            Severity::Medium => self.fail_on_medium,
            Severity::Low => self.fail_on_low,
            Severity::Info | Severity::Unknown => false,
        }
    }
}
