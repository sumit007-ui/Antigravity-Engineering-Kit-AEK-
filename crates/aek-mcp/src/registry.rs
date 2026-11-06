//! In-memory and filesystem registry for configured MCP servers.

use crate::config::McpServerConfig;
use crate::errors::McpError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Registry container for serializing and storing MCP server configurations.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct McpServerRegistry {
    #[serde(default)]
    pub servers: HashMap<String, McpServerConfig>,
}

impl McpServerRegistry {
    /// Creates a new empty server registry.
    #[must_use]
    pub fn new() -> Self {
        Self {
            servers: HashMap::new(),
        }
    }

    /// Registers a new server configuration after validation.
    pub fn register(&mut self, config: McpServerConfig) -> Result<(), McpError> {
        config.validate()?;
        self.servers.insert(config.id.clone(), config);
        Ok(())
    }

    /// Retrieves a server configuration by identifier.
    #[must_use]
    pub fn get(&self, id: &str) -> Option<&McpServerConfig> {
        self.servers.get(id)
    }

    /// Retrieves a mutable server configuration by identifier.
    pub fn get_mut(&mut self, id: &str) -> Option<&mut McpServerConfig> {
        self.servers.get_mut(id)
    }

    /// Returns a list of all registered server configurations sorted by identifier.
    #[must_use]
    pub fn list(&self) -> Vec<&McpServerConfig> {
        let mut list: Vec<&McpServerConfig> = self.servers.values().collect();
        list.sort_by(|a, b| a.id.cmp(&b.id));
        list
    }

    /// Removes a server configuration by identifier. Returns true if it existed.
    pub fn remove(&mut self, id: &str) -> bool {
        self.servers.remove(id).is_some()
    }

    /// Enables or disables a server by identifier.
    pub fn set_enabled(&mut self, id: &str, enabled: bool) -> Result<bool, McpError> {
        if let Some(s) = self.servers.get_mut(id) {
            s.enabled = enabled;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Saves the registry contents into a TOML file atomically.
    pub fn save_to_file(&self, path: &Path) -> Result<(), McpError> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let serialized = toml::to_string_pretty(self)
            .map_err(|e| McpError::InvalidConfiguration("registry".into(), e.to_string()))?;
        fs::write(path, serialized)?;
        Ok(())
    }

    /// Loads the registry from a TOML file.
    pub fn load_from_file(path: &Path) -> Result<Self, McpError> {
        if !path.exists() {
            return Ok(Self::new());
        }
        let content = fs::read_to_string(path)?;
        let registry: Self = toml::from_str(&content)
            .map_err(|e| McpError::InvalidConfiguration("registry".into(), e.to_string()))?;
        Ok(registry)
    }

    /// Scans a directory for individual server `.toml` definitions and adds them.
    pub fn load_directory(&mut self, dir: &Path) -> Result<usize, McpError> {
        if !dir.exists() {
            return Ok(0);
        }

        let mut loaded = 0;
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("toml") {
                // If it's registry.toml, try loading as a registry container first
                let file_name = path
                    .file_name()
                    .and_then(|s| s.to_str())
                    .unwrap_or_default();
                let content = fs::read_to_string(&path)?;
                if file_name == "registry.toml" {
                    if let Ok(reg) = toml::from_str::<McpServerRegistry>(&content) {
                        for (_, cfg) in reg.servers {
                            if self.register(cfg).is_ok() {
                                loaded += 1;
                            }
                        }
                        continue;
                    }
                }

                // Try parsing as single server config
                if let Ok(cfg) = toml::from_str::<McpServerConfig>(&content) {
                    if self.register(cfg).is_ok() {
                        loaded += 1;
                    }
                }
            }
        }

        Ok(loaded)
    }
}
