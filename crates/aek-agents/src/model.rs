//! Core models for multi-agent compatibility, profiles, and ownership manifests.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::path::PathBuf;
use std::str::FromStr;

/// Supported target AI coding agents.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AgentId {
    /// Google / Gemini Antigravity
    Antigravity,
    /// Anthropic Claude Code CLI
    ClaudeCode,
    /// OpenCode open-source coding agent
    OpenCode,
    /// Cursor AI IDE
    Cursor,
}

impl AgentId {
    /// Canonical kebab-case identifier string.
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Antigravity => "antigravity",
            Self::ClaudeCode => "claude-code",
            Self::OpenCode => "opencode",
            Self::Cursor => "cursor",
        }
    }

    /// Human-friendly display name.
    #[must_use]
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Antigravity => "Gemini Antigravity",
            Self::ClaudeCode => "Claude Code",
            Self::OpenCode => "OpenCode",
            Self::Cursor => "Cursor",
        }
    }

    /// Official documentation reference URL.
    #[must_use]
    pub fn documentation_url(&self) -> &'static str {
        match self {
            Self::Antigravity => "https://antigravity.google/docs",
            Self::ClaudeCode => "https://code.claude.com/docs/",
            Self::OpenCode => "https://opencode.ai/docs/",
            Self::Cursor => "https://cursor.com/docs",
        }
    }

    /// All supported agent identifiers.
    #[must_use]
    pub fn all() -> &'static [AgentId] {
        &[
            AgentId::Antigravity,
            AgentId::ClaudeCode,
            AgentId::OpenCode,
            AgentId::Cursor,
        ]
    }
}

impl fmt::Display for AgentId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for AgentId {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let normalized = s.trim().to_lowercase().replace('_', "-");
        match normalized.as_str() {
            "antigravity" | "gemini" | "gemini-antigravity" => Ok(Self::Antigravity),
            "claude-code" | "claude" | "claudecode" => Ok(Self::ClaudeCode),
            "opencode" | "open-code" => Ok(Self::OpenCode),
            "cursor" => Ok(Self::Cursor),
            other => Err(format!(
                "Unknown agent '{other}'. Supported: antigravity, claude-code, opencode, cursor"
            )),
        }
    }
}

/// Metadata describing an agent's platform integration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMetadata {
    pub id: AgentId,
    pub name: String,
    pub description: String,
    pub documentation_url: String,
    pub verified_date: String,
}

/// Agent installation detection status and evidence.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentDetection {
    pub agent_name: String,
    pub is_detected: bool,
    pub evidence: Vec<String>,
    #[serde(default)]
    pub version: Option<String>,
    #[serde(default)]
    pub executable_path: Option<PathBuf>,
    #[serde(default)]
    pub config_path: Option<PathBuf>,
}

/// Status and contents discovered during agent configuration inspection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfigInspection {
    pub agent_id: AgentId,
    pub global_rules_present: bool,
    pub workspace_rules_present: bool,
    pub skills_count: usize,
    pub mcp_servers_count: usize,
    pub config_files: Vec<PathBuf>,
}

/// Results of agent configuration validation and policy checks.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentValidationReport {
    pub agent_id: AgentId,
    pub is_valid: bool,
    pub findings: Vec<String>,
    pub warnings: Vec<String>,
}

/// Classification of agent configuration profiles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AgentProfileKind {
    AntigravityEngineering,
    ClaudeCodeEngineering,
    OpenCodeEngineering,
    CursorEngineering,
    MultiAgentEngineering,
    Custom,
}

/// Configured bundle of agent rules, skills, MCP servers, and validation rules.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentProfile {
    pub id: String,
    pub name: String,
    pub kind: AgentProfileKind,
    pub description: String,
    pub target_agents: Vec<AgentId>,
    pub enabled_rules: Vec<String>,
    pub enabled_skills: Vec<String>,
    pub mcp_servers: Vec<String>,
    pub validation_policy: String,
    pub permissions: AgentPermissions,
}

/// Permissions boundary enforced during agent configuration translation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentPermissions {
    pub allow_hooks: bool,
    pub allow_file_write: bool,
    pub allow_network: bool,
    pub require_approval_for_hooks: bool,
}

impl Default for AgentPermissions {
    fn default() -> Self {
        Self {
            allow_hooks: false,
            allow_file_write: true,
            allow_network: false,
            require_approval_for_hooks: true,
        }
    }
}

/// Record of an individual file managed and tracked by AEK.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManagedFileRecord {
    pub agent_id: AgentId,
    pub relative_path: PathBuf,
    pub content_hash: String,
    pub aek_version: String,
    pub installed_at: String,
    pub last_synced_at: String,
    pub backup_ref: Option<PathBuf>,
}

/// Ownership manifest tracked in `.aek/managed-agents.toml` to prevent silent overwrites.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AgentOwnershipManifest {
    pub schema_version: String,
    pub managed_files: HashMap<String, ManagedFileRecord>,
}

impl AgentOwnershipManifest {
    #[must_use]
    pub fn new() -> Self {
        Self {
            schema_version: "1.0.0".to_string(),
            managed_files: HashMap::new(),
        }
    }
}

/// Configuration exported into agent-native files.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentExportedConfig {
    pub agent_id: AgentId,
    pub primary_file: Option<PathBuf>,
    pub files: HashMap<PathBuf, String>,
}

/// Sync action planned for an individual configuration file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncItem {
    pub agent_id: AgentId,
    pub destination_path: PathBuf,
    pub content: String,
    pub content_hash: String,
    pub file_existed: bool,
    pub is_externally_modified: bool,
}

/// Staged multi-agent synchronization plan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncPlan {
    pub items: Vec<SyncItem>,
    pub conflicts: Vec<String>,
}

/// Result report from executing a multi-agent synchronization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncReport {
    pub files_written: usize,
    pub files_skipped: usize,
    pub conflicts_preserved: usize,
    pub conflicts_resolved: usize,
    pub backup_directory: Option<PathBuf>,
    pub dry_run: bool,
}

/// Summary report after removing AEK-managed files for an agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentUninstallReport {
    pub agent_id: AgentId,
    pub removed_files: Vec<PathBuf>,
    pub preserved_files: Vec<PathBuf>,
    pub dry_run: bool,
}

/// Returns current UTC timestamp in ISO 8601 format.
#[must_use]
pub fn utc_now_iso() -> String {
    chrono::Utc::now().to_rfc3339()
}
