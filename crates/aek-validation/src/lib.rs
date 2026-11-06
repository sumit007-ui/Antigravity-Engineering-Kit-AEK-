//! # aek-validation
//!
//! Project-aware validation pipeline, incremental detection, quality gates,
//! sandboxed execution, and structured reporting for the Antigravity Engineering Kit (AEK).

pub mod errors;
pub mod executor;
pub mod incremental;
pub mod pipeline;
pub mod planner;
pub mod policy;
pub mod quality_gate;
pub mod report;

pub use errors::ValidationError;
pub use executor::ValidationExecutor;
pub use incremental::GitChangeDetector;
pub use pipeline::{ValidationPipeline, ValidationStage, ValidationStep};
pub use planner::ValidationPlanner;
pub use policy::ValidationPolicy;
pub use quality_gate::{QualityGateEngine, QualityGateResult};
pub use report::{StepResult, StepStatus, ValidationReport};
