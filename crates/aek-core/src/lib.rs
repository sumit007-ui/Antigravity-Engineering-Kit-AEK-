//! # aek-core
//!
//! Core domain logic, system abstractions, and execution policies for the
//! Antigravity Engineering Kit (AEK).

pub mod context;
pub mod doctor;
pub mod error;
pub mod system;

pub use context::{AekContext, ExecutionMode};
pub use doctor::{
    AntigravityDetector, CheckStatus, DoctorCategory, DoctorCheck, DoctorReport, DoctorRunner,
    ToolDetector,
};
pub use error::AekCoreError;
pub use system::{CommandResult, PlatformInfo, execute_safe};
