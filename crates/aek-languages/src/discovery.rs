//! Executable discovery, version extraction, and cached probing.

use crate::model::{DiscoveredTool, ToolDefinition};
use aek_core::SafeCommand;
use aek_platform::find_executable_in_path;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::Duration;
use tracing::{debug, warn};

/// Engine for finding and caching toolchain executable statuses and versions.
#[derive(Debug, Clone)]
pub struct ToolchainDiscovery {
    cache: Arc<RwLock<HashMap<String, DiscoveredTool>>>,
}

impl Default for ToolchainDiscovery {
    fn default() -> Self {
        Self::new()
    }
}

impl ToolchainDiscovery {
    /// Creates a new `ToolchainDiscovery` instance with an empty cache.
    #[must_use]
    pub fn new() -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Discovers the status and version of a specific tool definition.
    /// Uses cached results if already probed.
    pub async fn discover_tool(&self, def: &ToolDefinition) -> DiscoveredTool {
        if let Ok(guard) = self.cache.read() {
            if let Some(tool) = guard.get(&def.id) {
                return tool.clone();
            }
        }

        let discovered = self.probe_tool(def).await;

        if let Ok(mut guard) = self.cache.write() {
            guard.insert(def.id.clone(), discovered.clone());
        }

        discovered
    }

    /// Internal probe executing PATH lookup and version check.
    async fn probe_tool(&self, def: &ToolDefinition) -> DiscoveredTool {
        let found_path = match find_executable_in_path(&def.binary) {
            Some(p) => p,
            None => {
                debug!("Binary '{}' not found on system PATH", def.binary);
                return DiscoveredTool {
                    definition: def.clone(),
                    path: None,
                    version: None,
                    is_available: false,
                };
            }
        };

        // Query version safely using argument array and timeout
        let version = if !def.version_args.is_empty() {
            let mut cmd = SafeCommand::new(&def.binary);
            cmd = cmd.args(&def.version_args);
            cmd = cmd.timeout(Duration::from_secs(4));

            match cmd.execute().await {
                Ok(res) => {
                    let out = if !res.stdout.is_empty() {
                        &res.stdout
                    } else {
                        &res.stderr
                    };
                    Some(extract_clean_version(out))
                }
                Err(err) => {
                    warn!("Failed to query version for '{}': {err}", def.binary);
                    None
                }
            }
        } else {
            None
        };

        DiscoveredTool {
            definition: def.clone(),
            path: Some(found_path),
            version,
            is_available: true,
        }
    }

    /// Clears the discovery cache.
    pub fn clear_cache(&self) {
        if let Ok(mut guard) = self.cache.write() {
            guard.clear();
        }
    }
}

/// Extracts a clean version substring from tool outputs.
/// Handles standard formats like:
/// - "rustc 1.85.0 (4d91de4e4 2025-02-17)" -> "1.85.0"
/// - "go version go1.22.4 windows/amd64" -> "1.22.4"
/// - "Python 3.12.3" -> "3.12.3"
/// - "clang version 18.1.8" -> "18.1.8"
/// - "0.13.0" -> "0.13.0"
#[must_use]
pub fn extract_clean_version(raw: &str) -> String {
    let first_line = raw.lines().next().unwrap_or(raw).trim();

    // Look for standard digit.digit patterns
    for token in first_line.split_whitespace() {
        let clean = token
            .trim_matches(|c: char| !c.is_numeric() && c != '.')
            .trim_start_matches("go")
            .trim_start_matches('v');

        let parts: Vec<&str> = clean.split('.').collect();
        if parts.len() >= 2 && parts[0].chars().all(|c| c.is_ascii_digit()) {
            return clean.to_string();
        }
    }

    first_line.chars().take(40).collect()
}
