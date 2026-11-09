# Security Policy

## Supported Versions

Only the latest active minor release receives security updates.

| Version | Supported          |
| :---    | :---:              |
| 0.2.x   | :white_check_mark: |
| < 0.2.0 | :x:                |

---

## Reporting a Vulnerability

The Antigravity Engineering Kit (AEK) maintainers are committed to proactive security and responsible disclosure.

**Please DO NOT report security vulnerabilities through public GitHub issues.**

### Reporting Method:
1. **GitHub Private Vulnerability Reporting (Preferred)**:
   Navigate to the [Security tab](https://github.com/google-antigravity/aek/security/advisories) of the repository and click **"Report a vulnerability"**.
2. **Direct Maintainer Contact**:
   If private vulnerability reporting is unavailable, contact the project maintainers via encrypted communication or private repository inquiry.

### What to Include in Your Report:
- Detailed description of the vulnerability and its potential impact.
- Minimal, reproducible test case or steps to trigger the condition.
- Target platform details (OS, architecture, AEK version).
- Proof of concept (PoC) scripts or configuration files if applicable.

### Remediation Timeline:
- **Initial Acknowledgement**: Within 48 hours.
- **Triage & Reproduction**: Within 5 business days.
- **Fix & Advisory Release**: Coordinated disclosure with patch release and CVE identifier where appropriate.

---

## Security Invariants in AEK

1. **Subprocess Isolation**: All subprocess commands are executed with direct argument arrays (`std::process::Command::new(binary).args(&[...])`). No command is passed through shell interpreters (`cmd.exe`, `powershell.exe`, or `/bin/sh`), preventing command injection.
2. **Path Traversal Protection**: All user-provided or external relative paths are normalized and checked for directory traversal (`..`, absolute paths, drive prefixes) before filesystem operations.
3. **Atomic Writes**: Configurations and release artifacts are written to temporary files and flushed before atomic rename, preventing file corruption or race conditions.
4. **Least Privilege**: AEK operates strictly within the calling user's permissions and does not require administrator/root access.
5. **No Automatic Script Execution**: AEK never executes untrusted project scripts, build hooks, or remote binaries without explicit user consent.

---

## Known Limitations & Threat Model

- **Prompt Injection**: LLMs and AI coding agents interpret natural language prompts probabilistically. While AEK structures prompts and parses schemas strictly, AEK cannot guarantee immunity from adversarial in-context prompt injections in third-party LLM providers.
- **External Dependencies & Tools**: AEK orchestrates external compilers, package managers, and MCP servers. Security boundaries depend on the isolation and execution integrity of those external tools on the host system.
