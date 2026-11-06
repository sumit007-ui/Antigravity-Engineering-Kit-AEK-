//! # aek-agents
//!
//! Universal multi-agent compatibility framework supporting Google Antigravity,
//! Claude Code, OpenCode, and Cursor.

pub mod adapter;
pub mod adapters;
pub mod antigravity;
pub mod capabilities;
pub mod conflicts;
pub mod diagnostics;
pub mod discovery;
pub mod errors;
pub mod model;
pub mod profiles;
pub mod registry;
pub mod synchronization;
pub mod translator;

// Re-exports for convenience
pub use adapter::{AgentAdapter, AgentDetection, InstallReport, McpMergeReport};
pub use adapters::{AntigravityAdapter, ClaudeCodeAdapter, CursorAdapter, OpenCodeAdapter};
pub use capabilities::{
    AgentCapabilities, AgentCapabilityCategory, AgentCapabilityRecord, AgentCapabilityRegistry,
    CapabilityStatus,
};
pub use conflicts::{ConflictResolution, ConflictStrategy};
pub use diagnostics::{AgentDiagnosticItem, AgentDoctor, AgentDoctorReport};
pub use discovery::AgentDiscoveryEngine;
pub use errors::AgentError;
pub use model::{
    AgentConfigInspection, AgentExportedConfig, AgentId, AgentMetadata, AgentOwnershipManifest,
    AgentPermissions, AgentProfile, AgentProfileKind, AgentUninstallReport, AgentValidationReport,
    ManagedFileRecord, SyncItem, SyncPlan, SyncReport,
};
pub use profiles::AgentProfileManager;
pub use registry::AgentRegistry;
pub use synchronization::MultiAgentSynchronizer;
pub use translator::AgentConfigTranslator;
