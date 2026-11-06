//! Pre-configured agent engineering profiles and profile manager.

use crate::errors::AgentError;
use crate::model::{AgentId, AgentPermissions, AgentProfile, AgentProfileKind};
use std::collections::HashMap;

/// Manages built-in and user-defined agent engineering profiles.
#[derive(Debug, Clone)]
pub struct AgentProfileManager {
    profiles: HashMap<String, AgentProfile>,
}

impl Default for AgentProfileManager {
    fn default() -> Self {
        Self::new()
    }
}

impl AgentProfileManager {
    #[must_use]
    pub fn new() -> Self {
        let mut manager = Self {
            profiles: HashMap::new(),
        };
        manager.register_default_profiles();
        manager
    }

    fn register_default_profiles(&mut self) {
        // 1. Antigravity Engineering
        self.profiles.insert(
            "antigravity-engineering".to_string(),
            AgentProfile {
                id: "antigravity-engineering".to_string(),
                name: "Antigravity Engineering".to_string(),
                kind: AgentProfileKind::AntigravityEngineering,
                description:
                    "Full suite configuration optimized for Google Antigravity IDE and sidecars"
                        .to_string(),
                target_agents: vec![AgentId::Antigravity],
                enabled_rules: vec![
                    "engineering-universal".to_string(),
                    "tdd-workflow".to_string(),
                    "graphify".to_string(),
                    "prompt-optimizer".to_string(),
                ],
                enabled_skills: vec![
                    "backend-architect".to_string(),
                    "rust-systems-engineer".to_string(),
                    "security-auditor".to_string(),
                    "production-code-reviewer".to_string(),
                ],
                mcp_servers: vec![
                    "sequential-thinking".to_string(),
                    "playwright".to_string(),
                    "filesystem".to_string(),
                ],
                validation_policy: "strict".to_string(),
                permissions: AgentPermissions::default(),
            },
        );

        // 2. Claude Code Engineering
        self.profiles.insert(
            "claude-code-engineering".to_string(),
            AgentProfile {
                id: "claude-code-engineering".to_string(),
                name: "Claude Code Engineering".to_string(),
                kind: AgentProfileKind::ClaudeCodeEngineering,
                description:
                    "Optimized instructions, runbooks, and MCP servers for Claude Code CLI"
                        .to_string(),
                target_agents: vec![AgentId::ClaudeCode],
                enabled_rules: vec![
                    "engineering-universal".to_string(),
                    "tdd-workflow".to_string(),
                ],
                enabled_skills: vec![
                    "backend-architect".to_string(),
                    "security-auditor".to_string(),
                ],
                mcp_servers: vec!["sequential-thinking".to_string(), "filesystem".to_string()],
                validation_policy: "standard".to_string(),
                permissions: AgentPermissions {
                    allow_hooks: false,
                    allow_file_write: true,
                    allow_network: false,
                    require_approval_for_hooks: true,
                },
            },
        );

        // 3. OpenCode Engineering
        self.profiles.insert(
            "opencode-engineering".to_string(),
            AgentProfile {
                id: "opencode-engineering".to_string(),
                name: "OpenCode Engineering".to_string(),
                kind: AgentProfileKind::OpenCodeEngineering,
                description:
                    "Agent specifications, instructions, and tools for OpenCode agent platform"
                        .to_string(),
                target_agents: vec![AgentId::OpenCode],
                enabled_rules: vec![
                    "engineering-universal".to_string(),
                    "tdd-workflow".to_string(),
                ],
                enabled_skills: vec![
                    "backend-architect".to_string(),
                    "devops-engineer".to_string(),
                ],
                mcp_servers: vec!["sequential-thinking".to_string()],
                validation_policy: "standard".to_string(),
                permissions: AgentPermissions::default(),
            },
        );

        // 4. Cursor Engineering
        self.profiles.insert(
            "cursor-engineering".to_string(),
            AgentProfile {
                id: "cursor-engineering".to_string(),
                name: "Cursor Engineering".to_string(),
                kind: AgentProfileKind::CursorEngineering,
                description: "MDC rules, legacy .cursorrules, and MCP integrations for Cursor IDE"
                    .to_string(),
                target_agents: vec![AgentId::Cursor],
                enabled_rules: vec![
                    "engineering-universal".to_string(),
                    "tdd-workflow".to_string(),
                ],
                enabled_skills: vec![
                    "backend-architect".to_string(),
                    "production-code-reviewer".to_string(),
                ],
                mcp_servers: vec!["sequential-thinking".to_string(), "filesystem".to_string()],
                validation_policy: "standard".to_string(),
                permissions: AgentPermissions::default(),
            },
        );

        // 5. Multi-Agent Engineering
        self.profiles.insert(
            "multi-agent-engineering".to_string(),
            AgentProfile {
                id: "multi-agent-engineering".to_string(),
                name: "Multi-Agent Universal Engineering".to_string(),
                kind: AgentProfileKind::MultiAgentEngineering,
                description: "Synchronized universal rules, runbooks, and MCP servers across all supported coding agents".to_string(),
                target_agents: vec![
                    AgentId::Antigravity,
                    AgentId::ClaudeCode,
                    AgentId::OpenCode,
                    AgentId::Cursor,
                ],
                enabled_rules: vec![
                    "engineering-universal".to_string(),
                    "tdd-workflow".to_string(),
                    "graphify".to_string(),
                    "prompt-optimizer".to_string(),
                ],
                enabled_skills: vec![
                    "backend-architect".to_string(),
                    "rust-systems-engineer".to_string(),
                    "security-auditor".to_string(),
                    "production-code-reviewer".to_string(),
                ],
                mcp_servers: vec![
                    "sequential-thinking".to_string(),
                    "playwright".to_string(),
                    "filesystem".to_string(),
                ],
                validation_policy: "strict".to_string(),
                permissions: AgentPermissions::default(),
            },
        );
    }

    #[must_use]
    pub fn get_profile(&self, id: &str) -> Option<&AgentProfile> {
        let key = id.trim().to_lowercase().replace('_', "-");
        self.profiles.get(&key)
    }

    #[must_use]
    pub fn list_profiles(&self) -> Vec<&AgentProfile> {
        let mut list: Vec<&AgentProfile> = self.profiles.values().collect();
        list.sort_by(|a, b| a.id.cmp(&b.id));
        list
    }

    pub fn validate_profile(&self, profile: &AgentProfile) -> Result<(), AgentError> {
        if profile.id.trim().is_empty() {
            return Err(AgentError::InvalidProfile(
                "Profile id cannot be empty".to_string(),
            ));
        }
        if profile.target_agents.is_empty() {
            return Err(AgentError::InvalidProfile(
                "Profile must target at least one agent".to_string(),
            ));
        }
        Ok(())
    }
}
