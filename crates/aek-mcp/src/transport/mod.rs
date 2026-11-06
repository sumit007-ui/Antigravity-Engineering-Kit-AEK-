//! MCP transport abstractions and implementations.

pub mod http;
pub mod stdio;

use crate::errors::McpError;
use std::future::Future;
use std::pin::Pin;

pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

/// Asynchronous transport interface for exchanging JSON-RPC 2.0 protocol messages.
pub trait McpTransport: Send + Sync {
    /// Sends a JSON-RPC request and awaits the matching response.
    fn send_request<'a>(
        &'a mut self,
        method: &'a str,
        params: serde_json::Value,
    ) -> BoxFuture<'a, Result<serde_json::Value, McpError>>;

    /// Sends a one-way JSON-RPC notification (no response expected).
    fn send_notification<'a>(
        &'a mut self,
        method: &'a str,
        params: serde_json::Value,
    ) -> BoxFuture<'a, Result<(), McpError>>;

    /// Gracefully closes the transport connection and terminates associated child processes.
    fn close<'a>(&'a mut self) -> BoxFuture<'a, Result<(), McpError>>;

    /// Checks if the underlying connection or subprocess is still running and healthy.
    fn is_alive(&self) -> bool;
}
