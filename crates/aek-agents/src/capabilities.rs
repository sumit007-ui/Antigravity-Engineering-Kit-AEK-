//! Agent capabilities and feature support matrix.

use serde::{Deserialize, Serialize};

/// Supported capabilities and constraints of a target AI coding agent.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentCapabilities {
    /// Whether the agent reads machine-wide global rules.
    pub supports_global_rules: bool,
    /// Whether the agent reads project-level workspace rules.
    pub supports_workspace_rules: bool,
    /// Whether the agent supports modular skill runbooks (`SKILL.md`).
    pub supports_skills: bool,
    /// Whether the agent supports Model Context Protocol (MCP) tool servers.
    pub supports_mcp: bool,
    /// Maximum byte limit for individual rule files.
    pub max_rule_bytes: usize,
    /// Aggregate token budget for always-on rules.
    pub rule_token_budget: usize,
}
