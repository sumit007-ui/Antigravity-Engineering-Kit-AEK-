//! Toolchain registry managing built-in and declarative tool definitions.

use crate::errors::LanguageError;
use crate::model::{LanguageId, ToolDefinition};
use crate::providers::get_all_providers;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use tracing::warn;

/// Registry of tool definitions indexed by tool identifier and language.
#[derive(Debug, Clone)]
pub struct ToolchainRegistry {
    tools: HashMap<String, ToolDefinition>,
}

impl Default for ToolchainRegistry {
    fn default() -> Self {
        let mut registry = Self {
            tools: HashMap::new(),
        };

        // Populate standard tools from providers
        for provider in get_all_providers() {
            for tool in provider.tools() {
                registry.register(tool).ok();
            }
        }

        registry
    }
}

impl ToolchainRegistry {
    /// Creates an empty toolchain registry.
    #[must_use]
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
        }
    }

    /// Registers a tool definition in the registry.
    pub fn register(&mut self, tool: ToolDefinition) -> Result<(), LanguageError> {
        if self.tools.contains_key(&tool.id) {
            return Err(LanguageError::InvalidConfig(format!(
                "Duplicate tool definition ID: {}",
                tool.id
            )));
        }
        self.tools.insert(tool.id.clone(), tool);
        Ok(())
    }

    /// Retrieves all tools associated with a specific programming language.
    #[must_use]
    pub fn get_tools_for_language(&self, language: &LanguageId) -> Vec<ToolDefinition> {
        let mut list: Vec<ToolDefinition> = self
            .tools
            .values()
            .filter(|t| t.language == *language)
            .cloned()
            .collect();
        list.sort_by(|a, b| a.id.cmp(&b.id));
        list
    }

    /// Retrieves all registered tool definitions.
    #[must_use]
    pub fn get_all_tools(&self) -> Vec<ToolDefinition> {
        let mut list: Vec<ToolDefinition> = self.tools.values().cloned().collect();
        list.sort_by(|a, b| a.id.cmp(&b.id));
        list
    }

    /// Looks up a tool definition by its unique identifier.
    #[must_use]
    pub fn find_tool(&self, id: &str) -> Option<&ToolDefinition> {
        self.tools.get(id)
    }

    /// Loads additional tool definitions from TOML files in a directory.
    pub fn load_directory(&mut self, dir: &Path) -> Result<usize, LanguageError> {
        if !dir.is_dir() {
            return Ok(0);
        }

        let mut loaded = 0;
        let entries = fs::read_dir(dir).map_err(LanguageError::Io)?;

        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("toml") {
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Ok(definitions) = toml::from_str::<Vec<ToolDefinition>>(&content) {
                        for def in definitions {
                            if self.register(def).is_ok() {
                                loaded += 1;
                            }
                        }
                    } else if let Ok(single) = toml::from_str::<ToolDefinition>(&content) {
                        if self.register(single).is_ok() {
                            loaded += 1;
                        }
                    } else {
                        warn!("Failed to parse tool definition in {}", path.display());
                    }
                }
            }
        }

        Ok(loaded)
    }
}
