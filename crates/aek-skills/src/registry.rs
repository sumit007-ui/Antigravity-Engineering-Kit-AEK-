//! In-memory and directory-backed registry for agent skills.

use crate::model::SkillBundle;
use crate::validator::parse_skill_markdown;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Central registry indexing agent skill bundles.
#[derive(Debug, Clone, Default)]
pub struct SkillRegistry {
    skills: HashMap<String, SkillBundle>,
}

impl SkillRegistry {
    /// Creates a new empty `SkillRegistry`.
    #[must_use]
    pub fn new() -> Self {
        Self {
            skills: HashMap::new(),
        }
    }

    /// Adds a skill bundle. Returns false if duplicate.
    pub fn add_skill(&mut self, skill: SkillBundle) -> bool {
        let name = skill.metadata.name.clone();
        match self.skills.entry(name) {
            std::collections::hash_map::Entry::Vacant(e) => {
                e.insert(skill);
                true
            }
            std::collections::hash_map::Entry::Occupied(_) => false,
        }
    }

    /// Retrieves a skill bundle by identifier.
    #[must_use]
    pub fn get_skill(&self, name: &str) -> Option<&SkillBundle> {
        self.skills.get(name)
    }

    /// Lists all registered skills.
    #[must_use]
    pub fn list_skills(&self) -> Vec<&SkillBundle> {
        let mut list: Vec<&SkillBundle> = self.skills.values().collect();
        list.sort_by(|a, b| a.metadata.name.cmp(&b.metadata.name));
        list
    }

    /// Scans a directory containing skill bundle subdirectories and indexes all valid skills.
    pub fn load_directory(&mut self, root: &Path) -> Result<usize, std::io::Error> {
        if !root.exists() {
            return Ok(0);
        }

        let mut count = 0;
        for entry in fs::read_dir(root)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                let skill_md = path.join("SKILL.md");
                if skill_md.is_file() {
                    if let Ok(content) = fs::read_to_string(&skill_md) {
                        if let Ok(meta) = parse_skill_markdown(&content) {
                            let bundle = SkillBundle {
                                metadata: meta,
                                root_dir: path.clone(),
                                instructions_path: skill_md,
                                body: content,
                            };
                            if self.add_skill(bundle) {
                                count += 1;
                            }
                        }
                    }
                }
            }
        }

        Ok(count)
    }
}
