//! Extensible universal agent adapter trait and common report types.

use crate::capabilities::AgentCapabilities;
pub use crate::errors::AgentError;
pub use crate::model::AgentDetection;
use crate::model::AgentId;
use aek_mcp::config::McpServerConfig;
use aek_rules::model::Rule;
use aek_skills::model::SkillBundle;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

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

/// Summary report for MCP server configuration merge.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpMergeReport {
    /// Total servers active in target configuration after merge.
    pub total_servers: usize,
    /// Number of new or updated servers added.
    pub merged_count: usize,
    /// Conflicts detected where existing server had identical key.
    pub conflicts_detected: Vec<String>,
    /// Path to target configuration file.
    pub target_file: PathBuf,
    /// Path to backup file created prior to modification.
    pub backup_file: Option<PathBuf>,
    /// Whether operation executed in dry-run mode.
    pub dry_run: bool,
}

/// Extensible interface representing an AI coding agent integration.
pub trait AgentAdapter: Send + Sync {
    /// Human-readable agent name.
    fn name(&self) -> &'static str;

    /// Canonical agent identifier.
    fn agent_id(&self) -> AgentId;

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

    /// Platform path for agent's native MCP configuration file.
    fn mcp_config_path(&self) -> Option<PathBuf>;

    /// Converts AEK MCP server definitions into the agent's native JSON configuration format.
    fn export_mcp_config(&self, servers: &[McpServerConfig]) -> serde_json::Value;

    /// Safely merges MCP server entries into agent configuration file with backup.
    fn merge_mcp_config(
        &self,
        servers: &[McpServerConfig],
        target_file: Option<&Path>,
        dry_run: bool,
    ) -> Result<McpMergeReport, AgentError>;

    /// Installs MCP servers into the agent's workspace configuration.
    fn install_mcp(
        &self,
        servers: &[McpServerConfig],
        workspace: Option<&Path>,
        dry_run: bool,
    ) -> Result<McpMergeReport, AgentError> {
        let path = match workspace {
            Some(ws) => self.workspace_mcp_path(ws),
            None => self
                .mcp_config_path()
                .unwrap_or_else(|| PathBuf::from("mcp.json")),
        };
        self.merge_mcp_config(servers, Some(&path), dry_run)
    }

    /// Target path for workspace MCP configuration.
    fn workspace_mcp_path(&self, workspace: &Path) -> PathBuf {
        workspace.join("mcp.json")
    }

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

    /// Analyzes the workspace for languages and recommends relevant skills.
    fn recommend_skills_for_path(&self, workspace: &Path) -> Vec<String>;

    /// Generates structured validation guidance and commands for the workspace.
    fn generate_validation_guidance(&self, workspace: &Path) -> String;

    /// Exports an optimized prompt into the agent's instructions directory.
    fn export_prompt_instruction(
        &self,
        prompt: &aek_prompt::model::PromptRecord,
        target_dir: &Path,
        dry_run: bool,
    ) -> Result<PathBuf, AgentError>;

    /// Generates structured security audit and scanner recommendations for the workspace.
    fn generate_security_guidance(&self, workspace: &Path) -> String;

    /// Exports a generated CI/CD workflow into the project workflow directory.
    fn export_ci_workflow(
        &self,
        target_dir: &Path,
        workflow_content: &str,
        dry_run: bool,
    ) -> Result<PathBuf, AgentError>;
}
