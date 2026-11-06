# Project Rule: Rust Systems Programming

## Standards & Toolchains
- **Edition**: Rust 2024 edition where supported (minimum Rust 1.85.0).
- **Tooling**: `cargo fmt`, `cargo clippy -- -D warnings`, `cargo test`.
- **Memory Safety**: Reject raw pointers and `unsafe` blocks unless encapsulated behind sound, formally-documented safe abstractions.
- **Error Handling**: Use `thiserror` for typed library errors; use `anyhow` for top-level application error reporting.
- **Asynchronous Execution**: Leverage Tokio with cancellation safety, bounded channels, and timeout enforcement.

