//! Command handler for `aek mcp`.

use crate::cli::McpSubcommand;
use aek_agents::adapter::AgentAdapter;
use aek_agents::antigravity::AntigravityAdapter;
use aek_mcp::client::McpClient;
use aek_mcp::config::{McpServerConfig, McpTransportConfig};
use aek_mcp::diagnostics::{McpCheckStatus, McpDoctor};
use aek_mcp::registry::McpServerRegistry;
use aek_mcp_integrations::standard_integrations_registry;
use aek_platform::paths::PlatformPaths;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

pub fn find_mcp_registry() -> (McpServerRegistry, PathBuf) {
    let platform = PlatformPaths::resolve();
    let user_reg_path = platform.config_dir.join("mcp_registry.toml");

    if user_reg_path.is_file() {
        if let Ok(reg) = McpServerRegistry::load_from_file(&user_reg_path) {
            if !reg.list().is_empty() {
                return (reg, user_reg_path);
            }
        }
    }

    // Try workspace repo locations
    let candidates = [
        PathBuf::from("integrations/mcp/registry.toml"),
        PathBuf::from("../integrations/mcp/registry.toml"),
        PathBuf::from("../../integrations/mcp/registry.toml"),
    ];

    for c in &candidates {
        if c.is_file() {
            if let Ok(reg) = McpServerRegistry::load_from_file(c) {
                return (reg, c.clone());
            }
        }
    }

    // Fall back to built-in standard integrations
    let scratch = platform.data_dir.join("scratch");
    let mut reg = standard_integrations_registry(scratch);

    // Also scan individual files if integrations/mcp exists
    let int_dir = PathBuf::from("integrations/mcp");
    if int_dir.is_dir() {
        let _ = reg.load_directory(&int_dir);
    }

    (reg, user_reg_path)
}

pub fn execute_mcp(action: &McpSubcommand) -> ExitCode {
    let rt = match tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
    {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Failed to initialize async Tokio runtime: {e}");
            return ExitCode::FAILURE;
        }
    };

    match action {
        McpSubcommand::List { json } => list_servers(*json),
        McpSubcommand::Show { name } => show_server(name),
        McpSubcommand::Register { config } => register_server(config),
        McpSubcommand::Remove { name } => remove_server(name),
        McpSubcommand::Enable { name } => enable_server(name),
        McpSubcommand::Disable { name } => disable_server(name),
        McpSubcommand::Connect { name } => rt.block_on(connect_server(name)),
        McpSubcommand::Disconnect { name } => disconnect_server(name),
        McpSubcommand::Doctor { server, json } => audit_servers(server.as_deref(), *json),
        McpSubcommand::Capabilities { name } => rt.block_on(query_capabilities(name)),
        McpSubcommand::Tools { name, json } => rt.block_on(list_tools(name, *json)),
        McpSubcommand::Install { name, dry_run } => install_server(name, *dry_run),
        McpSubcommand::Uninstall { name, dry_run } => uninstall_server(name, *dry_run),
        McpSubcommand::Export {
            agent,
            target,
            dry_run,
        } => export_servers(agent, target.as_deref(), *dry_run),
        McpSubcommand::Call {
            server,
            tool,
            input,
        } => rt.block_on(call_tool(server, tool, input.as_deref())),
    }
}

fn list_servers(as_json: bool) -> ExitCode {
    let (registry, _) = find_mcp_registry();
    let servers = registry.list();

    if as_json {
        let items: Vec<serde_json::Value> = servers
            .iter()
            .map(|s| {
                let transport_kind = match &s.transport {
                    McpTransportConfig::Stdio { .. } => "stdio",
                    McpTransportConfig::Http { .. } => "http",
                };
                serde_json::json!({
                    "id": s.id,
                    "name": s.name,
                    "description": s.description,
                    "transport": transport_kind,
                    "enabled": s.enabled,
                    "requires_auth": s.requires_auth,
                    "version": s.version,
                })
            })
            .collect();
        println!(
            "{}",
            serde_json::to_string_pretty(&items).unwrap_or_default()
        );
        return ExitCode::SUCCESS;
    }

    println!();
    println!("Available MCP Servers ({} registered):", servers.len());
    println!("{:-<75}", "");
    println!(
        "{:<22} {:<10} {:<10} Description",
        "Server ID", "Transport", "Status"
    );
    println!("{:-<75}", "");

    for s in &servers {
        let transport_str = match &s.transport {
            McpTransportConfig::Stdio { .. } => "stdio",
            McpTransportConfig::Http { .. } => "http",
        };
        let status = if s.enabled { "enabled" } else { "disabled" };
        let desc = if s.description.len() > 32 {
            format!("{}...", &s.description[..29])
        } else {
            s.description.clone()
        };
        println!("{:<22} {:<10} {:<10} {}", s.id, transport_str, status, desc);
    }

    println!("{:-<75}", "");
    ExitCode::SUCCESS
}

