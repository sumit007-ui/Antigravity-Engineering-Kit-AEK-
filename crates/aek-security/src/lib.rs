//! # aek-security
//!
//! Automated security auditing, static analysis integration (Semgrep),
//! vulnerability scanning (Trivy), dependency vulnerability audits, and quality gates for AEK.

pub mod dependencies;
pub mod doctor;
pub mod errors;
pub mod findings;
pub mod policy;
pub mod scanner;
pub mod semgrep;
pub mod trivy;

pub use dependencies::DependencyAuditor;
pub use doctor::{SecurityDoctor, SecurityDoctorReport, ToolCheckResult};
pub use errors::SecurityError;
pub use findings::{ScannerKind, SecurityFinding, Severity};
pub use policy::SecurityPolicy;
pub use scanner::{SecurityScanReport, SecurityScanner};
pub use semgrep::SemgrepAdapter;
pub use trivy::TrivyAdapter;
