# AEK v0.1.0 Open-Source Release Checklist

Use this checklist to verify every release gate before publishing a public release of Antigravity Engineering Kit (AEK).

---

## 1. Pre-Release Verification

- [x] Repository audit completed across all 16 crates.
- [x] Feature scope verified and classified in `docs/release-scope.md`.
- [x] Rust workspace builds with zero warnings (`cargo build --workspace`).
- [x] Formatting passes with exit code 0 (`cargo fmt --all -- --check`).
- [x] Clippy passes with zero warnings on all targets (`cargo clippy --workspace --all-targets -- -D warnings`).
- [x] Complete unit test suite passes with exit code 0 (`cargo test --workspace`).
- [x] Integration tests pass across all subcommands (`agent_cli_tests`, `cli_tests`, `init_tests`, etc.).
- [x] Security checks pass (`test_security_guards_path_traversal`, `test_security_injection_and_path_traversal_guards`).
- [x] Windows release build and installer tested (`packaging/package-windows.ps1`, `installers/windows/test-install.ps1`).
- [x] Linux release build and installer tested (`packaging/package-linux.sh`, `installers/linux/test-install.sh`).

---

## 2. Lifecycle & Runtime Validation

- [x] Independent user-space installation tested outside source tree.
- [x] In-place update mechanism tested (`aek update check`, `aek update --dry-run`).
- [x] Rollback snapshots verified (`aek rollback --list`).
- [x] Safe uninstallation verified (`aek uninstall --dry-run`).
- [x] Diagnostic doctors pass (`aek doctor`, `aek agent doctor`, `aek language doctor`, `aek security doctor`).

---

## 3. Documentation & Community Health

- [x] `README.md` complete, evidence-based, and polished.
- [x] `docs/architecture.md` matches actual crate structure with Mermaid diagrams.
- [x] `docs/getting-started.md` provides accurate quickstart commands.
- [x] `docs/cli-reference.md` reflects all implemented commands.
- [x] `docs/agent-compatibility.md` details Google Antigravity, Claude Code, Cursor, and OpenCode.
- [x] Dual-license approved: `LICENSE` (Apache-2.0 OR MIT), `LICENSE-APACHE`, `LICENSE-MIT`.
- [x] Contribution guidelines complete: `CONTRIBUTING.md`, `CODE_OF_CONDUCT.md`.
- [x] Security policy complete: `SECURITY.md`.
- [x] Community support document complete: `SUPPORT.md`.
- [x] GitHub issue & PR templates in place (`.github/ISSUE_TEMPLATE/*`, `.github/pull_request_template.md`).
- [x] Dependabot configuration in place (`.github/dependabot.yml`).
- [x] Code ownership defined (`.github/CODEOWNERS`).

---

## 4. Release Packaging & Security

- [x] Release tag format validated: `v0.1.0`.
- [x] `CHANGELOG.md` updated for `v0.1.0`.
- [x] Release notes prepared in `docs/release-notes-v0.1.0.md`.
- [x] Checksum calculation verified (SHA-256).
- [x] Release manifest schema adheres to `schemas/`.
- [x] No embedded secrets, tokens, or private paths in source code or committed files.
- [x] GitHub Actions CI green on `ubuntu-latest`, `windows-latest`, and `macos-latest`.

