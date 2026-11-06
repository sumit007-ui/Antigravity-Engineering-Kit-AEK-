//! Typed errors for the project validation engine.

use thiserror::Error;

/// Validation pipeline errors.
#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("I/O error during validation: {0}")]
    Io(#[from] std::io::Error),

    #[error("Language toolchain error: {0}")]
    Language(#[from] aek_languages::errors::LanguageError),

    #[error("Subprocess execution error: {0}")]
    Execution(#[from] aek_core::ExecutionError),

    #[error("Validation failed at step '{0}': exit code {1:?}")]
    StepFailed(String, Option<i32>),

    #[error("Validation timeout exceeded for step '{0}' ({1}s)")]
    Timeout(String, u64),

    #[error("Security policy violation: {0}")]
    Security(String),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}
