//! # aek-agents
//!
//! Extensible agent adapters and Google Antigravity integration.

pub mod adapter;
pub mod antigravity;
pub mod capabilities;

pub use adapter::{AgentAdapter, AgentDetection, AgentError, InstallReport};
pub use antigravity::AntigravityAdapter;
pub use capabilities::AgentCapabilities;
