# AEK MCP Troubleshooting Guide

Diagnostic procedures and solutions for common Model Context Protocol (MCP) server issues.

## 1. Diagnostic Health Check (`aek mcp doctor`)

Run `aek mcp doctor` to audit all registered servers against toolchains and credentials:

```bash
aek mcp doctor
aek mcp doctor --server github
```

Output indicators:
- `[PASS]`: Component or prerequisite verified and ready.
- `[WARN]`: Non-critical warning (e.g. missing optional environment variable or insecure HTTP endpoint).
- `[FAIL]`: Critical error preventing server launch (e.g. missing `npx` executable).

---

## 2. Common Issues & Solutions

### Missing Executable (`npx` or `node`)
- **Symptom**: `aek mcp doctor` reports `[FAIL] Command 'npx' not found on PATH`.
- **Cause**: Node.js is not installed or not exposed in the current system PATH.
- **Fix**: Install Node.js (>=18.0) and verify with `node -v` and `npx -v`.

### Missing Credentials for GitHub (`GITHUB_TOKEN`)
- **Symptom**: `aek mcp doctor --server github` reports `[WARN] Missing auth environment variables: GITHUB_TOKEN`.
- **Fix**: Export a GitHub Personal Access Token in your environment:
  ```bash
  # Linux / macOS
  export GITHUB_TOKEN="ghp_yourTokenHere"

  # Windows PowerShell
  $env:GITHUB_TOKEN="ghp_yourTokenHere"
  ```

### Tool Invocation Timed Out
- **Symptom**: `Operation timed out after 30 seconds`.
- **Cause**: The server is stuck downloading npm packages or waiting on interactive input.
- **Fix**: Pre-cache the npm package using `npx -y @modelcontextprotocol/server-...` manually once, or increase `timeout_seconds` in the server configuration.

### Antigravity Configuration Not Updating
- **Symptom**: Changes in AEK do not appear in Google Antigravity.
- **Fix**: Export configurations explicitly into Antigravity:
  ```bash
  aek mcp export --agent antigravity
  ```
  Verify that `~/.gemini/config/mcp_config.json` contains the updated `mcpServers` object.
