//! Configuration validation rules and constraints checking.

use crate::error::ConfigError;
use crate::model::AekConfig;

/// Validates an `AekConfig` instance against all system constraints.
///
/// # Errors
/// Returns `ConfigError::Validation` if any setting is outside allowed bounds.
pub fn validate_config(config: &AekConfig) -> Result<(), ConfigError> {
    config.validate()
}
