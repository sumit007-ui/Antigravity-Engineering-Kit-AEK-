//! Versioned storage, atomic persistence, diffing, and rollback for prompts.

use crate::errors::PromptError;
use crate::model::{PromptComparison, PromptRecord, compute_sha256};
use std::fs;
use std::path::PathBuf;

/// Manages historical versioned prompt records on disk.
#[derive(Debug, Clone)]
pub struct PromptVersionManager {
    base_dir: PathBuf,
}

impl PromptVersionManager {
    /// Creates a new `PromptVersionManager` at the specified base directory.
    #[must_use]
    pub fn new(base_dir: PathBuf) -> Self {
        Self { base_dir }
    }

    fn validate_id(&self, id: &str) -> Result<(), PromptError> {
        if id.contains('/') || id.contains('\\') || id.contains("..") || id.is_empty() {
            return Err(PromptError::Security(format!(
                "Invalid prompt or version identifier: '{id}'"
            )));
        }
        Ok(())
    }

    /// Persists a prompt version to disk. Returns content SHA-256 hash.
    pub fn save_version(&self, record: &PromptRecord) -> Result<String, PromptError> {
        self.validate_id(&record.id)?;
        self.validate_id(&record.version)?;

        let mut rec = record.clone();
        rec.content_hash = compute_sha256(rec.active_content());

        let dir = self.base_dir.join(&rec.id);
        fs::create_dir_all(&dir)?;

        let filename = format!("{}.json", rec.version);
        let path = dir.join(filename);
        let content = serde_json::to_string_pretty(&rec)?;

        // Write atomic
        let tmp = dir.join(format!("{}.tmp", rec.version));
        fs::write(&tmp, &content)?;
        fs::rename(&tmp, &path)?;

        // Update latest pointer
        let latest_path = dir.join("latest.json");
        fs::write(latest_path, content)?;

        Ok(rec.content_hash)
    }

    /// Lists all stored versions for a prompt identifier.
    pub fn list_versions(&self, prompt_id: &str) -> Result<Vec<PromptRecord>, PromptError> {
        self.validate_id(prompt_id)?;
        let dir = self.base_dir.join(prompt_id);
        if !dir.is_dir() {
            return Ok(Vec::new());
        }

        let mut list = Vec::new();
        for entry in fs::read_dir(dir)?.flatten() {
            let path = entry.path();
            if path.file_name().and_then(|n| n.to_str()) == Some("latest.json") {
                continue;
            }
            if path.extension().and_then(|e| e.to_str()) == Some("json") {
                if let Ok(raw) = fs::read_to_string(&path) {
                    if let Ok(rec) = serde_json::from_str::<PromptRecord>(&raw) {
                        list.push(rec);
                    }
                }
            }
        }

        list.sort_by(|a, b| a.version.cmp(&b.version));
        Ok(list)
    }

    /// Loads a specific version of a prompt.
    pub fn get_version(&self, prompt_id: &str, version: &str) -> Result<PromptRecord, PromptError> {
        self.validate_id(prompt_id)?;
        self.validate_id(version)?;

        let path = self
            .base_dir
            .join(prompt_id)
            .join(format!("{version}.json"));
        if !path.is_file() {
            return Err(PromptError::VersionNotFound(
                prompt_id.to_string(),
                version.to_string(),
            ));
        }

        let raw = fs::read_to_string(path)?;
        let rec = serde_json::from_str::<PromptRecord>(&raw)?;
        Ok(rec)
    }

    /// Loads the latest active version of a prompt.
    pub fn get_latest(&self, prompt_id: &str) -> Result<PromptRecord, PromptError> {
        self.validate_id(prompt_id)?;
        let latest_path = self.base_dir.join(prompt_id).join("latest.json");
        if latest_path.is_file() {
            let raw = fs::read_to_string(&latest_path)?;
            let rec = serde_json::from_str::<PromptRecord>(&raw)?;
            return Ok(rec);
        }

        let versions = self.list_versions(prompt_id)?;
        versions.into_iter().last().ok_or_else(|| {
            PromptError::VersionNotFound(prompt_id.to_string(), "latest".to_string())
        })
    }

    /// Rolls back the active latest prompt to an earlier version.
    pub fn rollback(
        &self,
        prompt_id: &str,
        target_version: &str,
    ) -> Result<PromptRecord, PromptError> {
        let record = self.get_version(prompt_id, target_version)?;
        let dir = self.base_dir.join(prompt_id);
        let latest_path = dir.join("latest.json");
        let content = serde_json::to_string_pretty(&record)?;
        fs::write(latest_path, content)?;
        Ok(record)
    }

    /// Computes a line diff comparison between two prompt versions.
    pub fn compare(
        &self,
        prompt_id: &str,
        ver_a: &str,
        ver_b: &str,
    ) -> Result<PromptComparison, PromptError> {
        let a = self.get_version(prompt_id, ver_a)?;
        let b = self.get_version(prompt_id, ver_b)?;

        let orig_a: Vec<&str> = a.original_content.lines().collect();
        let orig_b: Vec<&str> = b.original_content.lines().collect();
        let mut original_diff_lines = Vec::new();

        for line in &orig_a {
            if !orig_b.contains(line) {
                original_diff_lines.push(format!("- {line}"));
            }
        }
        for line in &orig_b {
            if !orig_a.contains(line) {
                original_diff_lines.push(format!("+ {line}"));
            }
        }

        Ok(PromptComparison {
            prompt_id: prompt_id.to_string(),
            version_a: ver_a.to_string(),
            version_b: ver_b.to_string(),
            hash_a: a.content_hash,
            hash_b: b.content_hash,
            original_diff_lines,
            optimized_diff_lines: Vec::new(),
        })
    }
}
