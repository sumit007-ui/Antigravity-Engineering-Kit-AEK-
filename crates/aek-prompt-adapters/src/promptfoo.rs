//! Promptfoo CLI evaluation and red-teaming adapter.

use aek_core::SafeCommand;
use aek_platform::find_executable_in_path;
use aek_prompt::errors::PromptError;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::time::Duration;
use tracing::{info, warn};

/// Exact pinned version of Promptfoo on npm registry.
/// Update via verified npm releases: check `npm view promptfoo version`.
pub const PINNED_PROMPTFOO_VERSION: &str = "0.125.1";

/// Parsed results from Promptfoo evaluation execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptfooEvalResult {
    pub total_tests: usize,
    pub passed_tests: usize,
    pub failed_tests: usize,
    pub score: f64,
    pub success: bool,
    /// Indicates whether this evaluation was simulated/offline fallback rather than
    /// executed against real LLMs and real evaluation assertions.
    #[serde(default)]
    pub simulated: bool,
}

/// Adapter orchestrating Promptfoo evaluation runs and configuration generation.
#[derive(Debug, Default, Clone)]
pub struct PromptfooAdapter;

impl PromptfooAdapter {
    /// Creates a new `PromptfooAdapter`.
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Generates a valid `promptfooconfig.yaml` configuration.
    #[must_use]
    pub fn generate_config(
        &self,
        description: &str,
        prompts: &[String],
        providers: &[String],
        tests_count: usize,
    ) -> String {
        let mut yaml = format!("description: \"{description}\"\n\nprompts:\n");
        for p in prompts {
            yaml.push_str(&format!("  - \"{p}\"\n"));
        }

        yaml.push_str("\nproviders:\n");
        for prov in providers {
            yaml.push_str(&format!("  - \"{prov}\"\n"));
        }

        yaml.push_str("\ndefaultTest:\n  options:\n    temperature: 0.1\n\ntests:\n");
        for i in 1..=tests_count {
            yaml.push_str(&format!("  - description: \"Test case {i}\"\n"));
            yaml.push_str("    vars:\n");
            yaml.push_str(&format!("      query: \"Sample evaluation input {i}\"\n"));
            yaml.push_str("    assert:\n");
            yaml.push_str("      - type: is-json\n");
            yaml.push_str("      - type: latency\n");
            yaml.push_str("        threshold: 3000\n");
        }

        yaml
    }

    /// Executes `promptfoo eval` via `npx` or local binary, or returns deterministic evaluation when offline.
    pub async fn run_eval(
        &self,
        config_path: &Path,
        output_path: &Path,
        no_cache: bool,
    ) -> Result<PromptfooEvalResult, PromptError> {
        let promptfoo_bin = match find_executable_in_path("promptfoo")
            .or_else(|| find_executable_in_path("npx"))
        {
            Some(b) => b,
            None => {
                return Err(PromptError::Evaluation(
                    "Promptfoo CLI or npx not found on system PATH. Install promptfoo (`npm i -g promptfoo`) to run evaluations.".to_string(),
                ));
            }
        };

        let is_npx = promptfoo_bin
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .starts_with("npx");

        let mut cmd = SafeCommand::new(promptfoo_bin.to_string_lossy());
        if is_npx {
            cmd = cmd
                .arg("-y")
                .arg(format!("promptfoo@{PINNED_PROMPTFOO_VERSION}"));
        }
        cmd = cmd
            .arg("eval")
            .arg("-c")
            .arg(config_path.to_string_lossy())
            .arg("-o")
            .arg(output_path.to_string_lossy());

        if no_cache {
            cmd = cmd.arg("--no-cache");
        }
        cmd = cmd.timeout(Duration::from_secs(120));

        info!("Executing Promptfoo command: {:?}", cmd);
        match cmd.execute().await {
            Ok(res) if res.success => {
                if !output_path.is_file() {
                    return Err(PromptError::Evaluation(format!(
                        "Promptfoo succeeded but did not write expected output file '{}'",
                        output_path.display()
                    )));
                }

                let raw = std::fs::read_to_string(output_path)?;
                let json_val = serde_json::from_str::<serde_json::Value>(&raw).map_err(|e| {
                    PromptError::Evaluation(format!("Invalid Promptfoo JSON report: {e}"))
                })?;

                let total = json_val["results"]["stats"]["successes"]
                    .as_u64()
                    .unwrap_or(0)
                    + json_val["results"]["stats"]["failures"]
                        .as_u64()
                        .unwrap_or(0);
                let passed = json_val["results"]["stats"]["successes"]
                    .as_u64()
                    .unwrap_or(0);
                let failed = json_val["results"]["stats"]["failures"]
                    .as_u64()
                    .unwrap_or(0);
                let score = if total > 0 {
                    (passed as f64) / (total as f64)
                } else {
                    0.0
                };

                Ok(PromptfooEvalResult {
                    total_tests: total as usize,
                    passed_tests: passed as usize,
                    failed_tests: failed as usize,
                    score,
                    success: failed == 0 && total > 0,
                    simulated: false,
                })
            }
            Ok(res) => {
                warn!(
                    "Promptfoo execution completed with exit code {:?}: {}",
                    res.status_code, res.stderr
                );
                Err(PromptError::Evaluation(format!(
                    "Promptfoo evaluation failed (exit {:?}): {}",
                    res.status_code, res.stderr
                )))
            }
            Err(err) => {
                warn!("Promptfoo execution error: {err}");
                Err(PromptError::Evaluation(format!(
                    "Promptfoo subprocess error: {err}"
                )))
            }
        }
    }
}
