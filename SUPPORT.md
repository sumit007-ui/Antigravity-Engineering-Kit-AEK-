# Getting Support for Antigravity Engineering Kit (AEK)

Thank you for using AEK! We are committed to providing reliable engineering infrastructure for AI coding workflows. Here is how you can get help and report issues.

---

## 1. Documentation & Troubleshooting

Before opening an issue, check the following resources:
- [Getting Started Guide](docs/getting-started.md)
- [Troubleshooting Runbook](docs/troubleshooting.md)
- [Frequently Asked Questions (FAQ)](docs/faq.md)
- [CLI Reference Guide](docs/cli-reference.md)
- [Multi-Agent Compatibility Guide](docs/agent-compatibility.md)

Run the built-in diagnostic doctor commands:
```bash
aek doctor            # Core system & runtime health
aek agent doctor      # Multi-agent configurations & tools
aek language doctor   # Compiler & toolchain detection
aek security doctor   # Security scanners & dependency auditors
aek mcp doctor        # Model Context Protocol servers
```

---

## 2. Asking Questions & Discussions

For general questions, architecture discussions, or best-practice questions:
- Use [GitHub Discussions](https://github.com/google-antigravity/aek/discussions).
- Check existing answered questions and RFC proposals.

---

## 3. Reporting Bugs & Compatibility Issues

If you encountered a reproducible bug or unsupported behavior:
- File a bug report via [GitHub Issues](https://github.com/google-antigravity/aek/issues/new/choose).
- Please provide the output of `aek doctor --json` and the steps to reproduce the issue.

---

## 4. Security Vulnerabilities

**Do NOT file public issues for security vulnerabilities.**

Follow the responsible disclosure policy described in [SECURITY.md](SECURITY.md) or submit via private GitHub Vulnerability Reporting.

