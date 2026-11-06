# AEK Project Validation Engine

The AEK Validation Engine provides automated, project-aware, reproducible validation pipelines across detected software projects and monorepos.

---

## Architecture Overview

```
                      ┌────────────────────────────┐
                      │    Validation Planner      │
                      │  (Dependency Graph & DAG)  │
                      └─────────────┬──────────────┘
                                    │
                                    ▼
                      ┌────────────────────────────┐
                      │   Validation Orchestrator  │
                      │  (Concurrency-bounded Run) │
                      └─────────────┬──────────────┘
                                    │
         ┌──────────────────────────┼──────────────────────────┐
         ▼                          ▼                          ▼
┌──────────────────┐       ┌──────────────────┐       ┌──────────────────┐
│  Language Stage  │       │  Language Stage  │       │  Security Scan   │
│  (fmt, clippy,   │       │  (test, build,   │       │  (Semgrep,       │
│   check)         │       │   typecheck)     │       │   Trivy, audit)  │
└────────┬─────────┘       └────────┬─────────┘       └────────┬─────────┘
         │                          │                          │
         └──────────────────────────┼──────────────────────────┘
                                    │
                                    ▼
                      ┌────────────────────────────┐
                      │    Quality Gate Engine     │
                      │ (Policy Threshold Checks)  │
                      └─────────────┬──────────────┘
                                    │
                                    ▼
                      ┌────────────────────────────┐
                      │  Multi-Format Reporters    │
                      │ (Terminal, JSON, SARIF, MD)│
                      └────────────────────────────┘
```

---

## Validation Lifecycle Stages

1. **Environment**: Verifies compilers, runtimes, and build tools are present on system PATH.
2. **Formatting**: Checks source code compliance with canonical formatters (`cargo fmt`, `ruff format`, `prettier`).
3. **Static Analysis**: Audits code quality, common bugs, and security pitfalls (`clippy`, `ruff check`, `golangci-lint`, `eslint`).
4. **Type Checking**: Validates static type systems where applicable (`tsc --noEmit`, `pyright`).
5. **Compilation / Check**: Validates that syntax and types compile without linking issues (`cargo check`, `go build`, `cmake --build`).
6. **Unit Testing**: Runs test suites with timeouts and non-zero exit code capture (`cargo test`, `go test`, `pytest`, `flutter test`).
7. **Integration Testing**: Extended integration test execution when `--full` flag or full validation policy is selected.
8. **Build Verification**: Ensures final artifacts compile successfully.
9. **Security Audit**: Scans dependencies and code via Semgrep, Trivy, and ecosystem native auditors.

---

## Incremental Validation (Git-Aware)

When running with `--changed` (or `--base <REF>`), the engine queries Git for modified, untracked, or committed differences:
- Only projects containing modified files are included in the execution plan.
- Conservative root fallbacks: If root build files (e.g. root `Cargo.toml`, root `package.json`) change, all workspace members are validated.
- Non-git environments gracefully fall back to full repository validation.

---

## Policy & Quality Gates

Validation execution is governed by TOML-based policies (`default.toml`, `strict.toml`, `ci.toml`):
- **Fail Fast**: Abort remaining steps immediately upon encountering a failed step.
- **Stage Filtering**: Selectively skip or include stages (e.g. skip integration tests in dev mode).
- **Concurrency Control**: Bounded parallel project execution (`max_concurrency`).
- **Security Thresholds**: Enforce zero-tolerance for critical or high severity vulnerabilities.

---

## CLI Usage

```bash
# Validate all projects detected in current directory
aek validate

# Validate only changed projects compared to Git base
aek validate --changed --base origin/main

# Dry-run validation (inspect planned execution DAG without running)
aek validate --dry-run

# Run with strict quality policy
aek validate --policy strict

# Output versioned JSON or SARIF report
aek validate --json > validation-report.json

# Manage policies
aek validate policy list
aek validate policy show strict
aek validate policy check config/validation/default.toml
```
