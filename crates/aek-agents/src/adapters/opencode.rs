//! OpenCode agent adapter implementation.

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

/// Adapter integrating with OpenCode AI agent platform.
#[derive(Debug, Clone, Default)]
pub struct OpenCodeAdapter {
    translator: AgentConfigTranslator,
}

impl OpenCodeAdapter {
    #[must_use]
    pub fn new() -> Self {
        Self {
            translator: AgentConfigTranslator::new(),
        }
    }

    fn user_opencode_dir() -> Option<PathBuf> {
        if let Ok(userprofile) = std::env::var("USERPROFILE") {
            if !userprofile.is_empty() {
                return Some(PathBuf::from(userprofile).join(".opencode"));
            }
        }
        if let Ok(home) = std::env::var("HOME") {
            if !home.is_empty() {
                return Some(PathBuf::from(home).join(".opencode"));
            }
        }
        None
    }
}

impl AgentAdapter for OpenCodeAdapter {
    fn name(&self) -> &'static str {
        "OpenCode"
    }

    fn agent_id(&self) -> AgentId {
        AgentId::OpenCode
    }

    fn detect(&self) -> AgentDetection {
        AgentDiscoveryEngine::new().detect_opencode(None)
    }

    fn capabilities(&self) -> AgentCapabilities {
        AgentCapabilities {
            supports_global_rules: true,
            supports_workspace_rules: true,
            supports_skills: true,
            supports_mcp: true,
            max_rule_bytes: 32_000,
            rule_token_budget: 20_000,
        }
    }

    fn global_rules_dir(&self) -> Option<PathBuf> {
        Self::user_opencode_dir()
    }

    fn workspace_rules_dir(&self, workspace: &Path) -> PathBuf {
        workspace.to_path_buf()
    }

    fn global_skills_dir(&self) -> Option<PathBuf> {
        Self::user_opencode_dir().map(|d| d.join("skills"))
    }

    fn workspace_skills_dir(&self, workspace: &Path) -> PathBuf {
        workspace.join(".opencode").join("skills")
    }

    fn mcp_config_path(&self) -> Option<PathBuf> {
        Self::user_opencode_dir().map(|d| d.join("config.json"))
    }

    fn workspace_mcp_path(&self, workspace: &Path) -> PathBuf {
        workspace.join(".opencode").join("config.json")
    }

    fn export_mcp_config(&self, servers: &[McpServerConfig]) -> serde_json::Value {
        self.translator
            .translate_mcp_servers(AgentId::OpenCode, servers)
            .unwrap_or_else(|_| json!({ "mcp_servers": {} }))
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
                AgentError::Security("Cannot resolve OpenCode config.json path".to_string())
            })?,
        };

        let mut existing_val: serde_json::Value = if path.is_file() {
            let content = fs::read_to_string(&path)?;
            serde_json::from_str(&content).unwrap_or_else(|_| json!({ "mcp_servers": {} }))
        } else {
            json!({ "mcp_servers": {} })
        };

        let mut conflicts = Vec::new();
        let mut merged_count = 0;

        let exported = self.export_mcp_config(servers);
        if let (Some(new_servers), Some(existing_servers)) = (
            exported.get("mcp_servers").and_then(|v| v.as_object()),
            existing_val
                .get_mut("mcp_servers")
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
            .get("mcp_servers")
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
        let agents_md_path = if target_dir.ends_with("AGENTS.md") {
            target_dir.to_path_buf()
        } else {
            target_dir.join("AGENTS.md")
        };

        let content = self.translator.translate_opencode_rules(rules);

        if !dry_run {
            if let Some(parent) = agents_md_path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(&agents_md_path, &content)?;

            // Also ensure .opencode/config.json exists
            let ws_root = if agents_md_path.ends_with("AGENTS.md") {
                agents_md_path.parent().unwrap_or(target_dir)
            } else {
                target_dir
            };
            let config_dir = ws_root.join(".opencode");
            fs::create_dir_all(&config_dir)?;
            let config_json = config_dir.join("config.json");
            if !config_json.exists() {
                fs::write(&config_json, "{\n  \"schema_version\": \"1.0.0\"\n}\n")?;
            }
        }

        Ok(InstallReport {
            installed_count: rules.len(),
            target_dir: agents_md_path,
            dry_run,
        })
    }

    fn install_skills(
        &self,
        skills: &[SkillBundle],
        target_dir: &Path,
        dry_run: bool,
    ) -> Result<InstallReport, AgentError> {
        let skills_root = target_dir.join(".opencode").join("skills");
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
            "devops-engineer".to_string(),
        ]
    }

    fn generate_validation_guidance(&self, _workspace: &Path) -> String {
        "OpenCode Agent Quality Checks:\nRun `aek validate`.".to_string()
    }

    fn export_prompt_instruction(
        &self,
        prompt: &aek_prompt::model::PromptRecord,
        target_dir: &Path,
        dry_run: bool,
    ) -> Result<PathBuf, AgentError> {
        let dest = target_dir.join("AGENTS.md");
        if !dry_run {
            let content = format!("\n\n### {}\n\n{}\n", prompt.name, prompt.active_content());
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
        "Audit OpenCode permissions profile in .opencode/config.json".to_string()
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
