//! Go language toolchain provider.

use super::LanguageProvider;
use crate::model::{
    CompatibilityItem, CompatibilityLevel, CompatibilityReport, DetectedProject, DiscoveredTool,
    LanguageId, ProjectEvidence, SetupPlan, ToolCategory, ToolDefinition, ToolRequirement,
};
use aek_platform::OsKind;
use std::fs;
use std::path::Path;

pub struct GoProvider;

impl LanguageProvider for GoProvider {
    fn id(&self) -> LanguageId {
        LanguageId::Go
    }

    fn display_name(&self) -> &'static str {
        "Go"
    }

    fn project_markers(&self) -> &[&'static str] {
        &["go.mod", "go.work"]
    }

    fn tools(&self) -> Vec<ToolDefinition> {
        let all_os = vec![OsKind::Windows, OsKind::Linux, OsKind::MacOS];
        vec![
            ToolDefinition {
                id: "go-compiler".to_string(),
                name: "Go Compiler & Toolchain".to_string(),
                language: LanguageId::Go,
                binary: "go".to_string(),
                version_args: vec!["version".to_string()],
                supported_os: all_os.clone(),
                requirement: ToolRequirement::Required,
                category: ToolCategory::Compiler,
                doc_url: "https://go.dev/doc/".to_string(),
                install_guidance: "Install from https://go.dev/dl/".to_string(),
            },
            ToolDefinition {
                id: "go-gopls".to_string(),
                name: "Go Language Server (gopls)".to_string(),
                language: LanguageId::Go,
                binary: "gopls".to_string(),
                version_args: vec!["version".to_string()],
                supported_os: all_os.clone(),
                requirement: ToolRequirement::Optional,
                category: ToolCategory::LanguageServer,
                doc_url: "https://pkg.go.dev/golang.org/x/tools/gopls".to_string(),
                install_guidance: "Run 'go install golang.org/x/tools/gopls@latest'".to_string(),
            },
            ToolDefinition {
                id: "go-dlv".to_string(),
                name: "Delve Debugger (dlv)".to_string(),
                language: LanguageId::Go,
                binary: "dlv".to_string(),
                version_args: vec!["version".to_string()],
                supported_os: all_os.clone(),
                requirement: ToolRequirement::Optional,
                category: ToolCategory::Debugger,
                doc_url: "https://github.com/go-delve/delve".to_string(),
                install_guidance: "Run 'go install github.com/go-delve/delve/cmd/dlv@latest'"
                    .to_string(),
            },
            ToolDefinition {
                id: "go-golangci-lint".to_string(),
                name: "GolangCI-Lint".to_string(),
                language: LanguageId::Go,
                binary: "golangci-lint".to_string(),
                version_args: vec!["version".to_string()],
                supported_os: all_os,
                requirement: ToolRequirement::Optional,
                category: ToolCategory::Linter,
                doc_url: "https://golangci-lint.run/".to_string(),
                install_guidance: "Install via https://golangci-lint.run/usage/install/"
                    .to_string(),
            },
        ]
    }

    fn detect_project(&self, dir: &Path) -> Option<DetectedProject> {
        let go_mod = dir.join("go.mod");
        let go_work = dir.join("go.work");

        if !go_mod.is_file() && !go_work.is_file() {
            return None;
        }

        let mut name = dir
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| "go-project".to_string());

        if let Ok(content) = fs::read_to_string(&go_mod) {
            for line in content.lines() {
                let trimmed = line.trim();
                if trimmed.starts_with("module ") {
                    if let Some(m) = trimmed.strip_prefix("module ") {
                        name = m.trim().to_string();
                        break;
                    }
                }
            }
        }

        Some(DetectedProject {
            path: dir.to_path_buf(),
            language: LanguageId::Go,
            name,
            evidence: vec![ProjectEvidence {
                file: if go_mod.is_file() {
                    "go.mod"
                } else {
                    "go.work"
                }
                .to_string(),
                matched_content: None,
                description: "Go module or workspace manifest detected".to_string(),
            }],
            monorepo_root: if go_work.is_file() {
                Some(dir.to_path_buf())
            } else {
                None
            },
            is_nested: false,
        })
    }

    fn check_compatibility(
        &self,
        project: &DetectedProject,
        tools: &[DiscoveredTool],
    ) -> CompatibilityReport {
        let mut items = Vec::new();
        let mut overall_level = CompatibilityLevel::Compatible;
        let mut remediation = Vec::new();

        let go_tool = tools.iter().find(|t| t.definition.binary == "go");

        if go_tool.is_some_and(|t| t.is_available) {
            items.push(CompatibilityItem {
                name: "Go Toolchain".to_string(),
                level: CompatibilityLevel::Compatible,
                message: format!(
                    "Go version {} available",
                    go_tool
                        .and_then(|t| t.version.as_deref())
                        .unwrap_or("unknown")
                ),
                remediation: None,
            });
        } else {
            overall_level = CompatibilityLevel::Incompatible;
            items.push(CompatibilityItem {
                name: "Go Toolchain".to_string(),
                level: CompatibilityLevel::Incompatible,
                message: "Required 'go' binary missing from system PATH".to_string(),
                remediation: Some("Install Go from https://go.dev/dl/".to_string()),
            });
            remediation.push("Install Go: https://go.dev/dl/".to_string());
        }

        let go_mod = project.path.join("go.mod");
        if let Ok(content) = fs::read_to_string(&go_mod) {
            for line in content.lines() {
                let trimmed = line.trim();
                if trimmed.starts_with("go ") {
                    if let Some(v) = trimmed.strip_prefix("go ") {
                        items.push(CompatibilityItem {
                            name: "Go Version Specification".to_string(),
                            level: CompatibilityLevel::Compatible,
                            message: format!("Module requires Go >= {v}"),
                            remediation: None,
                        });
                    }
                }
            }
        }

        CompatibilityReport {
            language: LanguageId::Go,
            overall_level,
            items,
            remediation_steps: remediation,
        }
    }

    fn recommended_skills(&self) -> Vec<&'static str> {
        vec!["go-backend-engineer"]
    }

    fn setup_plan(
        &self,
        missing_required: &[ToolDefinition],
        missing_optional: &[ToolDefinition],
    ) -> SetupPlan {
        let mut commands = Vec::new();
        let mut instructions = Vec::new();

        if missing_required.iter().any(|t| t.binary == "go") {
            instructions.push(
                "Download and install the official Go toolchain from https://go.dev/dl/"
                    .to_string(),
            );
        }

        for opt in missing_optional {
            match opt.binary.as_str() {
                "gopls" => commands.push("go install golang.org/x/tools/gopls@latest".to_string()),
                "dlv" => {
                    commands.push("go install github.com/go-delve/delve/cmd/dlv@latest".to_string())
                }
                "golangci-lint" => instructions.push(
                    "Install golangci-lint via https://golangci-lint.run/usage/install/"
                        .to_string(),
                ),
                _ => {}
            }
        }

        SetupPlan {
            language: LanguageId::Go,
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
            ("version", vec!["go".to_string(), "version".to_string()]),
            (
                "vet",
                vec!["go".to_string(), "vet".to_string(), "./...".to_string()],
            ),
            (
                "test",
                vec!["go".to_string(), "test".to_string(), "./...".to_string()],
            ),
            (
                "build",
                vec!["go".to_string(), "build".to_string(), "./...".to_string()],
            ),
        ]
    }
}
