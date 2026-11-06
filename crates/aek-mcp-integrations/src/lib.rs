//! # aek-mcp-integrations
//!
//! Verified configuration adapters and presets for official MCP servers:
//! Context7, GitHub, Playwright, Sequential Thinking, and Filesystem.

pub mod context7;
pub mod filesystem;
pub mod github;
pub mod playwright;
pub mod sequential_thinking;

pub use context7::context7_config;
pub use filesystem::filesystem_config;
pub use github::github_config;
pub use playwright::playwright_config;
pub use sequential_thinking::sequential_thinking_config;

use aek_mcp::config::McpServerConfig;
use aek_mcp::registry::McpServerRegistry;
use std::path::Path;

/// Returns a pre-configured `McpServerRegistry` containing all 5 initial integrations.
#[must_use]
pub fn standard_integrations_registry(default_scratch_dir: impl AsRef<Path>) -> McpServerRegistry {
    let mut reg = McpServerRegistry::new();
    let _ = reg.register(context7_config());
    let _ = reg.register(github_config());
    let _ = reg.register(playwright_config());
    let _ = reg.register(sequential_thinking_config());
    let _ = reg.register(filesystem_config(default_scratch_dir));
    reg
}

/// Returns a vector of the 5 standard MCP server configurations.
#[must_use]
pub fn standard_integrations_list(default_scratch_dir: impl AsRef<Path>) -> Vec<McpServerConfig> {
    vec![
        context7_config(),
        github_config(),
        playwright_config(),
        sequential_thinking_config(),
        filesystem_config(default_scratch_dir),
    ]
}

