//! # aek-validation
//!
//! Project-aware validation pipeline, sandboxed execution, and structured reporting
//! for the Antigravity Engineering Kit (AEK).

pub mod errors;
pub mod executor;
pub mod pipeline;
pub mod policy;
pub mod report;

pub use errors::ValidationError;
pub use executor::ValidationExecutor;
pub use pipeline::{ValidationPipeline, ValidationStage, ValidationStep};
pub use policy::ValidationPolicy;
pub use report::{StepResult, StepStatus, ValidationReport};
