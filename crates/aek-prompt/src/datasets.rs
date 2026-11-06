//! Evaluation dataset management, JSONL serialization, and train/val/test splitting.

use crate::errors::PromptError;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;
use std::path::Path;

/// A single prompt evaluation case with inputs, constraints, and criteria.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationCase {
    pub id: String,
    pub input_prompt: String,
    pub context: Option<String>,
    pub expected_properties: Vec<String>,
    pub constraints: Vec<String>,
    pub evaluation_criteria: String,
    pub reference_output: Option<String>,
    pub tags: Vec<String>,
}

/// Triple of (train, validation, test) evaluation partitions.
pub type DatasetSplit = (
    Vec<EvaluationCase>,
    Vec<EvaluationCase>,
    Vec<EvaluationCase>,
);

/// Manager handling loading, validation, and dataset splitting.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetManager {
    pub name: String,
    pub cases: Vec<EvaluationCase>,
}

impl DatasetManager {
    /// Creates a new dataset manager from an in-memory list of cases, validating IDs.
    pub fn from_cases(
        name: impl Into<String>,
        cases: Vec<EvaluationCase>,
    ) -> Result<Self, PromptError> {
        let mut ids = HashSet::new();
        for case in &cases {
            if !ids.insert(case.id.clone()) {
                return Err(PromptError::Dataset(format!(
                    "Duplicate case identifier: '{}'",
                    case.id
                )));
            }
        }

        Ok(Self {
            name: name.into(),
            cases,
        })
    }

    /// Loads dataset cases from a JSON or JSONL file.
    pub fn load_from_file(path: &Path) -> Result<Self, PromptError> {
        let content = fs::read_to_string(path)?;
        let mut cases = Vec::new();

        if path.extension().and_then(|e| e.to_str()) == Some("jsonl") {
            for (line_idx, line) in content.lines().enumerate() {
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    continue;
                }
                let case: EvaluationCase = serde_json::from_str(trimmed).map_err(|e| {
                    PromptError::Dataset(format!("Invalid JSONL at line {}: {e}", line_idx + 1))
                })?;
                cases.push(case);
            }
        } else {
            cases = serde_json::from_str(&content).map_err(|e| {
                PromptError::Dataset(format!("Invalid JSON in {}: {e}", path.display()))
            })?;
        }

        let name = path
            .file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| "dataset".to_string());

        Self::from_cases(name, cases)
    }

    /// Saves the dataset to a JSONL file.
    pub fn save_jsonl(&self, path: &Path) -> Result<(), PromptError> {
        let mut out = String::new();
        for case in &self.cases {
            let line = serde_json::to_string(case)?;
            out.push_str(&line);
            out.push('\n');
        }
        fs::write(path, out)?;
        Ok(())
    }

    /// Splits cases into deterministic (train, val, test) partitions.
    pub fn split(
        &self,
        train_ratio: f64,
        val_ratio: f64,
        test_ratio: f64,
    ) -> Result<DatasetSplit, PromptError> {
        let total_ratio = train_ratio + val_ratio + test_ratio;
        if (total_ratio - 1.0).abs() > 0.001 {
            return Err(PromptError::Dataset(
                "Split ratios must sum to 1.0".to_string(),
            ));
        }

        let n = self.cases.len();
        let train_end = ((n as f64) * train_ratio).round() as usize;
        let val_end = (train_end + ((n as f64) * val_ratio).round() as usize).min(n);

        let train = self.cases[0..train_end].to_vec();
        let val = self.cases[train_end..val_end].to_vec();
        let test = self.cases[val_end..n].to_vec();

        Ok((train, val, test))
    }
}
