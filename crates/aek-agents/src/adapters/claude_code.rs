//! Claude Code CLI adapter implementation.

use crate::adapter::{AgentAdapter, AgentDetection, AgentError, InstallReport, McpMergeReport};
use crate::capabilities::AgentCapabilities;
use crate::discovery::AgentDiscoveryEngine;
use crate::model::AgentId;
use crate::translator::AgentConfigTranslator;
use aek_mcp::config::McpServerConfig;
use aek_rules::model::Rule;
use aek_skills::model::SkillBundle;
use serde_json::json;
use std::fs;
use std::path::{Path, PathBuf};

/// Adapter integrating with Anthropic Claude Code CLI.
#[derive(Debug, Clone, Default)]
pub struct ClaudeCodeAdapter {
    translator: AgentConfigTranslator,
}

impl ClaudeCodeAdapter {
    #[must_use]
    pub fn new() -> Self {
        Self {
            translator: AgentConfigTranslator::new(),
        }
    }

    fn user_claude_dir() -> Option<PathBuf> {
        if let Ok(userprofile) = std::env::var("USERPROFILE") {
            if !userprofile.is_empty() {
                return Some(PathBuf::from(userprofile).join(".claude"));
            }
        }
        if let Ok(home) = std::env::var("HOME") {
            if !home.is_empty() {
                return Some(PathBuf::from(home).join(".claude"));
            }
        }
        None
    }
}

