//! Agent capability matrix, feature categories, and typed registry.

use crate::model::AgentId;
use serde::{Deserialize, Serialize};

/// High-level capability indicators preserved for backward compatibility.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentCapabilities {
    pub supports_global_rules: bool,
    pub supports_workspace_rules: bool,
    pub supports_skills: bool,
    pub supports_mcp: bool,
    pub max_rule_bytes: usize,
    pub rule_token_budget: usize,
}

/// Categories of agent engineering capabilities.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AgentCapabilityCategory {
    Rules,
    Skills,
    Mcp,
    Hooks,
    ProjectInstructions,
    GlobalInstructions,
    Permissions,
    ValidationWorkflows,
    PromptTemplates,
}

impl AgentCapabilityCategory {
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Rules => "RULES",
            Self::Skills => "SKILLS",
            Self::Mcp => "MCP",
            Self::Hooks => "HOOKS",
            Self::ProjectInstructions => "PROJECT_INSTRUCTIONS",
            Self::GlobalInstructions => "GLOBAL_INSTRUCTIONS",
            Self::Permissions => "PERMISSIONS",
            Self::ValidationWorkflows => "VALIDATION_WORKFLOWS",
            Self::PromptTemplates => "PROMPT_TEMPLATES",
        }
    }
}

/// Official support status for an agent feature.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CapabilityStatus {
    Supported,
    Partial,
    Unsupported,
    Unknown,
}

/// Detailed metadata record specifying support for a capability on a specific agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentCapabilityRecord {
    pub agent_id: AgentId,
    pub feature_id: String,
    pub category: AgentCapabilityCategory,
    pub support_status: CapabilityStatus,
    pub supported_scope: String,
    pub required_version: Option<String>,
    pub config_format: String,
    pub limitations: Option<String>,
    pub doc_ref: String,
    pub verification_date: String,
}

/// Registry storing verified capabilities for all AI coding agents.
#[derive(Debug, Clone, Default)]
pub struct AgentCapabilityRegistry {
    records: Vec<AgentCapabilityRecord>,
}

impl AgentCapabilityRegistry {
    #[must_use]
    pub fn new() -> Self {
        Self {
            records: Vec::new(),
        }
    }

