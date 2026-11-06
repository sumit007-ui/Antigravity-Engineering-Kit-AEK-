//! Extensible agent adapter trait and common report types.

use crate::capabilities::AgentCapabilities;
use aek_rules::model::Rule;
use aek_skills::model::SkillBundle;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use thiserror::Error;

/// Errors arising during agent adapter execution.
#[derive(Error, Debug)]
pub enum AgentError {
    #[error("I/O error during agent operation: {0}")]
    Io(#[from] std::io::Error),

    #[error("Rule installation error: {0}")]
    RuleInstall(#[from] aek_rules::installer::RuleInstallError),

    #[error("Skill installation error: {0}")]
    SkillInstall(#[from] aek_skills::installer::SkillInstallError),

    #[error("Target agent '{0}' is not installed or detectable")]
    AgentNotDetected(String),

    #[error("Security violation: {0}")]
    Security(String),
}

/// Agent installation detection status.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentDetection {
    /// Name of the agent.
    pub agent_name: String,
    /// Whether an active installation was reliably identified.
    pub is_detected: bool,
    /// Evidence or filesystem paths validating detection.
    pub evidence: Vec<String>,
}

/// Summary report for rule or skill installation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallReport {
    /// Number of items successfully processed.
    pub installed_count: usize,
    /// Destination directory on disk.
    pub target_dir: PathBuf,
    /// Whether operation executed in dry-run mode.
    pub dry_run: bool,
}

/// Extensible interface representing an AI coding agent integration.
pub trait AgentAdapter: Send + Sync {
    /// Human-readable agent name.
    fn name(&self) -> &'static str;

    /// Checks if the agent is present on the host environment.
    fn detect(&self) -> AgentDetection;

    /// Reports supported features and constraints.
    fn capabilities(&self) -> AgentCapabilities;

    /// Platform path for global rules directory.
    fn global_rules_dir(&self) -> Option<PathBuf>;

    /// Target directory for workspace rules.
    fn workspace_rules_dir(&self, workspace: &Path) -> PathBuf;

    /// Platform path for global skills directory.
    fn global_skills_dir(&self) -> Option<PathBuf>;

    /// Target directory for workspace skills.
    fn workspace_skills_dir(&self, workspace: &Path) -> PathBuf;

    /// Installs a set of rules into the specified target directory.
    fn install_rules(
        &self,
        rules: &[Rule],
        target_dir: &Path,
        dry_run: bool,
    ) -> Result<InstallReport, AgentError>;

    /// Installs a set of skill bundles into the specified target directory.
    fn install_skills(
        &self,
        skills: &[SkillBundle],
        target_dir: &Path,
        dry_run: bool,
    ) -> Result<InstallReport, AgentError>;
}
