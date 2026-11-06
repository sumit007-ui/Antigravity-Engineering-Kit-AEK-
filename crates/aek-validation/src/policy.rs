//! Validation execution policies and containment constraints.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Execution policy governing validation runs.
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
    /// Extra environment variables to pass to child processes.
    pub envs: HashMap<String, String>,
}

impl Default for ValidationPolicy {
    fn default() -> Self {
        Self {
            default_timeout_secs: 120,
            fail_fast: false,
            dry_run: false,
            full_validation: false,
            envs: HashMap::new(),
        }
    }
}
