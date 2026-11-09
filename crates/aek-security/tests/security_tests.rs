use aek_security::dependencies::DependencyAuditor;
use aek_security::findings::{ScannerKind, SecurityFinding, Severity};
use aek_security::policy::SecurityPolicy;
use aek_security::scanner::{SecurityScanReport, SecurityScanner};
use aek_security::semgrep::SemgrepAdapter;
use aek_security::trivy::TrivyAdapter;
use std::collections::HashMap;
use std::path::PathBuf;

#[test]
fn test_security_finding_model_and_severity() {
    let finding = SecurityFinding {
        id: "finding-01".to_string(),
        scanner: ScannerKind::Semgrep,
        rule_id: "rust.lang.security.insecure-cmd".to_string(),
        severity: Severity::High,
        category: "command-injection".to_string(),
        description: "Potential command injection in Command::new".to_string(),
        file_path: Some(PathBuf::from("src/process.rs")),
        line_number: Some(42),
        dependency: None,
        advisory_id: None,
        remediation: Some("Use argument arrays instead of shell interpolation".to_string()),
        confidence: Some("HIGH".to_string()),
        metadata: HashMap::new(),
    };

    assert_eq!(finding.severity, Severity::High);
    assert_eq!(finding.severity.as_str(), "HIGH");
    assert!(finding.severity >= Severity::Medium);

    let json = serde_json::to_string(&finding).expect("serialize finding");
    let deserialized: SecurityFinding = serde_json::from_str(&json).expect("deserialize finding");
    assert_eq!(deserialized.id, "finding-01");
    assert_eq!(deserialized.severity, Severity::High);
}

#[test]
fn test_semgrep_output_parsing() {
    let raw_semgrep_json = r#"{
        "results": [
            {
                "check_id": "rules.security.sql-injection",
                "path": "src/db.rs",
                "start": { "line": 15 },
                "extra": {
                    "message": "Unsanitized input passed to query",
                    "severity": "ERROR",
                    "metadata": { "cwe": "CWE-89" }
                }
            }
        ],
        "errors": []
    }"#;

    let adapter = SemgrepAdapter::new();
    let findings = adapter
        .parse_output(raw_semgrep_json)
        .expect("parse semgrep output");
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].scanner, ScannerKind::Semgrep);
    assert_eq!(findings[0].rule_id, "rules.security.sql-injection");
    assert_eq!(findings[0].severity, Severity::High);
    assert_eq!(findings[0].line_number, Some(15));
}

#[test]
fn test_trivy_output_parsing() {
    let raw_trivy_json = r#"{
        "SchemaVersion": 2,
        "Results": [
            {
                "Target": "Cargo.lock",
                "Class": "lang-pkgs",
                "Type": "cargo",
                "Vulnerabilities": [
                    {
                        "VulnerabilityID": "CVE-2024-12345",
                        "PkgName": "vulnerable-crate",
                        "InstalledVersion": "0.1.0",
                        "FixedVersion": "0.1.1",
                        "Severity": "CRITICAL",
                        "Title": "Memory corruption in parser",
                        "Description": "Buffer overflow when parsing malformed headers",
                        "PrimaryURL": "https://rustsec.org/advisories/CVE-2024-12345"
                    }
                ]
            }
        ]
    }"#;

    let adapter = TrivyAdapter::new();
    let findings = adapter
        .parse_output(raw_trivy_json)
        .expect("parse trivy output");
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].scanner, ScannerKind::Trivy);
    assert_eq!(findings[0].advisory_id.as_deref(), Some("CVE-2024-12345"));
    assert_eq!(findings[0].severity, Severity::Critical);
    assert_eq!(findings[0].dependency.as_deref(), Some("vulnerable-crate"));
}

#[test]
fn test_dependency_auditor_cargo_audit_parsing() {
    let raw_cargo_audit = r#"{
        "database": { "advisory-count": 750 },
        "vulnerabilities": {
            "found": true,
            "count": 1,
            "list": [
                {
                    "advisory": {
                        "id": "RUSTSEC-2023-0001",
                        "package": "time",
                        "title": "Potential segfault in time formatting",
                        "url": "https://rustsec.org/advisories/RUSTSEC-2023-0001",
                        "severity": "high"
                    },
                    "package": {
                        "name": "time",
                        "version": "0.1.42"
                    }
                }
            ]
        }
    }"#;

    let auditor = DependencyAuditor::new();
    let findings = auditor
        .parse_cargo_audit(raw_cargo_audit)
        .expect("parse cargo-audit");
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].scanner, ScannerKind::CargoAudit);
    assert_eq!(
        findings[0].advisory_id.as_deref(),
        Some("RUSTSEC-2023-0001")
    );
    assert_eq!(findings[0].dependency.as_deref(), Some("time"));
    assert_eq!(findings[0].severity, Severity::High);
}

