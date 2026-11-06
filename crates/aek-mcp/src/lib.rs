//! # aek-mcp
//!
//! Reusable Model Context Protocol (MCP) client engine, transport abstraction,
//! server registry, security enforcement, and diagnostics for coding agents.

pub mod capabilities;
pub mod client;
pub mod config;
pub mod diagnostics;
pub mod discovery;
pub mod errors;
pub mod lifecycle;
pub mod registry;
pub mod security;
pub mod transport;

pub use capabilities::{ClientCapabilities, ClientInfo, ServerCapabilities, ServerInfo};
pub use client::McpClient;
pub use config::{McpServerConfig, McpTransportConfig, PermissionProfile};
pub use diagnostics::{McpCheckItem, McpCheckStatus, McpDoctor, ServerDiagnosticReport};
pub use discovery::{McpContent, McpPrompt, McpResource, McpTool, McpToolResult};
pub use errors::McpError;
pub use lifecycle::{ServerLifecycle, ServerState};
pub use registry::McpServerRegistry;
pub use security::{
    ToolPermission, classify_tool_permission, redact_secrets, validate_path_containment,
};
pub use transport::McpTransport;
