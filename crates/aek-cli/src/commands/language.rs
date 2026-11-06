//! CLI command handlers for `aek language`.

use crate::cli::{
    LanguageArgs, LanguageCompatibilityArgs, LanguageDetectArgs, LanguageDoctorArgs,
    LanguageSetupArgs, LanguageSubcommand, LanguageToolsArgs, LanguageValidateArgs,
};
use aek_languages::compatibility::CompatibilityEngine;
use aek_languages::detection::ProjectDetector;
use aek_languages::diagnostics::LanguageDoctor;
use aek_languages::model::LanguageId;
use aek_languages::providers::{find_provider, get_all_providers};
use aek_languages::registry::ToolchainRegistry;
use aek_validation::executor::ValidationExecutor;
use aek_validation::pipeline::ValidationPipeline;
use aek_validation::policy::ValidationPolicy;
use anyhow::{Context, Result, anyhow};
use std::env;
use std::str::FromStr;

use std::process::ExitCode;

/// Entry point from `main` returning `ExitCode`.
pub fn execute_language(args: &LanguageArgs) -> ExitCode {
    let rt = match tokio::runtime::Runtime::new() {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Error initializing async runtime: {e}");
            return ExitCode::FAILURE;
        }
    };

    match rt.block_on(handle_language(args.clone())) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("Error: {err:?}");
            ExitCode::FAILURE
        }
    }
}

/// Dispatches subcommands under `aek language`.
pub async fn handle_language(args: LanguageArgs) -> Result<()> {
    match args.command {
        LanguageSubcommand::List => handle_list().await,
        LanguageSubcommand::Detect(d) => handle_detect(d).await,
        LanguageSubcommand::Doctor(d) => handle_doctor(d).await,
        LanguageSubcommand::Tools(t) => handle_tools(t).await,
        LanguageSubcommand::Compatibility(c) => handle_compatibility(c).await,
        LanguageSubcommand::Setup(s) => handle_setup(s).await,
        LanguageSubcommand::Validate(v) => handle_validate_lang(v).await,
    }
}

async fn handle_list() -> Result<()> {
    println!("\nSupported Language Ecosystems:");
    println!("{:-<75}", "");
    println!(
        "{:<15} {:<30} {:<25}",
        "Language ID", "Display Name", "Markers"
    );
    println!("{:-<75}", "");

    for provider in get_all_providers() {
        let markers = provider.project_markers().join(", ");
        println!(
            "{:<15} {:<30} {:<25}",
            provider.id().as_str(),
            provider.display_name(),
            markers
        );
    }
    println!("{:-<75}\n", "");
    Ok(())
}

async fn handle_detect(args: LanguageDetectArgs) -> Result<()> {
    let scan_dir = match args.path {
        Some(p) => p,
        None => env::current_dir().context("Failed to resolve current directory")?,
    };

    let detector = ProjectDetector::new();
    let projects = detector.detect(&scan_dir)?;

    if args.json {
        let json = serde_json::to_string_pretty(&projects)?;
        println!("{json}");
        return Ok(());
    }

    println!("\nDetected Projects in '{}':", scan_dir.display());
    println!("{:-<80}", "");
    if projects.is_empty() {
        println!("No supported programming language projects detected.");
        println!("{:-<80}\n", "");
        return Ok(());
    }

    for p in &projects {
        println!("Project:    {}", p.name);
        println!("Language:   {}", p.language);
        println!("Path:       {}", p.path.display());
        if p.is_nested {
            println!(
                "Structure:  Nested package (Monorepo root: {:?})",
                p.monorepo_root
            );
        }
        for ev in &p.evidence {
            println!("Evidence:   {} ({})", ev.file, ev.description);
        }

        if let Some(prov) = find_provider(&p.language) {
            let skills = prov.recommended_skills();
            if !skills.is_empty() {
                println!("Recommended Skills: {}", skills.join(", "));
            }
        }
        println!("{:-<80}", "");
    }
    println!();
    Ok(())
}

