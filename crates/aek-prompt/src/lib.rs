//! # aek-prompt
//!
//! AI prompt orchestration, optimization pipeline, evaluation engine,
//! dataset management, and versioning for the Antigravity Engineering Kit (AEK).

pub mod cache;
pub mod datasets;
pub mod errors;
pub mod evaluator;
pub mod model;
pub mod optimizer;
pub mod providers;
pub mod registry;
pub mod reports;
pub mod security;
pub mod templates;
pub mod versions;

pub use cache::PromptCache;
pub use datasets::{DatasetManager, EvaluationCase};
pub use errors::PromptError;
pub use evaluator::{EvaluationMetricCalculator, EvaluationScore};
pub use model::{PromptComparison, PromptRecord, PromptStatus, compute_sha256};
pub use optimizer::{AssembledPrompt, ContextAssembler};
pub use providers::{ModelProviderConfig, ProviderKind};
pub use registry::PromptRegistry;
pub use reports::{BenchmarkReport, EvaluationReport, PromptOptimizationReport};
pub use security::{InjectionRisk, detect_injection_risk, sanitize_template_variable};
pub use templates::PromptTemplate;
pub use versions::PromptVersionManager;
