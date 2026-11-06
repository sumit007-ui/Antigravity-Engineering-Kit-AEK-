//! Reusable asynchronous MCP client engine.

use crate::capabilities::{ClientCapabilities, ClientInfo, ServerCapabilities, ServerInfo};
use crate::config::{McpServerConfig, McpTransportConfig, PermissionProfile};
use crate::discovery::{McpContent, McpPrompt, McpResource, McpTool, McpToolResult};
use crate::errors::McpError;
use crate::lifecycle::ServerLifecycle;
use crate::security::{ToolPermission, classify_tool_permission};
use crate::transport::McpTransport;
use crate::transport::http::HttpTransport;
use crate::transport::stdio::StdioTransport;
use tracing::info;

pub const DEFAULT_PROTOCOL_VERSION: &str = "2024-11-05";

/// High-level client managing an active protocol session with an MCP server.
pub struct McpClient {
    server_id: String,
    transport: Box<dyn McpTransport>,
    lifecycle: ServerLifecycle,
    server_info: Option<ServerInfo>,
    server_capabilities: Option<ServerCapabilities>,
    protocol_version: String,
    permissions: PermissionProfile,
}

impl McpClient {
    /// Connects to an MCP server using its configured transport and executes handshake.
    pub async fn connect(config: &McpServerConfig) -> Result<Self, McpError> {
        config.validate()?;

        let mut lifecycle = ServerLifecycle::new(&config.id);
        lifecycle.transition_to_connecting()?;

        let transport: Box<dyn McpTransport> = match &config.transport {
            McpTransportConfig::Stdio {
                command,
                args,
                env,
                working_dir,
            } => {
                let stdio = StdioTransport::spawn(
                    &config.id,
                    command,
                    args,
                    env,
                    working_dir.as_ref(),
                    config.timeout_seconds,
                )
                .await?;
                Box::new(stdio)
            }
            McpTransportConfig::Http { url, headers } => {
                let http = HttpTransport::new(&config.id, url, headers, config.timeout_seconds)?;
                Box::new(http)
            }
        };

        let mut client = Self {
            server_id: config.id.clone(),
            transport,
            lifecycle,
            server_info: None,
            server_capabilities: None,
            protocol_version: DEFAULT_PROTOCOL_VERSION.to_string(),
            permissions: config.permissions.clone(),
        };

        client.initialize().await?;
        Ok(client)
    }

    /// Performs the `initialize` handshake and sends `notifications/initialized`.
    pub async fn initialize(&mut self) -> Result<(), McpError> {
        self.lifecycle.transition_to_initializing()?;

        let init_params = serde_json::json!({
            "protocolVersion": self.protocol_version,
            "capabilities": ClientCapabilities::default(),
            "clientInfo": ClientInfo::default(),
        });

        let response = self
            .transport
            .send_request("initialize", init_params)
            .await
            .inspect_err(|e| {
                self.lifecycle.transition_to_failed(e.to_string());
            })?;

        let server_info: ServerInfo = response
            .get("serverInfo")
            .and_then(|si| serde_json::from_value(si.clone()).ok())
            .unwrap_or_else(|| ServerInfo {
                name: self.server_id.clone(),
                version: "unknown".to_string(),
            });

        let capabilities: ServerCapabilities = response
            .get("capabilities")
            .and_then(|sc| serde_json::from_value(sc.clone()).ok())
            .unwrap_or_default();

        if let Some(proto) = response.get("protocolVersion").and_then(|p| p.as_str()) {
            self.protocol_version = proto.to_string();
        }

        // Send notifications/initialized
        self.transport
            .send_notification("notifications/initialized", serde_json::json!({}))
            .await?;

        self.server_info = Some(server_info.clone());
        self.server_capabilities = Some(capabilities.clone());
        self.lifecycle
            .transition_to_ready(server_info, capabilities)?;

        info!(server = %self.server_id, "MCP connection initialized successfully");
        Ok(())
    }

    /// Discovers tools offered by the MCP server.
    pub async fn list_tools(&mut self) -> Result<Vec<McpTool>, McpError> {
        let response = self
            .transport
            .send_request("tools/list", serde_json::json!({}))
            .await?;

        let raw_tools = response
            .get("tools")
            .and_then(|t| t.as_array())
            .cloned()
            .unwrap_or_default();

        let mut tools = Vec::new();
        for item in raw_tools {
            let name = item
                .get("name")
                .and_then(|n| n.as_str())
                .unwrap_or_default()
                .to_string();
            let description = item
                .get("description")
                .and_then(|d| d.as_str())
                .map(ToString::to_string);
            let input_schema = item
                .get("inputSchema")
                .cloned()
                .unwrap_or_else(|| serde_json::json!({ "type": "object" }));

            let permission = classify_tool_permission(&name, description.as_deref().unwrap_or(""));

            tools.push(McpTool {
                server_id: self.server_id.clone(),
                name,
                description,
                input_schema,
                permission,
            });
        }

        Ok(tools)
    }

