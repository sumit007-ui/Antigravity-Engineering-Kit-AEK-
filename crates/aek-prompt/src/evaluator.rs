//! Deterministic evaluation metrics, constraint satisfaction, and schema verification.

use serde::{Deserialize, Serialize};

/// Metric scores calculated for an output against evaluation criteria.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationScore {
    pub constraint_score: f64,
    pub schema_valid: bool,
    pub missing_keywords: Vec<String>,
    pub negative_violations: Vec<String>,
}

/// Calculator for evaluating candidate model outputs deterministically.
#[derive(Debug, Default, Clone)]
pub struct EvaluationMetricCalculator;

impl EvaluationMetricCalculator {
    /// Creates a new metric calculator.
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Evaluates text against expected keywords and negative constraints.
    pub fn evaluate_constraints(
        &self,
        output: &str,
        expected_keywords: &[impl AsRef<str>],
        negative_constraints: &[impl AsRef<str>],
    ) -> EvaluationScore {
        let lower = output.to_ascii_lowercase();

        let mut missing_keywords = Vec::new();
        for kw in expected_keywords {
            let kw_s = kw.as_ref();
            if !lower.contains(&kw_s.to_ascii_lowercase()) {
                missing_keywords.push(kw_s.to_string());
            }
        }

        let mut negative_violations = Vec::new();
        for neg in negative_constraints {
            let neg_s = neg.as_ref();
            if lower.contains(&neg_s.to_ascii_lowercase()) {
                negative_violations.push(neg_s.to_string());
            }
        }

        let total_expected = expected_keywords.len();
        let satisfied = total_expected.saturating_sub(missing_keywords.len());
        let mut constraint_score = if total_expected > 0 {
            (satisfied as f64) / (total_expected as f64)
        } else {
            1.0
        };

        // Penalize negative violations heavily
        if !negative_violations.is_empty() {
            constraint_score *= 0.5;
        }

        // Check if output parses as valid JSON or clean Markdown
        let schema_valid = serde_json::from_str::<serde_json::Value>(output).is_ok()
            || (output.contains('#') && output.contains('\n'));

        EvaluationScore {
            constraint_score,
            schema_valid,
            missing_keywords,
            negative_violations,
        }
    }
}
