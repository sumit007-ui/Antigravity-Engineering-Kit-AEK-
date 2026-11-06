//! Structured report models for optimization, evaluation, and benchmarking.

use serde::{Deserialize, Serialize};

/// Report generated after running prompt optimization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptOptimizationReport {
    pub prompt_id: String,
    pub adapter_used: String,
    pub baseline_length: usize,
    pub optimized_length: usize,
    pub duration_ms: u128,
    pub optimized_prompt: String,
}

/// Report generated after running evaluation against a dataset.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationReport {
    pub prompt_id: String,
    pub dataset_name: String,
    pub total_cases: usize,
    pub passed_cases: usize,
    pub constraint_satisfaction_rate: f64,
    pub schema_validity_rate: f64,
    pub duration_ms: u128,
}

/// Comparison benchmark report between baseline and candidate versions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkReport {
    pub prompt_id: String,
    pub baseline_version: String,
    pub candidate_version: String,
    pub baseline_score: f64,
    pub candidate_score: f64,
    pub regression_detected: bool,
    pub score_delta: f64,
}
