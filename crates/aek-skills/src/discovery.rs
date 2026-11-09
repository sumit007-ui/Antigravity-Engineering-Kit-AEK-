//! Project-aware skill discovery and recommendations.

use serde::{Deserialize, Serialize};
use std::path::Path;

/// A recommendation for an engineering skill based on project markers.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SkillRecommendation {
    /// Recommended skill identifier.
    pub skill_name: String,
    /// Rule weight / match strength heuristic between 0.0 and 1.0 (not a statistical probability).
    #[serde(alias = "match_strength")]
    pub confidence: f32,
    /// Human-readable explanation for the recommendation.
    pub reason: String,
}

/// Project inspection engine for skill recommendations.
pub struct SkillDiscoverer;

impl SkillDiscoverer {
    /// Inspects a workspace root directory and returns targeted skill recommendations.
    #[must_use]
    pub fn discover(workspace_root: &Path) -> Vec<SkillRecommendation> {
        let mut recommendations = Vec::new();

        // 1. Rust
        if workspace_root.join("Cargo.toml").exists() {
            recommendations.push(SkillRecommendation {
                skill_name: "rust-systems-engineer".to_string(),
                confidence: 1.0,
                reason: "Detected 'Cargo.toml' workspace manifest".to_string(),
            });
        }

        // 2. Go
        if workspace_root.join("go.mod").exists() {
            recommendations.push(SkillRecommendation {
                skill_name: "go-backend-engineer".to_string(),
                confidence: 1.0,
                reason: "Detected Go module manifest 'go.mod'".to_string(),
            });
        }

        // 3. C / C++
        if workspace_root.join("CMakeLists.txt").exists()
            || workspace_root.join("Makefile").exists()
        {
            recommendations.push(SkillRecommendation {
                skill_name: "cpp-systems-engineer".to_string(),
                confidence: 0.9,
                reason: "Detected C/C++ build manifests (CMakeLists.txt or Makefile)".to_string(),
            });
        }

        // 4. Zig
        if workspace_root.join("build.zig").exists() {
            recommendations.push(SkillRecommendation {
                skill_name: "zig-engineer".to_string(),
                confidence: 1.0,
                reason: "Detected Zig build script 'build.zig'".to_string(),
            });
        }

        // 5. Python
        if workspace_root.join("pyproject.toml").exists()
            || workspace_root.join("requirements.txt").exists()
            || workspace_root.join("setup.py").exists()
        {
            recommendations.push(SkillRecommendation {
                skill_name: "python-engineer".to_string(),
                confidence: 1.0,
                reason: "Detected Python project manifest (pyproject.toml/requirements.txt)"
                    .to_string(),
            });
        }

        // 6. Flutter
        if workspace_root.join("pubspec.yaml").exists() {
            recommendations.push(SkillRecommendation {
                skill_name: "flutter-engineer".to_string(),
                confidence: 1.0,
                reason: "Detected Flutter / Dart manifest 'pubspec.yaml'".to_string(),
            });
        }

        // 7. Robotics & ROS 2
        if workspace_root.join("package.xml").exists()
            || workspace_root.join("colcon.meta").exists()
        {
            recommendations.push(SkillRecommendation {
                skill_name: "robotics-ros2-engineer".to_string(),
                confidence: 1.0,
                reason: "Detected ROS 2 package manifest 'package.xml'".to_string(),
            });
        }

        // 8. DevOps & CI/CD
        if workspace_root.join("Dockerfile").exists()
            || workspace_root.join(".github").join("workflows").exists()
        {
            recommendations.push(SkillRecommendation {
                skill_name: "devops-engineer".to_string(),
                confidence: 0.9,
                reason:
                    "Detected containerization or CI/CD pipelines (Dockerfile / .github/workflows)"
                        .to_string(),
            });
        }

        // 9. Database Architect
        if workspace_root.join("migrations").exists() || workspace_root.join("schema.sql").exists()
        {
            recommendations.push(SkillRecommendation {
                skill_name: "database-architect".to_string(),
                confidence: 0.85,
                reason: "Detected database migration directory or schema definitions".to_string(),
            });
        }

        // 10. AI Agent Engineer
        if workspace_root.join("mcp_config.json").exists()
            || workspace_root.join(".agents").exists()
        {
            recommendations.push(SkillRecommendation {
                skill_name: "ai-agent-engineer".to_string(),
                confidence: 0.9,
                reason: "Detected Model Context Protocol or agent customization configuration"
                    .to_string(),
            });
        }

        // 11. Production Code Reviewer (Universal for any project with tests)
        if workspace_root.join("tests").exists() || workspace_root.join("test").exists() {
            recommendations.push(SkillRecommendation {
                skill_name: "production-code-reviewer".to_string(),
                confidence: 0.75,
                reason: "Detected automated test suite in workspace".to_string(),
            });
        }

        // Sort descending by confidence
        recommendations.sort_by(|a, b| {
            b.confidence
                .partial_cmp(&a.confidence)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        recommendations
    }
}
