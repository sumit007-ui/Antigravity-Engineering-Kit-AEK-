//! DSPy Python bridge adapter for declarative self-optimizing prompt compilation.

use aek_core::SafeCommand;
use aek_platform::find_executable_in_path;
use aek_prompt::errors::PromptError;
use aek_prompt::model::PromptRecord;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::time::Duration;
use tracing::{info, warn};

/// Payload sent from Rust engine to the Python DSPy bridge.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DspyBridgePayload {
    pub task_id: String,
    pub prompt_text: String,
    pub dataset_path: String,
    pub max_demos: usize,
}

/// Result returned from the DSPy optimizer compiler.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DspyOptimizationResult {
    pub task_id: String,
    pub optimized_instructions: String,
    pub demonstrations: Vec<String>,
    pub metric_score: f64,
    pub iterations: usize,
    pub success: bool,
    /// Indicates whether the result was produced by an offline simulated/passthrough path
    /// rather than a real executed DSPy optimization run.
    #[serde(default)]
    pub simulated: bool,
}

impl DspyOptimizationResult {
    /// Constructs a passthrough result when DSPy dependencies/bridge are unavailable.
    ///
    /// Preserves original prompt text, contains no demonstrations, 0 metric score,
    /// 0 iterations, success = false, and simulated = true.
    #[must_use]
    pub fn passthrough(task_id: impl Into<String>, prompt_text: impl Into<String>) -> Self {
        Self {
            task_id: task_id.into(),
            optimized_instructions: prompt_text.into(),
            demonstrations: Vec::new(),
            metric_score: 0.0,
            iterations: 0,
            success: false,
            simulated: true,
        }
    }
}

/// Adapter orchestrating DSPy prompt compilation via Python subprocess bridge.
#[derive(Debug, Default, Clone)]
pub struct DspyAdapter;

impl DspyAdapter {
    /// Creates a new `DspyAdapter`.
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Assembles the JSON payload for the DSPy optimizer bridge.
    #[must_use]
    pub fn build_bridge_payload(
        &self,
        prompt: &PromptRecord,
        dataset_path: &str,
        max_demos: usize,
    ) -> DspyBridgePayload {
        DspyBridgePayload {
            task_id: prompt.id.clone(),
            prompt_text: prompt.active_content().to_string(),
            dataset_path: dataset_path.to_string(),
            max_demos,
        }
    }

    /// Executes the DSPy bridge via safe subprocess or falls back to simulated compilation if Python/DSPy is offline.
    pub async fn run_bridge(
        &self,
        payload: &DspyBridgePayload,
        bridge_script: Option<&Path>,
    ) -> Result<DspyOptimizationResult, PromptError> {
        let python_bin =
            find_executable_in_path("python").or_else(|| find_executable_in_path("python3"));

        let script = bridge_script
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| PathBuf::from("integrations/prompt/dspy/bridge.py"));

        if let (Some(py), true) = (python_bin, script.is_file()) {
            info!(
                "Spawning DSPy bridge: {} {}",
                py.display(),
                script.display()
            );

            let payload_json = serde_json::to_string(payload)?;
            let tmp_input = tempfile::Builder::new()
                .prefix("dspy_in_")
                .suffix(".json")
                .tempfile()
                .map_err(PromptError::Io)?;

            std::fs::write(tmp_input.path(), &payload_json)?;

            let mut cmd = SafeCommand::new(py.to_string_lossy());
            cmd = cmd.arg(script.to_string_lossy());
            cmd = cmd.arg(tmp_input.path().to_string_lossy());
            cmd = cmd.timeout(Duration::from_secs(60));

            match cmd.execute().await {
                Ok(res) if res.success => {
                    match serde_json::from_str::<DspyOptimizationResult>(&res.stdout) {
                        Ok(parsed) => return Ok(parsed),
                        Err(err) => {
                            return Err(PromptError::Adapter(format!(
                                "DSPy bridge produced invalid JSON output: {err}"
                            )));
                        }
                    }
                }
                Ok(res) => {
                    warn!(
                        "DSPy bridge failed with status {:?}: {}",
                        res.status_code, res.stderr
                    );
                }
                Err(err) => {
                    warn!("DSPy subprocess error: {err}");
                }
            }
        }

        // Return passthrough result with simulated: true when DSPy dependencies/bridge are unavailable
        info!(
            "DSPy bridge offline or unavailable for '{}'; returning simulated passthrough",
            payload.task_id
        );
        Ok(DspyOptimizationResult::passthrough(
            &payload.task_id,
            &payload.prompt_text,
        ))
    }
}
