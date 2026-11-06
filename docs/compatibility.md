# Toolchain & Ecosystem Compatibility

AEK provides deterministic compatibility analysis across target platforms, compilers, SDKs, and project requirements.

---

## Compatibility Levels

- **`COMPATIBLE`**: All required toolchains are installed, versions meet project specifications, and target OS constraints are satisfied.
- **`WARNING`**: Non-critical dependencies are missing or secondary toolchains require attention (e.g. `CMake` present but compiler relies on Visual Studio Developer Command Prompt).
- **`INCOMPATIBLE`**: Required compilers or build systems are absent, or host operating system cannot satisfy the ecosystem requirements.
- **`UNKNOWN`**: Insufficient metadata available to determine status.

---

## Operating System Matrix

| Language | Windows | Linux | macOS | Notes |
|---|---|---|---|---|
| **Rust** | Full | Full | Full | `rustup` handles MSVC and GNU targets |
| **Go** | Full | Full | Full | Go toolchain fully self-contained |
| **C/C++** | MSVC / Clang | GCC / Clang | Apple Clang | Requires Visual Studio tools or LLVM on Windows |
| **Zig** | Full | Full | Full | Single self-contained binary across platforms |
| **Python** | Full | Full | Full | Python 3.10+ recommended |
| **JS/TS** | Full | Full | Full | Node.js LTS recommended |
| **Flutter** | Full | Full | Full | Desktop UI requires platform SDKs |
| **ROS 2** | Partial | Full (Tier 1) | Experimental | Windows requires manual binary archives or Docker |

---

## Remediation Engine

When incompatibility is detected, AEK produces actionable, verified remediation steps without downloading or executing untrusted remote scripts:

```bash
aek language compatibility rust
aek language compatibility cpp --path ./native-engine
```