impl AgentAdapter for ClaudeCodeAdapter {
    fn name(&self) -> &'static str {
        "Claude Code"
    }

    fn agent_id(&self) -> AgentId {
        AgentId::ClaudeCode
    }

    fn detect(&self) -> AgentDetection {
        AgentDiscoveryEngine::new().detect_claude_code(None)
    }

    fn capabilities(&self) -> AgentCapabilities {
        AgentCapabilities {
            supports_global_rules: true,
            supports_workspace_rules: true,
            supports_skills: true,
            supports_mcp: true,
            max_rule_bytes: 32_000,
            rule_token_budget: 25_000,
        }
    }

    fn global_rules_dir(&self) -> Option<PathBuf> {
        Self::user_claude_dir()
    }

    fn workspace_rules_dir(&self, workspace: &Path) -> PathBuf {
        workspace.to_path_buf()
    }

    fn global_skills_dir(&self) -> Option<PathBuf> {
        Self::user_claude_dir().map(|d| d.join("skills"))
    }

    fn workspace_skills_dir(&self, workspace: &Path) -> PathBuf {
        workspace.join(".claude").join("skills")
    }

    fn mcp_config_path(&self) -> Option<PathBuf> {
        Self::user_claude_dir().map(|d| d.join("mcp.json"))
    }

    fn workspace_mcp_path(&self, workspace: &Path) -> PathBuf {
        workspace.join(".claude").join("mcp.json")
    }

    fn export_mcp_config(&self, servers: &[McpServerConfig]) -> serde_json::Value {
        self.translator
            .translate_mcp_servers(AgentId::ClaudeCode, servers)
            .unwrap_or_else(|_| json!({ "mcpServers": {} }))
    }

    fn merge_mcp_config(
        &self,
        servers: &[McpServerConfig],
        target_file: Option<&Path>,
        dry_run: bool,
    ) -> Result<McpMergeReport, AgentError> {
        let path = match target_file {
            Some(p) => p.to_path_buf(),
            None => self.mcp_config_path().ok_or_else(|| {
                AgentError::Security("Cannot resolve Claude mcp.json path".to_string())
            })?,
        };

        let mut existing_val: serde_json::Value = if path.is_file() {
            let content = fs::read_to_string(&path)?;
            serde_json::from_str(&content).unwrap_or_else(|_| json!({ "mcpServers": {} }))
        } else {
            json!({ "mcpServers": {} })
        };

        let mut conflicts = Vec::new();
        let mut merged_count = 0;

        let exported = self.export_mcp_config(servers);
        if let (Some(new_servers), Some(existing_servers)) = (
            exported.get("mcpServers").and_then(|v| v.as_object()),
            existing_val
                .get_mut("mcpServers")
                .and_then(|v| v.as_object_mut()),
        ) {
            for (key, val) in new_servers {
                if existing_servers.contains_key(key) {
                    conflicts.push(key.clone());
                } else {
                    merged_count += 1;
                }
                existing_servers.insert(key.clone(), val.clone());
            }
        }

        let total_servers = existing_val
            .get("mcpServers")
            .and_then(|v| v.as_object())
            .map(|m| m.len())
            .unwrap_or(0);

        let backup_file = if !dry_run && path.is_file() {
            let timestamp = crate::model::utc_now_iso().replace([':', ' ', '-'], "_");
            let backup_path = path.with_extension(format!("json.{timestamp}.bak"));
            fs::copy(&path, &backup_path)?;
            Some(backup_path)
        } else {
            None
        };

        if !dry_run {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }
            let serialized = serde_json::to_string_pretty(&existing_val)?;
            fs::write(&path, serialized)?;
        }

        Ok(McpMergeReport {
            total_servers,
            merged_count,
            conflicts_detected: conflicts,
            target_file: path,
            backup_file,
            dry_run,
        })
    }

    fn install_rules(
        &self,
        rules: &[Rule],
        target_dir: &Path,
        dry_run: bool,
    ) -> Result<InstallReport, AgentError> {
        let claude_md_path = if target_dir.ends_with("CLAUDE.md") {
            target_dir.to_path_buf()
        } else {
            target_dir.join("CLAUDE.md")
        };

        let content = self.translator.translate_claude_rules(rules);

        if !dry_run {
            if let Some(parent) = claude_md_path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(&claude_md_path, &content)?;
        }

        Ok(InstallReport {
            installed_count: rules.len(),
            target_dir: claude_md_path,
            dry_run,
        })
    }

    fn install_skills(
        &self,
        skills: &[SkillBundle],
        target_dir: &Path,
        dry_run: bool,
    ) -> Result<InstallReport, AgentError> {
        let mut count = 0;
        for skill in skills {
            let clean_name = self
                .translator
                .sanitize_relative_path(&skill.metadata.name)?;
            let skill_dir = target_dir.join(&clean_name);
            let skill_md = skill_dir.join("SKILL.md");

            if !dry_run {
                fs::create_dir_all(&skill_dir)?;
                let content = self.translator.format_skill_markdown(skill);
                fs::write(&skill_md, content)?;
            }
            count += 1;
        }

        Ok(InstallReport {
            installed_count: count,
            target_dir: target_dir.to_path_buf(),
            dry_run,
        })
    }

    fn recommend_skills_for_path(&self, _workspace: &Path) -> Vec<String> {
        vec![
            "backend-architect".to_string(),
            "security-auditor".to_string(),
            "production-code-reviewer".to_string(),
        ]
    }

    fn generate_validation_guidance(&self, _workspace: &Path) -> String {
        "Claude Code Validation:\nRun `aek validate` prior to ending turns.".to_string()
    }

    fn export_prompt_instruction(
        &self,
        prompt: &aek_prompt::model::PromptRecord,
        target_dir: &Path,
        dry_run: bool,
    ) -> Result<PathBuf, AgentError> {
        let dest = target_dir.join("CLAUDE.md");
        if !dry_run {
            let content = format!("\n\n## {}\n\n{}\n", prompt.name, prompt.active_content());
            if dest.exists() {
                use std::io::Write;
                let mut f = fs::OpenOptions::new().append(true).open(&dest)?;
                f.write_all(content.as_bytes())?;
            } else {
                fs::write(&dest, content)?;
            }
        }
        Ok(dest)
    }

    fn generate_security_guidance(&self, _workspace: &Path) -> String {
        "Ensure Claude Code hooks require manual approval in settings.".to_string()
    }

    fn export_ci_workflow(
        &self,
        target_dir: &Path,
        workflow_content: &str,
        dry_run: bool,
    ) -> Result<PathBuf, AgentError> {
        let dest = target_dir.join("ci.yml");
        if !dry_run {
            if let Some(parent) = dest.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(&dest, workflow_content)?;
        }
        Ok(dest)
    }
}
