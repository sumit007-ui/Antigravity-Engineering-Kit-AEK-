//! Google Antigravity adapter implementation.

use crate::adapter::{AgentAdapter, AgentDetection, AgentError, InstallReport};
use crate::capabilities::AgentCapabilities;
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
}
