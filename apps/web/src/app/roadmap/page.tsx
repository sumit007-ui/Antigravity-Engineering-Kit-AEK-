"use client";

import { CheckCircle2, Clock, Sparkles } from "lucide-react";
import { LATEST_VERSION } from "@/lib/utils";

const PHASES = [
  {
    phase: "Phase 1",
    title: "Rust Architecture & Platform Abstraction",
    status: "Completed (v0.1.0)",
    completed: true,
    description: "Multi-crate Cargo workspace, platform-specific directory resolution (XDG/Windows Known Folders), path traversal defenses, safe subprocess engine.",
  },
  {
    phase: "Phase 2",
    title: "Configuration Management & Diagnostic Doctor",
    status: "Completed (v0.1.0)",
    completed: true,
    description: "Strict TOML schema validation, atomic file mutations with same-filesystem tempfiles, backup/restore rotation, and environment doctor.",
  },
  {
    phase: "Phase 3",
    title: "Engineering Rules & Specialized Skills Engine",
    status: "Completed (v0.1.0)",
    completed: true,
    description: "Tiered precedence system (Global → Workspace → Project), 24 KB size constraints, token budgeting, and 15 progressive runbooks.",
  },
  {
    phase: "Phase 4",
    title: "Model Context Protocol Client & Standard Integrations",
    status: "Completed (v0.1.0)",
    completed: true,
    description: "JSON-RPC 2.0 client engine, Stdio/HTTP transports, lifecycle supervision, and adapters for Context7, GitHub, Playwright, Sequential Thinking, Filesystem.",
  },
  {
    phase: "Phase 5",
    title: "Multi-Language Toolchains & 8-Stage Quality Gates",
    status: "Completed (v0.1.0)",
    completed: true,
    description: "Discovery and diagnostics across 8 ecosystems (Rust, Go, C/C++, Zig, Python, TypeScript, Flutter, ROS 2) with DAG execution pipeline.",
  },
  {
    phase: "Phase 6",
    title: "Prompt Optimization, DSPy & Evaluation Suites",
    status: "Completed (v0.1.0)",
    completed: true,
    description: "Structured prompt optimization, DSPy programmatic signature translation, and Promptfoo regression and security test suites.",
  },
  {
    phase: "Phase 7",
    title: "Validation Engine & Multi-Scanner Security Auditing",
    status: "Completed (v0.1.0)",
    completed: true,
    description: "Integrated Semgrep SAST, Trivy vulnerability scanning, package dependency auditing, normalized severity scoring, and OASIS SARIF reporting.",
  },
  {
    phase: "Phase 8",
    title: "Cross-Platform Distribution & Lifecycle Engine",
    status: "Completed (v0.1.0)",
    completed: true,
    description: "Windows & Linux binary packaging, user-space PowerShell and Bash installers, atomic self-updates, and rollback archives.",
  },
  {
    phase: "Phase 9",
    title: "Universal Multi-Agent Compatibility Framework",
    status: "Completed (v0.1.0)",
    completed: true,
    description: "Unified agent adapter interface, native config translators for Antigravity, Claude Code, Cursor, and OpenCode with SHA-256 state tracking.",
  },
  {
    phase: "Phase 10",
    title: "Production Hardening, Open-Source Launch & v0.1.0",
    status: "Completed (v0.1.0)",
    completed: true,
    description: "Automated GitHub Actions CI/CD matrix (Ubuntu, Windows, macOS), verifiable release binaries, documentation, and MIT/Apache-2.0 release.",
  },
  {
    phase: "Phase 11 (Upcoming)",
    title: "Local LLM Fine-Tuning Evaluator & Telemetry Sandboxing",
    status: "Planned (Q3 2026)",
    completed: false,
    description: "Direct integration with local Ollama/Llama.cpp runtimes for zero-token-cost prompt evaluation and offline rule validation.",
  },
  {
    phase: "Phase 12 (Upcoming)",
    title: "Live Distributed Team Rules Sync (Peer-to-Peer)",
    status: "Research (Q4 2026)",
    completed: false,
    description: "Optional cryptographic team rule synchronization over Git-backed signed commits without centralized servers.",
  },
];

export default function RoadmapPage() {
  return (
    <div className="py-20 px-4 sm:px-6 lg:px-8 max-w-content mx-auto">
      <div className="text-center max-w-3xl mx-auto mb-16">
        <div className="inline-flex items-center gap-2 px-3.5 py-1.5 rounded-full border border-flamingo/40 bg-brandy/20 text-xs font-mono text-custard mb-4">
          ✦ DEVELOPMENT EVOLUTION
        </div>
        <h1 className="font-editorial text-4xl sm:text-6xl lg:text-7xl text-ivory tracking-tight mb-4">
          The AEK <span className="italic text-gradient-editorial">Roadmap.</span>
        </h1>
        <p className="text-secondary-text text-base sm:text-lg">
          Track the completed 10-phase production foundation and future enhancements.
        </p>
      </div>

      <div className="relative border-l border-espresso-border/80 ml-4 sm:ml-32 md:ml-48 space-y-12">
        {PHASES.map((item, idx) => (
          <div key={idx} className="relative pl-8 sm:pl-12">
            {/* Timeline Marker */}
            <div
              className={`absolute -left-3 top-1.5 w-6 h-6 rounded-full border-2 flex items-center justify-center ${
                item.completed
                  ? "bg-brandy border-flamingo shadow-glow"
                  : "bg-espresso-surface border-espresso-border"
              }`}
            >
              {item.completed ? (
                <CheckCircle2 className="w-3.5 h-3.5 text-custard" />
              ) : (
                <Clock className="w-3.5 h-3.5 text-sand/60" />
              )}
            </div>

            {/* Content Card */}
            <div className="p-6 rounded-2xl ceramic-card border border-espresso-border">
              <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-2 mb-2">
                <span className="font-mono text-xs text-flamingo font-semibold tracking-wider">
                  {item.phase}
                </span>
                <span
                  className={`text-[11px] font-mono px-2.5 py-0.5 rounded-full border ${
                    item.completed
                      ? "bg-emerald-500/10 text-emerald-400 border-emerald-500/30"
                      : "bg-espresso-elevated text-sand/70 border-espresso-border"
                  }`}
                >
                  {item.status}
                </span>
              </div>
              <h3 className="font-editorial text-2xl text-ivory mb-2">{item.title}</h3>
              <p className="text-xs sm:text-sm text-sand/80 leading-relaxed">
                {item.description}
              </p>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
