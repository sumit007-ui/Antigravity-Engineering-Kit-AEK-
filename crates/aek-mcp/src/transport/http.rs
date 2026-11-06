//! Streamable HTTP / JSON-RPC asynchronous MCP transport.

use crate::errors::McpError;
use crate::transport::{BoxFuture, McpTransport};
use reqwest::Client;
use reqwest::header::{CONTENT_TYPE, HeaderMap, HeaderName, HeaderValue};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

/// HTTP transport communicating with an MCP endpoint via JSON-RPC 2.0 over HTTP/HTTPS.
pub struct HttpTransport {
    client: Client,
    endpoint_url: String,
    headers: HeaderMap,
    next_id: AtomicU64,
    timeout_duration: Duration,
    server_id: String,
    is_open: bool,
}

impl HttpTransport {
    /// Creates a new `HttpTransport` with verified TLS certificates and timeouts.
    pub fn new(
        server_id: &str,
        endpoint_url: &str,
        custom_headers: &HashMap<String, String>,
        timeout_seconds: u64,
    ) -> Result<Self, McpError> {
        let timeout_duration = Duration::from_secs(timeout_seconds);
        let client = Client::builder()
            .timeout(timeout_duration)
            .build()
            .map_err(|e| McpError::ConnectionFailed(server_id.to_string(), e.to_string()))?;

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        for (k, v) in custom_headers {
            if let (Ok(name), Ok(val)) = (
                HeaderName::from_bytes(k.as_bytes()),
                HeaderValue::from_str(v),
            ) {
                headers.insert(name, val);
            }
        }

        Ok(Self {
            client,
            endpoint_url: endpoint_url.to_string(),
            headers,
            next_id: AtomicU64::new(1),
            timeout_duration,
            server_id: server_id.to_string(),
            is_open: true,
        })
    }
}

impl McpTransport for HttpTransport {
    fn send_request<'a>(
        &'a mut self,
        method: &'a str,
        params: serde_json::Value,
    ) -> BoxFuture<'a, Result<serde_json::Value, McpError>> {
        Box::pin(async move {
            if !self.is_open {
                return Err(McpError::ConnectionFailed(
                    self.server_id.clone(),
                    "Transport connection closed".to_string(),
                ));
            }

            let req_id = self.next_id.fetch_add(1, Ordering::SeqCst);
            let request_payload = serde_json::json!({
                "jsonrpc": "2.0",
                "id": req_id,
                "method": method,
                "params": params,
            });

            let resp = self
                .client
                .post(&self.endpoint_url)
                .headers(self.headers.clone())
                .json(&request_payload)
                .send()
                .await
                .map_err(|e| {
                    if e.is_timeout() {
                        McpError::Timeout(self.timeout_duration.as_secs())
                    } else {
                        McpError::ConnectionFailed(self.server_id.clone(), e.to_string())
                    }
                })?;

            let json_body: serde_json::Value = resp
                .json()
                .await
                .map_err(|e| McpError::ConnectionFailed(self.server_id.clone(), e.to_string()))?;

            if let Some(err) = json_body.get("error") {
                let msg = err
                    .get("message")
                    .and_then(|m| m.as_str())
                    .unwrap_or("Unknown error");
                return Err(McpError::ToolExecutionFailed(
                    method.to_string(),
                    msg.to_string(),
                ));
            }

            if let Some(result) = json_body.get("result") {
                return Ok(result.clone());
            }

            Ok(serde_json::Value::Null)
        })
    }

    fn send_notification<'a>(
        &'a mut self,
        method: &'a str,
        params: serde_json::Value,
    ) -> BoxFuture<'a, Result<(), McpError>> {
        Box::pin(async move {
            let payload = serde_json::json!({
                "jsonrpc": "2.0",
                "method": method,
                "params": params,
            });

            let _ = self
                .client
                .post(&self.endpoint_url)
                .headers(self.headers.clone())
                .json(&payload)
                .send()
                .await;

            Ok(())
        })
    }

    fn close<'a>(&'a mut self) -> BoxFuture<'a, Result<(), McpError>> {
        Box::pin(async move {
            self.is_open = false;
            Ok(())
        })
    }

    fn is_alive(&self) -> bool {
        self.is_open
    }
}
