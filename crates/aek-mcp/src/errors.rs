//! Strongly typed errors for the Model Context Protocol (MCP) engine.

use thiserror::Error;

/// MCP protocol and subsystem errors.
#[derive(Error, Debug)]
pub enum McpError {
    #[error("Invalid configuration for server '{0}': {1}")]
    InvalidConfiguration(String, String),

    #[error("Missing required executable '{0}' for server '{1}'")]
    MissingExecutable(String, String),

    #[error("Missing credentials for server '{0}': {1}")]
    MissingCredentials(String, String),

    #[error("Connection failed to server '{0}': {1}")]
    ConnectionFailed(String, String),

    #[error("Initialization failed for server '{0}': {1}")]
    InitializationFailed(String, String),

    #[error("Unsupported protocol version '{0}' (expected {1})")]
    UnsupportedProtocolVersion(String, String),

    #[error("Unsupported capability '{0}' on server '{1}'")]
    UnsupportedCapability(String, String),

    #[error("Tool '{0}' not found on server '{1}'")]
    ToolNotFound(String, String),

    #[error("Invalid tool arguments for '{0}': {1}")]
    InvalidToolArguments(String, String),

    #[error("Tool execution failed for '{0}': {1}")]
    ToolExecutionFailed(String, String),

    #[error("Operation timed out after {0} seconds")]
    Timeout(u64),

    #[error("Permission denied for action '{0}': {1}")]
    PermissionDenied(String, String),

    #[error("Server '{0}' crashed unexpectedly: {1}")]
    ServerCrash(String, String),

    #[error("Configuration conflict for server '{0}': {1}")]
    ConfigurationConflict(String, String),

    #[error("Transport error: {0}")]
    Transport(String),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON-RPC serialization/deserialization error: {0}")]
    Json(#[from] serde_json::Error),
}

