# Project Language Detection Engine

AEK features an automatic, recursive, and bounded project detection engine designed to identify software projects, multi-language monorepos, and nested microservices.

---

## Supported Project Markers

| Ecosystem | Primary Markers | Secondary Markers | Manifest Content Inspection |
|---|---|---|---|
| **Rust** | `Cargo.toml` | `[workspace]` | Parses package name from `[package] name = "..."` |
| **Go** | `go.mod` | `go.work` | Extracts module name from `module <name>` |
| **C/C++** | `CMakeLists.txt` | `meson.build`, `compile_commands.json` | Extracts project name from `project(<name>)` |
| **Zig** | `build.zig` | `build.zig.zon` | Reads `.name = "<name>"` from zon manifest |
| **Python** | `pyproject.toml` | `requirements.txt`, `Pipfile`, `uv.lock` | Parses `[project] name = "..."` |
| **JS/TS** | `package.json` | `tsconfig.json`, `pnpm-workspace.yaml` | Parses `"name"` field from JSON |
| **Flutter/Dart** | `pubspec.yaml` | `environment.sdk` | Verifies `flutter:` SDK dependency |
| **ROS 2** | `package.xml` | `colcon.meta` | Validates `<package format="...">`, `<buildtool_depend>ament_...` |

---

## Detection Invariants & Safety Controls

1. **Cycle Prevention**: Canonicalizes directory paths and tracks visited inodes/paths in a `HashSet<PathBuf>` to eliminate symlink loops.
2. **Ignored Directories**: Never recursively scans build caches or third-party dependencies:
   - `target`, `node_modules`, `.git`, `venv`, `.venv`, `vendor`, `build`, `.dart_tool`, `install`, `log`, `dist`, `.next`, `out`, `__pycache__`
3. **Bounded Depth**: Configurable maximum recursive scanning depth (default: `4`) to prevent unbounded filesystem exploration.
4. **False Positive Filtering**: Files named `package.xml` are inspected for ROS 2 schema headers and `ament` tags to prevent misclassifying non-ROS XML manifests.
5. **Monorepo Detection**: Recognizes root workspaces (`Cargo.toml` workspace, `go.work`, `pnpm-workspace.yaml`) and tracks child crates as nested subprojects.

---

## CLI Usage

```bash
# Detect projects in current working directory
aek language detect

# Scan a specific directory
aek language detect --path /path/to/monorepo

# Emit structured JSON for agent consumption
aek language detect --json
```

