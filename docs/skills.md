# AEK Specialized Agent Skills

The **Antigravity Engineering Kit (AEK)** provides 15 production-grade AI agent skills adhering to the open Agent Skills standard and Google Antigravity progressive disclosure architecture.

## 1. Progressive Disclosure Architecture

Each skill is encapsulated in a directory containing a `SKILL.md` runbook with standardized YAML frontmatter:

```yaml
---
name: rust-systems-engineer
description: Expert Rust systems engineer for memory safety, concurrency, async Tokio, and workspace architecture.
---
```

Under Google Antigravity, only the skill name and description are ingested during initial context initialization (minimizing system prompt overhead). The full instruction body is loaded dynamically into agent context only when the skill is activated.

## 2. Bundled Specialized Skills

| Skill Identifier | Category | Primary Focus |
|---|---|---|
| `rust-systems-engineer` | Systems | Memory safety, ownership, async Tokio, FFI, SIMD, Cargo workspaces |
| `go-backend-engineer` | Backend | High-throughput services, goroutines, channels, context propagation |
| `cpp-systems-engineer` | Systems | Modern C++20/23, RAII, zero-cost abstractions, CMake, sanitizers |
| `zig-engineer` | Systems | Explicit allocators, comptime metaprogramming, cross-compilation |
| `python-engineer` | Core | Idiomatic typing, async asyncio, packaging (uv/pyproject.toml), DSPy |
| `flutter-engineer` | Mobile | Flutter & Dart UI, BLoC/Riverpod, platform channels, automated testing |
| `ai-agent-engineer` | AI Infra | Agent architectures, MCP client/server, DSPy pipelines, tool calling |
| `robotics-ros2-engineer` | Robotics | ROS 2 (Humble/Jazzy), Nav2, TF2 transforms, URDF, Gazebo simulations |
| `operating-system-engineer` | OS | Linux kernel modules, bootloaders, paging, scheduling, QEMU/GDB |
| `devops-engineer` | Platform | Multi-stage Docker packaging, CI/CD, GitHub Actions, Linux deployment |
| `security-auditor` | Security | Vulnerability scanning, static analysis, threat modeling, secret hygiene |
| `database-architect` | Data | PostgreSQL, SQLite, schema migrations, query optimization, connection pools |
| `distributed-systems-engineer` | Distributed | Consensus algorithms, gRPC/Protobuf, event streaming, idempotent APIs |
| `prompt-engineering-specialist` | AI Infra | Prompt optimization, DSPy programmatic signatures, Promptfoo evaluation |
| `production-code-reviewer` | Quality | Production readiness, security review, asymptotic complexity, regressions |

## 3. CLI Management Commands

```bash
# List all bundled and discovered skills
aek skills list

# Output skills in machine-readable JSON format
aek skills list --json

# Show metadata and full instructions for a skill
aek skills show rust-systems-engineer

# Validate all skill bundles against YAML frontmatter and character limits
aek skills validate

# Project-aware skill recommendations (inspects manifests in directory)
aek skills recommend
aek skills recommend --path /path/to/project

# Install a skill into active agent skills directory
aek skills install rust-systems-engineer --dry-run
aek skills install rust-systems-engineer

# Remove an installed skill
aek skills remove rust-systems-engineer --dry-run
aek skills remove rust-systems-engineer
```
