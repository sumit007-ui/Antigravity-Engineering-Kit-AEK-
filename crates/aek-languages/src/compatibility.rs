//! Compatibility engine checking toolchain, SDK, and OS constraints.

use crate::discovery::ToolchainDiscovery;
use crate::model::{CompatibilityReport, DetectedProject};
use crate::providers::find_provider;
use crate::registry::ToolchainRegistry;

/// Evaluates project and toolchain compatibility.
#[derive(Debug, Clone)]
pub struct CompatibilityEngine {
    discovery: ToolchainDiscovery,
    registry: ToolchainRegistry,
}

impl Default for CompatibilityEngine {
    fn default() -> Self {
        Self::new(ToolchainDiscovery::new(), ToolchainRegistry::default())
    }
}

impl CompatibilityEngine {
    /// Creates a new `CompatibilityEngine`.
    #[must_use]
    pub fn new(discovery: ToolchainDiscovery, registry: ToolchainRegistry) -> Self {
        Self {
            discovery,
            registry,
        }
    }

    /// Evaluates compatibility for a detected project.
    pub async fn check_project(&self, project: &DetectedProject) -> CompatibilityReport {
        let provider = match find_provider(&project.language) {
            Some(p) => p,
            None => {
                return CompatibilityReport {
                    language: project.language.clone(),
                    overall_level: crate::model::CompatibilityLevel::Unknown,
                    items: vec![crate::model::CompatibilityItem {
                        name: "Custom Language".to_string(),
                        level: crate::model::CompatibilityLevel::Unknown,
                        message: format!("No standard provider found for '{}'", project.language),
                        remediation: None,
                    }],
                    remediation_steps: Vec::new(),
                };
            }
        };

        let defs = self.registry.get_tools_for_language(&project.language);
        let mut discovered = Vec::new();
        for def in &defs {
            let tool = self.discovery.discover_tool(def).await;
            discovered.push(tool);
        }

        provider.check_compatibility(project, &discovered)
    }
}