    /// Populates registry with officially verified capabilities across all 4 agents.
    #[must_use]
    pub fn default_registry() -> Self {
        let mut reg = Self::new();

        // 1. Google / Gemini Antigravity
        reg.register(AgentCapabilityRecord {
            agent_id: AgentId::Antigravity,
            feature_id: "antigravity-global-rules".to_string(),
            category: AgentCapabilityCategory::Rules,
            support_status: CapabilityStatus::Supported,
            supported_scope: "Global (~/.gemini/antigravity/rules) & Workspace (.gemini/rules)"
                .to_string(),
            required_version: Some("2.0.0".to_string()),
            config_format: "Markdown (.md) with YAML frontmatter".to_string(),
            limitations: Some("24 KB file size boundary; 20k token aggregate budget".to_string()),
            doc_ref: "https://antigravity.google/docs/rules".to_string(),
            verification_date: "2026-11-06".to_string(),
        });
        reg.register(AgentCapabilityRecord {
            agent_id: AgentId::Antigravity,
            feature_id: "antigravity-skills".to_string(),
            category: AgentCapabilityCategory::Skills,
            support_status: CapabilityStatus::Supported,
            supported_scope: "Global & Workspace".to_string(),
            required_version: Some("2.0.0".to_string()),
            config_format: "SKILL.md folder bundles with progressive disclosure".to_string(),
            limitations: None,
            doc_ref: "https://antigravity.google/docs/skills".to_string(),
            verification_date: "2026-11-06".to_string(),
        });
        reg.register(AgentCapabilityRecord {
            agent_id: AgentId::Antigravity,
            feature_id: "antigravity-mcp".to_string(),
            category: AgentCapabilityCategory::Mcp,
            support_status: CapabilityStatus::Supported,
            supported_scope: "Global & Workspace".to_string(),
            required_version: Some("2.0.0".to_string()),
            config_format: "JSON stdio & http transport registry".to_string(),
            limitations: None,
            doc_ref: "https://antigravity.google/docs/mcp".to_string(),
            verification_date: "2026-11-06".to_string(),
        });
        reg.register(AgentCapabilityRecord {
            agent_id: AgentId::Antigravity,
            feature_id: "antigravity-hooks".to_string(),
            category: AgentCapabilityCategory::Hooks,
            support_status: CapabilityStatus::Partial,
            supported_scope: "Workspace plugin hooks".to_string(),
            required_version: Some("2.0.0".to_string()),
            config_format: "Plugin lifecycle handlers".to_string(),
            limitations: Some("Sandboxed within Antigravity extension boundary".to_string()),
            doc_ref: "https://antigravity.google/docs/plugins".to_string(),
            verification_date: "2026-11-06".to_string(),
        });

        // 2. Claude Code
        reg.register(AgentCapabilityRecord {
            agent_id: AgentId::ClaudeCode,
            feature_id: "claude-project-instructions".to_string(),
            category: AgentCapabilityCategory::ProjectInstructions,
            support_status: CapabilityStatus::Supported,
            supported_scope: "Workspace root (CLAUDE.md)".to_string(),
            required_version: Some("1.0.0".to_string()),
            config_format: "Markdown (CLAUDE.md)".to_string(),
            limitations: Some("Single project-wide instruction file parsed at startup".to_string()),
            doc_ref: "https://code.claude.com/docs/instructions".to_string(),
            verification_date: "2026-11-06".to_string(),
        });
        reg.register(AgentCapabilityRecord {
            agent_id: AgentId::ClaudeCode,
            feature_id: "claude-mcp".to_string(),
            category: AgentCapabilityCategory::Mcp,
            support_status: CapabilityStatus::Supported,
            supported_scope: "Workspace (.claude/mcp.json) & User (~/.claude/mcp.json)".to_string(),
            required_version: Some("1.0.0".to_string()),
            config_format: "JSON mcpServers object with command, args, env".to_string(),
            limitations: Some(
                "Stdio transport natively supported; requires command executable".to_string(),
            ),
            doc_ref: "https://code.claude.com/docs/mcp".to_string(),
            verification_date: "2026-11-06".to_string(),
        });
        reg.register(AgentCapabilityRecord {
            agent_id: AgentId::ClaudeCode,
            feature_id: "claude-skills".to_string(),
            category: AgentCapabilityCategory::Skills,
            support_status: CapabilityStatus::Supported,
            supported_scope: "Workspace (.claude/skills/) & User (~/.claude/skills/)".to_string(),
            required_version: Some("1.0.0".to_string()),
            config_format: "Markdown runbooks in .claude/skills/<name>/SKILL.md".to_string(),
            limitations: None,
            doc_ref: "https://code.claude.com/docs/skills".to_string(),
            verification_date: "2026-11-06".to_string(),
        });
        reg.register(AgentCapabilityRecord {
            agent_id: AgentId::ClaudeCode,
            feature_id: "claude-hooks".to_string(),
            category: AgentCapabilityCategory::Hooks,
            support_status: CapabilityStatus::Partial,
            supported_scope: "Workspace (.claude/settings.json or .claude/hooks/)".to_string(),
            required_version: Some("1.0.0".to_string()),
            config_format: "Shell scripts / settings command triggers".to_string(),
            limitations: Some(
                "Executes arbitrary shell commands; requires explicit user approval".to_string(),
            ),
            doc_ref: "https://code.claude.com/docs/hooks".to_string(),
            verification_date: "2026-11-06".to_string(),
        });
        reg.register(AgentCapabilityRecord {
            agent_id: AgentId::ClaudeCode,
            feature_id: "claude-rules".to_string(),
            category: AgentCapabilityCategory::Rules,
            support_status: CapabilityStatus::Supported,
            supported_scope: "Workspace (via CLAUDE.md sections)".to_string(),
            required_version: Some("1.0.0".to_string()),
            config_format: "Aggregated Markdown sections within CLAUDE.md".to_string(),
            limitations: Some(
                "No native per-file rule routing; consolidated into single document".to_string(),
            ),
            doc_ref: "https://code.claude.com/docs/instructions".to_string(),
            verification_date: "2026-11-06".to_string(),
        });

        // 3. OpenCode
        reg.register(AgentCapabilityRecord {
            agent_id: AgentId::OpenCode,
            feature_id: "opencode-instructions".to_string(),
            category: AgentCapabilityCategory::ProjectInstructions,
            support_status: CapabilityStatus::Supported,
            supported_scope: "Workspace root (AGENTS.md, .opencode/config.json)".to_string(),
            required_version: Some("1.0.0".to_string()),
            config_format: "AGENTS.md and JSON configuration".to_string(),
            limitations: None,
            doc_ref: "https://opencode.ai/docs/instructions".to_string(),
            verification_date: "2026-11-06".to_string(),
        });
        reg.register(AgentCapabilityRecord {
            agent_id: AgentId::OpenCode,
            feature_id: "opencode-mcp".to_string(),
            category: AgentCapabilityCategory::Mcp,
            support_status: CapabilityStatus::Supported,
            supported_scope: "Workspace (.opencode/config.json or opencode.toml)".to_string(),
            required_version: Some("1.0.0".to_string()),
            config_format: "JSON mcp_servers table".to_string(),
            limitations: None,
            doc_ref: "https://opencode.ai/docs/mcp".to_string(),
            verification_date: "2026-11-06".to_string(),
        });
        reg.register(AgentCapabilityRecord {
            agent_id: AgentId::OpenCode,
            feature_id: "opencode-skills".to_string(),
            category: AgentCapabilityCategory::Skills,
            support_status: CapabilityStatus::Supported,
            supported_scope: "Workspace (.opencode/skills/)".to_string(),
            required_version: Some("1.0.0".to_string()),
            config_format: "Markdown runbooks".to_string(),
            limitations: None,
            doc_ref: "https://opencode.ai/docs/skills".to_string(),
            verification_date: "2026-11-06".to_string(),
        });
        reg.register(AgentCapabilityRecord {
            agent_id: AgentId::OpenCode,
            feature_id: "opencode-rules".to_string(),
            category: AgentCapabilityCategory::Rules,
            support_status: CapabilityStatus::Supported,
            supported_scope: "Workspace (AGENTS.md)".to_string(),
            required_version: Some("1.0.0".to_string()),
            config_format: "Markdown sections in AGENTS.md".to_string(),
            limitations: None,
            doc_ref: "https://opencode.ai/docs/agents".to_string(),
            verification_date: "2026-11-06".to_string(),
        });

        // 4. Cursor
        reg.register(AgentCapabilityRecord {
            agent_id: AgentId::Cursor,
            feature_id: "cursor-mdc-rules".to_string(),
            category: AgentCapabilityCategory::Rules,
            support_status: CapabilityStatus::Supported,
            supported_scope: "Workspace (.cursor/rules/*.mdc) & Legacy (.cursorrules)".to_string(),
            required_version: Some("0.40.0".to_string()),
            config_format: "MDC Markdown with frontmatter (description, globs, alwaysApply)"
                .to_string(),
            limitations: Some(
                "Per-file MDC format supported in modern Cursor; legacy .cursorrules fallback"
                    .to_string(),
            ),
            doc_ref: "https://cursor.com/docs/context/rules-for-ai".to_string(),
            verification_date: "2026-11-06".to_string(),
        });
        reg.register(AgentCapabilityRecord {
            agent_id: AgentId::Cursor,
            feature_id: "cursor-mcp".to_string(),
            category: AgentCapabilityCategory::Mcp,
            support_status: CapabilityStatus::Supported,
            supported_scope: "Workspace (.cursor/mcp.json) & User (~/.cursor/mcp.json)".to_string(),
            required_version: Some("0.42.0".to_string()),
            config_format: "JSON mcpServers object".to_string(),
            limitations: Some("Requires Node or binary runtime configured on PATH".to_string()),
            doc_ref: "https://cursor.com/docs/context/model-context-protocol".to_string(),
            verification_date: "2026-11-06".to_string(),
        });
        reg.register(AgentCapabilityRecord {
            agent_id: AgentId::Cursor,
            feature_id: "cursor-skills".to_string(),
            category: AgentCapabilityCategory::Skills,
            support_status: CapabilityStatus::Supported,
            supported_scope: "Workspace (.cursor/skills/) & .cursor/rules/".to_string(),
            required_version: Some("0.40.0".to_string()),
            config_format: "Markdown skill documents".to_string(),
            limitations: None,
            doc_ref: "https://cursor.com/docs".to_string(),
            verification_date: "2026-11-06".to_string(),
        });
        reg.register(AgentCapabilityRecord {
            agent_id: AgentId::Cursor,
            feature_id: "cursor-prompt-templates".to_string(),
            category: AgentCapabilityCategory::PromptTemplates,
            support_status: CapabilityStatus::Partial,
            supported_scope: "Cursor system instructions & user rules".to_string(),
            required_version: Some("0.40.0".to_string()),
            config_format: "Markdown text".to_string(),
            limitations: Some("Custom prompt variable replacement is static".to_string()),
            doc_ref: "https://cursor.com/docs".to_string(),
            verification_date: "2026-11-06".to_string(),
        });

        reg
    }

    pub fn register(&mut self, record: AgentCapabilityRecord) {
        self.records.push(record);
    }

    #[must_use]
    pub fn get_status(
        &self,
        agent_id: AgentId,
        category: AgentCapabilityCategory,
    ) -> CapabilityStatus {
        self.records
            .iter()
            .find(|r| r.agent_id == agent_id && r.category == category)
            .map(|r| r.support_status)
            .unwrap_or(CapabilityStatus::Unknown)
    }

    #[must_use]
    pub fn for_agent(&self, agent_id: AgentId) -> Vec<AgentCapabilityRecord> {
        self.records
            .iter()
            .filter(|r| r.agent_id == agent_id)
            .cloned()
            .collect()
    }

    #[must_use]
    pub fn all_records(&self) -> &[AgentCapabilityRecord] {
        &self.records
    }
}
