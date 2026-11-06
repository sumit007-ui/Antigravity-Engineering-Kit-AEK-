//! Tool, resource, and prompt discovery representations.

use crate::security::ToolPermission;
use serde::{Deserialize, Serialize};

/// Discovered MCP tool definition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct McpTool {
    /// Identifier of the server providing this tool.
    pub server_id: String,
    /// Protocol tool name (as declared by the server).
    pub name: String,
    /// Optional tool description.
    pub description: Option<String>,
    /// JSON Schema describing expected arguments.
    pub input_schema: serde_json::Value,
    /// Inferred or assigned risk permission level.
    pub permission: ToolPermission,
}

impl McpTool {
    /// Qualified unique identifier avoiding cross-server naming collisions (`server_id/tool_name`).
    #[must_use]
    pub fn qualified_name(&self) -> String {
        format!("{}/{}", self.server_id, self.name)
    }
}

/// Output content item returned from a tool invocation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct McpContent {
    #[serde(rename = "type")]
    pub content_type: String,
    #[serde(default)]
    pub text: Option<String>,
}

/// Structured outcome of a tool execution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct McpToolResult {
    pub content: Vec<McpContent>,
    #[serde(default)]
    pub is_error: bool,
}

impl McpToolResult {
    /// Extracts combined text output from content items.
    #[must_use]
    pub fn text(&self) -> String {
        self.content
            .iter()
            .filter_map(|c| c.text.as_deref())
            .collect::<Vec<_>>()
            .join("\n")
    }
}

/// Discovered MCP resource.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct McpResource {
    pub uri: String,
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub mime_type: Option<String>,
}

/// Argument expected by an MCP prompt template.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct McpPromptArgument {
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub required: bool,
}

/// Discovered MCP prompt template.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct McpPrompt {
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub arguments: Vec<McpPromptArgument>,
}

