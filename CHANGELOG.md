# Changelog

All notable changes to the Antigravity Engineering Kit (AEK) will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
