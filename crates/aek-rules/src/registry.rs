//! In-memory and filesystem registry for engineering rules.

use crate::model::{Rule, RuleScope};
use crate::validator::validate_rule;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Central registry holding discovered and configured rules.
#[derive(Debug, Clone, Default)]
pub struct RuleRegistry {
    rules: HashMap<String, Vec<Rule>>,
}

impl RuleRegistry {
    /// Creates a new empty `RuleRegistry`.
    #[must_use]
    pub fn new() -> Self {
        Self {
            rules: HashMap::new(),
        }
    }

    /// Adds a rule to the registry.
    pub fn add_rule(&mut self, rule: Rule) {
        self.rules.entry(rule.id.clone()).or_default().push(rule);
    }

    /// Retrieves all rule variants matching an ID.
    #[must_use]
    pub fn get_rules_by_id(&self, id: &str) -> Option<&[Rule]> {
        self.rules.get(id).map(Vec::as_slice)
    }

    /// Returns a flat list of all registered rules across all scopes.
    #[must_use]
    pub fn list_all(&self) -> Vec<&Rule> {
        self.rules.values().flatten().collect()
    }

    /// Toggles the enabled flag of a rule by ID.
    pub fn set_enabled(&mut self, id: &str, enabled: bool) -> bool {
        if let Some(list) = self.rules.get_mut(id) {
            for r in list {
                r.enabled = enabled;
            }
            true
        } else {
            false
        }
    }

    /// Discovers and registers rules from a directory containing Markdown files.
    pub fn load_directory(
        &mut self,
        dir: &Path,
        scope: RuleScope,
    ) -> Result<usize, std::io::Error> {
        if !dir.exists() {
            return Ok(0);
        }

        let mut count = 0;
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("md") {
                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                    let content = fs::read_to_string(&path)?;
                    let name = content
                        .lines()
                        .find(|l| l.starts_with("# "))
                        .map(|l| l.trim_start_matches("# ").trim().to_string())
                        .unwrap_or_else(|| stem.to_string());

                    let priority = match scope {
                        RuleScope::Project => 300,
                        RuleScope::Workspace => 200,
                        RuleScope::Global => 100,
                    };

                    let rule = Rule {
                        id: stem.to_string(),
                        name,
                        description: format!("Loaded from {}", path.display()),
                        scope,
                        priority,
                        enabled: true,
                        source: path.display().to_string(),
                        content,
                        version: "1.0.0".to_string(),
                    };

                    if validate_rule(&rule).is_ok() {
                        self.add_rule(rule);
                        count += 1;
                    }
                }
            }
        }

        Ok(count)
    }
}
