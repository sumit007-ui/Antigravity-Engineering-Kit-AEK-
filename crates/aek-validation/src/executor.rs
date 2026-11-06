//! Execution engine for running validation pipeline steps with timeouts and security boundaries.

use crate::errors::ValidationError;
use crate::pipeline::ValidationPipeline;
use crate::policy::ValidationPolicy;
use crate::report::{StepResult, StepStatus, ValidationReport};
use aek_core::SafeCommand;
use std::time::{Duration, Instant};
use tracing::{info, warn};

/// Executor running validation pipelines under configured policies.
#[derive(Debug, Clone)]
pub struct ValidationExecutor {
    policy: ValidationPolicy,
}

impl Default for ValidationExecutor {
    fn default() -> Self {
        Self::new(ValidationPolicy::default())
    }
}

impl ValidationExecutor {
    /// Creates a new `ValidationExecutor` with the specified execution policy.
    #[must_use]
    pub fn new(policy: ValidationPolicy) -> Self {
        Self { policy }
    }

    /// Executes all steps within the pipeline and produces a `ValidationReport`.
    pub async fn execute(
        &self,
        pipeline: &ValidationPipeline,
    ) -> Result<ValidationReport, ValidationError> {
        let start_total = Instant::now();
        let mut results = Vec::new();
        let mut passed_count = 0;
        let mut failed_count = 0;
        let mut skipped_count = 0;

        for step in &pipeline.steps {
            if self.policy.dry_run {
                info!("[Dry Run] Would execute: {} {:?}", step.program, step.args);
                results.push(StepResult {
                    name: step.name.clone(),
                    stage: step.stage,
                    status: StepStatus::Skip,
                    duration_ms: 0,
                    exit_code: None,
                    stdout: format!(
                        "[Dry run] Planned command: {} {}",
                        step.program,
                        step.args.join(" ")
                    ),
                    stderr: String::new(),
                });
                skipped_count += 1;
                continue;
            }

            let start_step = Instant::now();
            let mut cmd = SafeCommand::new(&step.program);
            cmd = cmd.args(&step.args);
            cmd = cmd.current_dir(&step.working_dir);
            cmd = cmd.timeout(Duration::from_secs(step.timeout_secs));

            for (k, v) in &self.policy.envs {
                cmd = cmd.env(k, v);
            }

            match cmd.execute().await {
                Ok(res) => {
                    let duration_ms = start_step.elapsed().as_millis();
                    let status = if res.success {
                        passed_count += 1;
                        StepStatus::Pass
                    } else if step.is_optional {
                        skipped_count += 1;
                        StepStatus::Warn
                    } else {
                        failed_count += 1;
                        StepStatus::Fail
                    };

                    results.push(StepResult {
                        name: step.name.clone(),
                        stage: step.stage,
                        status,
                        duration_ms,
                        exit_code: res.status_code,
                        stdout: res.stdout,
                        stderr: res.stderr,
                    });

                    if !res.success && !step.is_optional && self.policy.fail_fast {
                        warn!(
                            "Step '{}' failed and fail_fast policy is active; stopping pipeline",
                            step.name
                        );
                        break;
                    }
                }
                Err(err) => {
                    let duration_ms = start_step.elapsed().as_millis();
                    failed_count += 1;
                    results.push(StepResult {
                        name: step.name.clone(),
                        stage: step.stage,
                        status: StepStatus::Error,
                        duration_ms,
                        exit_code: None,
                        stdout: String::new(),
                        stderr: err.to_string(),
                    });

                    if self.policy.fail_fast {
                        break;
                    }
                }
            }
        }

        let overall_success = failed_count == 0;
        let total_duration_ms = start_total.elapsed().as_millis();

        Ok(ValidationReport {
            project_name: pipeline.project_name.clone(),
            language: pipeline.language.to_string(),
            dry_run: self.policy.dry_run,
            overall_success,
            passed_count,
            failed_count,
            skipped_count,
            total_duration_ms,
            results,
        })
    }
}
