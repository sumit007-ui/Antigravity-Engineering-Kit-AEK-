# Authoring AEK Agent Skills

This guide details how to author new agent skills for **Antigravity Engineering Kit (AEK)** compatible with Google Antigravity and the open Agent Skills specification.

## 1. Skill Bundle Structure

A valid skill bundle is a directory containing a primary `SKILL.md` file and optional auxiliary resources:

```
my-custom-skill/
├── SKILL.md                  # Required primary instruction runbook
├── scripts/                  # Optional helper scripts executed by agents
│   └── analyze.py
├── references/               # Optional reference documentation
│   └── api-spec.md
└── examples/                 # Optional reference code and fixtures
    └── sample.rs
```

## 2. YAML Frontmatter Specification

Every `SKILL.md` must begin with a YAML frontmatter block enclosed by `---`:

```markdown
---
name: my-custom-skill
description: Comprehensive description in third-person detailing capabilities and activation context.
version: 1.0.0
---

# My Custom Skill Runbook

## Overview
...
```

### Constraints:
1. `name`: Must be lowercase alphanumeric characters separated by hyphens (e.g. `rust-systems-engineer`). No spaces or special characters.
2. `description`: Must be informative, concise, and at least 10 characters long.
3. `version`: Standard semantic versioning (e.g. `1.0.0`).

## 3. Instruction Body Best Practices

- **Relentless Precision**: Detail exact steps, commands, and expected outputs.
- **Verification Criteria**: Provide automated checks and test instructions so the agent can verify work before concluding.
- **Defensive Engineering**: Document error cases, security boundaries, and environment limitations.

## 4. Validating Your Skill

Validate your new skill using the AEK CLI:

```bash
aek skills validate --path ./path/to/my-custom-skill
```

