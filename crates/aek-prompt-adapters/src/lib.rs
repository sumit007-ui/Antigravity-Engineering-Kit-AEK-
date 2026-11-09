//! # aek-prompt-adapters
//!
//! Adapters integrating external prompt optimization and evaluation engines:
//! Prompt Optimizer, Stanford DSPy, and Promptfoo for the Antigravity Engineering Kit (AEK).

pub mod dspy;
pub mod prompt_optimizer;
pub mod promptfoo;

pub use dspy::{DspyAdapter, DspyBridgePayload, DspyOptimizationResult};
#[allow(deprecated)]
pub use prompt_optimizer::{DeterministicRuleFormatter, PromptOptimizerAdapter};
pub use promptfoo::{PromptfooAdapter, PromptfooEvalResult};
