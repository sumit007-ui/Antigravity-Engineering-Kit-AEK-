# AEK MCP Security Architecture & Permission Policies

Security is enforced at multiple layers within the **Antigravity Engineering Kit (AEK)** MCP framework to prevent prompt-injection escalation, unauthorized filesystem access, and credential leakage.

## 1. Tool Risk Classification

All discovered tools are categorized into deterministic permission tiers based on their identifier semantics and descriptions:

| Permission Level | Description | Default Policy |
|---|---|---|
| `READ_ONLY` | Querying documentation, inspecting status, read-only searches | Allowed |
| `FILESYSTEM_WRITE` | Creating, editing, or deleting local files | Denied unless explicitly permitted |
| `NETWORK_WRITE` | Mutating remote repositories (pushing commits, merging PRs) | Denied unless explicitly permitted |
| `SENSITIVE` | Executing terminal commands, shell scripts, or eval | Denied |
| `UNKNOWN` | Unrecognized or ambiguous tool signatures | Handled conservatively (Denied) |

### Permission Profiles
Each server configuration defines a `PermissionProfile`:
```toml
[permissions]
allow_read_only = true
allow_filesystem_write = false
allow_network_write = false
allow_sensitive = false
allowed_paths = ["./scratch"]
```

## 2. Path Containment & Filesystem Sandboxing

For filesystem servers (such as `@modelcontextprotocol/server-filesystem`):
- **No Unrestricted Root Access**: AEK forbids configuring unrestricted root access by default.
- **Traversal Defense**: Requests containing path traversal sequences (`..`) are rejected before transmission.
- **Canonical Boundary Verification**: Target paths are canonicalized and verified to reside strictly within configured `allowed_paths`.

## 3. Credential Hygiene & Secret Redaction

- **Environment References**: Authentication secrets use environment-variable references (e.g. `${GITHUB_TOKEN}`) instead of plaintext storage in configuration files.
- **Log Sanitization**: Bearer tokens, GitHub Personal Access Tokens (`ghp_*`), passwords, and API keys are automatically masked as `[REDACTED]` in diagnostic traces and stderr capture.
- **No Shared Credentials**: Server credential scopes are strictly isolated from one another.

## 4. Subprocess Isolation

- Local stdio servers are spawned with explicit argument arrays, preventing shell interpolation attacks (`sh -c` or `cmd /c`).
- On Windows, child processes are spawned with `CREATE_NO_WINDOW` to prevent visual disruption and terminal hijacking.
- Operations enforce hard timeout bounds to prevent hanging or denial-of-service states.

