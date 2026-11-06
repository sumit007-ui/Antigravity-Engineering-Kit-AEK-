//! Validation planning and task ordering.

use crate::pipeline::{ValidationPipeline, ValidationStage};
use crate::policy::ValidationPolicy;
use aek_languages::model::DetectedProject;

/// Planner generating structured execution pipelines for detected projects.
#[derive(Debug, Default, Clone)]
pub struct ValidationPlanner;

impl ValidationPlanner {
    /// Creates a new `ValidationPlanner`.
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Creates validation pipelines for each detected project based on the policy and change scope.
    pub fn create_plan(
        &self,
        projects: &[DetectedProject],
        policy: &ValidationPolicy,
        _changed_only: bool,
    ) -> Vec<ValidationPipeline> {
        let mut pipelines = Vec::new();

        for project in projects {
            let mut pipeline = ValidationPipeline::for_project(project);

            // Filter steps according to policy
            if !policy.full_validation {
                // In non-full mode, filter out extended integration testing if marked optional
                pipeline
                    .steps
                    .retain(|s| !(s.stage == ValidationStage::IntegrationTesting && s.is_optional));
            }

            // Adjust timeouts from policy
            for step in &mut pipeline.steps {
                if policy.default_timeout_secs > 0 {
                    step.timeout_secs = policy.default_timeout_secs;
                }
            }

            pipelines.push(pipeline);
        }

        pipelines
    }
}