async fn handle_doctor(args: LanguageDoctorArgs) -> Result<()> {
    let doctor = LanguageDoctor::default();

    if let Some(ref lang_str) = args.language {
        let lang_id = LanguageId::from_str(lang_str).unwrap();
        let report = doctor.audit_language(&lang_id).await;

        if args.json {
            println!("{}", serde_json::to_string_pretty(&report)?);
            return Ok(());
        }

        print_doctor_report(&report);
    } else {
        let mut reports = Vec::new();
        for provider in get_all_providers() {
            let report = doctor.audit_language(&provider.id()).await;
            reports.push(report);
        }

        if args.json {
            println!("{}", serde_json::to_string_pretty(&reports)?);
            return Ok(());
        }

        println!("\nAEK Language Toolchain Health Diagnostics:");
        println!("{:=<80}", "");
        for report in &reports {
            print_doctor_report(report);
        }
    }

    Ok(())
}

fn print_doctor_report(report: &aek_languages::diagnostics::LanguageDoctorReport) {
    println!(
        "\nLanguage: {} ({})  Status: [{}]",
        report.display_name, report.language, report.overall_status
    );
    println!("{:-<80}", "");
    println!(
        "{:<20} {:<12} {:<12} {:<30}",
        "Binary", "Requirement", "Status", "Version / Location"
    );
    println!("{:-<80}", "");

    for tool in &report.tools {
        let status_str = if tool.is_available {
            "INSTALLED"
        } else {
            "MISSING"
        };
        let ver = if tool.is_available {
            tool.version.as_deref().unwrap_or("Available")
        } else {
            "Not on PATH"
        };
        println!(
            "{:<20} {:<12} {:<12} {:<30}",
            tool.definition.binary,
            tool.definition.requirement.to_string(),
            status_str,
            ver
        );
    }

    if let Some(ref plan) = report.setup_plan {
        if !plan.missing_required.is_empty() || !plan.suggested_commands.is_empty() {
            println!("\nSetup Guidance:");
            for inst in &plan.instructions {
                println!("  * {inst}");
            }
            for cmd in &plan.suggested_commands {
                println!("  $ {cmd}");
            }
        }
    }
    println!("{:-<80}", "");
}

async fn handle_tools(args: LanguageToolsArgs) -> Result<()> {
    let lang_id = LanguageId::from_str(&args.language).unwrap();
    let registry = ToolchainRegistry::default();
    let tools = registry.get_tools_for_language(&lang_id);

    if tools.is_empty() {
        return Err(anyhow!(
            "No tools registered for language '{}'",
            args.language
        ));
    }

    println!("\nRegistered Tools for {}:", args.language);
    println!("{:-<80}", "");
    println!(
        "{:<18} {:<15} {:<12} {:<30}",
        "Name", "Binary", "Requirement", "Documentation"
    );
    println!("{:-<80}", "");

    for t in &tools {
        println!(
            "{:<18} {:<15} {:<12} {:<30}",
            t.name,
            t.binary,
            t.requirement.to_string(),
            t.doc_url
        );
    }
    println!("{:-<80}\n", "");
    Ok(())
}

async fn handle_compatibility(args: LanguageCompatibilityArgs) -> Result<()> {
    let lang_id = LanguageId::from_str(&args.language).unwrap();
    let scan_dir = match args.path {
        Some(p) => p,
        None => env::current_dir().context("Failed to resolve current directory")?,
    };

    let project = aek_languages::model::DetectedProject {
        path: scan_dir.clone(),
        language: lang_id.clone(),
        name: scan_dir
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| "project".to_string()),
        evidence: Vec::new(),
        monorepo_root: None,
        is_nested: false,
    };

    let engine = CompatibilityEngine::default();
    let report = engine.check_project(&project).await;

    println!(
        "\nCompatibility Report for {} in '{}':",
        lang_id,
        scan_dir.display()
    );
    println!("Overall Status: [{}]\n", report.overall_level);
    println!("{:-<80}", "");

    for item in &report.items {
        println!("[{}] {}: {}", item.level, item.name, item.message);
        if let Some(ref rem) = item.remediation {
            println!("     Remediation: {rem}");
        }
    }

    if !report.remediation_steps.is_empty() {
        println!("\nActionable Remediation:");
        for step in &report.remediation_steps {
            println!("  * {step}");
        }
    }
    println!("{:-<80}\n", "");
    Ok(())
}

