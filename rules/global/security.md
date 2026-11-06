# Global Engineering Principle: Security by Default

## Objective
Prevent vulnerabilities, command injections, credential leaks, and privilege escalations in all generated software.

## Security Invariants
1. **Zero Command Injection**: Never execute user input through shell evaluators (`sh -c`, `cmd.exe`). Always use argument arrays via direct process spawning.
2. **Least Privilege**: Operate strictly within unprivileged user boundaries. Never demand root/admin access for routine operations.
3. **Path Traversal Protection**: Validate all user-supplied paths with strict normalization and canonicalization to guarantee containment within authorized root directories.
4. **Secret Sanitization**: Never store or output plaintext tokens, private keys, or passwords. Redact credentials in logs and reports.
5. **Atomic File Persistence**: Never perform non-atomic updates on critical configuration files. Write to temporary files first and atomically replace targets.
