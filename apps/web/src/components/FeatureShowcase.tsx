import React from "react";
import Link from "next/link";
import { 
  BookOpen, 
  Sparkles, 
  Layers, 
  Code2, 
  Cpu, 
  CheckCircle, 
  ShieldAlert, 
  Boxes,
  ArrowRight
} from "lucide-react";

interface FeatureCardProps {
  title: string;
  description: string;
  icon: React.ElementType;
  status: "STABLE" | "BETA" | "EXPERIMENTAL";
  previewCode?: string;
  bulletPoints?: string[];
  link: string;
  className?: string;
  accentColor?: string;
}

const FEATURES: FeatureCardProps[] = [
  {
    title: "Global & Project Engineering Rules",
    description: "Tiered precedence resolution (Global -> Workspace -> Project) with 24 KB size bounds and token budgeting to prevent agent context pollution.",
    icon: BookOpen,
    status: "STABLE",
    previewCode: "$ aek rules list\nAvailable: precision-rule, testing, security, engineering",
    bulletPoints: ["Precedence resolution", "24 KB size ceiling", "Markdown frontmatter"],
    link: "/docs/rules",
    className: "md:col-span-2",
  },
  {
    title: "15 Specialized Agent Skills",
    description: "Progressive-disclosure engineering runbooks covering Rust, Go, C++, Zig, Python, Flutter, ROS 2, DevOps, and Security Auditing.",
    icon: Sparkles,
    status: "STABLE",
    previewCode: "$ aek skills discover\nRecommends: rust-systems-engineer (confidence: 100%)",
    bulletPoints: ["Project-aware auto-discovery", "Zero LLM cost matching", "Dry-run install/remove"],
    link: "/docs/skills",
    className: "md:col-span-1",
  },
  {
    title: "Model Context Protocol (MCP)",
    description: "High-throughput JSON-RPC 2.0 client engine with Stdio and Streamable HTTP transports. Out-of-the-box verified presets.",
    icon: Layers,
    status: "STABLE",
    previewCode: "$ aek mcp list\nServers: context7, github, playwright, sequential-thinking, fs",
    bulletPoints: ["Stdio & HTTP transports", "Lifecycle state machine", "Permission sandboxing"],
    link: "/docs/mcp",
    className: "md:col-span-1",
  },
  {
    title: "Universal Multi-Agent Compatibility",
    description: "Agent-agnostic core translating rules, skills, and MCP natively into Gemini Antigravity, Claude Code, Cursor, and OpenCode with SHA-256 ownership tracking.",
    icon: Boxes,
    status: "STABLE",
    previewCode: "$ aek agent sync --strategy replace-managed\nTranslates CLAUDE.md, .cursorrules, AGENTS.md",
    bulletPoints: ["Conflict detection", "5 Predefined engineering profiles", "Clean uninstallation"],
    link: "/docs/multi-agent",
    className: "md:col-span-2",
  },
  {
    title: "Language Toolchain Management",
    description: "Inspect project manifests and detect compilers, linters, and language servers across 8 ecosystems without manual setup.",
    icon: Code2,
    status: "STABLE",
    bulletPoints: ["Rust, Go, Python, TypeScript", "C/C++, Zig, Flutter, ROS 2", "Doctor diagnostics"],
    link: "/docs/languages",
    className: "md:col-span-1",
  },
  {
    title: "Automated 8-Stage Validation",
    description: "Deterministic DAG pipeline executing formatting, linting, type-checking, compilation, and testing with incremental Git diff detection.",
    icon: CheckCircle,
    status: "STABLE",
    bulletPoints: ["Configurable quality gates", "Incremental git change detection", "OASIS SARIF v2.1.0 output"],
    link: "/docs/validation",
    className: "md:col-span-1",
  },
  {
    title: "Multi-Scanner Security Auditing",
    description: "Orchestrate Semgrep SAST, Trivy vulnerability & misconfiguration scanning, and native package dependency auditors in one command.",
    icon: ShieldAlert,
    status: "BETA",
    bulletPoints: ["Normalized severity scoring", "Automated secret redaction", "Graceful offline fallback"],
    link: "/docs/security",
    className: "md:col-span-1",
  },
  {
    title: "AI Prompt Optimization & Eval",
    description: "Version-controlled prompt templates with content-hash commits, Jinja variable substitution, Stanford DSPy bridges, and Promptfoo test suites.",
    icon: Cpu,
    status: "BETA",
    bulletPoints: ["Strict variable schemas", "Promptfoo red-teaming", "DSPy modular compile"],
    link: "/docs/prompt-optimization",
    className: "md:col-span-1",
  },
];

