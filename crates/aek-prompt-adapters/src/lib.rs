//! # aek-prompt-adapters
//!
//! Adapters integrating external prompt optimization and evaluation engines:
//! Prompt Optimizer, Stanford DSPy, and Promptfoo for the Antigravity Engineering Kit (AEK).

pub mod dspy;
pub mod prompt_optimizer;
pub mod promptfoo;

pub use dspy::{DspyAdapter, DspyBridgePayload, DspyOptimizationResult};
pub use prompt_optimizer::PromptOptimizerAdapter;
pub use promptfoo::{PromptfooAdapter, PromptfooEvalResult};

