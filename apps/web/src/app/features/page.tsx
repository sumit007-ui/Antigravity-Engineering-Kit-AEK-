import Link from "next/link";
import Domain3DIcon, { DomainIconType } from "@/components/three/Domain3DIcon";
import TiltCard from "@/components/motion/TiltCard";

const FEATURE_SECTIONS: {
  id: string;
  type: DomainIconType;
  title: string;
  category: string;
  tagline: string;
  description: string;
  cliCommand: string;
  details: string[];
}[] = [
  {
    id: "rules",
    type: "rules",
    title: "Global & Project Engineering Rules",
    category: "STANDARDIZED PROTOCOLS",
    tagline: "Deterministic 3-tier precedence with token-budget enforcement.",
    description:
      "AEK resolves rule precedence across Global (~/.gemini or ~/.claude), Workspace, and Project tiers. Every rule file is audited against strict 24 KB limits and estimated token budgets to prevent LLM context saturation.",
    cliCommand: "aek rules list --audit",
    details: [
      "Strict precedence resolution: Global → Workspace → Project",
      "24 KB bounded rule size defense against context degradation",
      "Automated prompt-token budget estimation",
      "Idempotent installation with backup rotation",
    ],
  },
  {
    id: "skills",
    type: "skills",
    title: "15 Specialized Agent Skills",
    category: "PROGRESSIVE DISCLOSURE",
    tagline: "Curated engineering runbooks with automatic codebase detection.",
    description:
      "Progressive disclosure skills covering Rust, Go, C++, Zig, Python, TypeScript, Flutter, and ROS 2 robotics. Codebase manifests (Cargo.toml, package.json, go.mod) trigger targeted skill recommendations.",
    cliCommand: "aek skills discover",
    details: [
      "Progressive disclosure runbooks with clear operational bounds",
      "Project-aware discovery engine with confidence scoring",
      "Comprehensive domain coverage across 8 language ecosystems",
      "Native translation to Antigravity, Claude Code, Cursor, and OpenCode",
    ],
  },
  {
    id: "mcp",
    type: "mcp",
    title: "Model Context Protocol Client & Integrations",
    category: "TOOL ORCHESTRATION",
    tagline: "High-throughput JSON-RPC 2.0 client with Stdio and HTTP transports.",
    description:
      "Connect coding agents securely to real-world developer tools. Out-of-the-box verified integration adapters for Context7, GitHub, Playwright, Sequential Thinking, and Filesystem.",
    cliCommand: "aek mcp list --json",
    details: [
      "Complete JSON-RPC 2.0 client supporting Stdio and HTTP transports",
      "Subprocess supervision with automatic health restart",
      "Least-privilege permission tagging per MCP tool",
      "Instant multi-agent configuration export",
    ],
  },
  {
    id: "languages",
    type: "languages",
    title: "Language Toolchain Management",
    category: "ENVIRONMENT PREPAREDNESS",
    tagline: "Automated compiler, linter, and language server diagnostics.",
    description:
      "Inspect and verify language compilers, package managers, and formatters across 8 ecosystems. Diagnostic doctor provides non-destructive setup instructions if tools are missing.",
    cliCommand: "aek language doctor",
    details: [
      "Automatic project language detection across 8 ecosystems",
      "Compiler and linter health checks with actionable doctor output",
      "Toolchain version parsing and compatibility auditing",
      "Zero invasive installation without user confirmation",
    ],
  },
  {
    id: "validation",
    type: "validation",
    title: "8-Stage Quality Gate Verification",
    category: "DETERMINISTIC GATES",
    tagline: "Topological DAG verification pipeline with incremental Git detection.",
    description:
      "Automate code formatting, static linting, type-checking, unit tests, and security scans in an 8-stage verification pipeline. Fast incremental mode tests only files modified in git diff.",
    cliCommand: "aek validate --strict",
    details: [
      "Topological DAG execution planner with parallel stages",
      "Incremental Git change detection for sub-second execution",
      "Configurable quality gate thresholds and fail-fast flags",
      "OASIS SARIF v2.1.0, JSON, and GitHub Markdown job summaries",
    ],
  },
  {
    id: "security",
    type: "security",
    title: "Multi-Scanner Security Orchestration",
    category: "APPLICATION SECURITY",
    tagline: "Semgrep SAST, Trivy scanning, and ecosystem dependency auditors.",
    description:
      "Orchestrate static security analysis without credential leakage or telemetry. Checks for path traversal vulnerabilities, command injection risks, and known CVEs in dependencies.",
    cliCommand: "aek security audit",
    details: [
      "Semgrep SAST static analysis integration",
      "Trivy vulnerability and misconfiguration scanning",
      "Ecosystem dependency vulnerability checking (cargo-audit, npm audit)",
      "Normalized CVSS severity scoring and CI-friendly exit codes",
    ],
  },
];

export default function FeaturesPage() {
  return (
    <div className="py-20 px-4 sm:px-6 lg:px-8 max-w-content mx-auto">
      <div className="text-center max-w-3xl mx-auto mb-16">
        <div className="inline-flex items-center gap-2 px-3.5 py-1.5 rounded-full border border-flamingo/40 bg-brandy/20 text-xs font-mono text-custard mb-4">
          ✦ COMPREHENSIVE CAPABILITIES
        </div>
        <h1 className="font-editorial text-4xl sm:text-6xl text-ivory tracking-tight mb-4">
          The Eight <span className="italic text-gradient-editorial">Engineering Engines.</span>
        </h1>
        <p className="text-secondary-text text-base sm:text-lg">
          Explore the modular, high-performance engines that form the Antigravity Engineering Kit.
        </p>
      </div>

      <div className="space-y-12">
        {FEATURE_SECTIONS.map((sec) => (
          <div
            key={sec.id}
            id={sec.id}
            className="p-8 sm:p-10 rounded-3xl ceramic-card border border-espresso-border grid grid-cols-1 lg:grid-cols-12 gap-8 items-center"
          >
            <div className="lg:col-span-8 space-y-4">
              <div className="flex items-center gap-3">
                <Domain3DIcon type={sec.type} size="md" />
                <div>
                  <span className="text-[11px] font-mono tracking-widest text-flamingo uppercase font-semibold block">
                    {sec.category}
                  </span>
                  <h2 className="font-editorial text-2xl sm:text-3xl text-ivory">{sec.title}</h2>
                </div>
              </div>

              <p className="text-sm font-medium text-custard">{sec.tagline}</p>
              <p className="text-xs sm:text-sm text-sand/80 leading-relaxed">{sec.description}</p>

              <div className="pt-2">
                <ul className="grid grid-cols-1 sm:grid-cols-2 gap-2 text-xs text-sand/90">
                  {sec.details.map((item, i) => (
                    <li key={i} className="flex items-start gap-2">
                      <span className="text-flamingo font-bold">✓</span>
                      <span>{item}</span>
                    </li>
                  ))}
                </ul>
              </div>
            </div>

            <div className="lg:col-span-4 rounded-2xl bg-espresso-deep border border-espresso-border p-5 font-mono text-xs text-sand flex flex-col justify-between h-full">
              <div>
                <span className="text-[10px] text-muted-text block mb-2">CLI COMMAND DISPATCH</span>
                <code className="text-custard block p-2 rounded bg-espresso-surface border border-espresso-border">
                  $ {sec.cliCommand}
                </code>
              </div>
              <div className="pt-6">
                <Link
                  href={`/docs/getting-started`}
                  className="text-xs font-semibold text-flamingo hover:text-ivory transition-colors"
                >
                  Read documentation →
                </Link>
              </div>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
