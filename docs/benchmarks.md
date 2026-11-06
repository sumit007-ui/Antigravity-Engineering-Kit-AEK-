# AEK Performance Benchmarks Report — v0.1.0

This document records the empirical performance benchmarks executed for the **AEK v0.1.0** release.

---

## Benchmark Environment
- **Host OS**: Microsoft Windows 11 (x86_64)
- **Rust Toolchain**: `rustc 1.85.0+` (Edition 2024, release profile with LTO)
- **Hardware Target**: 8 GB RAM reference laptop profile (Intel Core i5)
- **Clock**: Monotonic high-resolution `std::time::Instant`

---

## Summary of Results

| Subsystem / Operation | Sample Size ($n$) | Min | Median | Mean | Max | Target SLA | Status |
| :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| **1. CLI Startup Latency (`aek --version`)** | 15 | 46.84 ms | **55.67 ms** | 67.37 ms | 106.72 ms | < 500 ms | **PASS** :white_check_mark: |
| **2. Config Parsing (`AekConfig` TOML)** | 50 | 290.7 µs | **301.8 µs** | 325.5 µs | 1.15 ms | < 5 ms | **PASS** :white_check_mark: |
| **3. Project Language Detection** | 30 | 4.31 ms | **6.24 ms** | 6.44 ms | 11.45 ms | < 50 ms | **PASS** :white_check_mark: |
| **4. Agent Discovery (`detect_all`)** | 30 | 29.28 ms | **33.96 ms** | 34.72 ms | 49.27 ms | < 100 ms | **PASS** :white_check_mark: |
| **5. Rules & Skills Loading** | 30 | 1.93 ms | **2.12 ms** | 7.54 ms | 160.25 ms | < 50 ms | **PASS** :white_check_mark: |
| **6. Validation DAG Planning** | 30 | 0.15 ms | **0.15 ms** | 0.16 ms | 0.25 ms | < 10 ms | **PASS** :white_check_mark: |
| **7. MCP Integrations Resolution** | 50 | 17.4 µs | **17.8 µs** | 19.4 µs | 94.6 µs | < 1 ms | **PASS** :white_check_mark: |
| **8. Idle Memory Footprint** | N/A | — | **< 25 MB** | — | — | < 50 MB | **PASS** :white_check_mark: |

---

## Methodology & Analysis

1. **Transient Lifecycle**: AEK avoids long-running background daemon processes. It initializes, executes requested operations, and terminates immediately.
2. **Cold vs. Warm Latencies**:
   - First invocations on Windows incur small process-creation and dynamic linking costs (45–100 ms).
   - In-process operations (AST parsing, TOML deserialization, MCP registry lookups) execute in microseconds.
3. **RAM Optimization**:
   - Zero unbounded buffers; memory allocations scale strictly with workspace size.
   - Operating memory remains well under 25 MB during standard diagnostics and rule/skill discovery.

