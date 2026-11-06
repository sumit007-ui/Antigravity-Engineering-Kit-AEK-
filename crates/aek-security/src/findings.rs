//! Normalized security finding models, severity classifications, and scanner identifiers.

use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;
use std::path::PathBuf;

/// Normalized severity level for a security finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Severity {
    Unknown,
    Info,
    Low,
    Medium,
    High,
    Critical,
}

impl Severity {
    /// Rank integer used for strict order comparisons.
    #[must_use]
    pub const fn rank(&self) -> u8 {
        match self {
            Self::Unknown => 0,
            Self::Info => 1,
            Self::Low => 2,
            Self::Medium => 3,
            Self::High => 4,
            Self::Critical => 5,
        }
    }

    /// String representation in uppercase.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Unknown => "UNKNOWN",
            Self::Info => "INFO",
            Self::Low => "LOW",
            Self::Medium => "MEDIUM",
            Self::High => "HIGH",
            Self::Critical => "CRITICAL",
        }
    }

    /// Parses a string into a normalized `Severity`.
    #[must_use]
    pub fn from_str_loose(s: &str) -> Self {
        match s.trim().to_uppercase().as_str() {
            "CRITICAL" => Self::Critical,
            "HIGH" | "ERROR" => Self::High,
            "MEDIUM" | "MODERATE" | "WARN" | "WARNING" => Self::Medium,
            "LOW" => Self::Low,
            "INFO" | "INFORMATIONAL" | "NOTE" => Self::Info,
            _ => Self::Unknown,
        }
    }
}

impl PartialOrd for Severity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Severity {
    fn cmp(&self, other: &Self) -> Ordering {
        self.rank().cmp(&other.rank())
    }
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Category or tool source of a security scanner.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ScannerKind {
    Semgrep,
    Trivy,
    CargoAudit,
    Govulncheck,
    PipAudit,
    NpmAudit,
    Custom,
}

impl fmt::Display for ScannerKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Semgrep => write!(f, "semgrep"),
            Self::Trivy => write!(f, "trivy"),
            Self::CargoAudit => write!(f, "cargo-audit"),
            Self::Govulncheck => write!(f, "govulncheck"),
            Self::PipAudit => write!(f, "pip-audit"),
            Self::NpmAudit => write!(f, "npm-audit"),
            Self::Custom => write!(f, "custom"),
        }
    }
}

/// A normalized security finding representing a vulnerability, flaw, or misconfiguration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityFinding {
    pub id: String,
    pub scanner: ScannerKind,
    pub rule_id: String,
    pub severity: Severity,
    pub category: String,
    pub description: String,
    pub file_path: Option<PathBuf>,
    pub line_number: Option<usize>,
    pub dependency: Option<String>,
    pub advisory_id: Option<String>,
    pub remediation: Option<String>,
    pub confidence: Option<String>,
    pub metadata: HashMap<String, serde_json::Value>,
}
