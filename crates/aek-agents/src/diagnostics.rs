//! Compatibility diagnostics and health doctor across supported agents.

use crate::discovery::AgentDiscoveryEngine;
use crate::model::AgentId;
use crate::registry::AgentRegistry;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Diagnostic finding for an individual agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentDiagnosticItem {
    pub agent_id: AgentId,
    pub name: String,
    pub is_detected: bool,
    pub version: Option<String>,
    pub status: String,
    pub rules_dir: Option<String>,
    pub skills_dir: Option<String>,
    pub mcp_path: Option<String>,
    pub issues: Vec<String>,
}

/// Comprehensive diagnostics report for all agents.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentDoctorReport {
    pub total_agents: usize,
    pub detected_agents: usize,
    pub agents: Vec<AgentDiagnosticItem>,
}

/// Diagnostic health doctor inspecting agent setups and compatibility.
#[derive(Debug, Clone, Default)]
pub struct AgentDoctor;

impl AgentDoctor {
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Evaluates health and configuration diagnostics for all registered agents.
    #[must_use]
    pub fn diagnose(
        &self,
        registry: &AgentRegistry,
        workspace: Option<&Path>,
    ) -> AgentDoctorReport {
        let discovery = AgentDiscoveryEngine::new();
        let mut items = Vec::new();
        let mut detected_count = 0;

        for adapter in registry.list_all() {
            let id = adapter.agent_id();
            let detection = match id {
                AgentId::Antigravity => discovery.detect_antigravity(workspace),
                AgentId::ClaudeCode => discovery.detect_claude_code(workspace),
                AgentId::OpenCode => discovery.detect_opencode(workspace),
                AgentId::Cursor => discovery.detect_cursor(workspace),
            };

            if detection.is_detected {
                detected_count += 1;
            }

            let mut issues = Vec::new();
            if !detection.is_detected {
                issues.push(
                    "Agent not detected in PATH or standard configuration directories".to_string(),
                );
            }

            let rules_dir = if let Some(ws) = workspace {
                Some(adapter.workspace_rules_dir(ws).display().to_string())
            } else {
                adapter.global_rules_dir().map(|d| d.display().to_string())
            };

            let skills_dir = if let Some(ws) = workspace {
                Some(adapter.workspace_skills_dir(ws).display().to_string())
            } else {
                adapter.global_skills_dir().map(|d| d.display().to_string())
            };

            let mcp_path = adapter.mcp_config_path().map(|d| d.display().to_string());

            let status = if detection.is_detected {
                "READY".to_string()
            } else {
                "AVAILABLE_TO_CONFIGURE".to_string()
            };

            items.push(AgentDiagnosticItem {
                agent_id: id,
                name: adapter.name().to_string(),
                is_detected: detection.is_detected,
                version: detection.version,
                status,
                rules_dir,
                skills_dir,
                mcp_path,
                issues,
            });
        }

        AgentDoctorReport {
            total_agents: items.len(),
            detected_agents: detected_count,
            agents: items,
        }
    }
}
