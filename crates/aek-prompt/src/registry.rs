//! Registry for managing and discovering prompt records.

use crate::errors::PromptError;
use crate::model::PromptRecord;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// In-memory registry of prompt records.
#[derive(Debug, Default, Clone)]
pub struct PromptRegistry {
    prompts: HashMap<String, PromptRecord>,
}

impl PromptRegistry {
    /// Creates an empty prompt registry.
    #[must_use]
    pub fn new() -> Self {
        Self {
            prompts: HashMap::new(),
        }
    }

    /// Registers a prompt record.
    pub fn register(&mut self, prompt: PromptRecord) -> Result<(), PromptError> {
        self.prompts.insert(prompt.id.clone(), prompt);
        Ok(())
    }

    /// Retrieves a prompt by ID.
    #[must_use]
    pub fn get(&self, id: &str) -> Option<&PromptRecord> {
        self.prompts.get(id)
    }

    /// Lists all registered prompts.
    #[must_use]
    pub fn list(&self) -> Vec<PromptRecord> {
        let mut list: Vec<PromptRecord> = self.prompts.values().cloned().collect();
        list.sort_by(|a, b| a.id.cmp(&b.id));
        list
    }

    /// Loads prompt records from JSON files in a directory.
    pub fn load_directory(&mut self, dir: &Path) -> Result<usize, PromptError> {
        if !dir.is_dir() {
            return Ok(0);
        }

        let mut loaded = 0;
        for entry in fs::read_dir(dir)?.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("json") {
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Ok(record) = serde_json::from_str::<PromptRecord>(&content) {
                        self.register(record)?;
                        loaded += 1;
                    }
                }
            }
        }
        Ok(loaded)
    }
}
