# Changelog

All notable changes to the Antigravity Engineering Kit (AEK) will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2026-11-06

### Added
- Phase 2 Rust Core Engine & CLI Framework:
  - New `aek-platform` crate providing cross-platform OS & architecture detection, standard directory resolution, and path traversal security invariants.
  - New `SafeCommand` subprocess execution engine in `aek-core` with argument-array execution, secret credential redaction, kill-on-drop timeout cancellation, and dry-run simulation.
  - Three resource profiles: `LIGHT`, `ENGINEERING`, and `FULL` with profile descriptors, CLI commands (`aek profile list`, `aek profile show`, `aek profile apply`), and preservation of unrelated user configuration settings.
  - Advanced configuration models with strict schema constraints, unknown-field rejection, and atomic temporary writes.
  - Crash-resilient backup & restore engine (`aek backup`, `aek backup list`, `aek restore <id>`) with alphanumeric identifier validation, path containment verification, and automatic safety backups before destructive operations.
  - Idempotent `aek init` supporting `--dry-run` and `--force` (with automatic pre-init safety backup).
  - Enhanced `aek doctor` reporting standard `PASS`, `WARN`, `FAIL`, and `SKIP` diagnostic outcomes, with reliable Antigravity detection without false positives from empty directories.
  - Standard JSON schema `schemas/aek-config.schema.json` and profile template definitions in `config/`.
  - Comprehensive documentation suite: `docs/architecture.md`, `docs/configuration.md`, and `docs/cli-reference.md`.
  - Comprehensive unit, integration, and security test suites.

## [0.1.0] - 2026-11-06

### Added
- Phase 1 Foundation:
  - Cargo workspace structure targeting Rust 2024 edition.
  - `aek-core` crate providing core error types, system execution abstractions, and `DoctorRunner` diagnostics.
  - `aek-config` crate supporting TOML configuration, strict constraint validation, and atomic filesystem persistence.
  - `aek-cli` crate implementing `aek --version`, `aek doctor` (human and JSON output modes), and `aek config validate`/`aek config init`.
  - Comprehensive unit and integration test suites covering doctor diagnostics, config serialization, and CLI invocations.
  - Cross-platform GitHub Actions CI workflow for Linux, Windows, and macOS.
  - Architecture Decision Record (ADR 0001) documenting crate boundaries and design decisions.
