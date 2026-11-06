//! # aek-core
//!
//! Core engine, execution abstractions, diagnostics, and profile tiers for the
//! Antigravity Engineering Kit (AEK).

pub mod context;
pub mod diagnostics;
pub mod doctor;
pub mod error;
pub mod execution;
pub mod profile;
pub mod system;

pub use context::{AekContext, ExecutionMode};
pub use diagnostics::{DiagnosticCheck, DiagnosticReport, DiagnosticStatus, DiagnosticsRunner};
pub use doctor::{
    AntigravityDetector, CheckStatus, DoctorCategory, DoctorCheck, DoctorReport, DoctorRunner,
    ToolDetector,
};
pub use error::{AekCoreError, ExecutionError, ProfileError};
pub use execution::{CommandResult, SafeCommand, redact_secrets};
pub use profile::{ProfileDescriptor, ProfileKind};
pub use system::{PlatformInfo, execute_safe};