    /// Invokes a named tool after enforcing permission policies.
    pub async fn call_tool(
        &mut self,
        tool_name: &str,
        arguments: serde_json::Value,
    ) -> Result<McpToolResult, McpError> {
        // Enforce permission policies
        let permission = classify_tool_permission(tool_name, "");
        match permission {
            ToolPermission::FilesystemWrite if !self.permissions.allow_filesystem_write => {
                return Err(McpError::PermissionDenied(
                    tool_name.to_string(),
                    "Filesystem write operations are disabled by policy".to_string(),
                ));
            }
            ToolPermission::NetworkWrite if !self.permissions.allow_network_write => {
                return Err(McpError::PermissionDenied(
                    tool_name.to_string(),
                    "Network write operations are disabled by policy".to_string(),
                ));
            }
            ToolPermission::Sensitive if !self.permissions.allow_sensitive => {
                return Err(McpError::PermissionDenied(
                    tool_name.to_string(),
                    "Sensitive commands are disabled by policy".to_string(),
                ));
            }
            _ => {}
        }

        let params = serde_json::json!({
            "name": tool_name,
            "arguments": arguments,
        });

        let response = self.transport.send_request("tools/call", params).await?;

        let is_error = response
            .get("isError")
            .and_then(|e| e.as_bool())
            .unwrap_or(false);

        let content = if let Some(items) = response.get("content").and_then(|c| c.as_array()) {
            items
                .iter()
                .map(|item| McpContent {
                    content_type: item
                        .get("type")
                        .and_then(|t| t.as_str())
                        .unwrap_or("text")
                        .to_string(),
                    text: item
                        .get("text")
                        .and_then(|t| t.as_str())
                        .map(ToString::to_string),
                })
                .collect()
        } else {
            Vec::new()
        };

        Ok(McpToolResult { content, is_error })
    }

    /// Discovers resources when supported by the server.
    pub async fn list_resources(&mut self) -> Result<Vec<McpResource>, McpError> {
        let response = self
            .transport
            .send_request("resources/list", serde_json::json!({}))
            .await?;
        let raw_resources = response
            .get("resources")
            .and_then(|r| r.as_array())
            .cloned()
            .unwrap_or_default();

        let mut resources = Vec::new();
        for item in raw_resources {
            if let (Some(uri), Some(name)) = (
                item.get("uri").and_then(|u| u.as_str()),
                item.get("name").and_then(|n| n.as_str()),
            ) {
                resources.push(McpResource {
                    uri: uri.to_string(),
                    name: name.to_string(),
                    description: item
                        .get("description")
                        .and_then(|d| d.as_str())
                        .map(ToString::to_string),
                    mime_type: item
                        .get("mimeType")
                        .and_then(|m| m.as_str())
                        .map(ToString::to_string),
                });
            }
        }

        Ok(resources)
    }

    /// Discovers prompt templates when supported by the server.
    pub async fn list_prompts(&mut self) -> Result<Vec<McpPrompt>, McpError> {
        let response = self
            .transport
            .send_request("prompts/list", serde_json::json!({}))
            .await?;
        let raw_prompts = response
            .get("prompts")
            .and_then(|p| p.as_array())
            .cloned()
            .unwrap_or_default();

        let mut prompts = Vec::new();
        for item in raw_prompts {
            if let Some(name) = item.get("name").and_then(|n| n.as_str()) {
                prompts.push(McpPrompt {
                    name: name.to_string(),
                    description: item
                        .get("description")
                        .and_then(|d| d.as_str())
                        .map(ToString::to_string),
                    arguments: Vec::new(),
                });
            }
        }

        Ok(prompts)
    }

    /// Sends a protocol ping to verify connection health.
    pub async fn ping(&mut self) -> Result<(), McpError> {
        let _ = self
            .transport
            .send_request("ping", serde_json::json!({}))
            .await?;
        Ok(())
    }

    /// Closes the session and shuts down associated transport resources.
    pub async fn close(&mut self) -> Result<(), McpError> {
        self.lifecycle.transition_to_stopping()?;
        let res = self.transport.close().await;
        self.lifecycle.transition_to_disconnected()?;
        res
    }

    #[must_use]
    pub fn server_id(&self) -> &str {
        &self.server_id
    }

    #[must_use]
    pub fn server_info(&self) -> Option<&ServerInfo> {
        self.server_info.as_ref()
    }

    #[must_use]
    pub fn server_capabilities(&self) -> Option<&ServerCapabilities> {
        self.server_capabilities.as_ref()
    }

    #[must_use]
    pub fn lifecycle(&self) -> &ServerLifecycle {
        &self.lifecycle
    }
}
