use aek_core::diagnostics::{DiagnosticCheck, DiagnosticReport, DiagnosticStatus};

#[test]
fn test_diagnostic_report_statuses() {
    let mut report = DiagnosticReport::new();
    assert!(report.is_healthy());

    report.add_check(DiagnosticCheck::new(
        "Rust",
        "Toolchain",
        DiagnosticStatus::Pass,
        Some("rustc 1.91.0".to_string()),
    ));
    assert_eq!(report.pass_count(), 1);
    assert_eq!(report.warn_count(), 0);
    assert_eq!(report.fail_count(), 0);
    assert_eq!(report.skip_count(), 0);
    assert!(report.is_healthy());

    report.add_check(DiagnosticCheck::new(
        "Python",
        "Runtime",
        DiagnosticStatus::Warn {
            message: "Python not found".to_string(),
        },
        None,
    ));
    assert_eq!(report.warn_count(), 1);
    assert!(report.is_healthy());

    report.add_check(DiagnosticCheck::new(
        "Antigravity",
        "Environment",
        DiagnosticStatus::Skip {
            reason: "Not installed".to_string(),
        },
        None,
    ));
    assert_eq!(report.skip_count(), 1);
    assert!(report.is_healthy());

    report.add_check(DiagnosticCheck::new(
        "Cargo",
        "Toolchain",
        DiagnosticStatus::Fail {
            reason: "Missing cargo".to_string(),
            remediation: Some("Install Rust".to_string()),
        },
        None,
    ));
    assert_eq!(report.fail_count(), 1);
    assert!(!report.is_healthy());
}
