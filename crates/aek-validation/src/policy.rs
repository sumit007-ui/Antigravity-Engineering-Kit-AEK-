//! Validation execution policies and containment constraints.

use crate::errors::ValidationError;
use aek_security::policy::SecurityPolicy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Execution policy governing validation runs, timeouts, and quality gates.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationPolicy {
    /// Maximum execution duration per step in seconds.
    pub default_timeout_secs: u64,
    /// Whether validation stops immediately on first non-optional failure.
    pub fail_fast: bool,
    /// Whether validation should preview steps without executing commands.
    pub dry_run: bool,
    /// Whether to run all extended stages (e.g. integration tests, release builds).
    pub full_validation: bool,
    /// Quality gate: fail on compilation error.
    pub fail_on_compile_error: bool,
    /// Quality gate: fail on test failure.
    pub fail_on_test_failure: bool,
    /// Quality gate: fail on lint/static analysis error.
    pub fail_on_lint_error: bool,
    /// Maximum parallel tasks allowed during validation.
    pub max_concurrent_tasks: usize,
    /// Extra environment variables to pass to child processes.
    pub envs: HashMap<String, String>,
    /// Security failure policy and thresholds.
    pub security: SecurityPolicy,
}

impl Default for ValidationPolicy {
    fn default() -> Self {
        Self {
            default_timeout_secs: 120,
            fail_fast: false,
            dry_run: false,
            full_validation: false,
            fail_on_compile_error: true,
            fail_on_test_failure: true,
            fail_on_lint_error: true,
            max_concurrent_tasks: 2,
            envs: HashMap::new(),
            security: SecurityPolicy::default(),
        }
    }
}

impl ValidationPolicy {
    /// Creates a strict validation policy where any warning or medium security issue fails.
    #[must_use]
    pub fn strict() -> Self {
        Self {
            fail_fast: true,
            full_validation: true,
            security: aek_security::SecurityPolicy {
                fail_on_medium: true,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    /// Creates a CI validation policy optimized for automated pipelines.
    #[must_use]
    pub fn ci() -> Self {
        Self {
            default_timeout_secs: 300,
            full_validation: true,
            ..Default::default()
        }
    }

    /// Loads a validation policy from a TOML configuration file.
    pub fn load_from_file(path: &Path) -> Result<Self, ValidationError> {
        let content = fs::read_to_string(path)?;
        let policy: Self = toml::from_str(&content)
            .map_err(|e| ValidationError::Policy(format!("Invalid policy TOML: {e}")))?;
        Ok(policy)
    }
}
