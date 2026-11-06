//! Configuration schema migration engine with automated backup and atomic execution.

use crate::errors::DistributionError;
use std::fs;
use std::path::Path;

/// Step function transforming configuration value between schema versions.
pub type MigrationFn =
    Box<dyn Fn(toml::Value) -> Result<toml::Value, DistributionError> + Send + Sync>;

/// An individual migration step between two sequential schema versions.
pub struct ConfigMigrationStep {
    pub from_version: u32,
    pub to_version: u32,
    pub description: String,
    pub migrate_fn: MigrationFn,
}

impl std::fmt::Debug for ConfigMigrationStep {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ConfigMigrationStep")
            .field("from_version", &self.from_version)
            .field("to_version", &self.to_version)
            .field("description", &self.description)
            .finish()
    }
}

/// Execution plan for migrating configuration schema versions.
#[derive(Debug)]
pub struct MigrationPlan {
    pub from_version: u32,
    pub to_version: u32,
    pub steps: Vec<usize>,
}

/// Engine coordinating schema detection, migration planning, and atomic application.
#[derive(Default)]
pub struct ConfigMigrator {
    steps: Vec<ConfigMigrationStep>,
}

impl ConfigMigrator {
    /// Constructs a new empty configuration migrator.
    #[must_use]
    pub fn new() -> Self {
        Self { steps: Vec::new() }
    }

    /// Registers a migration step.
    pub fn register_step(&mut self, step: ConfigMigrationStep) {
        self.steps.push(step);
    }

    /// Detects the active schema version in a TOML configuration file.
    pub fn detect_version(&self, config_path: &Path) -> Result<u32, DistributionError> {
        let content = fs::read_to_string(config_path)?;
        let val: toml::Value = toml::from_str(&content)?;

        if let Some(version) = val.get("schema_version").and_then(|v| v.as_integer()) {
            Ok(u32::try_from(version).unwrap_or(1))
        } else {
            Ok(1)
        }
    }

    /// Plans the migration sequence to reach `target_version`.
    pub fn plan_migration(
        &self,
        config_path: &Path,
        target_version: u32,
    ) -> Result<MigrationPlan, DistributionError> {
        let current_version = self.detect_version(config_path)?;

        if current_version == target_version {
            return Ok(MigrationPlan {
                from_version: current_version,
                to_version: target_version,
                steps: Vec::new(),
            });
        }

        if current_version > target_version {
            return Err(DistributionError::Migration(format!(
                "Cannot migrate backwards from schema version {current_version} to {target_version}"
            )));
        }

        let mut step_indices = Vec::new();
        let mut curr = current_version;

        while curr < target_version {
            if let Some((idx, next_step)) = self
                .steps
                .iter()
                .enumerate()
                .find(|(_, s)| s.from_version == curr)
            {
                step_indices.push(idx);
                curr = next_step.to_version;
            } else {
                return Err(DistributionError::Migration(format!(
                    "No migration path registered from schema version {curr} to {target_version}"
                )));
            }
        }

        Ok(MigrationPlan {
            from_version: current_version,
            to_version: target_version,
            steps: step_indices,
        })
    }

    /// Executes the migration plan, saving a safety backup copy prior to mutation.
    pub fn execute_migration(
        &self,
        config_path: &Path,
        plan: &MigrationPlan,
    ) -> Result<(), DistributionError> {
        if plan.steps.is_empty() {
            return Ok(());
        }

        // 1. Create safety backup
        let backup_path = config_path.with_extension("bak");
        fs::copy(config_path, &backup_path)?;

        // 2. Read and parse configuration
        let content = fs::read_to_string(config_path)?;
        let mut value: toml::Value = toml::from_str(&content)?;

        // 3. Sequentially apply steps
        for &step_idx in &plan.steps {
            let step = &self.steps[step_idx];
            value = (step.migrate_fn)(value)?;
        }

        // 4. Write modified configuration atomically
        let new_content = toml::to_string_pretty(&value)
            .map_err(|e| DistributionError::Migration(format!("Serialization error: {e}")))?;
        let temp_path = config_path.with_extension("tmp");
        fs::write(&temp_path, new_content)?;
        fs::rename(&temp_path, config_path)?;

        Ok(())
    }
}
