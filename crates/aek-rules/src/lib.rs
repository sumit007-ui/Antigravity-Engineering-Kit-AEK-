//! # aek-rules
//!
//! Rule models, discovery, precedence resolution, validation, and safe installation.

pub mod installer;
pub mod merger;
pub mod model;
pub mod registry;
pub mod resolver;
pub mod validator;

pub use installer::{RuleInstallError, RuleInstallResult, install_rule, remove_rule};
pub use merger::RuleMerger;
pub use model::{Rule, RuleScope};
pub use registry::RuleRegistry;
pub use resolver::RuleResolver;
pub use validator::{MAX_RULE_BYTES, RuleValidationError, validate_rule};