async fn handle_setup(args: LanguageSetupArgs) -> Result<()> {
    let lang_id = LanguageId::from_str(&args.language).unwrap();
    let doctor = LanguageDoctor::default();
    let report = doctor.audit_language(&lang_id).await;

    let plan = match report.setup_plan {
        Some(p) => p,
        None => return Err(anyhow!("No setup plan available for '{}'", args.language)),
    };

    println!("\nToolchain Setup Plan for {}:", report.display_name);
    println!("{:-<80}", "");

    if plan.missing_required.is_empty() && plan.missing_optional.is_empty() {
        println!("All required and optional tools are already installed on system PATH.");
        println!("{:-<80}\n", "");
        return Ok(());
    }

    if !plan.missing_required.is_empty() {
        println!("Missing Required Tools:");
        for t in &plan.missing_required {
            println!("  - {} ({}): {}", t.name, t.binary, t.install_guidance);
        }
        println!();
    }

    if !plan.missing_optional.is_empty() {
        println!("Missing Optional Tools:");
        for t in &plan.missing_optional {
            println!("  - {} ({}): {}", t.name, t.binary, t.install_guidance);
        }
        println!();
    }

    println!("Recommended Instructions:");
    for inst in &plan.instructions {
        println!("  * {inst}");
    }

    if !plan.suggested_commands.is_empty() {
        println!("\nSafe Commands:");
        for cmd in &plan.suggested_commands {
            println!("  $ {cmd}");
        }
    }

    if args.dry_run {
        println!("\n[Dry Run] No system modifications executed.");
    } else {
        println!(
            "\nNote: AEK does not execute remote installation scripts automatically for security."
        );
        println!("Please run the verified commands above using your system terminal.");
    }
    println!("{:-<80}\n", "");
    Ok(())
}

async fn handle_validate_lang(args: LanguageValidateArgs) -> Result<()> {
    let lang_id = LanguageId::from_str(&args.language).unwrap();
    let scan_dir = match args.path {
        Some(p) => p,
        None => env::current_dir().context("Failed to resolve current directory")?,
    };

    let project = aek_languages::model::DetectedProject {
        path: scan_dir.clone(),
        language: lang_id,
        name: scan_dir
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| "project".to_string()),
        evidence: Vec::new(),
        monorepo_root: None,
        is_nested: false,
    };

    let pipeline = ValidationPipeline::for_project(&project);
    let policy = ValidationPolicy {
        dry_run: args.dry_run,
        ..Default::default()
    };

    let executor = ValidationExecutor::new(policy);
    let report = executor.execute(&pipeline).await?;

    if args.json {
        println!("{}", serde_json::to_string_pretty(&report)?);
        return Ok(());
    }

    print_validation_report(&report);
    Ok(())
}

pub fn print_validation_report(report: &aek_validation::report::ValidationReport) {
    println!(
        "\nValidation Report: {} ({})",
        report.project_name, report.language
    );
    println!(
        "Overall Status: {}",
        if report.overall_success {
            "SUCCESS"
        } else {
            "FAILED"
        }
    );
    if report.dry_run {
        println!("Mode: DRY RUN (no commands executed)");
    }
    println!(
        "Passed: {} | Failed: {} | Skipped: {} | Total Time: {}ms",
        report.passed_count, report.failed_count, report.skipped_count, report.total_duration_ms
    );
    println!("{:-<80}", "");
    println!(
        "{:<30} {:<18} {:<10} {:<10}",
        "Step", "Stage", "Status", "Duration"
    );
    println!("{:-<80}", "");

    for r in &report.results {
        println!(
            "{:<30} {:<18} {:<10} {}ms",
            r.name,
            r.stage.to_string(),
            r.status.to_string(),
            r.duration_ms
        );
        if (r.status == aek_validation::report::StepStatus::Fail
            || r.status == aek_validation::report::StepStatus::Error)
            && !r.stderr.is_empty()
        {
            println!("  [stderr] {}", r.stderr.trim());
        }
    }
    println!("{:-<80}\n", "");
}
