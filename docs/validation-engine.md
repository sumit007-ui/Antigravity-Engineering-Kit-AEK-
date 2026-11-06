# AEK Project Validation Engine

The AEK Validation Engine provides automated, project-aware, and reproducible validation pipelines across detected software projects.

---

## Validation Lifecycle Stages

1. **Environment**: Verifies compilers, runtimes, and build tools are present on system PATH.
2. **Formatting**: Checks source code compliance with canonical formatters (`cargo fmt`, `ruff format`, `prettier`).
3. **Static Analysis**: Audits code quality, common bugs, and security pitfalls (`clippy`, `ruff check`, `golangci-lint`, `eslint`).
4. **Type Checking**: Validates static type systems where applicable (`tsc --noEmit`, `pyright`).
5. **Compilation / Check**: Validates that syntax and types compile without linking issues (`cargo check`, `go build`, `cmake --build`).
6. **Unit Testing**: Runs test suites with timeouts and non-zero exit code capture (`cargo test`, `go test`, `pytest`, `flutter test`).
7. **Integration Testing**: Extended integration test execution when `--full` flag is provided.
8. **Build Verification**: Ensures final artifacts compile successfully.

---

## Security & Execution Policies

- **Argument Array Execution**: Commands are never passed to an intermediate shell interpreter (`sh -c` or `cmd.exe /c`), preventing command injection attacks.
- **Enforced Timeouts**: Each step defaults to 120 seconds (configurable) to prevent hung processes or deadlocks.
- **Dry Run Support**: The `--dry-run` flag parses manifests and prints planned execution pipelines without launching child processes.
- **Fail-Fast Option**: Can abort remaining steps immediately on first failure.
- **Untrusted Script Protection**: Does not execute arbitrary lifecycle scripts from repository manifests without explicit trusted policy.

---

## CLI Usage

```bash
# Validate all projects detected in current directory
aek validate

# Validate in dry-run mode (previews pipeline commands)
aek validate --dry-run

# Run validation with machine-readable JSON output
aek validate --json

# Run validation on a specific directory
aek validate --path ./services/core --dry-run

# Run language-specific validation pipeline
aek language validate rust
```