fn show_server(name: &str) -> ExitCode {
    let (registry, _) = find_mcp_registry();
    match registry.get(name) {
        Some(s) => {
            println!();
            println!("Server: {}", s.id);
            println!("Display Name: {}", s.name);
            println!("Description:  {}", s.description);
            println!(
                "Status:       {}",
                if s.enabled { "Enabled" } else { "Disabled" }
            );
            println!("Version:      {}", s.version);
            println!("Timeout:      {}s", s.timeout_seconds);
            println!(
                "Requires Auth:{}",
                if s.requires_auth { "YES" } else { "NO" }
            );
            if !s.auth_env_vars.is_empty() {
                println!("Auth Env Vars:{}", s.auth_env_vars.join(", "));
            }
            if !s.required_dependencies.is_empty() {
                println!("Dependencies: {}", s.required_dependencies.join(", "));
            }
            println!("{:-<60}", "");
            println!("Transport:");
            match &s.transport {
                McpTransportConfig::Stdio {
                    command,
                    args,
                    env,
                    working_dir,
                } => {
                    println!("  Type:    stdio");
                    println!("  Command: {}", command);
                    println!("  Args:    {:?}", args);
                    if !env.is_empty() {
                        println!("  Env:     {:?}", env);
                    }
                    if let Some(wd) = working_dir {
                        println!("  WorkDir: {}", wd.display());
                    }
                }
                McpTransportConfig::Http { url, headers } => {
                    println!("  Type:    http");
                    println!("  URL:     {}", url);
                    if !headers.is_empty() {
                        println!("  Headers: {:?}", headers);
                    }
                }
            }
            println!("{:-<60}", "");
            ExitCode::SUCCESS
        }
        None => {
            eprintln!("Error: Server '{name}' not found in registry.");
            ExitCode::FAILURE
        }
    }
}

fn register_server(config_path: &Path) -> ExitCode {
    if !config_path.is_file() {
        eprintln!(
            "Error: Configuration file '{}' not found.",
            config_path.display()
        );
        return ExitCode::FAILURE;
    }

    let content = match std::fs::read_to_string(config_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error reading config file: {e}");
            return ExitCode::FAILURE;
        }
    };

    let cfg: McpServerConfig = match toml::from_str(&content) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error parsing server config: {e}");
            return ExitCode::FAILURE;
        }
    };

    let (mut registry, save_path) = find_mcp_registry();
    let server_id = cfg.id.clone();
    if let Err(e) = registry.register(cfg) {
        eprintln!("Error validating server: {e}");
        return ExitCode::FAILURE;
    }

    if let Err(e) = registry.save_to_file(&save_path) {
        eprintln!("Failed to persist registry to {}: {e}", save_path.display());
        return ExitCode::FAILURE;
    }

    println!(
        "Successfully registered server '{server_id}' in {}",
        save_path.display()
    );
    ExitCode::SUCCESS
}

fn remove_server(name: &str) -> ExitCode {
    let (mut registry, save_path) = find_mcp_registry();
    if registry.remove(name) {
        let _ = registry.save_to_file(&save_path);
        println!("Successfully removed server '{name}' from registry.");
        ExitCode::SUCCESS
    } else {
        eprintln!("Server '{name}' was not found in registry.");
        ExitCode::FAILURE
    }
}

fn enable_server(name: &str) -> ExitCode {
    let (mut registry, save_path) = find_mcp_registry();
    match registry.set_enabled(name, true) {
        Ok(true) => {
            let _ = registry.save_to_file(&save_path);
            println!("MCP server '{name}' enabled.");
            ExitCode::SUCCESS
        }
        _ => {
            eprintln!("Server '{name}' not found in registry.");
            ExitCode::FAILURE
        }
    }
}

fn disable_server(name: &str) -> ExitCode {
    let (mut registry, save_path) = find_mcp_registry();
    match registry.set_enabled(name, false) {
        Ok(true) => {
            let _ = registry.save_to_file(&save_path);
            println!("MCP server '{name}' disabled.");
            ExitCode::SUCCESS
        }
        _ => {
            eprintln!("Server '{name}' not found in registry.");
            ExitCode::FAILURE
        }
    }
}

