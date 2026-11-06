//! Configuration translation engine converting AEK artifacts into agent-native formats.

use crate::errors::AgentError;
use crate::model::AgentId;
use aek_mcp::config::{McpServerConfig, McpTransportConfig};
use aek_rules::model::Rule;
use aek_skills::model::SkillBundle;
use serde_json::json;
use std::path::{Component, Path, PathBuf};

/// Translates universal AEK rules, skills, and MCP configurations into agent-native representations.
#[derive(Debug, Clone, Default)]
pub struct AgentConfigTranslator;

impl AgentConfigTranslator {
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Verifies and sanitizes a relative path to prevent directory traversal attacks.
    pub fn sanitize_relative_path(&self, raw_path: &str) -> Result<PathBuf, AgentError> {
        if raw_path.contains("..")
            || raw_path.starts_with('/')
            || raw_path.starts_with('\\')
            || raw_path.contains(':')
        {
            return Err(AgentError::Security(format!(
                "Path traversal detected in '{raw_path}'"
            )));
        }

        let path = Path::new(raw_path);
        let mut clean = PathBuf::new();

        for component in path.components() {
            match component {
                Component::Normal(c) => clean.push(c),
                Component::CurDir => {}
                Component::ParentDir | Component::RootDir | Component::Prefix(_) => {
                    return Err(AgentError::Security(format!(
                        "Path traversal detected in '{raw_path}'"
                    )));
                }
            }
        }

        if clean.as_os_str().is_empty() {
            return Err(AgentError::Security(
                "Resolved path is empty or root".to_string(),
            ));
        }

        Ok(clean)
    }

    /// Translates rules for Claude Code into consolidated `CLAUDE.md`.
    #[must_use]
    pub fn translate_claude_rules(&self, rules: &[Rule]) -> String {
        let mut out = String::from("# CLAUDE.md — Project Instructions & Engineering Rules\n\n");
        out.push_str("> Managed automatically by Antigravity Engineering Kit (AEK).\n\n");

        for rule in rules {
            out.push_str(&format!("## {}\n\n", rule.name));
            out.push_str(&rule.content);
            out.push_str("\n\n---\n\n");
        }

        out
    }

    /// Translates rules for OpenCode into consolidated `AGENTS.md`.
    #[must_use]
    pub fn translate_opencode_rules(&self, rules: &[Rule]) -> String {
        let mut out = String::from("# AGENTS.md — OpenCode Instructions & Constraints\n\n");
        out.push_str("> Managed automatically by Antigravity Engineering Kit (AEK).\n\n");

        for rule in rules {
            out.push_str(&format!("### {}\n\n", rule.name));
            out.push_str(&rule.content);
            out.push_str("\n\n");
        }

        out
    }

    /// Translates an individual rule into Cursor MDC format (`.cursor/rules/<name>.mdc`).
    #[must_use]
    pub fn translate_cursor_mdc_rule(&self, rule: &Rule) -> String {
        let mut out = String::from("---\n");
        out.push_str(&format!("description: \"{}\"\n", rule.description));
        out.push_str("globs: \"*\"\n");
        out.push_str("alwaysApply: true\n");
        out.push_str("---\n\n");
        out.push_str(&rule.content);
        out.push('\n');
        out
    }

    /// Translates rules into legacy Cursor `.cursorrules` document.
    #[must_use]
    pub fn translate_cursor_legacy_rules(&self, rules: &[Rule]) -> String {
        let mut out = String::from("# .cursorrules — Project Invariants\n\n");
        for rule in rules {
            out.push_str(&format!("## {}\n", rule.name));
            out.push_str(&rule.content);
            out.push_str("\n\n");
        }
        out
    }

    /// Translates AEK MCP server definitions into target agent's native JSON representation.
    pub fn translate_mcp_servers(
        &self,
        agent_id: AgentId,
        servers: &[McpServerConfig],
    ) -> Result<serde_json::Value, AgentError> {
        let mut mcp_servers_map = serde_json::Map::new();

        for server in servers {
            if !server.enabled {
                continue;
            }

            match &server.transport {
                McpTransportConfig::Stdio {
                    command, args, env, ..
                } => {
                    let mut entry = serde_json::Map::new();
                    entry.insert("command".to_string(), json!(command));
                    entry.insert("args".to_string(), json!(args));
                    if !env.is_empty() {
                        entry.insert("env".to_string(), json!(env));
                    }
                    mcp_servers_map.insert(server.id.clone(), serde_json::Value::Object(entry));
                }
                McpTransportConfig::Http { url, headers } => {
                    if agent_id == AgentId::ClaudeCode {
                        // Claude Code CLI prefers stdio or requires custom bridge
                        tracing::warn!(
                            "Server '{}' uses HTTP transport which may require proxy in Claude Code",
                            server.id
                        );
                    }
                    let mut entry = serde_json::Map::new();
                    entry.insert("url".to_string(), json!(url));
                    if !headers.is_empty() {
                        entry.insert("headers".to_string(), json!(headers));
                    }
                    mcp_servers_map.insert(server.id.clone(), serde_json::Value::Object(entry));
                }
            }
        }

        match agent_id {
            AgentId::OpenCode => Ok(json!({
                "mcp_servers": mcp_servers_map
            })),
            AgentId::Antigravity | AgentId::ClaudeCode | AgentId::Cursor => Ok(json!({
                "mcpServers": mcp_servers_map
            })),
        }
    }

    /// Formats a skill bundle into standard markdown runbook format.
    #[must_use]
    pub fn format_skill_markdown(&self, skill: &SkillBundle) -> String {
        let mut out = String::from("---\n");
        out.push_str(&format!("name: \"{}\"\n", skill.metadata.name));
        out.push_str(&format!(
            "description: \"{}\"\n",
            skill.metadata.description
        ));
        out.push_str("---\n\n");
        out.push_str(&skill.body);
        out.push('\n');
        out
    }
}
