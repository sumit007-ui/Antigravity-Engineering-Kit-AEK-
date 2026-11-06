# AEK MCP Architecture & Engine

The **Antigravity Engineering Kit (AEK)** provides an agent-independent Model Context Protocol (MCP) client engine, typed transport abstraction, server registry, and lifecycle management framework.

```
+-------------------------------------------------------------+
|                        AEK CLI                              |
|   (aek mcp list, show, doctor, tools, export, call, ...)    |
+-------------------------------------------------------------+
                               |
                               v
+-------------------------------------------------------------+
|                       aek-mcp                               |
|  +-------------------------------------------------------+  |
|  |                     McpClient                         |  |
|  | - Handshake (initialize / notifications/initialized)   |  |
|  | - Capability Negotiation (tools, resources, prompts)  |  |
|  | - Tool Execution & Structured Result Parsing          |  |
|  +-------------------------------------------------------+  |
|  |                  McpServerRegistry                    |  |
|  | - TOML Persistence, Dynamic Discovery, Management     |  |
|  +-------------------------------------------------------+  |
|  |                   ServerLifecycle                     |  |
|  | - DISCONNECTED -> CONNECTING -> INITIALIZING -> READY  |  |
|  | - DEGRADED / FAILED / STOPPING                        |  |
|  +-------------------------------------------------------+  |
|  |                  Security & Policy                    |  |
|  | - Tool Risk Classification & Permission Profiles      |  |
|  | - Secret Redaction & Path Containment Validation      |  |
|  +-------------------------------------------------------+  |
|  |                   McpTransport                        |  |
|  |  +---------------------+   +-----------------------+  |  |
|  |  |   StdioTransport    |   |     HttpTransport     |  |  |
|  |  | (Tokio Child Async) |   | (Reqwest + Rustls TLS)|  |  |
|  |  +---------------------+   +-----------------------+  |  |
+-------------------------------------------------------------+
       |                                             |
       v (Stdio pipes)                               v (HTTPS / SSE)
+-----------------------+                  +-------------------+
|  Local MCP Subprocess |                  | Remote MCP Server |
|  (GitHub, Playwright, |                  |    (Context7)     |
|   SequentialThinking) |                  +-------------------+
+-----------------------+
```

## 1. Core Components

### 1.1 McpClient
Manages active JSON-RPC 2.0 protocol sessions:
- Implements the official MCP protocol handshake (`protocolVersion: 2024-11-05`).
- Dynamically discovers server capabilities (`tools`, `resources`, `prompts`).
- Performs tool listing (`tools/list`) and invocation (`tools/call`).
- Ensures clean session termination via `close()`.

### 1.2 Transports
- **`StdioTransport`**: Spawns local child processes via `tokio::process::Command` without shell wrappers. Handles newline-delimited JSON-RPC, captures stderr diagnostics asynchronously, and enforces strict execution timeouts.
- **`HttpTransport`**: Manages remote endpoints over HTTP/HTTPS with verified TLS certificates and timeouts via `reqwest`.

### 1.3 Lifecycle State Machine
```
DISCONNECTED ──> CONNECTING ──> INITIALIZING ──> READY
     ^               │                │            │
     │               v                v            v
  STOPPING <────── FAILED <─────── DEGRADED <──────+
```

### 1.4 Agent Adapters
The MCP client engine remains independent of any specific AI coding agent. The Google Antigravity adapter in `aek-agents` translates AEK server configurations into Antigravity's native `mcp_config.json` schema (`mcpServers`), preserving existing user configurations with automatic safety backups.
