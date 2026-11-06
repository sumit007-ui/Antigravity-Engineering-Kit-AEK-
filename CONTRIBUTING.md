# Contributing to Antigravity Engineering Kit (AEK)

We welcome contributions from the open-source community! To maintain code quality, security, and reliability, all contributions must follow these guidelines.

---

## Engineering Standards

1. **Test-Driven Development (TDD)**:
   - Always write tests specifying behavior before implementing new features or bug fixes.
   - Run tests to confirm the expected failure (RED), implement the minimal clean solution (GREEN), and refactor (REFACTOR).
2. **Ruthless Precision**:
   - Zero compiler or clippy warnings (`-D warnings`).
   - Complete implementations only: no `TODO: implement later` placeholders.
   - Direct subprocess execution without shell wrappers to prevent command injection.
3. **Cross-Platform Compatibility**:
   - All code must build and pass tests on Windows, Linux, and macOS.
4. **Performance Targets**:
   - Design for machines with 8 GB RAM. Avoid unbounded memory allocations, long-running idle processes, or redundant I/O.

---

## Local Development Workflow

### 1. Verification Checklist

Before submitting a Pull Request, ensure that all of the following commands pass with exit code 0:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

### 2. Commit Guidelines

- Use conventional commits format: `feat:`, `fix:`, `docs:`, `refactor:`, `test:`, `chore:`.
- Reference relevant issue numbers where applicable.

---

## Pull Request Process

1. Fork the repository and create your branch from `main`.
2. Implement your changes following TDD.
3. Include or update documentation and architecture decision records where appropriate.
4. Verify that CI passes on all matrix platforms.
5. Submit your PR with a clear description of the problem solved.

