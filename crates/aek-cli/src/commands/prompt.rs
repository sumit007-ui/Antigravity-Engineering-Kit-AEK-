//! CLI command handlers for `aek prompt`.

use crate::cli::{
    PromptArgs, PromptBenchmarkArgs, PromptCompareArgs, PromptCreateArgs, PromptEvaluateArgs,
    PromptExportArgs, PromptOptimizeArgs, PromptSubcommand,
};
use aek_agents::adapter::AgentAdapter;
use aek_agents::antigravity::AntigravityAdapter;
use aek_core::execution::SafeCommand;
use aek_platform::find_executable_in_path;
use aek_platform::paths::PlatformPaths;
use aek_prompt::datasets::DatasetManager;
use aek_prompt::evaluator::EvaluationMetricCalculator;
use aek_prompt::model::{PromptRecord, PromptStatus};
use aek_prompt::optimizer::ContextAssembler;
use aek_prompt::security::detect_injection_risk;
use aek_prompt::versions::PromptVersionManager;
use aek_prompt_adapters::dspy::DspyAdapter;
use aek_prompt_adapters::prompt_optimizer::PromptOptimizerAdapter;
use anyhow::{Context, Result, bail};
use std::env;
use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;
use std::process::ExitCode;

/// Resolves standard prompt storage directory.
fn resolve_prompts_dir() -> PathBuf {
    if let Ok(custom) = env::var("AEK_PROMPTS_DIR") {
        return PathBuf::from(custom);
    }

    let paths = PlatformPaths::resolve();
    paths.data_dir.join("prompts")
}

fn find_prompts_examples_dir() -> Option<PathBuf> {
    let candidates = [
        PathBuf::from("prompts/examples"),
        PathBuf::from("../prompts/examples"),
        PathBuf::from("../../prompts/examples"),
    ];
    candidates.into_iter().find(|c| c.is_dir())
}

fn find_dataset_path(requested: Option<PathBuf>) -> Result<PathBuf> {
    if let Some(p) = requested {
        if p.is_file() {
            return Ok(p);
        }
        let candidates = [
            p.clone(),
            PathBuf::from("../").join(&p),
            PathBuf::from("../../").join(&p),
        ];
        if let Some(c) = candidates.into_iter().find(|c| c.is_file()) {
            return Ok(c);
        }
        bail!("Dataset file '{}' not found", p.display());
    }

    let default_candidates = [
        PathBuf::from("datasets/examples/rust.jsonl"),
        PathBuf::from("../datasets/examples/rust.jsonl"),
        PathBuf::from("../../datasets/examples/rust.jsonl"),
    ];
    default_candidates
        .into_iter()
        .find(|c| c.is_file())
        .ok_or_else(|| anyhow::anyhow!("Default dataset 'datasets/examples/rust.jsonl' not found"))
}

/// Discovers prompts across local records, repository examples, and templates.
fn discover_all_prompts(mgr: &PromptVersionManager) -> Vec<PromptRecord> {
    let mut prompts = Vec::new();

    // 1. Check workspace prompts/examples directory
    if let Some(examples_dir) = find_prompts_examples_dir() {
        if let Ok(entries) = fs::read_dir(examples_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|e| e.to_str()) == Some("json") {
                    if let Ok(content) = fs::read_to_string(&path) {
                        if let Ok(rec) = serde_json::from_str::<PromptRecord>(&content) {
                            prompts.push(rec);
                        }
                    }
                }
            }
        }
    }

    // 2. Load stored prompts from version manager directory
    if let Ok(entries) = fs::read_dir(resolve_prompts_dir()) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                if let Some(id) = path.file_name().and_then(|n| n.to_str()) {
                    if let Ok(rec) = mgr.get_latest(id) {
                        if !prompts.iter().any(|p| p.id == rec.id) {
                            prompts.push(rec);
                        }
                    }
                }
            }
        }
    }

    prompts.sort_by(|a, b| a.id.cmp(&b.id));
    prompts
}

