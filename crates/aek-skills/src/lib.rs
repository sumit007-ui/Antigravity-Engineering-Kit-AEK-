//! # aek-skills
//!
//! Skill models, project-aware discovery, validation, activation, and safe installation.

pub mod activation;
pub mod discovery;
pub mod installer;
pub mod model;
pub mod registry;
pub mod validator;

pub use activation::SkillActivationState;
pub use discovery::{SkillDiscoverer, SkillRecommendation};
pub use installer::{SkillInstallError, SkillInstallResult, install_skill, remove_skill};
pub use model::{SkillBundle, SkillMetadata};
pub use registry::SkillRegistry;
pub use validator::{SkillValidationError, parse_skill_markdown};
