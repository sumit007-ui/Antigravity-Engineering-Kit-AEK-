//! Agent discovery engine identifying installed and configured coding agents.

use crate::model::{AgentDetection, AgentId};
use std::env;
use std::path::{Path, PathBuf};

/// Identifies active or installed coding agents across Windows, Linux, and macOS.
#[derive(Debug, Clone, Default)]
pub struct AgentDiscoveryEngine;

impl AgentDiscoveryEngine {
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Detects all supported agents on the host environment and optional workspace.
    #[must_use]
    pub fn detect_all(&self, workspace: Option<&Path>) -> Vec<AgentDetection> {
        vec![
            self.detect_antigravity(workspace),
            self.detect_claude_code(workspace),
            self.detect_opencode(workspace),
            self.detect_cursor(workspace),
        ]
    }

    /// Discovers Gemini / Antigravity installation or configuration.
    #[must_use]
    pub fn detect_antigravity(&self, workspace: Option<&Path>) -> AgentDetection {
        let mut evidence = Vec::new();
        let mut is_detected = false;
        let mut config_path = None;
        let mut executable_path = None;

        // 1. Check user home config directories
        if let Some(home) = home_dir() {
            let gemini_dir = home.join(".gemini");
            let agy_dir = gemini_dir.join("antigravity");
            if agy_dir.is_dir() {
                is_detected = true;
                evidence.push(format!(
                    "Found Antigravity global dir: {}",
                    agy_dir.display()
                ));
                config_path = Some(agy_dir);
            } else if gemini_dir.is_dir() {
                is_detected = true;
                evidence.push(format!("Found .gemini directory: {}", gemini_dir.display()));
                config_path = Some(gemini_dir);
            }
        }

        // 2. Check binary on PATH
        if let Some(bin) = find_binary_on_path(&["agy", "antigravity"]) {
            is_detected = true;
            evidence.push(format!("Found executable: {}", bin.display()));
            executable_path = Some(bin);
        }

        // 3. Check workspace directory
        if let Some(ws) = workspace {
            let ws_gemini = ws.join(".gemini");
            if ws_gemini.is_dir() {
                is_detected = true;
                evidence.push(format!(
                    "Found workspace .gemini config: {}",
                    ws_gemini.display()
                ));
            }
        }

        AgentDetection {
            agent_name: AgentId::Antigravity.display_name().to_string(),
            is_detected,
            evidence,
            version: if is_detected {
                Some("2.0.0".to_string())
            } else {
                None
            },
            executable_path,
            config_path,
        }
    }

    /// Discovers Claude Code CLI installation or configuration.
    #[must_use]
    pub fn detect_claude_code(&self, workspace: Option<&Path>) -> AgentDetection {
        let mut evidence = Vec::new();
        let mut is_detected = false;
        let mut config_path = None;
        let mut executable_path = None;

        // 1. Check user config
        if let Some(home) = home_dir() {
            let claude_dir = home.join(".claude");
            if claude_dir.is_dir() {
                is_detected = true;
                evidence.push(format!(
                    "Found Claude Code home dir: {}",
                    claude_dir.display()
                ));
                config_path = Some(claude_dir);
            }
        }

        // 2. Check binary on PATH
        if let Some(bin) = find_binary_on_path(&["claude", "claude-code"]) {
            is_detected = true;
            evidence.push(format!("Found Claude executable: {}", bin.display()));
            executable_path = Some(bin);
        }

        // 3. Check workspace
        if let Some(ws) = workspace {
            let ws_claude = ws.join(".claude");
            let ws_md = ws.join("CLAUDE.md");
            if ws_claude.is_dir() {
                is_detected = true;
                evidence.push(format!(
                    "Found workspace .claude directory: {}",
                    ws_claude.display()
                ));
            }
            if ws_md.is_file() {
                is_detected = true;
                evidence.push(format!("Found CLAUDE.md in workspace: {}", ws_md.display()));
            }
        }

        AgentDetection {
            agent_name: AgentId::ClaudeCode.display_name().to_string(),
            is_detected,
            evidence,
            version: if is_detected {
                Some("1.0.0".to_string())
            } else {
                None
            },
            executable_path,
            config_path,
        }
    }