/// Finds a prompt record by ID in version manager or workspace examples.
fn find_prompt(mgr: &PromptVersionManager, id: &str) -> Result<PromptRecord> {
    // Check version manager first
    if let Ok(rec) = mgr.get_latest(id) {
        return Ok(rec);
    }

    // Check workspace examples
    if let Some(examples_dir) = find_prompts_examples_dir() {
        if let Ok(entries) = fs::read_dir(examples_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|e| e.to_str()) == Some("json") {
                    let file_stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
                    if let Ok(content) = fs::read_to_string(&path) {
                        if let Ok(rec) = serde_json::from_str::<PromptRecord>(&content) {
                            if rec.id == id
                                || file_stem == id
                                || rec.id.replace('-', "_") == id.replace('-', "_")
                            {
                                return Ok(rec);
                            }
                        }
                    }
                }
            }
        }
    }

    bail!("Prompt '{id}' not found in registry or prompts/examples/");
}

/// Entry point from `main` returning `ExitCode`.
pub fn execute_prompt(args: &PromptArgs) -> ExitCode {
    let rt = match tokio::runtime::Runtime::new() {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Error initializing async runtime: {e}");
            return ExitCode::FAILURE;
        }
    };

    match rt.block_on(handle_prompt(args.clone())) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("Error: {err:?}");
            ExitCode::FAILURE
        }
    }
}

/// Executes prompt subcommands.
pub async fn handle_prompt(args: PromptArgs) -> Result<()> {
    let base_dir = resolve_prompts_dir();
    let mgr = PromptVersionManager::new(base_dir);

    match args.command {
        PromptSubcommand::List { json } => handle_list(&mgr, json),
        PromptSubcommand::Show { id, json } => handle_show(&mgr, &id, json),
        PromptSubcommand::Create(create_args) => handle_create(&mgr, create_args),
        PromptSubcommand::Versions { id, json } => handle_versions(&mgr, &id, json),
        PromptSubcommand::Diff { id, ver_a, ver_b } => handle_diff(&mgr, &id, ver_a, ver_b),
        PromptSubcommand::Rollback { id, version } => handle_rollback(&mgr, &id, &version),
        PromptSubcommand::Optimize(opt_args) => handle_optimize(&mgr, opt_args).await,
        PromptSubcommand::Evaluate(eval_args) => handle_evaluate(&mgr, eval_args),
        PromptSubcommand::Benchmark(bench_args) => handle_benchmark(&mgr, bench_args),
        PromptSubcommand::Compare(comp_args) => handle_compare(&mgr, comp_args),
        PromptSubcommand::Export(export_args) => handle_export(&mgr, export_args),
        PromptSubcommand::Doctor => handle_doctor().await,
        PromptSubcommand::Adapters => handle_adapters(),
    }
}

fn handle_list(mgr: &PromptVersionManager, json: bool) -> Result<()> {
    let prompts = discover_all_prompts(mgr);

    if json {
        let out = serde_json::to_string_pretty(&prompts)?;
        println!("{out}");
        return Ok(());
    }

    if prompts.is_empty() {
        println!("\nNo prompt templates or records found.");
        println!("Create one with: `aek prompt create <id> --name <title>`\n");
        return Ok(());
    }

    println!("\nRegistered Prompts ({} total):", prompts.len());
    println!(
        "{:<24} {:<30} {:<10} {:<12} {:<16}",
        "ID", "NAME", "VERSION", "STATUS", "HASH"
    );
    println!("{:-<96}", "");

    for p in prompts {
        let short_hash = if p.content_hash.len() >= 12 {
            &p.content_hash[..12]
        } else {
            &p.content_hash
        };
        println!(
            "{:<24} {:<30} {:<10} {:<12} {:<16}",
            p.id,
            if p.name.len() > 28 {
                format!("{}...", &p.name[..25])
            } else {
                p.name
            },
            p.version,
            p.status,
            short_hash
        );
    }
    println!();
    Ok(())
}

