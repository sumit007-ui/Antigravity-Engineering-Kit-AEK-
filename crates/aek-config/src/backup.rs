//! Backup and restoration engine for AEK configuration files.

use crate::error::ConfigError;
use crate::loader::load_config_from_file;
use crate::writer::save_config_atomic;
use aek_platform::paths::PlatformPaths;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

/// Metadata describing an individual configuration backup.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupMetadata {
    /// Unique identifier for the backup.
    pub id: String,
    /// UTC creation timestamp in ISO 8601 format.
    pub created_at: String,
    /// Profile configured in this backup.
    pub profile: String,
    /// File size in bytes.
    pub file_size: u64,
    /// Absolute filesystem path to the backup file.
    pub path: PathBuf,
    /// Optional context or reason for creating the backup.
    pub reason: Option<String>,
}

/// Result of a backup restoration operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestoreResult {
    /// Identifier of the restored backup.
    pub backup_id: String,
    /// Target path where configuration was restored.
    pub target_path: PathBuf,
    /// Whether the operation ran in dry-run mode without mutating disk.
    pub dry_run: bool,
    /// Safety backup ID created prior to restoration (if target existed).
    pub safety_backup_id: Option<String>,
}

/// Backup engine providing atomic backups, validation, and safe restoration.
pub struct BackupEngine;

impl BackupEngine {
    /// Validates a user-supplied backup identifier to prevent path traversal or injection.
    pub fn validate_backup_id(id: &str) -> Result<(), ConfigError> {
        let trimmed = id.trim();
        if trimmed.is_empty() {
            return Err(ConfigError::InvalidBackupId(
                "Identifier cannot be empty".to_string(),
            ));
        }

        // Strict whitelist: alphanumeric, hyphens, and underscores only
        let is_valid = trimmed
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_');

        if !is_valid || trimmed.contains("..") || trimmed.contains('/') || trimmed.contains('\\') {
            return Err(ConfigError::InvalidBackupId(trimmed.to_string()));
        }

        Ok(())
    }

    /// Creates an atomic, validated backup of the configuration file.
    ///
    /// # Errors
    /// Returns `ConfigError` if config is missing, invalid, or cannot be saved.
    pub fn create_backup(
        config_path: &Path,
        backup_dir: &Path,
        reason: Option<&str>,
    ) -> Result<BackupMetadata, ConfigError> {
        if !config_path.exists() {
            return Err(ConfigError::NotFound(config_path.to_path_buf()));
        }

        // Verify configuration validity before backing up
        let config = load_config_from_file(config_path)?;

        if !backup_dir.exists() {
            fs::create_dir_all(backup_dir)?;
        }

        let now = Utc::now();
        let timestamp = now.format("%Y%m%d-%H%M%S").to_string();
        let nonce = format!(
            "{:06x}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.subsec_nanos() % 0x1000000)
                .unwrap_or(0)
        );

        let backup_id = format!("backup-{timestamp}-{nonce}");
        let backup_file = backup_dir.join(format!("{backup_id}.toml"));

        // Save backup atomically
        save_config_atomic(&backup_file, &config)?;

        let file_size = fs::metadata(&backup_file).map(|m| m.len()).unwrap_or(0);

        Ok(BackupMetadata {
            id: backup_id,
            created_at: now.to_rfc3339(),
            profile: config.general.profile.to_string(),
            file_size,
            path: backup_file,
            reason: reason.map(ToString::to_string),
        })
    }

    /// Lists all valid backups in the specified backup directory, sorted newest first.
    pub fn list_backups(backup_dir: &Path) -> Result<Vec<BackupMetadata>, ConfigError> {
        if !backup_dir.exists() {
            return Ok(Vec::new());
        }

        let mut backups = Vec::new();

        for entry in fs::read_dir(backup_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("toml") {
                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                    if Self::validate_backup_id(stem).is_ok() {
                        if let Ok(config) = load_config_from_file(&path) {
                            let metadata = fs::metadata(&path)?;
                            let created_at = metadata
                                .created()
                                .or_else(|_| metadata.modified())
                                .map(|t| chrono::DateTime::<Utc>::from(t).to_rfc3339())
                                .unwrap_or_else(|_| Utc::now().to_rfc3339());

                            backups.push(BackupMetadata {
                                id: stem.to_string(),
                                created_at,
                                profile: config.general.profile.to_string(),
                                file_size: metadata.len(),
                                path,
                                reason: None,
                            });
                        }
                    }
                }
            }
        }

        // Sort descending by timestamp/ID
        backups.sort_by(|a, b| b.id.cmp(&a.id));

        Ok(backups)
    }

    /// Restores a backup to the target configuration location.
    ///
    /// # Safety Invariants
    /// 1. Rejects path traversal and malformed IDs.
    /// 2. Ensures the backup file resides strictly within the authorized `backup_dir`.
    /// 3. Validates the TOML syntax and schema to detect corrupted backups.
    /// 4. Automatically creates a safety backup of existing target before replacement.
    /// 5. Writes target atomically.
    pub fn restore_backup(
        backup_id: &str,
        backup_dir: &Path,
        target_path: &Path,
        dry_run: bool,
    ) -> Result<RestoreResult, ConfigError> {
        Self::validate_backup_id(backup_id)?;

        let backup_file = backup_dir.join(format!("{backup_id}.toml"));

        // Path containment verification
        if !PlatformPaths::is_safe_subpath(backup_dir, &backup_file) {
            return Err(ConfigError::SecurityViolation(format!(
                "Backup path '{}' escapes authorized backup directory",
                backup_file.display()
            )));
        }

        if !backup_file.exists() {
            return Err(ConfigError::NotFound(backup_file));
        }

        // Validate contents to detect corrupted backups
        let config =
            load_config_from_file(&backup_file).map_err(|e| ConfigError::CorruptedBackup {
                id: backup_id.to_string(),
                details: format!("Failed to parse backup content: {e}"),
            })?;

        if dry_run {
            return Ok(RestoreResult {
                backup_id: backup_id.to_string(),
                target_path: target_path.to_path_buf(),
                dry_run: true,
                safety_backup_id: None,
            });
        }

        // Create safety backup of existing active configuration if present
        let safety_backup_id = if target_path.exists() {
            let safety_meta = Self::create_backup(
                target_path,
                backup_dir,
                Some(&format!("pre-restore-safety-for-{backup_id}")),
            )?;
            Some(safety_meta.id)
        } else {
            None
        };

        // Atomically replace target
        save_config_atomic(target_path, &config)?;

        // Verify restoration on disk
        let verified = load_config_from_file(target_path)?;
        if verified != config {
            return Err(ConfigError::AtomicWrite(
                "Restored configuration failed post-write verification".to_string(),
            ));
        }

        Ok(RestoreResult {
            backup_id: backup_id.to_string(),
            target_path: target_path.to_path_buf(),
            dry_run: false,
            safety_backup_id,
        })
    }
}