async fn connect_server(name: &str) -> ExitCode {
    let (registry, _) = find_mcp_registry();
    let config = match registry.get(name) {
        Some(c) => c,
        None => {
            eprintln!("Server '{name}' not found in registry.");
            return ExitCode::FAILURE;
        }
    };

    println!("Connecting to MCP server '{name}'...");
    match McpClient::connect(config).await {
        Ok(mut client) => {
            println!("Connected successfully!");
            if let Some(info) = client.server_info() {
                println!("Server Name:    {}", info.name);
                println!("Server Version: {}", info.version);
            }
            let _ = client.close().await;
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("Connection failed for server '{name}': {e}");
            ExitCode::FAILURE
        }
    }
}

fn disconnect_server(name: &str) -> ExitCode {
    println!("Session for MCP server '{name}' cleanly disconnected.");
    ExitCode::SUCCESS
}

fn audit_servers(server_filter: Option<&str>, as_json: bool) -> ExitCode {
    let (registry, _) = find_mcp_registry();
    let mut reports = Vec::new();

    if let Some(s_name) = server_filter {
        if let Some(cfg) = registry.get(s_name) {
            reports.push(McpDoctor::audit_server(cfg));
        } else {
            eprintln!("Server '{s_name}' not found in registry.");
            return ExitCode::FAILURE;
        }
    } else {
        for cfg in registry.list() {
            reports.push(McpDoctor::audit_server(cfg));
        }
    }

    if as_json {
        println!(
            "{}",
            serde_json::to_string_pretty(&reports).unwrap_or_default()
        );
        return ExitCode::SUCCESS;
    }

    println!();
    println!(
        "MCP Server Diagnostics & Health Check ({} servers audited):",
        reports.len()
    );
    println!("{:-<75}", "");

    for rep in &reports {
        println!(
            "Server: {:<20} Overall Status: {}",
            rep.server_id, rep.status
        );
        for item in &rep.checks {
            let symbol = match item.status {
                McpCheckStatus::Pass => "[PASS]",
                McpCheckStatus::Warn => "[WARN]",
                McpCheckStatus::Fail => "[FAIL]",
            };
            println!("  {:<7} {:<28} -> {}", symbol, item.name, item.message);
        }
        println!("{:-<75}", "");
    }

    ExitCode::SUCCESS
}

async fn query_capabilities(name: &str) -> ExitCode {
    let (registry, _) = find_mcp_registry();
    let config = match registry.get(name) {
        Some(c) => c,
        None => {
            eprintln!("Server '{name}' not found in registry.");
            return ExitCode::FAILURE;
        }
    };

    println!("Querying capabilities for server '{name}'...");
    match McpClient::connect(config).await {
        Ok(mut client) => {
            let caps = client.server_capabilities().cloned().unwrap_or_default();
            println!();
            println!("Server Capabilities for '{}':", name);
            println!("{:-<50}", "");
            println!(
                "Tools Support:     {}",
                if caps.supports_tools() { "YES" } else { "NO" }
            );
            println!(
                "Resources Support: {}",
                if caps.supports_resources() {
                    "YES"
                } else {
                    "NO"
                }
            );
            println!(
                "Prompts Support:   {}",
                if caps.supports_prompts() { "YES" } else { "NO" }
            );
            println!("{:-<50}", "");
            let _ = client.close().await;
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("Failed to query capabilities for '{name}': {e}");
            ExitCode::FAILURE
        }
    }
}

async fn list_tools(name: &str, as_json: bool) -> ExitCode {
    let (registry, _) = find_mcp_registry();
    let config = match registry.get(name) {
        Some(c) => c,
        None => {
            eprintln!("Server '{name}' not found in registry.");
            return ExitCode::FAILURE;
        }
    };

    match McpClient::connect(config).await {
        Ok(mut client) => match client.list_tools().await {
            Ok(tools) => {
                if as_json {
                    println!(
                        "{}",
                        serde_json::to_string_pretty(&tools).unwrap_or_default()
                    );
                } else {
                    println!();
                    println!("Discovered Tools for '{}' ({} found):", name, tools.len());
                    println!("{:-<75}", "");
                    println!("{:<32} {:<18} Description", "Tool Name", "Permission");
                    println!("{:-<75}", "");
                    for t in &tools {
                        let desc = t.description.as_deref().unwrap_or("No description");
                        println!("{:<32} {:<18} {}", t.name, t.permission.to_string(), desc);
                    }
                    println!("{:-<75}", "");
                }
                let _ = client.close().await;
                ExitCode::SUCCESS
            }
            Err(e) => {
                eprintln!("Failed to list tools for '{name}': {e}");
                let _ = client.close().await;
                ExitCode::FAILURE
            }
        },
        Err(e) => {
            eprintln!("Failed to connect to '{name}': {e}");
            ExitCode::FAILURE
        }
    }
}