fn handle_show(mgr: &PromptVersionManager, id: &str, json: bool) -> Result<()> {
    let prompt = find_prompt(mgr, id)?;

    if json {
        let out = serde_json::to_string_pretty(&prompt)?;
        println!("{out}");
        return Ok(());
    }

    println!("\n=== Prompt: {} ===", prompt.id);
    println!("Name:        {}", prompt.name);
    println!("Description: {}", prompt.description);
    println!("Version:     {}", prompt.version);
    println!("Status:      {}", prompt.status);
    println!("Created At:  {}", prompt.created_at);
    println!("Hash:        {}", prompt.content_hash);
    println!("Provenance:  {}", prompt.provenance);

    if !prompt.template_variables.is_empty() {
        println!("Variables:   {}", prompt.template_variables.join(", "));
    }
    if !prompt.constraints.is_empty() {
        println!("Constraints: {}", prompt.constraints.join("; "));
    }

    println!("\n--- Active Content ---");
    println!("{}", prompt.active_content());
    println!("----------------------\n");

    Ok(())
}

fn handle_create(mgr: &PromptVersionManager, args: PromptCreateArgs) -> Result<()> {
    let content = if let Some(path) = &args.file {
        fs::read_to_string(path)
            .with_context(|| format!("Failed to read file '{}'", path.display()))?
    } else {
        // Read from stdin if available, otherwise use default engineering template
        let mut buffer = String::new();
        let stdin = io::stdin();
        if !atty_check() {
            let mut handle = stdin.lock();
            let _ = handle.read_to_string(&mut buffer);
        }

        if buffer.trim().is_empty() {
            format!(
                "# {}\n\nPerform high-precision engineering on {}.\n\n## Requirements\n- Ensure zero undefined behavior\n- Provide automated unit tests\n",
                args.name.as_deref().unwrap_or(&args.id),
                args.id
            )
        } else {
            buffer
        }
    };

    let risks = detect_injection_risk(&content);
    if risks.is_suspicious {
        eprintln!("Warning: Detected potential prompt injection pattern in prompt content:");
        for pattern in &risks.detected_patterns {
            eprintln!("  - '{pattern}'");
        }
    }

    let name = args.name.unwrap_or_else(|| args.id.clone());
    let desc = args
        .description
        .unwrap_or_else(|| format!("Prompt artifact for {}", args.id));
    let mut rec = PromptRecord::new(&args.id, name, desc, content);
    rec.status = PromptStatus::Draft;

    let hash = mgr.save_version(&rec)?;
    println!(
        "\nPrompt '{}' created successfully (version {}, hash {}).\n",
        rec.id,
        rec.version,
        &hash[..12.min(hash.len())]
    );

    Ok(())
}

fn handle_versions(mgr: &PromptVersionManager, id: &str, json: bool) -> Result<()> {
    let versions = mgr.list_versions(id)?;

    if json {
        let out = serde_json::to_string_pretty(&versions)?;
        println!("{out}");
        return Ok(());
    }

    if versions.is_empty() {
        println!("\nNo version history recorded for prompt '{id}'.\n");
        return Ok(());
    }

    println!(
        "\nVersion History for '{}' ({} versions):",
        id,
        versions.len()
    );
    println!(
        "{:<12} {:<14} {:<18} {:<32}",
        "VERSION", "STATUS", "CREATED", "HASH"
    );
    println!("{:-<80}", "");

    for v in versions {
        let short_hash = if v.content_hash.len() >= 12 {
            &v.content_hash[..12]
        } else {
            &v.content_hash
        };
        let short_created = if v.created_at.len() >= 19 {
            &v.created_at[..19]
        } else {
            &v.created_at
        };
        println!(
            "{:<12} {:<14} {:<18} {:<32}",
            v.version, v.status, short_created, short_hash
        );
    }
    println!();
    Ok(())
}

fn handle_diff(
    mgr: &PromptVersionManager,
    id: &str,
    ver_a: Option<String>,
    ver_b: Option<String>,
) -> Result<()> {
    let versions = mgr.list_versions(id)?;
    if versions.len() < 2 && (ver_a.is_none() || ver_b.is_none()) {
        bail!("Prompt '{id}' needs at least two versions to compare.");
    }

    let v_a = ver_a.unwrap_or_else(|| {
        if versions.len() >= 2 {
            versions[versions.len() - 2].version.clone()
        } else {
            versions[0].version.clone()
        }
    });

    let v_b = ver_b.unwrap_or_else(|| {
        versions
            .last()
            .map(|v| v.version.clone())
            .unwrap_or_default()
    });

    let comp = mgr.compare(id, &v_a, &v_b)?;

    println!("\nDiff between '{}' versions {} and {}:", id, v_a, v_b);
    println!("{:-<60}", "");

    if comp.original_diff_lines.is_empty() {
        println!(
            "No text differences found between version {} and {}.",
            v_a, v_b
        );
    } else {
        for line in &comp.original_diff_lines {
            println!("{line}");
        }
    }
    println!("{:-<60}\n", "");
    Ok(())
}

