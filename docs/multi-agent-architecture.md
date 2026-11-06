# AEK Phase 9 — Universal Multi-Agent Compatibility Framework

## Overview

The **Antigravity Engineering Kit (AEK)** provides an agent-independent, open-source AI developer toolkit. While initially optimized for **Google Gemini Antigravity**, AEK's architecture abstracts AI engineering capabilities into a universal compatibility layer that translates rules, skills, Model Context Protocol (MCP) server registries, prompt templates, and quality gates across multiple premier AI coding agents:

1. **Gemini Antigravity**
2. **Claude Code** (Anthropic)
3. **OpenCode** (Open-source agent)
4. **Cursor** (Anysphere)

## Architectural Design

The universal compatibility engine operates as a pipeline:

```
  ┌────────────────────────────────────────────────────────┐
  │              AEK Core Canonical Source                 │
  │ (Global Rules, Skills Runbooks, MCP Registry, Prompts) │
  └───────────────────────────┬────────────────────────────┘
                              │
                              ▼
  ┌────────────────────────────────────────────────────────┐
  │               Capability Resolution Engine             │
  │    (Queries AgentCapabilityRegistry for support)       │
  └───────────────────────────┬────────────────────────────┘
                              │
                              ▼
  ┌────────────────────────────────────────────────────────┐
  │              Agent-Specific Translators                │
  ├──────────────┬──────────────┬─────────────┬────────────┤
  │ Antigravity  │ Claude Code  │  OpenCode   │   Cursor   │
  │ .gemini/     │  CLAUDE.md   │  AGENTS.md  │ .cursor/   │
  │ rules & mcp  │  .claude/    │  .opencode/ │ rules/.mdc │
  └──────────────┴──────────────┴─────────────┴────────────┘
                              │
                              ▼
  ┌────────────────────────────────────────────────────────┐
  │         Multi-Agent Synchronization & Conflict         │
  │    (Ownership Tracking via .aek/managed-agents.toml)   │
  └────────────────────────────────────────────────────────┘
```

## Universal Agent Adapter Interface

The core trait [`AgentAdapter`](file:///crates/aek-agents/src/adapter.rs) defines uniform lifecycles:

- **Detection**: Non-intrusive discovery of binaries (`agy`, `claude`, `opencode`, `cursor`) and workspace configuration roots.
- **Capability Introspection**: Granular checks across 9 categories (`RULES`, `SKILLS`, `MCP`, `HOOKS`, `PROJECT_INSTRUCTIONS`, `GLOBAL_INSTRUCTIONS`, `PERMISSIONS`, `VALIDATION_WORKFLOWS`, `PROMPT_TEMPLATES`).
- **Translation**: Idiomatic formatting without lossy or dangerous modifications.
- **Safe Installation & Rollback**: Atomic writes accompanied by pre-modification backups and SHA256 ownership manifests.

