//! Cursor AI IDE adapter implementation.

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

/// Adapter integrating with Cursor AI IDE.
#[derive(Debug, Clone, Default)]
pub struct CursorAdapter {
    translator: AgentConfigTranslator,
}

impl CursorAdapter {
    #[must_use]
    pub fn new() -> Self {
        Self {
            translator: AgentConfigTranslator::new(),
        }
    }

    fn user_cursor_dir() -> Option<PathBuf> {
        if let Ok(userprofile) = std::env::var("USERPROFILE") {
            if !userprofile.is_empty() {
                return Some(PathBuf::from(userprofile).join(".cursor"));
            }
        }
        if let Ok(home) = std::env::var("HOME") {
            if !home.is_empty() {
                return Some(PathBuf::from(home).join(".cursor"));
            }
        }
        None
    }
}

impl AgentAdapter for CursorAdapter {
    fn name(&self) -> &'static str {
        "Cursor"
    }

    fn agent_id(&self) -> AgentId {
        AgentId::Cursor
    }

    fn detect(&self) -> AgentDetection {
        AgentDiscoveryEngine::new().detect_cursor(None)
    }

    fn capabilities(&self) -> AgentCapabilities {
        AgentCapabilities {
            supports_global_rules: true,
            supports_workspace_rules: true,
            supports_skills: true,
            supports_mcp: true,
            max_rule_bytes: 16_000,
            rule_token_budget: 15_000,
        }
    }

    fn global_rules_dir(&self) -> Option<PathBuf> {
        Self::user_cursor_dir().map(|d| d.join("rules"))
    }

    fn workspace_rules_dir(&self, workspace: &Path) -> PathBuf {
        workspace.join(".cursor").join("rules")
    }

    fn global_skills_dir(&self) -> Option<PathBuf> {
        Self::user_cursor_dir().map(|d| d.join("skills"))
    }

    fn workspace_skills_dir(&self, workspace: &Path) -> PathBuf {
        workspace.join(".cursor").join("skills")
    }

    fn mcp_config_path(&self) -> Option<PathBuf> {
        Self::user_cursor_dir().map(|d| d.join("mcp.json"))
    }

    fn workspace_mcp_path(&self, workspace: &Path) -> PathBuf {
        workspace.join(".cursor").join("mcp.json")
    }

    fn export_mcp_config(&self, servers: &[McpServerConfig]) -> serde_json::Value {
        self.translator
            .translate_mcp_servers(AgentId::Cursor, servers)
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
                AgentError::Security("Cannot resolve Cursor mcp.json path".to_string())
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
        let ws_root = if target_dir.ends_with("rules") {
            target_dir
                .parent()
                .and_then(|p| p.parent())
                .unwrap_or(target_dir)
        } else {
            target_dir
        };

        let mdc_rules_dir = ws_root.join(".cursor").join("rules");

        if !dry_run {
            fs::create_dir_all(&mdc_rules_dir)?;
        }

        for rule in rules {
            let clean_name = self.translator.sanitize_relative_path(&rule.id)?;
            let file_name = format!("{}.mdc", clean_name.display());
            let mdc_path = mdc_rules_dir.join(file_name);
            let content = self.translator.translate_cursor_mdc_rule(rule);

            if !dry_run {
                fs::write(&mdc_path, content)?;
            }
        }

        // Also install legacy fallback .cursorrules in workspace root
        let legacy_file = ws_root.join(".cursorrules");
        let legacy_content = self.translator.translate_cursor_legacy_rules(rules);
        if !dry_run {
            fs::write(&legacy_file, legacy_content)?;
        }

        Ok(InstallReport {
            installed_count: rules.len(),
            target_dir: mdc_rules_dir,
            dry_run,
        })
    }

    fn install_skills(
        &self,
        skills: &[SkillBundle],
        target_dir: &Path,
        dry_run: bool,
    ) -> Result<InstallReport, AgentError> {
        let skills_root = target_dir.join(".cursor").join("skills");
        let mut count = 0;

        for skill in skills {
            let clean_name = self
                .translator
                .sanitize_relative_path(&skill.metadata.name)?;
            let skill_dir = skills_root.join(&clean_name);
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
            target_dir: skills_root,
            dry_run,
        })
    }

    fn recommend_skills_for_path(&self, _workspace: &Path) -> Vec<String> {
        vec![
            "backend-architect".to_string(),
            "production-code-reviewer".to_string(),
        ]
    }

    fn generate_validation_guidance(&self, _workspace: &Path) -> String {
        "Cursor AI Rules Verification:\nRun `aek validate`.".to_string()
    }

    fn export_prompt_instruction(
        &self,
        prompt: &aek_prompt::model::PromptRecord,
        target_dir: &Path,
        dry_run: bool,
    ) -> Result<PathBuf, AgentError> {
        let rules_dir = target_dir.join(".cursor").join("rules");
        let dest = rules_dir.join(format!("{}.mdc", prompt.id));
        if !dry_run {
            fs::create_dir_all(&rules_dir)?;
            let content = format!(
                "---\ndescription: \"Prompt instruction: {}\"\nglobs: \"*\"\nalwaysApply: true\n---\n\n{}\n",
                prompt.name,
                prompt.active_content()
            );
            fs::write(&dest, content)?;
        }
        Ok(dest)
    }

    fn generate_security_guidance(&self, _workspace: &Path) -> String {
        "Verify Cursor MCP configuration file containment and permissions.".to_string()
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
