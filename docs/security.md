# AEK Security Architecture & Invariants

The **Antigravity Engineering Kit (AEK)** enforces strict security invariants across all subsystems to protect developers and agent environments from arbitrary code execution, configuration tampering, and path traversal vulnerabilities.

## 1. Path Containment & Traversal Prevention

- **Identifier Sanitization**: Rule and skill identifiers are validated to contain strictly lowercase alphanumeric characters and hyphens (`[a-z0-9-]+`). Path navigation tokens (`..`, `/`, `\`) are strictly forbidden.
- **Canonical Boundary Verification**: When installing or removing rules and skills, AEK canonicalizes target paths and verifies that resolved filesystem targets remain strictly within the designated parent directory.
- **Symlink Attacks**: AEK avoids following unverified external symlinks when copying rule or skill files.

## 2. Atomic Configuration & Idempotency

- **Atomic Writes**: All configuration, rule, and skill modifications utilize temporary files and atomic rename operations (`tempfile` + platform replace). If an operation fails mid-flight, original files are never left in a corrupted or half-written state.
- **Safety Backups**: Overwriting actions automatically trigger a snapshot backup into AEK's verified backup store (`~/.gemini/aek/backups/`).

## 3. Subprocess Execution Security

- **No Shell Execution**: Subprocesses are executed via direct binary invocations using Rust's `std::process::Command` without shell wrappers (`sh -c` or `cmd /c`), eliminating shell injection vulnerabilities.
- **Strict Timeouts & Resource Caps**: Commands are bound to deterministic timeouts (defaulting to 30 seconds for background checks) to prevent denial-of-service hanging.
- **Secret Redaction**: Environment variables and CLI arguments containing common sensitive key names (`KEY`, `SECRET`, `TOKEN`, `PASSWORD`, `AUTH`) are redacted from logs and diagnostic traces.

## 4. Rule & Skill Sandboxing

- Rules and skills are stored as declarative Markdown runbooks. They cannot execute arbitrary shell code during discovery or indexing.
- Over-sized rules exceeding the 24 KB per file limit are rejected during validation to prevent LLM context buffer overflows and prompt truncation attacks.