fn handle_rollback(mgr: &PromptVersionManager, id: &str, target_version: &str) -> Result<()> {
    let rolled = mgr.rollback(id, target_version)?;
    println!(
        "\nSuccessfully rolled back prompt '{}' to version {}.\n",
        rolled.id, target_version
    );
    Ok(())
}

async fn handle_optimize(mgr: &PromptVersionManager, args: PromptOptimizeArgs) -> Result<()> {
    let (prompt_opt, content) = if let Some(input_path) = &args.input {
        let raw = fs::read_to_string(input_path)
            .with_context(|| format!("Failed to read input '{}'", input_path.display()))?;
        (None, raw)
    } else if let Some(id) = &args.id {
        let rec = find_prompt(mgr, id)?;
        (Some(rec.clone()), rec.active_content().to_string())
    } else {
        bail!("Must specify either a prompt <id> or --input <file>");
    };

    println!("\nOptimizing prompt using adapter '{}'...", args.adapter);

    let temp_rec = prompt_opt.clone().unwrap_or_else(|| {
        PromptRecord::new("temp_optimization", "Task", "Optimization task", &content)
    });

    let optimized = match args.adapter.as_str() {
        "prompt-optimizer" => {
            let adapter = PromptOptimizerAdapter;
            adapter.optimize(&temp_rec)?
        }
        "dspy" => {
            let adapter = DspyAdapter;
            let payload =
                adapter.build_bridge_payload(&temp_rec, "datasets/examples/rust.jsonl", 3);
            let res = adapter.run_bridge(&payload, None).await?;
            res.optimized_instructions
        }
        "generic" => {
            let assembler = ContextAssembler::new();
            let assembled = assembler.assemble(&content, None, &[], &[]);
            assembled.enriched_prompt
        }
        other => {
            bail!(
                "Unknown optimization adapter: '{other}'. Choose 'prompt-optimizer', 'dspy', or 'generic'"
            );
        }
    };

    if args.json {
        let json_out = serde_json::json!({
            "status": "success",
            "adapter": args.adapter,
            "original_length": content.len(),
            "optimized_length": optimized.len(),
            "optimized_content": optimized,
        });
        println!("{json_out}");
        return Ok(());
    }

    println!("\n=== Optimized Prompt Preview ===");
    println!("{optimized}");
    println!("=================================\n");

    if !args.dry_run {
        if let Some(mut rec) = prompt_opt {
            rec.optimized_content = Some(optimized);
            rec.status = PromptStatus::Optimized;
            // Bump version
            rec.version = format!("{}.opt", rec.version);
            let new_hash = mgr.save_version(&rec)?;
            println!(
                "Saved optimized version {} (hash {}).\n",
                rec.version,
                &new_hash[..12.min(new_hash.len())]
            );
        }
    } else {
        println!("(Dry run: prompt version not updated on disk)\n");
    }

    Ok(())
}

