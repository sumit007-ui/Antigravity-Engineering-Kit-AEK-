# AEK Language Toolchains

The Antigravity Engineering Kit (AEK) provides a typed, cross-platform language toolchain engine that models compilers, SDKs, build systems, linters, formatters, and language servers across eight major ecosystems:

1. **Rust**: `cargo`, `rustc`, `rustup`, `rustfmt`, `clippy`, `rust-analyzer`, `cargo-nextest`, `bacon`, `sccache`
2. **Go**: `go`, `gopls`, `dlv`, `golangci-lint`
3. **C/C++**: `cmake`, `ninja`, `clang`, `clang++`, `gcc`, `g++`, `clangd`, `clang-tidy`, `gdb`, `lldb`
4. **Zig**: `zig`, `zls`
5. **Python**: `python` / `python3`, `uv`, `pip`, `ruff`, `pyright`, `pytest`
6. **JavaScript / TypeScript**: `node`, `npm`, `pnpm`, `yarn`, `tsc`, `eslint`, `prettier`, `vitest`
7. **Flutter / Dart**: `flutter`, `dart`
8. **ROS 2 (Robot Operating System)**: `colcon`, `ros2`, `rosdep`

---

## Tool Definition Schema

Tool definitions are declared using typed structures and validated via `schemas/language-toolchain.schema.json`.

```toml
id = "rust-cargo"
name = "Cargo"
language = "rust"
binary = "cargo"
version_args = ["--version"]
supported_os = ["windows", "linux", "macos"]
requirement = "required"
category = "package_manager"
doc_url = "https://doc.rust-lang.org/cargo/"
install_guidance = "Install via rustup from https://rustup.rs/"
```

### Requirement Classifications

- **Required**: Essential for compilation and execution. Absence flags an overall toolchain `FAIL`.
- **Recommended**: Highly beneficial for modern development (e.g. `uv` for Python, `rustup` for Rust).
- **Optional**: Auxiliary developer tooling (e.g. formatters, linters, debuggers, language servers).

---

## Tool Discovery Engine

The discovery engine probes the host system dynamically without assuming fixed installation paths:

1. **PATH Resolution**: Uses platform-native path splitting and `.exe` extension candidates on Windows.
2. **Safe Version Probing**: Executes binaries using argument arrays without shell interpolation with an enforced 4-second timeout.
3. **In-Memory Caching**: Thread-safe `RwLock` cache eliminates redundant subprocess invocations, minimizing memory usage and CPU cycles on 8 GB RAM developer machines.
4. **Sanitized Output**: Removes multi-line noise, ANSI escape codes, and extracts clean semantic version tokens.

---

## CLI Inspection Commands

```bash
# List all supported language ecosystems
aek language list

# Inspect all registered tools for a language
aek language tools rust
aek language tools python

# Run comprehensive environment diagnostics
aek language doctor
aek language doctor go
aek language doctor cpp

# Generate a safe installation plan for missing tools
aek language setup zig --dry-run
```