fn install_server(name: &str, dry_run: bool) -> ExitCode {
    let (registry, _) = find_mcp_registry();
    let server = match registry.get(name) {
        Some(s) => s,
        None => {
            eprintln!("Server '{name}' not found in registry.");
            return ExitCode::FAILURE;
        }
    };

    let adapter = AntigravityAdapter::new();
    match adapter.merge_mcp_config(std::slice::from_ref(server), None, dry_run) {
        Ok(rep) => {
            if dry_run {
                println!(
                    "[Dry run] Would install MCP server '{}' into: {}",
                    name,
                    rep.target_file.display()
                );
            } else {
                println!(
                    "Successfully installed MCP server '{}' into {}",
                    name,
                    rep.target_file.display()
                );
            }
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("Failed to install server '{name}': {e}");
            ExitCode::FAILURE
        }
    }
}

fn uninstall_server(name: &str, dry_run: bool) -> ExitCode {
    if dry_run {
        println!("[Dry run] Would uninstall MCP server '{name}'");
        return ExitCode::SUCCESS;
    }
    let (mut registry, save_path) = find_mcp_registry();
    if registry.remove(name) {
        let _ = registry.save_to_file(&save_path);
        println!("Successfully uninstalled server '{name}'.");
        ExitCode::SUCCESS
    } else {
        println!("Server '{name}' was not installed in registry.");
        ExitCode::SUCCESS
    }
}

fn export_servers(agent: &str, target: Option<&Path>, dry_run: bool) -> ExitCode {
    if !agent.eq_ignore_ascii_case("antigravity") {
        eprintln!("Currently supported agent for export is 'antigravity'. Given: '{agent}'");
        return ExitCode::FAILURE;
    }

    let (registry, _) = find_mcp_registry();
    let servers: Vec<_> = registry.list().into_iter().cloned().collect();

    let adapter = AntigravityAdapter::new();
    match adapter.merge_mcp_config(&servers, target, dry_run) {
        Ok(rep) => {
            if dry_run {
                println!();
                println!(
                    "[Dry run] Exported {} MCP servers for Google Antigravity:",
                    rep.merged_count
                );
                let json_val = adapter.export_mcp_config(&servers);
                println!(
                    "{}",
                    serde_json::to_string_pretty(&json_val).unwrap_or_default()
                );
                println!("Destination: {}", rep.target_file.display());
            } else {
                println!();
                println!(
                    "Successfully exported {} MCP servers to: {}",
                    rep.merged_count,
                    rep.target_file.display()
                );
                if let Some(bak) = rep.backup_file {
                    println!("Safety backup created at: {}", bak.display());
                }
            }
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("Failed to export MCP servers to Antigravity: {e}");
            ExitCode::FAILURE
        }
    }
}

async fn call_tool(server: &str, tool: &str, input_json: Option<&str>) -> ExitCode {
    let (registry, _) = find_mcp_registry();
    let config = match registry.get(server) {
        Some(c) => c,
        None => {
            eprintln!("Server '{server}' not found in registry.");
            return ExitCode::FAILURE;
        }
    };

    let arguments: serde_json::Value = match input_json {
        Some(s) => match serde_json::from_str(s) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Error: Argument input must be valid JSON: {e}");
                return ExitCode::FAILURE;
            }
        },
        None => serde_json::json!({}),
    };

    println!("Invoking tool '{server}/{tool}'...");
    match McpClient::connect(config).await {
        Ok(mut client) => match client.call_tool(tool, arguments).await {
            Ok(result) => {
                println!();
                if result.is_error {
                    println!("Tool returned error status:");
                } else {
                    println!("Tool Output:");
                }
                println!("{:-<60}", "");
                println!("{}", result.text());
                println!("{:-<60}", "");
                let _ = client.close().await;
                if result.is_error {
                    ExitCode::FAILURE
                } else {
                    ExitCode::SUCCESS
                }
            }
            Err(e) => {
                eprintln!("Tool invocation failed: {e}");
                let _ = client.close().await;
                ExitCode::FAILURE
            }
        },
        Err(e) => {
            eprintln!("Failed to connect to server '{server}': {e}");
            ExitCode::FAILURE
        }
    }
}
