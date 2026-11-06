//! Python language toolchain provider.

use super::LanguageProvider;
use crate::model::{
    CompatibilityItem, CompatibilityLevel, CompatibilityReport, DetectedProject, DiscoveredTool,
    LanguageId, ProjectEvidence, SetupPlan, ToolCategory, ToolDefinition, ToolRequirement,
};
use aek_platform::OsKind;
use std::fs;
use std::path::Path;

pub struct PythonProvider;

impl LanguageProvider for PythonProvider {
    fn id(&self) -> LanguageId {
        LanguageId::Python
    }

    fn display_name(&self) -> &'static str {
        "Python"
    }

    fn project_markers(&self) -> &[&'static str] {
        &["pyproject.toml", "requirements.txt", "Pipfile", "uv.lock"]
    }

    fn tools(&self) -> Vec<ToolDefinition> {
        let all_os = vec![OsKind::Windows, OsKind::Linux, OsKind::MacOS];
        vec![
            ToolDefinition {
                id: "python-runtime".to_string(),
                name: "Python Runtime".to_string(),
                language: LanguageId::Python,
                binary: if cfg!(windows) {
                    "python".to_string()
                } else {
                    "python3".to_string()
                },
                version_args: vec!["--version".to_string()],
                supported_os: all_os.clone(),
                requirement: ToolRequirement::Required,
                category: ToolCategory::Compiler,
                doc_url: "https://docs.python.org/3/".to_string(),
                install_guidance: "Install Python 3 from https://www.python.org/".to_string(),
            },
            ToolDefinition {
                id: "python-uv".to_string(),
                name: "uv Package Manager".to_string(),
                language: LanguageId::Python,
                binary: "uv".to_string(),
                version_args: vec!["--version".to_string()],
                supported_os: all_os.clone(),
                requirement: ToolRequirement::Recommended,
                category: ToolCategory::PackageManager,
                doc_url: "https://docs.astral.sh/uv/".to_string(),
                install_guidance: "Install uv via official installer or pip".to_string(),
            },
            ToolDefinition {
                id: "python-pip".to_string(),
                name: "Pip Package Installer".to_string(),
                language: LanguageId::Python,
                binary: "pip".to_string(),
                version_args: vec!["--version".to_string()],
                supported_os: all_os.clone(),
                requirement: ToolRequirement::Optional,
                category: ToolCategory::PackageManager,
                doc_url: "https://pip.pypa.io/".to_string(),
                install_guidance: "Bundled with standard Python installations".to_string(),
            },
            ToolDefinition {
                id: "python-ruff".to_string(),
                name: "Ruff Linter & Formatter".to_string(),
                language: LanguageId::Python,
                binary: "ruff".to_string(),
                version_args: vec!["--version".to_string()],
                supported_os: all_os.clone(),
                requirement: ToolRequirement::Optional,
                category: ToolCategory::Linter,
                doc_url: "https://docs.astral.sh/ruff/".to_string(),
                install_guidance: "Install via 'uv tool install ruff' or 'pip install ruff'"
                    .to_string(),
            },
            ToolDefinition {
                id: "python-pyright".to_string(),
                name: "Pyright Type Checker".to_string(),
                language: LanguageId::Python,
                binary: "pyright".to_string(),
                version_args: vec!["--version".to_string()],
                supported_os: all_os.clone(),
                requirement: ToolRequirement::Optional,
                category: ToolCategory::LanguageServer,
                doc_url: "https://microsoft.github.io/pyright/".to_string(),
                install_guidance: "Install via 'npm install -g pyright' or 'pip install pyright'"
                    .to_string(),
            },
            ToolDefinition {
                id: "python-pytest".to_string(),
                name: "Pytest Framework".to_string(),
                language: LanguageId::Python,
                binary: "pytest".to_string(),
                version_args: vec!["--version".to_string()],
                supported_os: all_os,
                requirement: ToolRequirement::Optional,
                category: ToolCategory::Other("test-runner".to_string()),
                doc_url: "https://docs.pytest.org/".to_string(),
                install_guidance: "Install via 'pip install pytest'".to_string(),
            },
        ]
    }

    fn detect_project(&self, dir: &Path) -> Option<DetectedProject> {
        let pyproject = dir.join("pyproject.toml");
        let requirements = dir.join("requirements.txt");
        let pipfile = dir.join("Pipfile");
        let uv_lock = dir.join("uv.lock");

        if !pyproject.is_file()
            && !requirements.is_file()
            && !pipfile.is_file()
            && !uv_lock.is_file()
        {
            return None;
        }

        let mut name = dir
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| "python-project".to_string());

        if let Ok(content) = fs::read_to_string(&pyproject) {
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
        }

        let marker = if pyproject.is_file() {
            "pyproject.toml"
        } else if uv_lock.is_file() {
            "uv.lock"
        } else if pipfile.is_file() {
            "Pipfile"
        } else {
            "requirements.txt"
        };

        Some(DetectedProject {
            path: dir.to_path_buf(),
            language: LanguageId::Python,
            name,
            evidence: vec![ProjectEvidence {
                file: marker.to_string(),
                matched_content: None,
                description: "Python project specification or dependencies detected".to_string(),
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

        let py = tools
            .iter()
            .find(|t| t.definition.binary == "python" || t.definition.binary == "python3");

        if py.is_some_and(|t| t.is_available) {
            let ver = py.and_then(|t| t.version.as_deref()).unwrap_or("unknown");
            items.push(CompatibilityItem {
                name: "Python Runtime".to_string(),
                level: CompatibilityLevel::Compatible,
                message: format!("Python version {ver} available"),
                remediation: None,
            });
        } else {
            overall_level = CompatibilityLevel::Incompatible;
            items.push(CompatibilityItem {
                name: "Python Runtime".to_string(),
                level: CompatibilityLevel::Incompatible,
                message: "Python runtime missing from system PATH".to_string(),
                remediation: Some("Install Python 3 from https://www.python.org/".to_string()),
            });
            remediation.push("Install Python 3: https://www.python.org/".to_string());
        }

        CompatibilityReport {
            language: LanguageId::Python,
            overall_level,
            items,
            remediation_steps: remediation,
        }
    }

    fn recommended_skills(&self) -> Vec<&'static str> {
        vec!["python-engineer"]
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
            .any(|t| t.category == ToolCategory::Compiler)
        {
            instructions
                .push("Install Python 3 from https://www.python.org/downloads/".to_string());
        }

        for opt in missing_optional {
            match opt.binary.as_str() {
                "uv" => instructions.push(
                    "Install uv: https://docs.astral.sh/uv/getting-started/installation/"
                        .to_string(),
                ),
                "ruff" => commands.push("pip install ruff".to_string()),
                "pyright" => commands.push("pip install pyright".to_string()),
                "pytest" => commands.push("pip install pytest".to_string()),
                _ => {}
            }
        }

        SetupPlan {
            language: LanguageId::Python,
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
                "lint",
                vec!["ruff".to_string(), "check".to_string(), ".".to_string()],
            ),
            (
                "format",
                vec![
                    "ruff".to_string(),
                    "format".to_string(),
                    "--check".to_string(),
                    ".".to_string(),
                ],
            ),
            ("test", vec!["pytest".to_string()]),
        ]
    }
}
