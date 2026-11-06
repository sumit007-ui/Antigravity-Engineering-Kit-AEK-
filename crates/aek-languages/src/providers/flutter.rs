//! Flutter and Dart language toolchain provider.

use super::LanguageProvider;
use crate::model::{
    CompatibilityItem, CompatibilityLevel, CompatibilityReport, DetectedProject, DiscoveredTool,
    LanguageId, ProjectEvidence, SetupPlan, ToolCategory, ToolDefinition, ToolRequirement,
};
use aek_platform::OsKind;
use std::fs;
use std::path::Path;

pub struct FlutterProvider;

impl LanguageProvider for FlutterProvider {
    fn id(&self) -> LanguageId {
        LanguageId::Flutter
    }

    fn display_name(&self) -> &'static str {
        "Flutter / Dart"
    }

    fn project_markers(&self) -> &[&'static str] {
        &["pubspec.yaml"]
    }

    fn tools(&self) -> Vec<ToolDefinition> {
        let all_os = vec![OsKind::Windows, OsKind::Linux, OsKind::MacOS];
        vec![
            ToolDefinition {
                id: "flutter-sdk".to_string(),
                name: "Flutter SDK".to_string(),
                language: LanguageId::Flutter,
                binary: "flutter".to_string(),
                version_args: vec!["--version".to_string()],
                supported_os: all_os.clone(),
                requirement: ToolRequirement::Required,
                category: ToolCategory::Sdk,
                doc_url: "https://docs.flutter.dev/".to_string(),
                install_guidance:
                    "Install Flutter from https://docs.flutter.dev/get-started/install".to_string(),
            },
            ToolDefinition {
                id: "flutter-dart".to_string(),
                name: "Dart SDK".to_string(),
                language: LanguageId::Flutter,
                binary: "dart".to_string(),
                version_args: vec!["--version".to_string()],
                supported_os: all_os,
                requirement: ToolRequirement::Required,
                category: ToolCategory::Compiler,
                doc_url: "https://dart.dev/guides".to_string(),
                install_guidance:
                    "Bundled with Flutter SDK or standalone from https://dart.dev/get-dart"
                        .to_string(),
            },
        ]
    }

    fn detect_project(&self, dir: &Path) -> Option<DetectedProject> {
        let pubspec = dir.join("pubspec.yaml");
        if !pubspec.is_file() {
            return None;
        }

        let content = fs::read_to_string(&pubspec).ok()?;
        let mut name = dir
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| "flutter-project".to_string());

        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("name:") {
                if let Some(n) = trimmed.strip_prefix("name:") {
                    let cleaned = n.trim().trim_matches('"').trim_matches('\'');
                    if !cleaned.is_empty() {
                        name = cleaned.to_string();
                        break;
                    }
                }
            }
        }

        let is_flutter = content.contains("sdk: flutter") || content.contains("flutter:");

        Some(DetectedProject {
            path: dir.to_path_buf(),
            language: LanguageId::Flutter,
            name,
            evidence: vec![ProjectEvidence {
                file: "pubspec.yaml".to_string(),
                matched_content: if is_flutter {
                    Some("Flutter SDK dependency".to_string())
                } else {
                    Some("Dart package manifest".to_string())
                },
                description: "Dart / Flutter package specification detected".to_string(),
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

        let flutter = tools.iter().find(|t| t.definition.binary == "flutter");
        let dart = tools.iter().find(|t| t.definition.binary == "dart");

        if flutter.is_some_and(|t| t.is_available) {
            let ver = flutter
                .and_then(|t| t.version.as_deref())
                .unwrap_or("unknown");
            items.push(CompatibilityItem {
                name: "Flutter SDK".to_string(),
                level: CompatibilityLevel::Compatible,
                message: format!("Flutter SDK version {ver} available"),
                remediation: None,
            });
        } else if dart.is_some_and(|t| t.is_available) {
            let ver = dart.and_then(|t| t.version.as_deref()).unwrap_or("unknown");
            items.push(CompatibilityItem {
                name: "Dart SDK (Standalone)".to_string(),
                level: CompatibilityLevel::Warning,
                message: format!("Dart SDK {ver} available (Flutter SDK not on PATH)"),
                remediation: Some("Install Flutter SDK if UI targets are needed".to_string()),
            });
            overall_level = CompatibilityLevel::Warning;
        } else {
            overall_level = CompatibilityLevel::Incompatible;
            items.push(CompatibilityItem {
                name: "Flutter/Dart Toolchain".to_string(),
                level: CompatibilityLevel::Incompatible,
                message: "Neither Flutter nor Dart SDK found on system PATH".to_string(),
                remediation: Some(
                    "Install Flutter SDK from https://docs.flutter.dev/get-started/install"
                        .to_string(),
                ),
            });
            remediation.push(
                "Install Flutter SDK: https://docs.flutter.dev/get-started/install".to_string(),
            );
        }

        CompatibilityReport {
            language: LanguageId::Flutter,
            overall_level,
            items,
            remediation_steps: remediation,
        }
    }

    fn recommended_skills(&self) -> Vec<&'static str> {
        vec!["flutter-engineer"]
    }

    fn setup_plan(
        &self,
        missing_required: &[ToolDefinition],
        missing_optional: &[ToolDefinition],
    ) -> SetupPlan {
        let commands = Vec::new();
        let mut instructions = Vec::new();

        if missing_required.iter().any(|t| t.binary == "flutter") {
            instructions.push("Download Flutter SDK from https://docs.flutter.dev/get-started/install and add bin directory to PATH".to_string());
        }

        SetupPlan {
            language: LanguageId::Flutter,
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
            (
                "analyze",
                vec!["flutter".to_string(), "analyze".to_string()],
            ),
            ("test", vec!["flutter".to_string(), "test".to_string()]),
        ]
    }
}
