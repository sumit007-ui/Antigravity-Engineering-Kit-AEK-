//! Quality gate engine evaluating validation results and security findings.

use crate::pipeline::ValidationStage;
use crate::policy::ValidationPolicy;
use crate::report::{StepStatus, ValidationReport};
use aek_security::findings::SecurityFinding;
use serde::{Deserialize, Serialize};

/// Outcome of quality gate evaluation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityGateResult {
    pub passed: bool,
    pub failure_reasons: Vec<String>,
    pub total_violations: usize,
    pub summary: String,
}

/// Evaluator enforcing quality gates against validation reports and security findings.
#[derive(Debug, Default, Clone)]
pub struct QualityGateEngine {
    pub policy: ValidationPolicy,
}

impl QualityGateEngine {
    /// Creates a new `QualityGateEngine` with the specified policy.
    #[must_use]
    pub fn new(policy: ValidationPolicy) -> Self {
        Self { policy }
    }

    /// Evaluates validation reports and security findings against quality gates.
    pub fn evaluate(
        &self,
        report: &ValidationReport,
        findings: &[SecurityFinding],
    ) -> QualityGateResult {
        let mut failure_reasons = Vec::new();

        // 1. Evaluate Validation Steps
        for step in &report.results {
            if step.status == StepStatus::Fail || step.status == StepStatus::Error {
                match step.stage {
                    ValidationStage::Compilation if self.policy.fail_on_compile_error => {
                        failure_reasons
                            .push(format!("Compilation failure in step '{}'", step.name));
                    }
                    ValidationStage::UnitTesting if self.policy.fail_on_test_failure => {
                        failure_reasons.push(format!("Unit test failure in step '{}'", step.name));
                    }
                    ValidationStage::StaticAnalysis if self.policy.fail_on_lint_error => {
                        failure_reasons
                            .push(format!("Static analysis failure in step '{}'", step.name));
                    }
                    _ if !report.dry_run => {
                        failure_reasons.push(format!("Validation failure in step '{}'", step.name));
                    }
                    _ => {}
                }
            }
        }

        // 2. Evaluate Security Findings
        for finding in findings {
            if self.policy.security.violates_policy(finding) {
                failure_reasons.push(format!(
                    "Security policy violation: {} finding '{}' ({})",
                    finding.severity, finding.rule_id, finding.description
                ));
            }
        }

        let passed = failure_reasons.is_empty();
        let total_violations = failure_reasons.len();
        let summary = if passed {
            format!("Quality gate PASSED for project '{}'", report.project_name)
        } else {
            format!(
                "Quality gate FAILED for project '{}' with {} violation(s)",
                report.project_name, total_violations
            )
        };

        QualityGateResult {
            passed,
            failure_reasons,
            total_violations,
            summary,
        }
    }
}
