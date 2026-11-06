# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

---

## Reporting a Vulnerability

The Antigravity Engineering Kit (AEK) team takes software security seriously. If you discover a vulnerability or potential security risk, please do not disclose it publicly in an open issue.

Instead, please send a responsible disclosure report to the maintainers at:
`security@antigravity.dev` (or via private GitHub Vulnerability Reporting).

Please include:
- A description of the vulnerability and its potential impact.
- Exact steps or minimal reproduction code to trigger the issue.
- Details regarding target operating system and environment.

We commit to acknowledging your report within 48 hours and providing a remediation timeline.

---

## Security Invariants in AEK

1. **Subprocess Isolation**: Commands are invoked directly via `std::process::Command` with typed arguments array. No invocation passes through `cmd.exe`, `powershell.exe`, or `/bin/sh` string evaluation, mitigating shell injection vulnerabilities.
2. **Atomic Writes**: Configuration files are written to unique temporary files and synced before atomic rename, ensuring zero corruption during unexpected process termination.
3. **Least Privilege**: AEK operates strictly within the invoking user's privilege boundary and does not require elevated permissions.
