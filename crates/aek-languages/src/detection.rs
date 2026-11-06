//! Project detection engine with recursive bounded directory scanning and ignore rules.

use crate::errors::LanguageError;
use crate::model::DetectedProject;
use crate::providers::{LanguageProvider, get_all_providers};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tracing::debug;

const DEFAULT_MAX_DEPTH: usize = 4;

const IGNORED_DIRS: &[&str] = &[
    "target",
    "node_modules",
    ".git",
    "venv",
    ".venv",
    "env",
    ".env",
    "vendor",
    "build",
    ".dart_tool",
    "install",
    "log",
    "dist",
    ".next",
    "out",
    "__pycache__",
    ".idea",
    ".vscode",
];

/// Engine for detecting projects across multiple programming language ecosystems.
#[derive(Clone)]
pub struct ProjectDetector {
    providers: Vec<Arc<dyn LanguageProvider>>,
    max_depth: usize,
}

impl Default for ProjectDetector {
    fn default() -> Self {
        Self::new()
    }
}

impl ProjectDetector {
    /// Creates a new `ProjectDetector` with all standard providers and default depth.
    #[must_use]
    pub fn new() -> Self {
        Self {
            providers: get_all_providers(),
            max_depth: DEFAULT_MAX_DEPTH,
        }
    }

    /// Sets the maximum recursive directory scan depth.
    #[must_use]
    pub fn with_max_depth(mut self, depth: usize) -> Self {
        self.max_depth = depth;
        self
    }

    /// Detects all projects in the specified root directory.
    pub fn detect(&self, root: &Path) -> Result<Vec<DetectedProject>, LanguageError> {
        let mut projects = Vec::new();
        let mut visited = HashSet::new();

        self.scan_recursive(root, 0, None, &mut visited, &mut projects)?;

        // Sort projects deterministically by path
        projects.sort_by(|a, b| a.path.cmp(&b.path));
        Ok(projects)
    }

    fn scan_recursive(
        &self,
        current: &Path,
        depth: usize,
        monorepo_root: Option<&Path>,
        visited: &mut HashSet<PathBuf>,
        results: &mut Vec<DetectedProject>,
    ) -> Result<(), LanguageError> {
        if depth > self.max_depth {
            return Ok(());
        }

        // Canonicalize to prevent symlink traversal loops
        let canonical = match fs::canonicalize(current) {
            Ok(p) => p,
            Err(_) => current.to_path_buf(),
        };

        if !visited.insert(canonical) {
            debug!("Skipping already visited path: {}", current.display());
            return Ok(());
        }

        // Check each provider against the current directory
        let mut detected_here = false;
        let mut current_monorepo = monorepo_root;

        for provider in &self.providers {
            if let Some(mut project) = provider.detect_project(current) {
                if let Some(root) = current_monorepo {
                    project.monorepo_root = Some(root.to_path_buf());
                    project.is_nested = true;
                } else if project.monorepo_root.is_some() {
                    current_monorepo = Some(current);
                }

                results.push(project);
                detected_here = true;
            }
        }

        // Read subdirectories
        let read_dir = match fs::read_dir(current) {
            Ok(entries) => entries,
            Err(_) => return Ok(()),
        };

        for entry in read_dir.flatten() {
            let path = entry.path();
            if path.is_dir() {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    // Check ignore list
                    if IGNORED_DIRS.contains(&name) || name.starts_with('.') {
                        continue;
                    }
                }

                self.scan_recursive(&path, depth + 1, current_monorepo, visited, results)?;
            }
        }

        let _ = detected_here;
        Ok(())
    }
}
