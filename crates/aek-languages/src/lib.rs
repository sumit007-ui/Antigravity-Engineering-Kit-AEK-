//! # aek-languages
//!
//! Language toolchain management, project detection, tool discovery,
//! and compatibility analysis for the Antigravity Engineering Kit (AEK).

pub mod compatibility;
pub mod detection;
pub mod diagnostics;
pub mod discovery;
pub mod errors;
pub mod model;
pub mod providers;
pub mod registry;

pub use compatibility::CompatibilityEngine;
pub use detection::ProjectDetector;
pub use diagnostics::{LanguageDoctor, LanguageDoctorReport};
pub use discovery::{ToolchainDiscovery, extract_clean_version};
pub use errors::LanguageError;
pub use model::{
    CompatibilityItem, CompatibilityLevel, CompatibilityReport, DetectedProject, DiscoveredTool,
    LanguageId, ProjectEvidence, SetupPlan, ToolCategory, ToolDefinition, ToolRequirement,
};
pub use providers::{LanguageProvider, find_provider, get_all_providers};
pub use registry::ToolchainRegistry;
