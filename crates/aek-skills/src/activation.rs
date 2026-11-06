//! Skill activation state management.

use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Tracks which skills are currently selected for active agent turns.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct SkillActivationState {
    /// Set of active skill names.
    pub active_skills: HashSet<String>,
}

impl SkillActivationState {
    /// Creates a new empty activation state.
    #[must_use]
    pub fn new() -> Self {
        Self {
            active_skills: HashSet::new(),
        }
    }

    /// Activates a skill by name.
    pub fn activate(&mut self, skill_name: impl Into<String>) {
        self.active_skills.insert(skill_name.into());
    }

    /// Deactivates a skill by name.
    pub fn deactivate(&mut self, skill_name: &str) -> bool {
        self.active_skills.remove(skill_name)
    }

    /// Checks if a skill is active.
    #[must_use]
    pub fn is_active(&self, skill_name: &str) -> bool {
        self.active_skills.contains(skill_name)
    }
}

