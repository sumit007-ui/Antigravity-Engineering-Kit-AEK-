//! JavaScript & TypeScript language toolchain provider.

use super::LanguageProvider;
use crate::model::{
    CompatibilityItem, CompatibilityLevel, CompatibilityReport, DetectedProject, DiscoveredTool,
    LanguageId, ProjectEvidence, SetupPlan, ToolCategory, ToolDefinition, ToolRequirement,
};
use aek_platform::OsKind;
use std::fs;
use std::path::Path;

pub struct JavascriptProvider;

impl LanguageProvider for JavascriptProvider {
    fn id(&self) -> LanguageId {
        LanguageId::Javascript
    }

    fn display_name(&self) -> &'static str {
        "JavaScript / TypeScript"
    }

    fn project_markers(&self) -> &[&'static str] {
        &["package.json", "tsconfig.json", "pnpm-workspace.yaml"]
    }

    fn tools(&self) -> Vec<ToolDefinition> {
        let all_os = vec![OsKind::Windows, OsKind::Linux, OsKind::MacOS];
        vec![
            ToolDefinition {
                id: "js-node".to_string(),
                name: "Node.js Runtime".to_string(),
                language: LanguageId::Javascript,
                binary: "node".to_string(),
                version_args: vec!["--version".to_string()],
                supported_os: all_os.clone(),
                requirement: ToolRequirement::Required,
                category: ToolCategory::Compiler,
                doc_url: "https://nodejs.org/en/docs".to_string(),
                install_guidance: "Install Node.js LTS from https://nodejs.org/".to_string(),
            },
            ToolDefinition {
                id: "js-npm".to_string(),
                name: "Node Package Manager (npm)".to_string(),
                language: LanguageId::Javascript,
                binary: "npm".to_string(),
                version_args: vec!["--version".to_string()],
                supported_os: all_os.clone(),
                requirement: ToolRequirement::Required,
                category: ToolCategory::PackageManager,
                doc_url: "https://docs.npmjs.com/".to_string(),
                install_guidance: "Bundled with Node.js".to_string(),
            },
            ToolDefinition {
                id: "js-pnpm".to_string(),
                name: "Fast Disk-Efficient Package Manager (pnpm)".to_string(),
                language: LanguageId::Javascript,
                binary: "pnpm".to_string(),
                version_args: vec!["--version".to_string()],
                supported_os: all_os.clone(),
                requirement: ToolRequirement::Optional,
                category: ToolCategory::PackageManager,
                doc_url: "https://pnpm.io/".to_string(),
                install_guidance: "Run 'npm install -g pnpm'".to_string(),
            },
            ToolDefinition {
                id: "js-yarn".to_string(),
                name: "Yarn Package Manager".to_string(),
                language: LanguageId::Javascript,
                binary: "yarn".to_string(),
                version_args: vec!["--version".to_string()],
                supported_os: all_os.clone(),
                requirement: ToolRequirement::Optional,
                category: ToolCategory::PackageManager,
                doc_url: "https://yarnpkg.com/".to_string(),
                install_guidance: "Run 'npm install -g yarn'".to_string(),
            },
            ToolDefinition {
                id: "js-typescript".to_string(),
                name: "TypeScript Compiler (tsc)".to_string(),
                language: LanguageId::Javascript,
                binary: "tsc".to_string(),
                version_args: vec!["--version".to_string()],
                supported_os: all_os.clone(),
                requirement: ToolRequirement::Optional,
                category: ToolCategory::Compiler,
                doc_url: "https://www.typescriptlang.org/docs/".to_string(),
                install_guidance: "Run 'npm install -g typescript'".to_string(),
            },
            ToolDefinition {
                id: "js-eslint".to_string(),
                name: "ESLint".to_string(),
                language: LanguageId::Javascript,
                binary: "eslint".to_string(),
                version_args: vec!["--version".to_string()],
                supported_os: all_os.clone(),
                requirement: ToolRequirement::Optional,
                category: ToolCategory::Linter,
                doc_url: "https://eslint.org/".to_string(),
                install_guidance: "Install via project devDependencies or 'npm install -g eslint'"
                    .to_string(),
            },
            ToolDefinition {
                id: "js-prettier".to_string(),
                name: "Prettier Formatter".to_string(),
                language: LanguageId::Javascript,
                binary: "prettier".to_string(),
                version_args: vec!["--version".to_string()],
                supported_os: all_os.clone(),
                requirement: ToolRequirement::Optional,
                category: ToolCategory::Formatter,
                doc_url: "https://prettier.io/".to_string(),
                install_guidance: "Run 'npm install -g prettier'".to_string(),
            },
            ToolDefinition {
                id: "js-vitest".to_string(),
                name: "Vitest Test Runner".to_string(),
                language: LanguageId::Javascript,
                binary: "vitest".to_string(),
                version_args: vec!["--version".to_string()],
                supported_os: all_os,
                requirement: ToolRequirement::Optional,
                category: ToolCategory::Other("test-runner".to_string()),
                doc_url: "https://vitest.dev/".to_string(),
                install_guidance: "Install via project devDependencies".to_string(),
            },
        ]
    }

    fn detect_project(&self, dir: &Path) -> Option<DetectedProject> {
        let pkg_json = dir.join("package.json");
        let tsconfig = dir.join("tsconfig.json");
        let pnpm_work = dir.join("pnpm-workspace.yaml");

        if !pkg_json.is_file() && !tsconfig.is_file() && !pnpm_work.is_file() {
            return None;
        }

        let mut name = dir
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| "javascript-project".to_string());

        if let Ok(content) = fs::read_to_string(&pkg_json) {
            if let Ok(val) = serde_json::from_str::<serde_json::Value>(&content) {
                if let Some(n) = val.get("name").and_then(|v| v.as_str()) {
                    name = n.to_string();
                }
            }
        }

        let is_workspace = pnpm_work.is_file();

        Some(DetectedProject {
            path: dir.to_path_buf(),
            language: LanguageId::Javascript,
            name,
            evidence: vec![ProjectEvidence {
                file: if pkg_json.is_file() {
                    "package.json"
                } else if tsconfig.is_file() {
                    "tsconfig.json"
                } else {
                    "pnpm-workspace.yaml"
                }
                .to_string(),
                matched_content: None,
                description: "JavaScript/TypeScript manifest detected".to_string(),
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
        _project: &DetectedProject,
        tools: &[DiscoveredTool],
    ) -> CompatibilityReport {
        let mut items = Vec::new();
        let mut overall_level = CompatibilityLevel::Compatible;
        let mut remediation = Vec::new();

        let node = tools.iter().find(|t| t.definition.binary == "node");

        if node.is_some_and(|t| t.is_available) {
            let ver = node.and_then(|t| t.version.as_deref()).unwrap_or("unknown");
            items.push(CompatibilityItem {
                name: "Node.js Runtime".to_string(),
                level: CompatibilityLevel::Compatible,
                message: format!("Node.js version {ver} available"),
                remediation: None,
            });
        } else {
            overall_level = CompatibilityLevel::Incompatible;
            items.push(CompatibilityItem {
                name: "Node.js Runtime".to_string(),
                level: CompatibilityLevel::Incompatible,
                message: "Node.js runtime missing from system PATH".to_string(),
                remediation: Some("Install Node.js LTS from https://nodejs.org/".to_string()),
            });
            remediation.push("Install Node.js: https://nodejs.org/".to_string());
        }

        CompatibilityReport {
            language: LanguageId::Javascript,
            overall_level,
            items,
            remediation_steps: remediation,
        }
    }

    fn recommended_skills(&self) -> Vec<&'static str> {
        vec!["production-code-reviewer", "devops-engineer"]
    }

    fn setup_plan(
        &self,
        missing_required: &[ToolDefinition],
        missing_optional: &[ToolDefinition],
    ) -> SetupPlan {
        let mut commands = Vec::new();
        let mut instructions = Vec::new();

        if missing_required.iter().any(|t| t.binary == "node") {
            instructions.push("Download and install Node.js from https://nodejs.org/".to_string());
        }

        for opt in missing_optional {
            match opt.binary.as_str() {
                "pnpm" => commands.push("npm install -g pnpm".to_string()),
                "yarn" => commands.push("npm install -g yarn".to_string()),
                "tsc" => commands.push("npm install -g typescript".to_string()),
                "prettier" => commands.push("npm install -g prettier".to_string()),
                "eslint" => commands.push("npm install -g eslint".to_string()),
                _ => {}
            }
        }

        SetupPlan {
            language: LanguageId::Javascript,
            missing_required: missing_required.to_vec(),
            missing_optional: missing_optional.to_vec(),
            suggested_commands: commands,
            instructions,
        }
    }

    fn default_validation_commands(
        &self,
        project: &DetectedProject,
    ) -> Vec<(&'static str, Vec<String>)> {
        let tsconfig = project.path.join("tsconfig.json");
        let mut cmds = Vec::new();
        if tsconfig.is_file() {
            cmds.push(("typecheck", vec!["tsc".to_string(), "--noEmit".to_string()]));
        }
        cmds.push(("test", vec!["npm".to_string(), "test".to_string()]));
        cmds
    }
}