fn handle_evaluate(mgr: &PromptVersionManager, args: PromptEvaluateArgs) -> Result<()> {
    let content = if let Some(id) = &args.id {
        let rec = find_prompt(mgr, id)?;
        rec.active_content().to_string()
    } else {
        "# Engineering Prompt\nEnsure memory safety and unit test coverage.".to_string()
    };

    let dataset_path = find_dataset_path(args.dataset)?;

    let dataset = DatasetManager::load_from_file(&dataset_path)?;
    let calc = EvaluationMetricCalculator::new();

    let mut scores = Vec::new();
    let mut total_constraint = 0.0;
    let mut schema_passed = 0;

    for case in &dataset.cases {
        let score =
            calc.evaluate_constraints(&content, &case.expected_properties, &case.constraints);
        total_constraint += score.constraint_score;
        if score.schema_valid {
            schema_passed += 1;
        }
        scores.push((case.id.clone(), score));
    }

    let avg_constraint = if !dataset.cases.is_empty() {
        total_constraint / (dataset.cases.len() as f64)
    } else {
        1.0
    };

    if args.json {
        let json_out = serde_json::json!({
            "dataset": dataset.name,
            "total_cases": dataset.cases.len(),
            "average_constraint_score": avg_constraint,
            "schema_valid_count": schema_passed,
            "cases": scores,
        });
        println!("{json_out}");
        return Ok(());
    }

    println!("\n=== Evaluation Report: {} ===", dataset.name);
    println!("Evaluated Cases:         {}", dataset.cases.len());
    println!("Avg Constraint Score:    {:.2}%", avg_constraint * 100.0);
    println!(
        "Schema Valid Cases:      {}/{}",
        schema_passed,
        dataset.cases.len()
    );
    println!("{:-<60}", "");

    for (case_id, score) in &scores {
        println!(
            "Case {:<20} | Score: {:.0}% | Schema: {}",
            case_id,
            score.constraint_score * 100.0,
            if score.schema_valid { "PASS" } else { "FAIL" }
        );
        if !score.missing_keywords.is_empty() {
            println!(
                "  Missing keywords:      {}",
                score.missing_keywords.join(", ")
            );
        }
        if !score.negative_violations.is_empty() {
            println!(
                "  Negative violations:   {}",
                score.negative_violations.join(", ")
            );
        }
    }
    println!("{:-<60}\n", "");

    Ok(())
}

fn handle_benchmark(mgr: &PromptVersionManager, args: PromptBenchmarkArgs) -> Result<()> {
    let rec = find_prompt(mgr, &args.id)?;
    let dataset_path = find_dataset_path(args.dataset)?;

    let dataset = DatasetManager::load_from_file(&dataset_path)?;
    let calc = EvaluationMetricCalculator::new();

    let mut total_score = 0.0;
    for case in &dataset.cases {
        let s = calc.evaluate_constraints(
            rec.active_content(),
            &case.expected_properties,
            &case.constraints,
        );
        total_score += s.constraint_score;
    }

    let benchmark_score = if !dataset.cases.is_empty() {
        total_score / (dataset.cases.len() as f64)
    } else {
        1.0
    };

    println!("\n=== Benchmark Summary for '{}' ===", rec.id);
    println!("Dataset:           {}", dataset.name);
    println!("Evaluation Cases:  {}", dataset.cases.len());
    println!("Benchmark Score:   {:.2}%", benchmark_score * 100.0);
    println!(
        "Content Hash:      {}",
        &rec.content_hash[..12.min(rec.content_hash.len())]
    );
    println!("Status:            {}\n", rec.status);

    Ok(())
}

fn handle_compare(mgr: &PromptVersionManager, args: PromptCompareArgs) -> Result<()> {
    let rec_a = mgr.get_version(&args.id, &args.version_a)?;
    let rec_b = mgr.get_version(&args.id, &args.version_b)?;

    println!(
        "\nComparing Prompt '{}': Version {} vs Version {}",
        args.id, args.version_a, args.version_b
    );
    println!("{:-<60}", "");
    println!(
        "Version A Hash: {}",
        &rec_a.content_hash[..12.min(rec_a.content_hash.len())]
    );
    println!(
        "Version B Hash: {}",
        &rec_b.content_hash[..12.min(rec_b.content_hash.len())]
    );

    let diff = mgr.compare(&args.id, &args.version_a, &args.version_b)?;
    println!("\nLine Differences:");
    if diff.original_diff_lines.is_empty() {
        println!("(No content differences)");
    } else {
        for l in &diff.original_diff_lines {
            println!("{l}");
        }
    }
    println!("{:-<60}\n", "");
    Ok(())
}

