//! Validation stages, steps, and pipeline generation.

use aek_languages::model::{DetectedProject, LanguageId};
use aek_languages::providers::find_provider;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::path::PathBuf;

/// High-level lifecycle stage in project validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationStage {
    Environment,
    Formatting,
    StaticAnalysis,
    TypeChecking,
    Compilation,
    UnitTesting,
    IntegrationTesting,
    BuildVerification,
}

impl fmt::Display for ValidationStage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Environment => write!(f, "Environment"),
            Self::Formatting => write!(f, "Formatting"),
            Self::StaticAnalysis => write!(f, "Static Analysis"),
            Self::TypeChecking => write!(f, "Type Checking"),
            Self::Compilation => write!(f, "Compilation"),
            Self::UnitTesting => write!(f, "Unit Testing"),
            Self::IntegrationTesting => write!(f, "Integration Testing"),
            Self::BuildVerification => write!(f, "Build Verification"),
        }
    }
}

/// A discrete, executable verification task within a pipeline.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationStep {
    pub name: String,
    pub stage: ValidationStage,
    pub program: String,
    pub args: Vec<String>,
    pub working_dir: PathBuf,
    pub is_optional: bool,
    pub timeout_secs: u64,
}

/// Complete pipeline of verification steps for a project.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationPipeline {
    pub project_name: String,
    pub language: LanguageId,
    pub steps: Vec<ValidationStep>,
}

impl ValidationPipeline {
    /// Generates a validation pipeline tailored to the detected project and language.
    #[must_use]
    pub fn for_project(project: &DetectedProject) -> Self {
        let mut steps = Vec::new();

        if let Some(provider) = find_provider(&project.language) {
            let default_cmds = provider.default_validation_commands(project);
            for (action, full_cmd) in default_cmds {
                if full_cmd.is_empty() {
                    continue;
                }

                let program = full_cmd[0].clone();
                let args = full_cmd[1..].to_vec();

                let stage = match action {
                    "format" => ValidationStage::Formatting,
                    "lint" => ValidationStage::StaticAnalysis,
                    "typecheck" => ValidationStage::TypeChecking,
                    "check" => ValidationStage::Compilation,
                    "build" => ValidationStage::BuildVerification,
                    "test" => ValidationStage::UnitTesting,
                    _ => ValidationStage::Environment,
                };

                steps.push(ValidationStep {
                    name: format!("{}: {}", project.name, action),
                    stage,
                    program,
                    args,
                    working_dir: project.path.clone(),
                    is_optional: false,
                    timeout_secs: 120,
                });
            }
        }

        Self {
            project_name: project.name.clone(),
            language: project.language.clone(),
            steps,
        }
    }
}
