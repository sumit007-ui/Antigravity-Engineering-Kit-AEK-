//! # aek-reporting
//!
//! Structured report generation for AEK: terminal summaries, JSON reports,
//! and OASIS SARIF v2.1.0 output for GitHub Code Scanning and IDEs.

pub mod json;
pub mod sarif;
pub mod summary;
pub mod terminal;

pub use json::JsonReportGenerator;
pub use sarif::SarifReportGenerator;
pub use summary::MarkdownSummaryGenerator;
pub use terminal::TerminalReportGenerator;

