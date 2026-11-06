//! Bounded in-memory prompt optimization cache.

use crate::model::compute_sha256;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Thread-safe in-memory prompt optimization cache.
#[derive(Debug, Clone)]
pub struct PromptCache {
    capacity: usize,
    entries: Arc<RwLock<HashMap<String, String>>>,
}

impl Default for PromptCache {
    fn default() -> Self {
        Self::new(100)
    }
}

impl PromptCache {
    /// Creates a new `PromptCache` with bounded capacity.
    #[must_use]
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            entries: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Computes a deterministic cache key.
    #[must_use]
    pub fn make_key(prompt_hash: &str, context: &str, adapter: &str) -> String {
        let combined = format!("{prompt_hash}:{context}:{adapter}");
        compute_sha256(&combined)
    }

    /// Retrieves cached prompt content if present.
    #[must_use]
    pub fn get(&self, key: &str) -> Option<String> {
        if let Ok(guard) = self.entries.read() {
            guard.get(key).cloned()
        } else {
            None
        }
    }

    /// Inserts an entry into the cache, pruning if over capacity.
    pub fn insert(&self, key: String, value: String) {
        if let Ok(mut guard) = self.entries.write() {
            if guard.len() >= self.capacity {
                // Remove arbitrary first key to maintain bounded size
                if let Some(first) = guard.keys().next().cloned() {
                    guard.remove(&first);
                }
            }
            guard.insert(key, value);
        }
    }

    /// Clears the cache.
    pub fn clear(&self) {
        if let Ok(mut guard) = self.entries.write() {
            guard.clear();
        }
    }
}
