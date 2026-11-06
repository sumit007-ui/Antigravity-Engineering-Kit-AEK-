# AEK Standard MCP Integrations

The **Antigravity Engineering Kit (AEK)** provides verified configuration adapters and presets for five widely used, maintained MCP servers.

## 1. Context7 (Upstash)

- **Identifier**: `context7`
- **Source**: `https://github.com/upstash/context7`
- **Transport**: `http` (`https://mcp.context7.com/mcp`)
- **Purpose**: Provides up-to-date documentation search and library references for modern developer stacks.
- **Authentication**: None required for standard usage.
- **Dependencies**: None (HTTP endpoint).

## 2. GitHub MCP Server

- **Identifier**: `github`
- **Source**: `@modelcontextprotocol/server-github`
- **Transport**: `stdio` (`npx -y @modelcontextprotocol/server-github`)
- **Purpose**: Access repository file contents, search issues, view pull requests, and query commit histories.
- **Authentication**: Requires `GITHUB_TOKEN` environment variable.
- **Dependencies**: `node`, `npx`.
- **Permissions**: Read-only by default; write operations (pushing commits, creating PRs) require explicit approval.

## 3. Playwright MCP Server

- **Identifier**: `playwright`
- **Source**: `@playwright/mcp@latest`
- **Transport**: `stdio` (`npx -y @playwright/mcp@latest --headless`)
- **Purpose**: Browser automation, DOM inspection, taking screenshots, and running end-to-end web workflows.
- **Authentication**: None required.
- **Dependencies**: `node`, `npx`.
- **Mode**: Runs in headless mode to protect developer environments.

## 4. Sequential Thinking Server

- **Identifier**: `sequential-thinking`
- **Source**: `@modelcontextprotocol/server-sequential-thinking`
- **Transport**: `stdio` (`npx -y @modelcontextprotocol/server-sequential-thinking`)
- **Purpose**: Dynamic multi-step structured reasoning, thought decomposition, and hypothesis revision.
- **Authentication**: None required.
- **Dependencies**: `node`, `npx`.

## 5. Controlled Filesystem MCP Server

- **Identifier**: `filesystem`
- **Source**: `@modelcontextprotocol/server-filesystem`
- **Transport**: `stdio` (`npx -y @modelcontextprotocol/server-filesystem <allowed_dir>`)
- **Purpose**: Controlled file reading and writing within designated workspace directories.
- **Authentication**: None required.
- **Dependencies**: `node`, `npx`.
- **Sandboxing**: Strictly restricted to designated directory paths (`./scratch` or configured root).