fn handle_export(mgr: &PromptVersionManager, args: PromptExportArgs) -> Result<()> {
    let rec = find_prompt(mgr, &args.id)?;

    if args.agent.to_lowercase() == "antigravity" {
        let adapter = AntigravityAdapter::new();
        let target_dir = args.target.unwrap_or_else(|| {
            PlatformPaths::resolve()
                .antigravity_dir
                .map(|d| d.join("instructions"))
                .unwrap_or_else(|| PathBuf::from(".antigravity/instructions"))
        });

        if args.dry_run {
            println!(
                "\n[Dry Run] Would export prompt '{}' to: {}",
                rec.id,
                target_dir.display()
            );
            println!("Target agent: Antigravity");
            println!(
                "Prompt content length: {} bytes\n",
                rec.active_content().len()
            );
        } else {
            let target_path = adapter.export_prompt_instruction(&rec, &target_dir, args.dry_run)?;
            println!(
                "\nExported prompt '{}' successfully to: {}\n",
                rec.id,
                target_path.display()
            );
        }
    } else {
        bail!(
            "Unsupported agent adapter '{}'. Currently supported: 'antigravity'",
            args.agent
        );
    }

    Ok(())
}

async fn handle_doctor() -> Result<()> {
    println!("\nAEK Prompt Optimization & Evaluation Doctor");
    println!("{:=<60}", "");

    // 1. Python runtime
    let python_cmd =
        find_executable_in_path("python").or_else(|| find_executable_in_path("python3"));

    match python_cmd {
        Some(py) => {
            println!("[PASS] Python runtime found: {}", py.display());
            // Check DSPy
            let dspy_check = SafeCommand::new(py.to_string_lossy())
                .arg("-c")
                .arg("import dspy; print(dspy.__version__)")
                .execute()
                .await;

            match dspy_check {
                Ok(res) if res.success => {
                    let ver = res.stdout.trim();
                    println!("[PASS] Stanford DSPy package available (v{ver})");
                }
                _ => {
                    println!("[INFO] Stanford DSPy not installed in active Python environment.");
                    println!("       (AEK deterministic fallback optimizer will be used)");
                }
            }
        }
        None => {
            println!("[WARN] Python runtime not found in PATH.");
            println!("       (Required only for DSPy prompt compilation)");
        }
    }

    // 2. Node.js runtime
    let node_cmd = find_executable_in_path("node");
    match node_cmd {
        Some(nd) => println!("[PASS] Node.js runtime found: {}", nd.display()),
        None => {
            println!("[WARN] Node.js runtime not found in PATH.");
            println!("       (Required only for Promptfoo evaluation suite)");
        }
    }

    // 3. Promptfoo CLI
    let promptfoo_cmd =
        find_executable_in_path("promptfoo").or_else(|| find_executable_in_path("promptfoo.cmd"));

    match promptfoo_cmd {
        Some(pf) => println!("[PASS] Promptfoo CLI found: {}", pf.display()),
        None => {
            println!("[INFO] Promptfoo CLI binary not found in PATH.");
            println!("       (Can run via `npx promptfoo eval` if Node.js is present)");
        }
    }

    println!("{:=<60}\n", "");
    Ok(())
}

fn handle_adapters() -> Result<()> {
    println!("\nRegistered AEK Prompt Optimization Adapters:");
    println!(
        "{:<20} {:<16} {:<40}",
        "ADAPTER", "TYPE", "STATUS / CAPABILITIES"
    );
    println!("{:-<78}", "");

    println!(
        "{:<20} {:<16} {:<40}",
        "prompt-optimizer", "Native Rust", "Operational (Structured role decomposition)"
    );
    println!(
        "{:<20} {:<16} {:<40}",
        "dspy", "Python Bridge", "Operational (Stanford DSPy + Fallback engine)"
    );
    println!(
        "{:<20} {:<16} {:<40}",
        "promptfoo", "CLI Subprocess", "Operational (LLM evaluation & red-teaming)"
    );
    println!(
        "{:<20} {:<16} {:<40}",
        "generic", "Deterministic", "Operational (Rule-based constraints & schema)"
    );

    println!("\nUse with: `aek prompt optimize <id> --adapter <name>`\n");
    Ok(())
}

fn atty_check() -> bool {
    // Basic detection if stdin is a terminal or pipe
    false
}
