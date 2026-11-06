//! Rust language toolchain provider.

use super::LanguageProvider;
use crate::model::{
    CompatibilityItem, CompatibilityLevel, CompatibilityReport, DetectedProject, DiscoveredTool,
    LanguageId, ProjectEvidence, SetupPlan, ToolCategory, ToolDefinition, ToolRequirement,
};
use aek_platform::OsKind;
use std::fs;
use std::path::Path;

pub struct RustProvider;

impl LanguageProvider for RustProvider {
    fn id(&self) -> LanguageId {
        LanguageId::Rust
    }

    fn display_name(&self) -> &'static str {
        "Rust"
    }

    fn project_markers(&self) -> &[&'static str] {
        &["Cargo.toml"]
    }

    fn tools(&self) -> Vec<ToolDefinition> {
        let all_os = vec![OsKind::Windows, OsKind::Linux, OsKind::MacOS];
        vec![
            ToolDefinition {
                id: "rust-cargo".to_string(),
                name: "Cargo".to_string(),
                language: LanguageId::Rust,
                binary: "cargo".to_string(),
                version_args: vec!["--version".to_string()],
                supported_os: all_os.clone(),
                requirement: ToolRequirement::Required,
                category: ToolCategory::PackageManager,
                doc_url: "https://doc.rust-lang.org/cargo/".to_string(),
                install_guidance: "Install via rustup from https://rustup.rs/".to_string(),
            },
            ToolDefinition {
                id: "rust-compiler".to_string(),
                name: "Rust Compiler (rustc)".to_string(),
                language: LanguageId::Rust,
                binary: "rustc".to_string(),
                version_args: vec!["--version".to_string()],
                supported_os: all_os.clone(),
                requirement: ToolRequirement::Required,
                category: ToolCategory::Compiler,
                doc_url: "https://doc.rust-lang.org/rustc/".to_string(),
                install_guidance: "Install via rustup from https://rustup.rs/".to_string(),
            },
            ToolDefinition {
                id: "rust-rustup".to_string(),
                name: "Rustup Toolchain Manager".to_string(),
                language: LanguageId::Rust,
                binary: "rustup".to_string(),
                version_args: vec!["--version".to_string()],
                supported_os: all_os.clone(),
                requirement: ToolRequirement::Recommended,
                category: ToolCategory::Sdk,
                doc_url: "https://rustup.rs/".to_string(),
                install_guidance: "Download from https://rustup.rs/".to_string(),
            },
            ToolDefinition {
                id: "rust-rustfmt".to_string(),
                name: "Rustfmt Code Formatter".to_string(),
                language: LanguageId::Rust,
                binary: "rustfmt".to_string(),
                version_args: vec!["--version".to_string()],
                supported_os: all_os.clone(),
                requirement: ToolRequirement::Optional,
                category: ToolCategory::Formatter,
                doc_url: "https://github.com/rust-lang/rustfmt".to_string(),
                install_guidance: "Run 'rustup component add rustfmt'".to_string(),
            },
            ToolDefinition {
                id: "rust-clippy".to_string(),
                name: "Clippy Linter".to_string(),
                language: LanguageId::Rust,
                binary: "cargo-clippy".to_string(),
                version_args: vec!["--version".to_string()],
                supported_os: all_os.clone(),
                requirement: ToolRequirement::Optional,
                category: ToolCategory::Linter,
                doc_url: "https://github.com/rust-lang/rust-clippy".to_string(),
                install_guidance: "Run 'rustup component add clippy'".to_string(),
            },
            ToolDefinition {
                id: "rust-analyzer".to_string(),
                name: "Rust Analyzer LSP".to_string(),
                language: LanguageId::Rust,
                binary: "rust-analyzer".to_string(),
                version_args: vec!["--version".to_string()],
                supported_os: all_os.clone(),
                requirement: ToolRequirement::Optional,
                category: ToolCategory::LanguageServer,
                doc_url: "https://rust-analyzer.github.io/".to_string(),
                install_guidance: "Run 'rustup component add rust-analyzer'".to_string(),
            },
            ToolDefinition {
                id: "rust-nextest".to_string(),
                name: "Cargo Nextest".to_string(),
                language: LanguageId::Rust,
                binary: "cargo-nextest".to_string(),
                version_args: vec!["--version".to_string()],
                supported_os: all_os.clone(),
                requirement: ToolRequirement::Optional,
                category: ToolCategory::Other("test-runner".to_string()),
                doc_url: "https://nexte.st/".to_string(),
                install_guidance: "Run 'cargo install cargo-nextest'".to_string(),
            },
            ToolDefinition {
                id: "rust-sccache".to_string(),
                name: "Sccache Compiler Cache".to_string(),
                language: LanguageId::Rust,
                binary: "sccache".to_string(),
                version_args: vec!["--version".to_string()],
                supported_os: all_os,
                requirement: ToolRequirement::Optional,
                category: ToolCategory::Other("cache".to_string()),
                doc_url: "https://github.com/mozilla/sccache".to_string(),
                install_guidance: "Run 'cargo install sccache'".to_string(),
            },
        ]
    }

    fn detect_project(&self, dir: &Path) -> Option<DetectedProject> {
        let manifest_path = dir.join("Cargo.toml");
        if !manifest_path.is_file() {
            return None;
        }

        let content = fs::read_to_string(&manifest_path).ok()?;
        let mut name = dir
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| "rust-project".to_string());

        // Extract package name from [package] name = "..."
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("name =") || trimmed.starts_with("name=") {
                if let Some(val) = trimmed.split('=').nth(1) {
                    let cleaned = val.trim().trim_matches('"').trim_matches('\'');
                    if !cleaned.is_empty() {
                        name = cleaned.to_string();
                        break;
                    }
                }
            }
        }

        let is_workspace = content.contains("[workspace]");

        Some(DetectedProject {
            path: dir.to_path_buf(),
            language: LanguageId::Rust,
            name,
            evidence: vec![ProjectEvidence {
                file: "Cargo.toml".to_string(),
                matched_content: if is_workspace {
                    Some("workspace manifest".to_string())
                } else {
                    Some("package manifest".to_string())
                },
                description: "Cargo project manifest detected".to_string(),
            }],
            monorepo_root: if is_workspace {
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

        let cargo = tools.iter().find(|t| t.definition.binary == "cargo");
        let rustc = tools.iter().find(|t| t.definition.binary == "rustc");

        if cargo.is_some_and(|t| t.is_available) && rustc.is_some_and(|t| t.is_available) {
            items.push(CompatibilityItem {
                name: "Rust Toolchain".to_string(),
                level: CompatibilityLevel::Compatible,
                message: format!(
                    "Rust compiler {} and Cargo available",
                    rustc
                        .and_then(|t| t.version.as_deref())
                        .unwrap_or("unknown")
                ),
                remediation: None,
            });
        } else {
            overall_level = CompatibilityLevel::Incompatible;
            items.push(CompatibilityItem {
                name: "Rust Toolchain".to_string(),
                level: CompatibilityLevel::Incompatible,
                message: "Required 'cargo' or 'rustc' binary missing from system PATH".to_string(),
                remediation: Some("Install Rust using rustup from https://rustup.rs/".to_string()),
            });
            remediation.push("Install rustup: https://rustup.rs/".to_string());
        }

        // Check rust-version in Cargo.toml if available
        let manifest = project.path.join("Cargo.toml");
        if let Ok(content) = fs::read_to_string(&manifest) {
            for line in content.lines() {
                let trimmed = line.trim();
                if trimmed.starts_with("rust-version =") || trimmed.starts_with("rust-version=") {
                    if let Some(req_ver) = trimmed.split('=').nth(1) {
                        let cleaned = req_ver.trim().trim_matches('"').trim_matches('\'');
                        items.push(CompatibilityItem {
                            name: "Rust Edition / Version Requirement".to_string(),
                            level: CompatibilityLevel::Compatible,
                            message: format!("Project requires Rust version >= {cleaned}"),
                            remediation: None,
                        });
                    }
                }
            }
        }

        CompatibilityReport {
            language: LanguageId::Rust,
            overall_level,
            items,
            remediation_steps: remediation,
        }
    }

    fn recommended_skills(&self) -> Vec<&'static str> {
        vec!["rust-systems-engineer"]
    }

    fn setup_plan(
        &self,
        missing_required: &[ToolDefinition],
        missing_optional: &[ToolDefinition],
    ) -> SetupPlan {
        let mut commands = Vec::new();
        let mut instructions = Vec::new();

        if missing_required
            .iter()
            .any(|t| t.binary == "cargo" || t.binary == "rustc")
        {
            instructions.push(
                "Install the official Rust toolchain via rustup: https://rustup.rs/".to_string(),
            );
            if cfg!(windows) {
                instructions.push(
                    "Download and run rustup-init.exe from https://win.rustup.rs/".to_string(),
                );
            } else {
                instructions.push(
                    "Run: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
                        .to_string(),
                );
            }
        }

        for opt in missing_optional {
            match opt.binary.as_str() {
                "rustfmt" => commands.push("rustup component add rustfmt".to_string()),
                "cargo-clippy" => commands.push("rustup component add clippy".to_string()),
                "rust-analyzer" => commands.push("rustup component add rust-analyzer".to_string()),
                "cargo-nextest" => commands.push("cargo install cargo-nextest".to_string()),
                "sccache" => commands.push("cargo install sccache".to_string()),
                _ => {}
            }
        }

        SetupPlan {
            language: LanguageId::Rust,
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
                "check",
                vec![
                    "cargo".to_string(),
                    "check".to_string(),
                    "--workspace".to_string(),
                ],
            ),
            (
                "format",
                vec![
                    "cargo".to_string(),
                    "fmt".to_string(),
                    "--all".to_string(),
                    "--".to_string(),
                    "--check".to_string(),
                ],
            ),
            (
                "lint",
                vec![
                    "cargo".to_string(),
                    "clippy".to_string(),
                    "--workspace".to_string(),
                    "--all-targets".to_string(),
                    "--".to_string(),
                    "-D".to_string(),
                    "warnings".to_string(),
                ],
            ),
            (
                "test",
                vec![
                    "cargo".to_string(),
                    "test".to_string(),
                    "--workspace".to_string(),
                ],
            ),
        ]
    }
}