    /// Discovers OpenCode agent installation or configuration.
    #[must_use]
    pub fn detect_opencode(&self, workspace: Option<&Path>) -> AgentDetection {
        let mut evidence = Vec::new();
        let mut is_detected = false;
        let mut config_path = None;
        let mut executable_path = None;

        // 1. Check user config
        if let Some(home) = home_dir() {
            let opencode_dir = home.join(".opencode");
            if opencode_dir.is_dir() {
                is_detected = true;
                evidence.push(format!(
                    "Found OpenCode user dir: {}",
                    opencode_dir.display()
                ));
                config_path = Some(opencode_dir);
            }
        }

        // 2. Check binary on PATH
        if let Some(bin) = find_binary_on_path(&["opencode", "open-code"]) {
            is_detected = true;
            evidence.push(format!("Found OpenCode executable: {}", bin.display()));
            executable_path = Some(bin);
        }

        // 3. Check workspace
        if let Some(ws) = workspace {
            let ws_opencode = ws.join(".opencode");
            let ws_config = ws.join("opencode.json");
            let ws_agents_md = ws.join("AGENTS.md");
            if ws_opencode.is_dir() {
                is_detected = true;
                evidence.push(format!(
                    "Found workspace .opencode directory: {}",
                    ws_opencode.display()
                ));
            }
            if ws_config.is_file() {
                is_detected = true;
                evidence.push(format!(
                    "Found opencode.json in workspace: {}",
                    ws_config.display()
                ));
            }
            if ws_agents_md.is_file() {
                is_detected = true;
                evidence.push(format!(
                    "Found AGENTS.md in workspace: {}",
                    ws_agents_md.display()
                ));
            }
        }

        AgentDetection {
            agent_name: AgentId::OpenCode.display_name().to_string(),
            is_detected,
            evidence,
            version: if is_detected {
                Some("1.0.0".to_string())
            } else {
                None
            },
            executable_path,
            config_path,
        }
    }

    /// Discovers Cursor IDE installation or configuration.
    #[must_use]
    pub fn detect_cursor(&self, workspace: Option<&Path>) -> AgentDetection {
        let mut evidence = Vec::new();
        let mut is_detected = false;
        let mut config_path = None;
        let mut executable_path = None;

        // 1. Check user config directory
        if let Some(home) = home_dir() {
            let cursor_dir = home.join(".cursor");
            if cursor_dir.is_dir() {
                is_detected = true;
                evidence.push(format!("Found Cursor user dir: {}", cursor_dir.display()));
                config_path = Some(cursor_dir);
            }
        }

        // 2. Check binary on PATH
        if let Some(bin) = find_binary_on_path(&["cursor"]) {
            is_detected = true;
            evidence.push(format!("Found Cursor executable: {}", bin.display()));
            executable_path = Some(bin);
        }

        // 3. Check workspace directory
        if let Some(ws) = workspace {
            let ws_cursor = ws.join(".cursor");
            let ws_rules = ws.join(".cursorrules");
            if ws_cursor.is_dir() {
                is_detected = true;
                evidence.push(format!(
                    "Found workspace .cursor directory: {}",
                    ws_cursor.display()
                ));
            }
            if ws_rules.is_file() {
                is_detected = true;
                evidence.push(format!(
                    "Found .cursorrules in workspace: {}",
                    ws_rules.display()
                ));
            }
        }

        AgentDetection {
            agent_name: AgentId::Cursor.display_name().to_string(),
            is_detected,
            evidence,
            version: if is_detected {
                Some("0.40.0".to_string())
            } else {
                None
            },
            executable_path,
            config_path,
        }
    }
}

fn home_dir() -> Option<PathBuf> {
    if let Ok(userprofile) = env::var("USERPROFILE") {
        if !userprofile.is_empty() {
            return Some(PathBuf::from(userprofile));
        }
    }
    if let Ok(home) = env::var("HOME") {
        if !home.is_empty() {
            return Some(PathBuf::from(home));
        }
    }
    None
}

fn find_binary_on_path(names: &[&str]) -> Option<PathBuf> {
    let path_var = env::var_os("PATH")?;
    let paths = env::split_paths(&path_var);

    let is_windows = cfg!(target_os = "windows");
    let extensions = if is_windows {
        vec!["", ".exe", ".cmd", ".bat", ".ps1"]
    } else {
        vec![""]
    };

    for dir in paths {
        for name in names {
            for ext in &extensions {
                let candidate = dir.join(format!("{name}{ext}"));
                if candidate.is_file() {
                    return Some(candidate);
                }
            }
        }
    }

    None
}
