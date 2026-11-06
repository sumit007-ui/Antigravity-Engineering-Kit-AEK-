//! Standard I/O asynchronous MCP transport.

use crate::errors::McpError;
use crate::transport::{BoxFuture, McpTransport};
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Stdio;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, Lines};
use tokio::process::{Child, ChildStdin, ChildStdout, Command};
use tokio::time::timeout;
use tracing::debug;

/// Stdio transport managing a supervised child process communicating over stdin/stdout.
pub struct StdioTransport {
    child: Child,
    stdin: ChildStdin,
    lines: Lines<BufReader<ChildStdout>>,
    next_id: AtomicU64,
    timeout_duration: Duration,
    server_id: String,
}

impl StdioTransport {
    /// Spawns a new MCP server child process and establishes stdio transport streams.
    pub async fn spawn(
        server_id: &str,
        command_str: &str,
        args: &[String],
        env: &HashMap<String, String>,
        working_dir: Option<&PathBuf>,
        timeout_seconds: u64,
    ) -> Result<Self, McpError> {
        let mut cmd = Command::new(command_str);
        cmd.args(args);
        cmd.stdin(Stdio::piped());
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        if let Some(wd) = working_dir {
            cmd.current_dir(wd);
        }

        for (k, v) in env {
            // Support environment variable expansion (e.g. `${GITHUB_TOKEN}` -> read from host env)
            let resolved_value = if v.starts_with("${") && v.ends_with('}') {
                let var_name = &v[2..v.len() - 1];
                std::env::var(var_name).unwrap_or_default()
            } else {
                v.clone()
            };
            cmd.env(k, resolved_value);
        }

        // Platform-specific child flags (e.g. prevent console window popup on Windows)
        #[cfg(target_os = "windows")]
        {
            const CREATE_NO_WINDOW: u32 = 0x08000000;
            cmd.creation_flags(CREATE_NO_WINDOW);
        }

        let mut child = cmd.spawn().map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                McpError::MissingExecutable(command_str.to_string(), server_id.to_string())
            } else {
                McpError::ConnectionFailed(server_id.to_string(), e.to_string())
            }
        })?;

        let stdin = child.stdin.take().ok_or_else(|| {
            McpError::ConnectionFailed(
                server_id.to_string(),
                "Failed to open child stdin".to_string(),
            )
        })?;
        let stdout = child.stdout.take().ok_or_else(|| {
            McpError::ConnectionFailed(
                server_id.to_string(),
                "Failed to open child stdout".to_string(),
            )
        })?;

        // Background reader for stderr diagnostics so child doesn't block if buffer fills
        if let Some(stderr) = child.stderr.take() {
            let s_id = server_id.to_string();
            tokio::spawn(async move {
                let mut reader = BufReader::new(stderr).lines();
                while let Ok(Some(line)) = reader.next_line().await {
                    debug!(target: "mcp_stderr", server = %s_id, "{}", line);
                }
            });
        }

        let reader = BufReader::new(stdout);
        let lines = reader.lines();

        Ok(Self {
            child,
            stdin,
            lines,
            next_id: AtomicU64::new(1),
            timeout_duration: Duration::from_secs(timeout_seconds),
            server_id: server_id.to_string(),
        })
    }
}

impl McpTransport for StdioTransport {
    fn send_request<'a>(
        &'a mut self,
        method: &'a str,
        params: serde_json::Value,
    ) -> BoxFuture<'a, Result<serde_json::Value, McpError>> {
        Box::pin(async move {
            let req_id = self.next_id.fetch_add(1, Ordering::SeqCst);
            let request_payload = serde_json::json!({
                "jsonrpc": "2.0",
                "id": req_id,
                "method": method,
                "params": params,
            });

            let mut raw = serde_json::to_string(&request_payload)?;
            raw.push('\n');

            let write_fut = async {
                self.stdin.write_all(raw.as_bytes()).await?;
                self.stdin.flush().await?;
                Ok::<(), std::io::Error>(())
            };

            timeout(self.timeout_duration, write_fut)
                .await
                .map_err(|_| McpError::Timeout(self.timeout_duration.as_secs()))?
                .map_err(|e| McpError::ConnectionFailed(self.server_id.clone(), e.to_string()))?;

            // Await matching JSON-RPC response
            let read_fut = async {
                loop {
                    match self.lines.next_line().await {
                        Ok(Some(line)) => {
                            let trimmed = line.trim();
                            if trimmed.is_empty() || !trimmed.starts_with('{') {
                                continue; // Skip log or greeting text
                            }

                            if let Ok(val) = serde_json::from_str::<serde_json::Value>(trimmed) {
                                if let Some(id) = val.get("id").and_then(|i| i.as_u64()) {
                                    if id == req_id {
                                        if let Some(err) = val.get("error") {
                                            let msg = err
                                                .get("message")
                                                .and_then(|m| m.as_str())
                                                .unwrap_or("Unknown error");
                                            return Err(McpError::ToolExecutionFailed(
                                                method.to_string(),
                                                msg.to_string(),
                                            ));
                                        }
                                        if let Some(result) = val.get("result") {
                                            return Ok(result.clone());
                                        }
                                        return Ok(serde_json::Value::Null);
                                    }
                                }
                            }
                        }
                        Ok(None) => {
                            return Err(McpError::ServerCrash(
                                self.server_id.clone(),
                                "Server process closed stdout unexpectedly".to_string(),
                            ));
                        }
                        Err(e) => {
                            return Err(McpError::ConnectionFailed(
                                self.server_id.clone(),
                                e.to_string(),
                            ));
                        }
                    }
                }
            };

            timeout(self.timeout_duration, read_fut)
                .await
                .map_err(|_| McpError::Timeout(self.timeout_duration.as_secs()))?
        })
    }

    fn send_notification<'a>(
        &'a mut self,
        method: &'a str,
        params: serde_json::Value,
    ) -> BoxFuture<'a, Result<(), McpError>> {
        Box::pin(async move {
            let notification_payload = serde_json::json!({
                "jsonrpc": "2.0",
                "method": method,
                "params": params,
            });

            let mut raw = serde_json::to_string(&notification_payload)?;
            raw.push('\n');

            let write_fut = async {
                self.stdin.write_all(raw.as_bytes()).await?;
                self.stdin.flush().await?;
                Ok::<(), std::io::Error>(())
            };

            timeout(self.timeout_duration, write_fut)
                .await
                .map_err(|_| McpError::Timeout(self.timeout_duration.as_secs()))?
                .map_err(|e| McpError::ConnectionFailed(self.server_id.clone(), e.to_string()))?;

            Ok(())
        })
    }

    fn close<'a>(&'a mut self) -> BoxFuture<'a, Result<(), McpError>> {
        Box::pin(async move {
            let _ = self.child.kill().await;
            let _ = self.child.wait().await;
            Ok(())
        })
    }

    fn is_alive(&self) -> bool {
        self.child.id().is_some()
    }
}
