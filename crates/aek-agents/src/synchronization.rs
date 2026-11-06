//! Multi-agent synchronization engine with conflict detection and ownership tracking.

use crate::conflicts::ConflictStrategy;
use crate::errors::AgentError;
use crate::model::{
    AgentId, AgentOwnershipManifest, ManagedFileRecord, SyncItem, SyncPlan, SyncReport, utc_now_iso,
};
use crate::translator::AgentConfigTranslator;
use aek_mcp::config::McpServerConfig;
use aek_rules::model::Rule;
use aek_skills::model::SkillBundle;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};

/// Coordinates atomic synchronization and conflict resolution across multiple coding agents.
#[derive(Debug, Clone)]
pub struct MultiAgentSynchronizer {
    translator: AgentConfigTranslator,
}

impl Default for MultiAgentSynchronizer {
    fn default() -> Self {
        Self::new()
    }
}

impl MultiAgentSynchronizer {
    #[must_use]
    pub fn new() -> Self {
        Self {
            translator: AgentConfigTranslator::new(),
        }
    }

    /// Computes the SHA256 hex digest of the string content.
    #[must_use]
    pub fn compute_sha256(content: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(content.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    /// Resolves the ownership manifest path in the workspace.
    #[must_use]
    pub fn manifest_path(workspace: &Path) -> PathBuf {
        workspace.join(".aek").join("managed-agents.toml")
    }

    /// Loads the existing ownership manifest from disk or returns a fresh one.
    pub fn load_manifest(workspace: &Path) -> AgentOwnershipManifest {
        let path = Self::manifest_path(workspace);
        if path.is_file() {
            if let Ok(content) = fs::read_to_string(&path) {
                if let Ok(manifest) = toml::from_str(&content) {
                    return manifest;
                }
            }
        }
        AgentOwnershipManifest::new()
    }

    /// Saves the ownership manifest to disk atomically.
    pub fn save_manifest(
        workspace: &Path,
        manifest: &AgentOwnershipManifest,
    ) -> Result<(), AgentError> {
        let path = Self::manifest_path(workspace);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let serialized = toml::to_string_pretty(manifest)?;
        fs::write(path, serialized)?;
        Ok(())
    }

    /// Plans the synchronization actions for the requested agents and artifacts without writing.
    pub fn plan_sync(
        &self,
        workspace: &Path,
        agents: &[AgentId],
        rules: &[Rule],
        skills: &[SkillBundle],
        servers: &[McpServerConfig],
    ) -> Result<SyncPlan, AgentError> {
        let manifest = Self::load_manifest(workspace);
        let mut items = Vec::new();
        let mut conflicts = Vec::new();

        for agent in agents {
            match agent {
                AgentId::Antigravity => {
                    // 1. Rules
                    let rules_dir = workspace.join(".gemini").join("rules");
                    for rule in rules {
                        let clean_name = self.translator.sanitize_relative_path(&rule.id)?;
                        let dest = rules_dir.join(format!("{}.md", clean_name.display()));
                        let content = format!("# {}\n\n{}", rule.name, rule.content);
                        self.push_sync_item(
                            *agent,
                            dest,
                            content,
                            &manifest,
                            &mut items,
                            &mut conflicts,
                        );
                    }
                    // 2. Skills
                    let skills_dir = workspace.join(".gemini").join("skills");
                    for skill in skills {
                        let clean_name = self
                            .translator
                            .sanitize_relative_path(&skill.metadata.name)?;
                        let dest = skills_dir.join(&clean_name).join("SKILL.md");
                        let content = self.translator.format_skill_markdown(skill);
                        self.push_sync_item(
                            *agent,
                            dest,
                            content,
                            &manifest,
                            &mut items,
                            &mut conflicts,
                        );
                    }
                    // 3. MCP
                    if !servers.is_empty() {
                        let dest = workspace.join(".gemini").join("mcp_config.json");
                        let json_val = self.translator.translate_mcp_servers(*agent, servers)?;
                        let content = serde_json::to_string_pretty(&json_val)?;
                        self.push_sync_item(
                            *agent,
                            dest,
                            content,
                            &manifest,
                            &mut items,
                            &mut conflicts,
                        );
                    }
                }
                AgentId::ClaudeCode => {
                    // 1. Rules in CLAUDE.md
                    if !rules.is_empty() {
                        let dest = workspace.join("CLAUDE.md");
                        let content = self.translator.translate_claude_rules(rules);
                        self.push_sync_item(
                            *agent,
                            dest,
                            content,
                            &manifest,
                            &mut items,
                            &mut conflicts,
                        );
                    }
                    // 2. Skills in .claude/skills/
                    let skills_dir = workspace.join(".claude").join("skills");
                    for skill in skills {
                        let clean_name = self
                            .translator
                            .sanitize_relative_path(&skill.metadata.name)?;
                        let dest = skills_dir.join(&clean_name).join("SKILL.md");
                        let content = self.translator.format_skill_markdown(skill);
                        self.push_sync_item(
                            *agent,
                            dest,
                            content,
                            &manifest,
                            &mut items,
                            &mut conflicts,
                        );
                    }
                    // 3. MCP in .claude/mcp.json
                    if !servers.is_empty() {
                        let dest = workspace.join(".claude").join("mcp.json");
                        let json_val = self.translator.translate_mcp_servers(*agent, servers)?;
                        let content = serde_json::to_string_pretty(&json_val)?;
                        self.push_sync_item(
                            *agent,
                            dest,
                            content,
                            &manifest,
                            &mut items,
                            &mut conflicts,
                        );
                    }
                }
                AgentId::OpenCode => {
                    // 1. Instructions in AGENTS.md
                    if !rules.is_empty() {
                        let dest = workspace.join("AGENTS.md");
                        let content = self.translator.translate_opencode_rules(rules);
                        self.push_sync_item(
                            *agent,
                            dest,
                            content,
                            &manifest,
                            &mut items,
                            &mut conflicts,
                        );
                    }
                    // 2. Skills in .opencode/skills/
                    let skills_dir = workspace.join(".opencode").join("skills");
                    for skill in skills {
                        let clean_name = self
                            .translator
                            .sanitize_relative_path(&skill.metadata.name)?;
                        let dest = skills_dir.join(&clean_name).join("SKILL.md");
                        let content = self.translator.format_skill_markdown(skill);
                        self.push_sync_item(
                            *agent,
                            dest,
                            content,
                            &manifest,
                            &mut items,
                            &mut conflicts,
                        );
                    }
                    // 3. MCP & config in .opencode/config.json
                    let dest = workspace.join(".opencode").join("config.json");
                    let json_val = self.translator.translate_mcp_servers(*agent, servers)?;
                    let content = serde_json::to_string_pretty(&json_val)?;
                    self.push_sync_item(
                        *agent,
                        dest,
                        content,
                        &manifest,
                        &mut items,
                        &mut conflicts,
                    );
                }
                AgentId::Cursor => {
                    // 1. MDC rules in .cursor/rules/*.mdc
                    let cursor_rules_dir = workspace.join(".cursor").join("rules");
                    for rule in rules {
                        let clean_name = self.translator.sanitize_relative_path(&rule.id)?;
                        let dest = cursor_rules_dir.join(format!("{}.mdc", clean_name.display()));
                        let content = self.translator.translate_cursor_mdc_rule(rule);
                        self.push_sync_item(
                            *agent,
                            dest,
                            content,
                            &manifest,
                            &mut items,
                            &mut conflicts,
                        );
                    }
                    // Legacy fallback .cursorrules
                    if !rules.is_empty() {
                        let dest = workspace.join(".cursorrules");
                        let content = self.translator.translate_cursor_legacy_rules(rules);
                        self.push_sync_item(
                            *agent,
                            dest,
                            content,
                            &manifest,
                            &mut items,
                            &mut conflicts,
                        );
                    }
                    // 2. Skills in .cursor/skills/
                    let skills_dir = workspace.join(".cursor").join("skills");
                    for skill in skills {
                        let clean_name = self
                            .translator
                            .sanitize_relative_path(&skill.metadata.name)?;
                        let dest = skills_dir.join(&clean_name).join("SKILL.md");
                        let content = self.translator.format_skill_markdown(skill);
                        self.push_sync_item(
                            *agent,
                            dest,
                            content,
                            &manifest,
                            &mut items,
                            &mut conflicts,
                        );
                    }
                    // 3. MCP in .cursor/mcp.json
                    if !servers.is_empty() {
                        let dest = workspace.join(".cursor").join("mcp.json");
                        let json_val = self.translator.translate_mcp_servers(*agent, servers)?;
                        let content = serde_json::to_string_pretty(&json_val)?;
                        self.push_sync_item(
                            *agent,
                            dest,
                            content,
                            &manifest,
                            &mut items,
                            &mut conflicts,
                        );
                    }
                }
            }
        }

        Ok(SyncPlan { items, conflicts })
    }

    fn push_sync_item(
        &self,
        agent_id: AgentId,
        destination_path: PathBuf,
        content: String,
        manifest: &AgentOwnershipManifest,
        items: &mut Vec<SyncItem>,
        conflicts: &mut Vec<String>,
    ) {
        let content_hash = Self::compute_sha256(&content);
        let file_existed = destination_path.is_file();
        let mut is_externally_modified = false;

        if file_existed {
            if let Ok(disk_content) = fs::read_to_string(&destination_path) {
                let disk_hash = Self::compute_sha256(&disk_content);
                let rel_key = destination_path.to_string_lossy().to_string();

                if let Some(record) = manifest.managed_files.get(&rel_key) {
                    if record.content_hash != disk_hash && disk_hash != content_hash {
                        is_externally_modified = true;
                        conflicts.push(destination_path.display().to_string());
                    }
                } else if disk_hash != content_hash {
                    // Exists but unmanaged by AEK: collision!
                    is_externally_modified = true;
                    conflicts.push(destination_path.display().to_string());
                }
            }
        }

        items.push(SyncItem {
            agent_id,
            destination_path,
            content,
            content_hash,
            file_existed,
            is_externally_modified,
        });
    }

    /// Executes the synchronization plan according to the given conflict strategy.
    pub fn execute_sync(
        &self,
        workspace: &Path,
        plan: &SyncPlan,
        strategy: ConflictStrategy,
    ) -> Result<SyncReport, AgentError> {
        let mut manifest = Self::load_manifest(workspace);
        let mut files_written = 0;
        let mut files_skipped = 0;
        let mut conflicts_preserved = 0;
        let mut conflicts_resolved = 0;
        let mut backup_directory = None;

        let backup_dir = workspace.join(".aek").join("backups");

        for item in &plan.items {
            if item.file_existed && item.is_externally_modified {
                match strategy {
                    ConflictStrategy::KeepExisting => {
                        conflicts_preserved += 1;
                        files_skipped += 1;
                        continue;
                    }
                    ConflictStrategy::Skip => {
                        files_skipped += 1;
                        continue;
                    }
                    ConflictStrategy::ManualResolution => {
                        return Err(AgentError::Conflict(format!(
                            "Unresolved external modifications in '{}'",
                            item.destination_path.display()
                        )));
                    }
                    ConflictStrategy::ReplaceManaged => {
                        // Backup existing file before overwrite
                        fs::create_dir_all(&backup_dir)?;
                        let timestamp = utc_now_iso().replace([':', ' ', '-'], "_");
                        let file_name = item
                            .destination_path
                            .file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("backup");
                        let backup_file = backup_dir.join(format!("{file_name}.{timestamp}.bak"));
                        fs::copy(&item.destination_path, &backup_file)?;
                        backup_directory = Some(backup_dir.clone());
                        conflicts_resolved += 1;
                    }
                }
            }

            // Create parent directories
            if let Some(parent) = item.destination_path.parent() {
                fs::create_dir_all(parent)?;
            }

            // Write content
            fs::write(&item.destination_path, &item.content)?;
            files_written += 1;

            // Update ownership manifest
            let rel_key = item.destination_path.to_string_lossy().to_string();
            let now = utc_now_iso();
            manifest.managed_files.insert(
                rel_key,
                ManagedFileRecord {
                    agent_id: item.agent_id,
                    relative_path: item.destination_path.clone(),
                    content_hash: item.content_hash.clone(),
                    aek_version: env!("CARGO_PKG_VERSION").to_string(),
                    installed_at: now.clone(),
                    last_synced_at: now,
                    backup_ref: backup_directory.clone(),
                },
            );
        }

        Self::save_manifest(workspace, &manifest)?;

        Ok(SyncReport {
            files_written,
            files_skipped,
            conflicts_preserved,
            conflicts_resolved,
            backup_directory,
            dry_run: false,
        })
    }
}
