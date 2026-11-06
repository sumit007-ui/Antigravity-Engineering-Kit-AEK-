//! Central registry managing all registered AI coding agent adapters.

use crate::adapter::AgentAdapter;
use crate::adapters::{AntigravityAdapter, ClaudeCodeAdapter, CursorAdapter, OpenCodeAdapter};
use crate::model::{AgentDetection, AgentId};
use std::collections::HashMap;

/// Registry storing active agent adapters.
pub struct AgentRegistry {
    adapters: HashMap<AgentId, Box<dyn AgentAdapter>>,
}

impl Default for AgentRegistry {
    fn default() -> Self {
        Self::default_registry()
    }
}

impl AgentRegistry {
    #[must_use]
    pub fn new() -> Self {
        Self {
            adapters: HashMap::new(),
        }
    }

    /// Creates registry initialized with all four built-in agent adapters.
    #[must_use]
    pub fn default_registry() -> Self {
        let mut reg = Self::new();
        reg.register(Box::new(AntigravityAdapter::new()));
        reg.register(Box::new(ClaudeCodeAdapter::new()));
        reg.register(Box::new(OpenCodeAdapter::new()));
        reg.register(Box::new(CursorAdapter::new()));
        reg
    }

    /// Registers an agent adapter instance.
    pub fn register(&mut self, adapter: Box<dyn AgentAdapter>) {
        self.adapters.insert(adapter.agent_id(), adapter);
    }

    /// Retrieves an adapter by its typed AgentId.
    #[must_use]
    pub fn get(&self, id: AgentId) -> Option<&dyn AgentAdapter> {
        self.adapters.get(&id).map(std::convert::AsRef::as_ref)
    }

    /// Retrieves an adapter by string name or slug.
    #[must_use]
    pub fn get_by_name(&self, name: &str) -> Option<&dyn AgentAdapter> {
        if let Ok(id) = name.parse::<AgentId>() {
            return self.get(id);
        }
        self.adapters
            .values()
            .find(|a| a.name().eq_ignore_ascii_case(name.trim()))
            .map(std::convert::AsRef::as_ref)
    }

    /// Returns the number of registered agent adapters.
    #[must_use]
    pub fn len(&self) -> usize {
        self.adapters.len()
    }

    /// Returns whether any adapters are registered.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.adapters.is_empty()
    }

    /// Lists all registered adapters in deterministic order.
    #[must_use]
    pub fn list_all(&self) -> Vec<&dyn AgentAdapter> {
        let mut list = Vec::new();
        for id in AgentId::all() {
            if let Some(adapter) = self.adapters.get(id) {
                list.push(adapter.as_ref());
            }
        }
        list
    }

    /// Runs detection across all registered adapters.
    #[must_use]
    pub fn detect_all(&self) -> Vec<AgentDetection> {
        self.list_all().into_iter().map(|a| a.detect()).collect()
    }
}
