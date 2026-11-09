//! C and C++ language toolchain provider.

use super::LanguageProvider;
use crate::model::{
    CompatibilityItem, CompatibilityLevel, CompatibilityReport, DetectedProject, DiscoveredTool,
    LanguageId, ProjectEvidence, SetupPlan, ToolCategory, ToolDefinition, ToolRequirement,
};
use aek_platform::OsKind;
use std::fs;
use std::path::Path;

pub struct CppProvider;

impl LanguageProvider for CppProvider {
    fn id(&self) -> LanguageId {
        LanguageId::Cpp
    }

    fn display_name(&self) -> &'static str {
        "C/C++"
    }

    fn project_markers(&self) -> &[&'static str] {
        &["CMakeLists.txt", "meson.build", "compile_commands.json"]
    }

    fn tools(&self) -> Vec<ToolDefinition> {
        let all_os = vec![OsKind::Windows, OsKind::Linux, OsKind::MacOS];
        vec![
            ToolDefinition {
                id: "cpp-cmake".to_string(),
                name: "CMake Build System".to_string(),
                language: LanguageId::Cpp,
                binary: "cmake".to_string(),
                version_args: vec!["--version".to_string()],
                supported_os: all_os.clone(),
                requirement: ToolRequirement::Required,
                category: ToolCategory::BuildSystem,
                doc_url: "https://cmake.org/documentation/".to_string(),
                install_guidance: "Install CMake from https://cmake.org/download/".to_string(),
            },
            ToolDefinition {
                id: "cpp-ninja".to_string(),
                name: "Ninja Build System".to_string(),
                language: LanguageId::Cpp,
                binary: "ninja".to_string(),
                version_args: vec!["--version".to_string()],
                supported_os: all_os.clone(),
                requirement: ToolRequirement::Optional,
                category: ToolCategory::BuildSystem,
                doc_url: "https://ninja-build.org/".to_string(),
                install_guidance: "Install Ninja build system".to_string(),
            },
            ToolDefinition {
                id: "cpp-clang".to_string(),
                name: "Clang C Compiler".to_string(),
                language: LanguageId::Cpp,
                binary: "clang".to_string(),
                version_args: vec!["--version".to_string()],
                supported_os: all_os.clone(),
                requirement: ToolRequirement::Optional,
                category: ToolCategory::Compiler,
                doc_url: "https://clang.llvm.org/".to_string(),
                install_guidance: "Install LLVM/Clang toolchain".to_string(),
            },
            ToolDefinition {
                id: "cpp-clangxx".to_string(),
                name: "Clang++ C++ Compiler".to_string(),
                language: LanguageId::Cpp,
                binary: "clang++".to_string(),
                version_args: vec!["--version".to_string()],
                supported_os: all_os.clone(),
                requirement: ToolRequirement::Optional,
                category: ToolCategory::Compiler,
                doc_url: "https://clang.llvm.org/".to_string(),
                install_guidance: "Install LLVM/Clang toolchain".to_string(),
            },
            ToolDefinition {
                id: "cpp-gcc".to_string(),
                name: "GCC C Compiler".to_string(),
                language: LanguageId::Cpp,
                binary: "gcc".to_string(),
                version_args: vec!["--version".to_string()],
                supported_os: vec![OsKind::Linux, OsKind::Windows],
                requirement: ToolRequirement::Optional,
                category: ToolCategory::Compiler,
                doc_url: "https://gcc.gnu.org/".to_string(),
                install_guidance: "Install GCC or MinGW toolchain".to_string(),
            },
            ToolDefinition {
                id: "cpp-gxx".to_string(),
                name: "G++ C++ Compiler".to_string(),
                language: LanguageId::Cpp,
                binary: "g++".to_string(),
                version_args: vec!["--version".to_string()],
                supported_os: vec![OsKind::Linux, OsKind::Windows],
                requirement: ToolRequirement::Optional,
                category: ToolCategory::Compiler,
                doc_url: "https://gcc.gnu.org/".to_string(),
                install_guidance: "Install G++ or MinGW toolchain".to_string(),
            },
            ToolDefinition {
                id: "cpp-clangd".to_string(),
                name: "Clangd Language Server".to_string(),
                language: LanguageId::Cpp,
                binary: "clangd".to_string(),
                version_args: vec!["--version".to_string()],
                supported_os: all_os.clone(),
                requirement: ToolRequirement::Optional,
                category: ToolCategory::LanguageServer,
                doc_url: "https://clangd.llvm.org/".to_string(),
                install_guidance: "Install clangd via LLVM".to_string(),
            },
            ToolDefinition {
                id: "cpp-clang-tidy".to_string(),
                name: "Clang-Tidy Linter".to_string(),
                language: LanguageId::Cpp,
                binary: "clang-tidy".to_string(),
                version_args: vec!["--version".to_string()],
                supported_os: all_os,
                requirement: ToolRequirement::Optional,
                category: ToolCategory::Linter,
                doc_url: "https://clang.llvm.org/extra/clang-tidy/".to_string(),
                install_guidance: "Install clang-tidy via LLVM tools".to_string(),
            },
        ]
    }

    fn detect_project(&self, dir: &Path) -> Option<DetectedProject> {
        let cmake = dir.join("CMakeLists.txt");
        let meson = dir.join("meson.build");
        let compile_commands = dir.join("compile_commands.json");

        if !cmake.is_file() && !meson.is_file() && !compile_commands.is_file() {
            return None;
        }

        let mut name = dir
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| "cpp-project".to_string());

        if let Ok(content) = fs::read_to_string(&cmake) {
            for line in content.lines() {
                let trimmed = line.trim();
                if trimmed.starts_with("project(") || trimmed.starts_with("project (") {
                    if let Some(open) = trimmed.find('(') {
                        let rest = &trimmed[open + 1..];
                        let candidate = rest
                            .split_whitespace()
                            .next()
                            .unwrap_or("")
                            .trim_matches(')');
                        if !candidate.is_empty() {
                            name = candidate.to_string();
                            break;
                        }
                    }
                }
            }
        }

        Some(DetectedProject {
            path: dir.to_path_buf(),
            language: LanguageId::Cpp,
            name,
            evidence: vec![ProjectEvidence {
                file: if cmake.is_file() {
                    "CMakeLists.txt"
                } else if meson.is_file() {
                    "meson.build"
                } else {
                    "compile_commands.json"
                }
                .to_string(),
                matched_content: None,
                description: "C/C++ build system or compilation database detected".to_string(),
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

        let cmake = tools.iter().find(|t| t.definition.binary == "cmake");
        let compiler = tools.iter().find(|t| {
            t.definition.binary == "clang++"
                || t.definition.binary == "g++"
                || t.definition.binary == "clang"
                || t.definition.binary == "gcc"
        });

        if cmake.is_some_and(|t| t.is_available) {
            items.push(CompatibilityItem {
                name: "Build System".to_string(),
                level: CompatibilityLevel::Compatible,
                message: format!(
                    "CMake version {} available",
                    cmake
                        .and_then(|t| t.version.as_deref())
                        .unwrap_or("unknown")
                ),
                remediation: None,
            });
        } else {
            overall_level = CompatibilityLevel::Warning;
            items.push(CompatibilityItem {
                name: "Build System".to_string(),
                level: CompatibilityLevel::Warning,
                message: "CMake not found on system PATH".to_string(),
                remediation: Some("Install CMake from https://cmake.org/download/".to_string()),
            });
        }

        if let Some(comp) = compiler.filter(|t| t.is_available) {
            items.push(CompatibilityItem {
                name: "C/C++ Compiler".to_string(),
                level: CompatibilityLevel::Compatible,
                message: format!(
                    "Found compiler '{}' ({})",
                    comp.definition.binary,
                    comp.version.as_deref().unwrap_or("unknown")
                ),
                remediation: None,
            });
        } else {
            if overall_level != CompatibilityLevel::Incompatible {
                overall_level = CompatibilityLevel::Warning;
            }
            items.push(CompatibilityItem {
                name: "C/C++ Compiler".to_string(),
                level: CompatibilityLevel::Warning,
                message: "Neither Clang nor GCC found on system PATH (MSVC may be in Developer Command Prompt)".to_string(),
                remediation: Some("Install LLVM/Clang or Visual Studio C++ build tools".to_string()),
            });
            remediation.push("Install Clang or Visual Studio Build Tools".to_string());
        }

        CompatibilityReport {
            language: LanguageId::Cpp,
            overall_level,
            items,
            remediation_steps: remediation,
        }
    }

    fn recommended_skills(&self) -> Vec<&'static str> {
        vec!["cpp-systems-engineer"]
    }

    fn setup_plan(
        &self,
        missing_required: &[ToolDefinition],
        missing_optional: &[ToolDefinition],
    ) -> SetupPlan {
        let mut commands = Vec::new();
        let mut instructions = Vec::new();

        if cfg!(windows) {
            instructions.push("On Windows, install LLVM or Visual Studio Build Tools with C++ workload: https://visualstudio.microsoft.com/visual-cpp-build-tools/".to_string());
            commands.push("winget install Kitware.CMake".to_string());
            commands.push("winget install LLVM.LLVM".to_string());
        } else {
            instructions.push("On Linux, install build-essential and cmake: sudo apt install build-essential cmake ninja-build clang clangd".to_string());
            commands
                .push("sudo apt update && sudo apt install -y build-essential cmake".to_string());
        }

        SetupPlan {
            language: LanguageId::Cpp,
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
        let cmake = project.path.join("CMakeLists.txt");
        if cmake.is_file() {
            vec![
                (
                    "configure",
                    vec![
                        "cmake".to_string(),
                        "-B".to_string(),
                        "build".to_string(),
                        "-S".to_string(),
                        ".".to_string(),
                    ],
                ),
                (
                    "build",
                    vec![
                        "cmake".to_string(),
                        "--build".to_string(),
                        "build".to_string(),
                    ],
                ),
                (
                    "test",
                    vec![
                        "ctest".to_string(),
                        "--test-dir".to_string(),
                        "build".to_string(),
                        "--output-on-failure".to_string(),
                    ],
                ),
            ]
        } else {
            vec![("check", vec!["cmake".to_string(), "--version".to_string()])]
        }
    }
}
