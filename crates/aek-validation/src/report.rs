//! Structured validation results and status classifications.

use crate::pipeline::ValidationStage;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Execution status for a validation step.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StepStatus {
    Pass,
    Fail,
    Skip,
    Warn,
    Error,
}

impl fmt::Display for StepStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pass => write!(f, "PASS"),
            Self::Fail => write!(f, "FAIL"),
            Self::Skip => write!(f, "SKIP"),
            Self::Warn => write!(f, "WARN"),
            Self::Error => write!(f, "ERROR"),
        }
    }
}

/// Detailed execution result of a single validation step.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepResult {
    pub name: String,
    pub stage: ValidationStage,
    pub status: StepStatus,
    pub duration_ms: u128,
    pub exit_code: Option<i32>,
    pub stdout: String,
    pub stderr: String,
}

/// Comprehensive report aggregating all validation step outcomes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationReport {
    pub project_name: String,
    pub language: String,
    pub dry_run: bool,
    pub overall_success: bool,
    pub passed_count: usize,
    pub failed_count: usize,
    pub skipped_count: usize,
    pub total_duration_ms: u128,
    pub results: Vec<StepResult>,
}