export default function FeatureShowcase() {
  return (
    <section className="py-20 bg-background relative border-t border-border/50">
      <div className="max-w-[1280px] mx-auto px-4 sm:px-6 lg:px-8">
        {/* Section Header */}
        <div className="text-center max-w-3xl mx-auto mb-16">
          <div className="inline-flex items-center gap-2 px-3 py-1 rounded-full bg-surface-elevated border border-border text-xs font-mono text-accent mb-4">
            MODULAR TOOLKIT
          </div>
          <h2 className="text-3xl sm:text-4xl lg:text-5xl font-extrabold text-primary-text tracking-tight mb-4">
            Everything your AI coding workflow needs.
          </h2>
          <p className="text-secondary-text text-base sm:text-lg">
            One modular toolkit for engineering consistency, multi-agent compatibility, and verifiable code quality.
          </p>
        </div>

        {/* Bento Grid Layout */}
        <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
          {FEATURES.map((feat) => {
            const Icon = feat.icon;
            return (
              <div
                key={feat.title}
                className={`group p-6 sm:p-7 rounded-2xl bg-surface border border-border hover:border-accent/40 transition-all flex flex-col justify-between shadow-sm hover:shadow-lg hover:shadow-black/60 ${feat.className}`}
              >
                <div>
                  <div className="flex items-center justify-between mb-5">
                    <div className="w-10 h-10 rounded-xl bg-surface-elevated border border-border flex items-center justify-center text-accent group-hover:scale-105 transition-transform">
                      <Icon className="w-5 h-5" />
                    </div>
                    <span
                      className={`text-[10px] font-mono px-2 py-0.5 rounded border ${
                        feat.status === "STABLE"
                          ? "bg-status-success/10 text-status-success border-status-success/30"
                          : "bg-status-warning/10 text-status-warning border-status-warning/30"
                      }`}
                    >
                      {feat.status}
                    </span>
                  </div>

                  <h3 className="text-lg font-bold text-primary-text mb-2 tracking-tight group-hover:text-accent transition-colors">
                    {feat.title}
                  </h3>
                  <p className="text-xs sm:text-sm text-secondary-text leading-relaxed mb-4">
                    {feat.description}
                  </p>

                  {feat.previewCode && (
                    <div className="p-3 rounded-lg bg-[#08090D] border border-border/80 font-mono text-xs text-secondary-text/90 mb-4 overflow-x-auto whitespace-pre">
                      {feat.previewCode}
                    </div>
                  )}

                  {feat.bulletPoints && (
                    <ul className="space-y-1.5 mb-6 text-xs text-secondary-text">
                      {feat.bulletPoints.map((pt, i) => (
                        <li key={i} className="flex items-center gap-2">
                          <span className="w-1.5 h-1.5 rounded-full bg-accent"></span>
                          <span>{pt}</span>
                        </li>
                      ))}
                    </ul>
                  )}
                </div>

                <Link
                  href={feat.link}
                  className="inline-flex items-center gap-1.5 text-xs font-semibold text-accent hover:text-accent-hover transition-colors focus:outline-none focus-visible:underline"
                >
                  <span>Explore documentation</span>
                  <ArrowRight className="w-3.5 h-3.5 transition-transform group-hover:translate-x-1" />
                </Link>
              </div>
            );
          })}
        </div>
      </div>
    </section>
  );
}

