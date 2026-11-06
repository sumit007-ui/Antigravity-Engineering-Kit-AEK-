//! ROS 2 (Robot Operating System) toolchain provider.

use super::LanguageProvider;
use crate::model::{
    CompatibilityItem, CompatibilityLevel, CompatibilityReport, DetectedProject, DiscoveredTool,
    LanguageId, ProjectEvidence, SetupPlan, ToolCategory, ToolDefinition, ToolRequirement,
};
use aek_platform::OsKind;
use std::fs;
use std::path::Path;

pub struct Ros2Provider;

impl LanguageProvider for Ros2Provider {
    fn id(&self) -> LanguageId {
        LanguageId::Ros2
    }

    fn display_name(&self) -> &'static str {
        "ROS 2 (Robot Operating System)"
    }

    fn project_markers(&self) -> &[&'static str] {
        &["package.xml", "colcon.meta"]
    }

    fn tools(&self) -> Vec<ToolDefinition> {
        let all_os = vec![OsKind::Linux, OsKind::Windows];
        vec![
            ToolDefinition {
                id: "ros2-colcon".to_string(),
                name: "Colcon Build Tool".to_string(),
                language: LanguageId::Ros2,
                binary: "colcon".to_string(),
                version_args: vec!["--version".to_string()],
                supported_os: all_os.clone(),
                requirement: ToolRequirement::Required,
                category: ToolCategory::BuildSystem,
                doc_url: "https://colcon.readthedocs.io/".to_string(),
                install_guidance: "Install via 'pip install colcon-common-extensions'".to_string(),
            },
            ToolDefinition {
                id: "ros2-cli".to_string(),
                name: "ROS 2 Command Line Interface".to_string(),
                language: LanguageId::Ros2,
                binary: "ros2".to_string(),
                version_args: vec!["--version".to_string()],
                supported_os: all_os.clone(),
                requirement: ToolRequirement::Optional,
                category: ToolCategory::Sdk,
                doc_url: "https://docs.ros.org/".to_string(),
                install_guidance: "Install from official ROS 2 distribution (Humble/Iron/Jazzy)"
                    .to_string(),
            },
            ToolDefinition {
                id: "ros2-rosdep".to_string(),
                name: "Rosdep Dependency Tool".to_string(),
                language: LanguageId::Ros2,
                binary: "rosdep".to_string(),
                version_args: vec!["--version".to_string()],
                supported_os: all_os,
                requirement: ToolRequirement::Optional,
                category: ToolCategory::PackageManager,
                doc_url: "https://docs.ros.org/en/independent/api/rosdep/html/".to_string(),
                install_guidance: "Install via pip or python3-rosdep".to_string(),
            },
        ]
    }

    fn detect_project(&self, dir: &Path) -> Option<DetectedProject> {
        let pkg_xml = dir.join("package.xml");
        let colcon_meta = dir.join("colcon.meta");

        if !pkg_xml.is_file() && !colcon_meta.is_file() {
            return None;
        }

        // If package.xml exists, inspect contents to strictly verify it is a ROS 2 package
        let mut name = dir
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| "ros2-package".to_string());

        if pkg_xml.is_file() {
            let content = fs::read_to_string(&pkg_xml).ok()?;
            let is_ros_package = content.contains("<package format=")
                || content.contains("ament_cmake")
                || content.contains("ament_python")
                || content.contains("<buildtool_depend>")
                || content.contains("http://download.ros.org/schema/package_format");

            if !is_ros_package && !colcon_meta.is_file() {
                // Not a valid ROS 2 package manifest
                return None;
            }

            // Extract package name: <name>...</name>
            if let Some(start) = content.find("<name>") {
                if let Some(end) = content[start + 6..].find("</name>") {
                    let extracted = content[start + 6..start + 6 + end].trim();
                    if !extracted.is_empty() {
                        name = extracted.to_string();
                    }
                }
            }
        }

        Some(DetectedProject {
            path: dir.to_path_buf(),
            language: LanguageId::Ros2,
            name,
            evidence: vec![ProjectEvidence {
                file: if pkg_xml.is_file() {
                    "package.xml"
                } else {
                    "colcon.meta"
                }
                .to_string(),
                matched_content: Some("ROS 2 package manifest / colcon metadata".to_string()),
                description: "ROS 2 robotic package manifest detected".to_string(),
            }],
            monorepo_root: if colcon_meta.is_file() {
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

        let ros_distro = std::env::var("ROS_DISTRO").ok();
        if let Some(distro) = ros_distro {
            items.push(CompatibilityItem {
                name: "ROS Distribution Environment".to_string(),
                level: CompatibilityLevel::Compatible,
                message: format!("ROS_DISTRO set to '{distro}'"),
                remediation: None,
            });
        } else {
            overall_level = CompatibilityLevel::Warning;
            items.push(CompatibilityItem {
                name: "ROS Distribution Environment".to_string(),
                level: CompatibilityLevel::Warning,
                message: "ROS_DISTRO environment variable not detected. Host workspace may not be sourced.".to_string(),
                remediation: Some("Source your ROS 2 environment (e.g. 'source /opt/ros/humble/setup.bash' or 'call C:\\ros2_humble\\local_setup.bat')".to_string()),
            });
        }

        let colcon = tools.iter().find(|t| t.definition.binary == "colcon");
        if colcon.is_some_and(|t| t.is_available) {
            let ver = colcon
                .and_then(|t| t.version.as_deref())
                .unwrap_or("unknown");
            items.push(CompatibilityItem {
                name: "Colcon Build Tool".to_string(),
                level: CompatibilityLevel::Compatible,
                message: format!("colcon version {ver} available"),
                remediation: None,
            });
        } else {
            if overall_level != CompatibilityLevel::Incompatible {
                overall_level = CompatibilityLevel::Warning;
            }
            items.push(CompatibilityItem {
                name: "Colcon Build Tool".to_string(),
                level: CompatibilityLevel::Warning,
                message: "colcon not found on PATH".to_string(),
                remediation: Some(
                    "Install colcon via 'pip install colcon-common-extensions'".to_string(),
                ),
            });
            remediation.push("Install colcon: pip install colcon-common-extensions".to_string());
        }

        CompatibilityReport {
            language: LanguageId::Ros2,
            overall_level,
            items,
            remediation_steps: remediation,
        }
    }

    fn recommended_skills(&self) -> Vec<&'static str> {
        vec!["robotics-ros2-engineer"]
    }

    fn setup_plan(
        &self,
        missing_required: &[ToolDefinition],
        missing_optional: &[ToolDefinition],
    ) -> SetupPlan {
        let mut commands = Vec::new();
        let mut instructions = Vec::new();

        if missing_required.iter().any(|t| t.binary == "colcon") {
            commands.push("pip install colcon-common-extensions".to_string());
        }

        instructions.push(
            "Follow official ROS 2 installation instructions: https://docs.ros.org/".to_string(),
        );

        SetupPlan {
            language: LanguageId::Ros2,
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
                "build",
                vec![
                    "colcon".to_string(),
                    "build".to_string(),
                    "--symlink-install".to_string(),
                ],
            ),
            ("test", vec!["colcon".to_string(), "test".to_string()]),
        ]
    }
}
