//! Language provider interface and registry of supported ecosystems.

pub mod cpp;
pub mod flutter;
pub mod go;
pub mod javascript;
pub mod python;
pub mod ros2;
pub mod rust;
pub mod zig;

use crate::model::{
    CompatibilityReport, DetectedProject, DiscoveredTool, LanguageId, SetupPlan, ToolDefinition,
};
use std::path::Path;
use std::sync::Arc;

/// Extensible interface implemented by each language ecosystem provider.
pub trait LanguageProvider: Send + Sync {
    /// Canonical language identifier.
    fn id(&self) -> LanguageId;

    /// User-facing display name.
    fn display_name(&self) -> &'static str;

    /// File or directory markers used to identify this project on disk.
    fn project_markers(&self) -> &[&'static str];

    /// Complete list of standard tool definitions for this language.
    fn tools(&self) -> Vec<ToolDefinition>;

    /// Inspects a directory to determine if it is a valid project root for this language.
    fn detect_project(&self, dir: &Path) -> Option<DetectedProject>;

    /// Evaluates toolchain and project compatibility.
    fn check_compatibility(
        &self,
        project: &DetectedProject,
        tools: &[DiscoveredTool],
    ) -> CompatibilityReport;

    /// Recommended AEK skill IDs for projects of this language.
    fn recommended_skills(&self) -> Vec<&'static str>;

    /// Generates a safe, non-destructive toolchain installation plan for missing tools.
    fn setup_plan(
        &self,
        missing_required: &[ToolDefinition],
        missing_optional: &[ToolDefinition],
    ) -> SetupPlan;

    /// Default validation command steps for projects of this language.
    fn default_validation_commands(
        &self,
        project: &DetectedProject,
    ) -> Vec<(&'static str, Vec<String>)>;
}

/// Returns instances of all built-in language providers.
#[must_use]
pub fn get_all_providers() -> Vec<Arc<dyn LanguageProvider>> {
    vec![
        Arc::new(rust::RustProvider),
        Arc::new(go::GoProvider),
        Arc::new(cpp::CppProvider),
        Arc::new(zig::ZigProvider),
        Arc::new(python::PythonProvider),
        Arc::new(javascript::JavascriptProvider),
        Arc::new(flutter::FlutterProvider),
        Arc::new(ros2::Ros2Provider),
    ]
}

/// Looks up a language provider by its language identifier.
#[must_use]
pub fn find_provider(id: &LanguageId) -> Option<Arc<dyn LanguageProvider>> {
    get_all_providers().into_iter().find(|p| p.id() == *id)
}
