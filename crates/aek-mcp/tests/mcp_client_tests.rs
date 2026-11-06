// Unit and integration tests for AEK MCP Client and Lifecycle

use aek_mcp::capabilities::{ServerCapabilities, ServerInfo};
use aek_mcp::discovery::McpTool;
use aek_mcp::lifecycle::ServerLifecycle;
use aek_mcp::security::ToolPermission;

#[test]
fn test_server_lifecycle_transitions() {
    let mut lc = ServerLifecycle::new("test-server");
    assert!(lc.is_disconnected());

    assert!(lc.transition_to_connecting().is_ok());
    assert!(lc.is_connecting());

    assert!(lc.transition_to_initializing().is_ok());
    assert!(lc.is_initializing());

    let server_info = ServerInfo {
        name: "mock-server".to_string(),
        version: "1.0.0".to_string(),
    };
    let caps = ServerCapabilities {
        tools: Some(serde_json::json!({ "listChanged": true })),
        resources: None,
        prompts: None,
    };

    assert!(lc.transition_to_ready(server_info, caps).is_ok());
    assert!(lc.is_ready());

    assert!(lc.transition_to_degraded("High latency").is_ok());
    assert!(lc.is_degraded());

    assert!(lc.transition_to_stopping().is_ok());
    assert!(lc.transition_to_disconnected().is_ok());
    assert!(lc.is_disconnected());
}

#[test]
fn test_invalid_lifecycle_transition() {
    let mut lc = ServerLifecycle::new("test-server");
    // Cannot jump directly from Disconnected to Ready
    let err = lc.transition_to_ready(
        ServerInfo {
            name: "x".into(),
            version: "1".into(),
        },
        ServerCapabilities::default(),
    );
    assert!(err.is_err());
}

#[test]
fn test_mcp_tool_qualified_name_and_classification() {
    let tool = McpTool {
        server_id: "context7".to_string(),
        name: "query_docs".to_string(),
        description: Some("Fetch library documentation".to_string()),
        input_schema: serde_json::json!({
            "type": "object",
            "properties": {
                "library": { "type": "string" },
                "query": { "type": "string" }
            },
            "required": ["library", "query"]
        }),
        permission: ToolPermission::ReadOnly,
    };

    assert_eq!(tool.qualified_name(), "context7/query_docs");
    assert_eq!(tool.permission, ToolPermission::ReadOnly);
}