#[test]
fn test_security_policy_evaluation() {
    let policy = SecurityPolicy {
        fail_on_critical: true,
        fail_on_high: true,
        fail_on_medium: false,
        fail_on_low: false,
        ignored_rules: vec!["ignored-rule".to_string()],
        ignored_paths: vec!["tests/*".to_string()],
        allow_missing_scanners: false,
    };

    let critical_finding = SecurityFinding {
        id: "f-crit".to_string(),
        scanner: ScannerKind::Trivy,
        rule_id: "CVE-2024-9999".to_string(),
        severity: Severity::Critical,
        category: "vuln".to_string(),
        description: "Critical vulnerability".to_string(),
        file_path: Some(PathBuf::from("Cargo.lock")),
        line_number: None,
        dependency: Some("bad-pkg".to_string()),
        advisory_id: Some("CVE-2024-9999".to_string()),
        remediation: None,
        confidence: None,
        metadata: HashMap::new(),
    };

    assert!(policy.violates_policy(&critical_finding));

    let medium_finding = SecurityFinding {
        id: "f-med".to_string(),
        scanner: ScannerKind::Semgrep,
        rule_id: "rule.med".to_string(),
        severity: Severity::Medium,
        category: "style".to_string(),
        description: "Medium style issue".to_string(),
        file_path: Some(PathBuf::from("src/main.rs")),
        line_number: Some(10),
        dependency: None,
        advisory_id: None,
        remediation: None,
        confidence: None,
        metadata: HashMap::new(),
    };

    assert!(!policy.violates_policy(&medium_finding));

    let ignored_finding = SecurityFinding {
        id: "f-ign".to_string(),
        scanner: ScannerKind::Semgrep,
        rule_id: "ignored-rule".to_string(),
        severity: Severity::Critical,
        category: "security".to_string(),
        description: "Should be ignored by rule id".to_string(),
        file_path: Some(PathBuf::from("src/lib.rs")),
        line_number: None,
        dependency: None,
        advisory_id: None,
        remediation: None,
        confidence: None,
        metadata: HashMap::new(),
    };

    assert!(!policy.violates_policy(&ignored_finding));
}

#[test]
fn test_security_injection_and_path_traversal_guards() {
    let auditor = DependencyAuditor::new();
    // Path traversal in target dir must be rejected
    assert!(
        auditor
            .audit_directory(&PathBuf::from("../../etc/passwd"))
            .is_err()
    );
    assert!(
        auditor
            .audit_directory(&PathBuf::from("foo/../bar"))
            .is_err()
    );
    assert!(
        auditor
            .audit_directory(&PathBuf::from(r"foo\..\bar"))
            .is_err()
    );
}

#[tokio::test]
async fn test_missing_scanners_fail_closed_and_incomplete() {
    // When scanners (e.g. semgrep / trivy) are missing on host, scan must mark incomplete and policy_violated = true
    let scanner = SecurityScanner::new();
    let temp_dir = std::env::temp_dir();
    let report = scanner
        .scan(&temp_dir, true, true, false, 5)
        .await
        .expect("scanner should complete returning scan report");

    // If semgrep or trivy are not installed, incomplete must be true and policy_violated must be true
    if scanner.semgrep.detect().is_none() || scanner.trivy.detect().is_none() {
        assert!(report.incomplete);
        assert!(report.policy_violated);
        assert!(!report.skipped_scanners.is_empty());
    }
}

#[tokio::test]
async fn test_missing_scanners_allowed_by_policy_opt_in() {
    let policy = SecurityPolicy {
        allow_missing_scanners: true,
        ..Default::default()
    };
    let scanner = SecurityScanner::with_policy(policy);
    let temp_dir = std::env::temp_dir();
    let report = scanner
        .scan(&temp_dir, true, true, false, 5)
        .await
        .expect("scan succeeded");

    if report.incomplete {
        // Since allow_missing_scanners is true and there are no critical/high findings, policy_violated should be false
        assert!(!report.policy_violated);
    }
}

#[tokio::test]
async fn test_direct_scanner_error_when_binary_missing() {
    let semgrep = SemgrepAdapter::new();
    if semgrep.detect().is_none() {
        let res = semgrep.scan(&PathBuf::from("."), None, 5).await;
        assert!(
            res.is_err(),
            "Direct semgrep scan must fail when binary is missing"
        );
    }

    let trivy = TrivyAdapter::new();
    if trivy.detect().is_none() {
        let res = trivy.scan(&PathBuf::from("."), 5).await;
        assert!(
            res.is_err(),
            "Direct trivy scan must fail when binary is missing"
        );
    }
}

#[test]
fn test_security_scan_report_serialization_includes_skipped() {
    let report = SecurityScanReport {
        target_dir: PathBuf::from("."),
        total_findings: 0,
        critical_count: 0,
        high_count: 0,
        medium_count: 0,
        low_count: 0,
        policy_violated: true,
        findings: Vec::new(),
        skipped_scanners: vec!["semgrep".to_string(), "trivy".to_string()],
        incomplete: true,
    };
    let json = serde_json::to_string(&report).expect("serialize report");
    assert!(json.contains("skipped_scanners"));
    assert!(json.contains("incomplete"));
    assert!(json.contains("semgrep"));

    let deserialized: SecurityScanReport = serde_json::from_str(&json).expect("deserialize report");
    assert!(deserialized.incomplete);
    assert_eq!(deserialized.skipped_scanners.len(), 2);
}
