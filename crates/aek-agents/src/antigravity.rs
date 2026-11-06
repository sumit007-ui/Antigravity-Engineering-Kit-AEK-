//! Google Antigravity adapter implementation.

use crate::adapter::{AgentAdapter, AgentDetection, AgentError, InstallReport, McpMergeReport};
use crate::capabilities::AgentCapabilities;
use aek_mcp::config::{McpServerConfig, McpTransportConfig};
use aek_platform::detection::find_executable_in_path;
use aek_platform::paths::PlatformPaths;
use aek_rules::installer::install_rule;
use aek_rules::model::Rule;
use aek_skills::installer::install_skill;
use aek_skills::model::SkillBundle;
use std::path::{Path, PathBuf};

/// Adapter interfacing with Google Antigravity runtime, rules, and skills roots.
#[derive(Debug, Clone)]
pub struct AntigravityAdapter {
    paths: PlatformPaths,
}

impl Default for AntigravityAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl AntigravityAdapter {
    /// Creates a new `AntigravityAdapter`.
    #[must_use]
    pub fn new() -> Self {
        Self {
            paths: PlatformPaths::resolve(),
        }
    }
}

impl AgentAdapter for AntigravityAdapter {
    fn name(&self) -> &'static str {
        "Google Antigravity"
    }

    fn detect(&self) -> AgentDetection {
        let mut evidence = Vec::new();

        // 1. agy CLI
        if let Some(bin) = find_executable_in_path("agy") {
            evidence.push(format!("CLI binary: {}", bin.display()));
        }

        // 2. Global Antigravity config / MCP
        if let Some(ref dir) = self.paths.antigravity_dir {
            let mcp_cfg = dir.join("config").join("mcp_config.json");
            if mcp_cfg.is_file() {
                evidence.push(format!("Global MCP Config: {}", mcp_cfg.display()));
            } else if dir.join("brain").is_dir() {
                evidence.push(format!("Platform Directory: {}", dir.display()));
            }
        }

        let is_detected = !evidence.is_empty();

        AgentDetection {
            agent_name: self.name().to_string(),
            is_detected,
            evidence,
        }
    }

    fn capabilities(&self) -> AgentCapabilities {
        AgentCapabilities {
            supports_global_rules: true,
            supports_workspace_rules: true,
            supports_skills: true,
            supports_mcp: true,
            max_rule_bytes: 24_000,
            rule_token_budget: 20_000,
        }
    }

    fn global_rules_dir(&self) -> Option<PathBuf> {
        self.paths
            .antigravity_dir
            .as_ref()
            .map(|d| d.join("config").join("rules"))
    }

    fn workspace_rules_dir(&self, workspace: &Path) -> PathBuf {
        workspace.join(".agents").join("rules")
    }

    fn global_skills_dir(&self) -> Option<PathBuf> {
        self.paths
            .antigravity_dir
            .as_ref()
            .map(|d| d.join("config").join("skills"))
    }

    fn workspace_skills_dir(&self, workspace: &Path) -> PathBuf {
        workspace.join(".agents").join("skills")
    }

    fn mcp_config_path(&self) -> Option<PathBuf> {
        if let Some(ref dir) = self.paths.antigravity_dir {
            let cfg1 = dir.join("config").join("mcp_config.json");
            if cfg1.is_file() {
                return Some(cfg1);
            }
            let cfg2 = dir.join("mcp_config.json");
            if cfg2.is_file() {
                return Some(cfg2);
            }
            Some(cfg1)
        } else {
            None
        }
    }

    fn export_mcp_config(&self, servers: &[McpServerConfig]) -> serde_json::Value {
        let mut map = serde_json::Map::new();
        for s in servers {
            if !s.enabled {
                continue;
            }
            let entry = match &s.transport {
                McpTransportConfig::Stdio {
                    command, args, env, ..
                } => {
                    let mut obj = serde_json::Map::new();
                    obj.insert(
                        "command".to_string(),
                        serde_json::Value::String(command.clone()),
                    );
                    obj.insert(
                        "args".to_string(),
                        serde_json::Value::Array(
                            args.iter()
                                .map(|a| serde_json::Value::String(a.clone()))
                                .collect(),
                        ),
                    );
                    if !env.is_empty() {
                        let mut env_obj = serde_json::Map::new();
                        for (k, v) in env {
                            env_obj.insert(k.clone(), serde_json::Value::String(v.clone()));
                        }
                        obj.insert("env".to_string(), serde_json::Value::Object(env_obj));
                    }
                    serde_json::Value::Object(obj)
                }
                McpTransportConfig::Http { url, .. } => {
                    let mut obj = serde_json::Map::new();
                    obj.insert("url".to_string(), serde_json::Value::String(url.clone()));
                    serde_json::Value::Object(obj)
                }
            };
            map.insert(s.id.clone(), entry);
        }

        serde_json::json!({
            "mcpServers": map
        })
    }

    fn merge_mcp_config(
        &self,
        servers: &[McpServerConfig],
        target_file: Option<&Path>,
        dry_run: bool,
    ) -> Result<McpMergeReport, AgentError> {
        let dest_path = match target_file {
            Some(p) => p.to_path_buf(),
            None => self.mcp_config_path().ok_or_else(|| {
                AgentError::AgentNotDetected("Antigravity MCP config path not resolved".to_string())
            })?,
        };

        let mut root: serde_json::Value = if dest_path.exists() {
            let content = std::fs::read_to_string(&dest_path)?;
            serde_json::from_str(&content)
                .unwrap_or_else(|_| serde_json::json!({ "mcpServers": {} }))
        } else {
            serde_json::json!({ "mcpServers": {} })
        };

        if !root.is_object() {
            root = serde_json::json!({ "mcpServers": {} });
        }

        let root_obj = root.as_object_mut().ok_or_else(|| {
            AgentError::Security("Root of MCP config is not an object".to_string())
        })?;

        let mcp_servers_val = root_obj
            .entry("mcpServers")
            .or_insert_with(|| serde_json::json!({}));
        let mcp_servers = mcp_servers_val.as_object_mut().ok_or_else(|| {
            AgentError::Security("mcpServers in target config is not an object".to_string())
        })?;

        let mut conflicts = Vec::new();
        let mut merged = 0;

        let exported = self.export_mcp_config(servers);
        if let Some(new_map) = exported.get("mcpServers").and_then(|m| m.as_object()) {
            for (key, val) in new_map {
                if mcp_servers.contains_key(key) {
                    conflicts.push(key.clone());
                }
                mcp_servers.insert(key.clone(), val.clone());
                merged += 1;
            }
        }

        let total_servers = mcp_servers.len();
        let mut backup_path = None;

        if !dry_run {
            if let Some(parent) = dest_path.parent() {
                std::fs::create_dir_all(parent)?;
            }

            if dest_path.exists() {
                let ts = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs();
                let backup = dest_path.with_extension(format!("bak.{ts}"));
                std::fs::copy(&dest_path, &backup)?;
                backup_path = Some(backup);
            }

            let formatted = serde_json::to_string_pretty(&root)?;
            std::fs::write(&dest_path, formatted)?;
        }

        Ok(McpMergeReport {
            total_servers,
            merged_count: merged,
            conflicts_detected: conflicts,
            target_file: dest_path,
            backup_file: backup_path,
            dry_run,
        })
    }

    fn install_rules(
        &self,
        rules: &[Rule],
        target_dir: &Path,
        dry_run: bool,
    ) -> Result<InstallReport, AgentError> {
        let mut installed = 0;
        for rule in rules {
            install_rule(rule, target_dir, dry_run)?;
            installed += 1;
        }

        Ok(InstallReport {
            installed_count: installed,
            target_dir: target_dir.to_path_buf(),
            dry_run,
        })
    }

    fn install_skills(
        &self,
        skills: &[SkillBundle],
        target_dir: &Path,
        dry_run: bool,
    ) -> Result<InstallReport, AgentError> {
        let mut installed = 0;
        for skill in skills {
            install_skill(skill, target_dir, dry_run)?;
            installed += 1;
        }

        Ok(InstallReport {
            installed_count: installed,
            target_dir: target_dir.to_path_buf(),
            dry_run,
        })
    }

    fn recommend_skills_for_path(&self, workspace: &Path) -> Vec<String> {
        let detector = aek_languages::detection::ProjectDetector::new();
        let projects = detector.detect(workspace).unwrap_or_default();
        let mut skills = std::collections::BTreeSet::new();

        for project in &projects {
            if let Some(provider) = aek_languages::providers::find_provider(&project.language) {
                for skill in provider.recommended_skills() {
                    skills.insert(skill.to_string());
                }
            }
        }

        skills.into_iter().collect()
    }

    fn generate_validation_guidance(&self, workspace: &Path) -> String {
        let detector = aek_languages::detection::ProjectDetector::new();
        let projects = detector.detect(workspace).unwrap_or_default();

        if projects.is_empty() {
            return "No recognizable language projects detected in workspace.".to_string();
        }

        let mut output = String::from("# Google Antigravity Validation Guidance\n\n");
        for project in &projects {
            let pipeline = aek_validation::pipeline::ValidationPipeline::for_project(project);
            output.push_str(&format!(
                "## Project: {} ({})\n",
                project.name, project.language
            ));
            output.push_str(&format!("Directory: `{}`\n\n", project.path.display()));
            output.push_str("Recommended verification commands:\n");
            for step in &pipeline.steps {
                output.push_str(&format!(
                    "- **{}** ({}): `{} {}`\n",
                    step.name,
                    step.stage,
                    step.program,
                    step.args.join(" ")
                ));
            }
            output.push('\n');
        }

        output
    }
}
