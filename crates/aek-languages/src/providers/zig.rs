//! Zig language toolchain provider.

use super::LanguageProvider;
use crate::model::{
    CompatibilityItem, CompatibilityLevel, CompatibilityReport, DetectedProject, DiscoveredTool,
    LanguageId, ProjectEvidence, SetupPlan, ToolCategory, ToolDefinition, ToolRequirement,
};
use aek_platform::OsKind;
use std::fs;
use std::path::Path;

pub struct ZigProvider;

impl LanguageProvider for ZigProvider {
    fn id(&self) -> LanguageId {
        LanguageId::Zig
    }

    fn display_name(&self) -> &'static str {
        "Zig"
    }

    fn project_markers(&self) -> &[&'static str] {
        &["build.zig", "build.zig.zon"]
    }

    fn tools(&self) -> Vec<ToolDefinition> {
        let all_os = vec![OsKind::Windows, OsKind::Linux, OsKind::MacOS];
        vec![
            ToolDefinition {
                id: "zig-compiler".to_string(),
                name: "Zig Compiler & Build Tool".to_string(),
                language: LanguageId::Zig,
                binary: "zig".to_string(),
                version_args: vec!["version".to_string()],
                supported_os: all_os.clone(),
                requirement: ToolRequirement::Required,
                category: ToolCategory::Compiler,
                doc_url: "https://ziglang.org/documentation/".to_string(),
                install_guidance: "Download from https://ziglang.org/download/".to_string(),
            },
            ToolDefinition {
                id: "zig-zls".to_string(),
                name: "Zig Language Server (zls)".to_string(),
                language: LanguageId::Zig,
                binary: "zls".to_string(),
                version_args: vec!["--version".to_string()],
                supported_os: all_os,
                requirement: ToolRequirement::Optional,
                category: ToolCategory::LanguageServer,
                doc_url: "https://github.com/zigtools/zls".to_string(),
                install_guidance: "Download from https://github.com/zigtools/zls/releases"
                    .to_string(),
            },
        ]
    }

    fn detect_project(&self, dir: &Path) -> Option<DetectedProject> {
        let build_zig = dir.join("build.zig");
        let build_zon = dir.join("build.zig.zon");

        if !build_zig.is_file() && !build_zon.is_file() {
            return None;
        }

        let mut name = dir
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| "zig-project".to_string());

        if let Ok(content) = fs::read_to_string(&build_zon) {
            for line in content.lines() {
                let trimmed = line.trim();
                if trimmed.starts_with(".name =") || trimmed.starts_with(".name=") {
                    if let Some(val) = trimmed.split('=').nth(1) {
                        let cleaned = val
                            .trim()
                            .trim_matches(',')
                            .trim_matches('"')
                            .trim_matches('\'');
                        if !cleaned.is_empty() {
                            name = cleaned.to_string();
                            break;
                        }
                    }
                }
            }
        }

        Some(DetectedProject {
            path: dir.to_path_buf(),
            language: LanguageId::Zig,
            name,
            evidence: vec![ProjectEvidence {
                file: if build_zig.is_file() {
                    "build.zig"
                } else {
                    "build.zig.zon"
                }
                .to_string(),
                matched_content: None,
                description: "Zig build script or package manifest detected".to_string(),
            }],
            monorepo_root: None,
            is_nested: false,
        })
    }

    fn check_compatibility(
        &self,
        _project: &DetectedProject,
        tools: &[DiscoveredTool],
    ) -> CompatibilityReport {
        let mut items = Vec::new();
        let mut overall_level = CompatibilityLevel::Compatible;
        let mut remediation = Vec::new();

        let zig_tool = tools.iter().find(|t| t.definition.binary == "zig");

        if zig_tool.is_some_and(|t| t.is_available) {
            let ver = zig_tool
                .and_then(|t| t.version.as_deref())
                .unwrap_or("unknown");
            items.push(CompatibilityItem {
                name: "Zig Compiler".to_string(),
                level: CompatibilityLevel::Compatible,
                message: format!("Zig version {ver} available"),
                remediation: None,
            });
        } else {
            overall_level = CompatibilityLevel::Incompatible;
            items.push(CompatibilityItem {
                name: "Zig Compiler".to_string(),
                level: CompatibilityLevel::Incompatible,
                message: "Required 'zig' binary missing from system PATH".to_string(),
                remediation: Some("Download Zig from https://ziglang.org/download/".to_string()),
            });
            remediation.push("Install Zig: https://ziglang.org/download/".to_string());
        }

        CompatibilityReport {
            language: LanguageId::Zig,
            overall_level,
            items,
            remediation_steps: remediation,
        }
    }

    fn recommended_skills(&self) -> Vec<&'static str> {
        vec!["zig-engineer"]
    }

    fn setup_plan(
        &self,
        missing_required: &[ToolDefinition],
        missing_optional: &[ToolDefinition],
    ) -> SetupPlan {
        let mut commands = Vec::new();
        let mut instructions = Vec::new();

        if missing_required.iter().any(|t| t.binary == "zig") {
            if cfg!(windows) {
                commands.push("winget install zig.zig".to_string());
            } else {
                instructions.push("Install Zig via package manager or download official release from https://ziglang.org/download/".to_string());
            }
        }

        if missing_optional.iter().any(|t| t.binary == "zls") {
            instructions.push(
                "Download matching ZLS release from https://github.com/zigtools/zls/releases"
                    .to_string(),
            );
        }

        SetupPlan {
            language: LanguageId::Zig,
            missing_required: missing_required.to_vec(),
            missing_optional: missing_optional.to_vec(),
            suggested_commands: commands,
            instructions,
        }
    }

    fn default_validation_commands(
        &self,
        _project: &DetectedProject,
    ) -> Vec<(&'static str, Vec<String>)> {
        vec![
            ("build", vec!["zig".to_string(), "build".to_string()]),
            (
                "test",
                vec!["zig".to_string(), "build".to_string(), "test".to_string()],
            ),
        ]
    }
}
