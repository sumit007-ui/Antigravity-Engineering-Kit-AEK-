//! Language toolchain diagnostics and doctor runner.

use crate::discovery::ToolchainDiscovery;
use crate::model::{DiscoveredTool, LanguageId, SetupPlan, ToolRequirement};
use crate::providers::find_provider;
use crate::registry::ToolchainRegistry;
use serde::{Deserialize, Serialize};

/// Diagnostic report for a single language toolchain on host system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageDoctorReport {
    pub language: LanguageId,
    pub display_name: String,
    pub overall_status: String,
    pub tools: Vec<DiscoveredTool>,
    pub setup_plan: Option<SetupPlan>,
}

/// Runner for auditing language toolchain health across host system.
#[derive(Debug, Clone)]
pub struct LanguageDoctor {
    discovery: ToolchainDiscovery,
    registry: ToolchainRegistry,
}

impl Default for LanguageDoctor {
    fn default() -> Self {
        Self::new(ToolchainDiscovery::new(), ToolchainRegistry::default())
    }
}

impl LanguageDoctor {
    /// Creates a new `LanguageDoctor`.
    #[must_use]
    pub fn new(discovery: ToolchainDiscovery, registry: ToolchainRegistry) -> Self {
        Self {
            discovery,
            registry,
        }
    }

    /// Audits a specific language.
    pub async fn audit_language(&self, lang: &LanguageId) -> LanguageDoctorReport {
        let provider = find_provider(lang);
        let display_name = provider
            .as_ref()
            .map(|p| p.display_name().to_string())
            .unwrap_or_else(|| lang.to_string());

        let defs = self.registry.get_tools_for_language(lang);
        let mut discovered = Vec::new();
        let mut missing_required = Vec::new();
        let mut missing_optional = Vec::new();

        for def in defs {
            let tool = self.discovery.discover_tool(&def).await;
            if !tool.is_available {
                if def.requirement == ToolRequirement::Required {
                    missing_required.push(def.clone());
                } else {
                    missing_optional.push(def.clone());
                }
            }
            discovered.push(tool);
        }

        let overall_status = if !missing_required.is_empty() {
            "FAIL".to_string()
        } else if !missing_optional.is_empty() {
            "WARN".to_string()
        } else {
            "PASS".to_string()
        };

        let setup_plan = provider.map(|p| p.setup_plan(&missing_required, &missing_optional));

        LanguageDoctorReport {
            language: lang.clone(),
            display_name,
            overall_status,
            tools: discovered,
            setup_plan,
        }
    }
}
