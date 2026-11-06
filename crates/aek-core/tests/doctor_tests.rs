use aek_core::doctor::{
    CheckStatus, DoctorCategory, DoctorCheck, DoctorReport, DoctorRunner, ToolDetector,
};
use std::path::Path;

#[test]
fn test_doctor_report_health_status() {
    let mut report = DoctorReport::new();
    assert!(report.is_healthy());
    assert_eq!(report.total_count(), 0);

    report.add_check(DoctorCheck::new(
        "Rust Toolchain",
        DoctorCategory::Toolchain,
        CheckStatus::Ok,
        Some("rustc 1.91.0".to_string()),
    ));
    assert!(report.is_healthy());
    assert_eq!(report.ok_count(), 1);
    assert_eq!(report.warning_count(), 0);
    assert_eq!(report.failed_count(), 0);

    report.add_check(DoctorCheck::new(
        "Antigravity Workspace",
        DoctorCategory::Antigravity,
        CheckStatus::Warning {
            message: "No .agents/ directory found in workspace".to_string(),
        },
        None,
    ));
    // Warnings do not fail the overall health check
    assert!(report.is_healthy());
    assert_eq!(report.warning_count(), 1);

    report.add_check(DoctorCheck::new(
        "Missing Tool",
        DoctorCategory::Toolchain,
        CheckStatus::Failed {
            reason: "Binary not found in PATH".to_string(),
            remediation: Some("Install missing tool".to_string()),
        },
        None,
    ));
    // A failed check makes the report unhealthy
    assert!(!report.is_healthy());
    assert_eq!(report.failed_count(), 1);
    assert_eq!(report.total_count(), 3);
}

#[test]
fn test_tool_detector_existing_and_missing() {
    // rustc must be present on the dev system
    let rustc_check = ToolDetector::detect_tool("rustc", &["--version"], DoctorCategory::Toolchain);
    assert_eq!(rustc_check.name, "rustc");
    assert!(matches!(rustc_check.status, CheckStatus::Ok));
    assert!(rustc_check.details.is_some());

    // A clearly nonexistent binary must report failure with remediation
    let missing_check = ToolDetector::detect_tool(
        "nonexistent_bin_aek_xyz_999",
        &["--version"],
        DoctorCategory::Toolchain,
    );
    assert_eq!(missing_check.name, "nonexistent_bin_aek_xyz_999");
    assert!(matches!(missing_check.status, CheckStatus::Failed { .. }));
}

#[test]
fn test_doctor_runner_run_all() {
    let report = DoctorRunner::run_all(Some(Path::new(".")));
    assert!(report.total_count() >= 4);

    // Verify categories are represented
    let categories: Vec<DoctorCategory> = report.checks.iter().map(|c| c.category).collect();
    assert!(categories.contains(&DoctorCategory::Toolchain));
    assert!(categories.contains(&DoctorCategory::Platform));
    assert!(categories.contains(&DoctorCategory::Antigravity));
}
