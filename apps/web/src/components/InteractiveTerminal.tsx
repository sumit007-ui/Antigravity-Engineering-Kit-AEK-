"use client";

import { useState } from "react";
import { Copy, Check } from "lucide-react";
import SectionHeader from "@/components/primitives/SectionHeader";
import FilterChip from "@/components/primitives/FilterChip";

interface TerminalTab {
  id: string;
  label: string;
  command: string;
  output: string[];
}

const TABS: TerminalTab[] = [
  {
    id: "init",
    label: "[01] INITIALIZE",
    command: "aek init --profile engineering",
    output: [
      "[INFO] Resolving system directories: %LOCALAPPDATA%\\aek",
      "[INFO] Detecting project toolchains: Rust 1.85 (detected), Node 22 (detected)",
      "[INFO] Tiered profile selected: ENGINEERING tier (<50 MB memory budget)",
      "[INFO] Initializing .aek/config.toml (strict schema, atomic commit)",
      "[SUCCESS] AEK engineering workspace initialized with zero telemetry.",
    ],
  },
  {
    id: "sync",
    label: "[02] AGENT SYNC",
    command: "aek agent sync --all --strategy replace-managed",
    output: [
      "[INFO] Scanning target agent directories...",
      "[PASS] Google Antigravity: ~/.gemini/rules/ (3 rules, 5 MCP servers exported)",
      "[PASS] Anthropic Claude Code: ./CLAUDE.md & .claude/mcp.json (synchronized)",
      "[PASS] Cursor AI IDE: .cursor/rules/*.mdc & .cursor/mcp.json (synchronized)",
      "[PASS] OpenCode: AGENTS.md & .opencode/skills/ (synchronized)",
      "[SUCCESS] Synchronized 4 agent runtimes. Cryptographic manifest written: .aek/managed-agents.toml",
    ],
  },
  {
    id: "validate",
    label: "[03] QUALITY GATES",
    command: "aek validate --strict --fail-fast",
    output: [
      "[DAG] Stage 1/8: Code Formatting Verification ... [PASS: 0.12s]",
      "[DAG] Stage 2/8: Static Linter (Clippy/ESLint)  ... [PASS: 0.45s]",
      "[DAG] Stage 3/8: Type Consistency Check       ... [PASS: 0.28s]",
      "[DAG] Stage 4/8: Compilation & Build          ... [PASS: 0.84s]",
      "[DAG] Stage 5/8: Unit & Integration Suites    ... [PASS: 1.12s] (148 passed)",
      "[DAG] Stage 6/8: Memory & Benchmark Safety    ... [PASS: 0.09s]",
      "[SUCCESS] Quality gates passed (100% exit code 0). Report emitted: aek-summary.sarif",
    ],
  },
  {
    id: "mcp",
    label: "[04] MCP DISCOVERY",
    command: "aek mcp list --status",
    output: [
      "ID                   TRANSPORT   SECURITY LEVEL    STATUS",
      "──────────────────────────────────────────────────────────",
      "context7             Stdio       Read-Only         Active (PID 14920)",
      "github               Stdio       Least-Privilege   Active (PID 14924)",
      "playwright           Stdio       Sandboxed         Active (PID 14928)",
      "sequential-thinking  Stdio       Read-Only         Active (PID 14932)",
      "filesystem           Stdio       Sandboxed         Active (PID 14936)",
      "[INFO] 5 standard MCP servers operational via JSON-RPC 2.0.",
    ],
  },
];

export default function InteractiveTerminal() {
  const [activeTab, setActiveTab] = useState(0);
  const [copied, setCopied] = useState(false);

  const current = TABS[activeTab];

  const handleCopy = () => {
    navigator.clipboard.writeText(current.command);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <section className="py-28 px-4 sm:px-6 lg:px-8 bg-space-black border-t border-space-border relative">
      <div className="max-w-content mx-auto">
        <SectionHeader
          sectionNum="04"
          coordinates="15°29'N 73°49'E • SEC_04"
          title="Deterministic control at your fingertips."
          subtitle="Safe subprocess execution with direct argument arrays. Zero shell injection vulnerabilities."
          align="center"
        />

        {/* Filter Chips / Action Switchers */}
        <div className="flex flex-wrap items-center justify-center gap-2 mb-8 mt-10">
          {TABS.map((tab, idx) => (
            <FilterChip
              key={tab.id}
              label={tab.label}
              active={activeTab === idx}
              onClick={() => setActiveTab(idx)}
            />
          ))}
        </div>

        {/* Terminal Window Frame */}
        <div className="max-w-4xl mx-auto rounded-2xl overflow-hidden border-2 border-primary-gold/40 bg-space-black shadow-hard-md">
          {/* Top Bar with traffic lights & command switcher */}
          <div className="bg-space-surface px-4 py-3 border-b-2 border-space-border flex items-center justify-between">
            <div className="flex items-center gap-2">
              <span className="w-3 h-3 rounded-full bg-red-500/80 border border-red-600/50" />
              <span className="w-3 h-3 rounded-full bg-amber-500/80 border border-amber-600/50" />
              <span className="w-3 h-3 rounded-full bg-emerald-500/80 border border-emerald-600/50" />
              <span className="font-mono text-xs text-primary-gold ml-2 font-bold">
                aek-terminal ~ v0.1.0 • 15°29&apos;N 73°49&apos;E
              </span>
            </div>

            <button
              onClick={handleCopy}
              className="flex items-center gap-1.5 px-3 py-1 rounded-lg bg-space-deep hover:bg-space-surface border border-primary-gold/30 text-xs font-mono text-primary-gold hover:text-paper-beige transition-colors"
              aria-label="Copy command"
            >
              {copied ? (
                <>
                  <Check className="w-3.5 h-3.5 text-emerald-400" />
                  <span className="text-emerald-400">COPIED</span>
                </>
              ) : (
                <>
                  <Copy className="w-3.5 h-3.5 text-sand" />
                  <span>COPY</span>
                </>
              )}
            </button>
          </div>

          {/* Terminal Command Line */}
          <div className="p-6 bg-space-black font-mono text-xs sm:text-sm">
            <div className="flex items-center gap-2 text-primary-gold pb-4 border-b border-space-border/60 mb-6">
              <span className="text-warm-orange font-bold font-pixel text-xs">{">"}</span>
              <span className="text-paper-beige font-semibold select-all tracking-wide">{current.command}</span>
              <span className="w-2 h-4 bg-primary-gold inline-block animate-pulse ml-1" />
            </div>

            {/* Terminal Output */}
            <div className="space-y-2.5 text-sand/90 min-h-[180px]">
              {current.output.map((line, i) => {
                const isPass = line.includes("[PASS") || line.includes("[SUCCESS");
                const isInfo = line.includes("[INFO");
                const isDag = line.includes("[DAG");

                return (
                  <div key={i} className="leading-relaxed flex items-start gap-2">
                    <span className="text-primary-gold/40 select-none font-bold">»</span>
                    <span
                      className={
                        isPass
                          ? "text-emerald-400 font-medium"
                          : isInfo
                          ? "text-primary-gold"
                          : isDag
                          ? "text-warm-orange"
                          : "text-sand/90"
                      }
                    >
                      {line}
                    </span>
                  </div>
                );
              })}
            </div>
          </div>

          {/* Subprocess Isolation Badge */}
          <div className="bg-space-surface px-6 py-2.5 border-t-2 border-space-border text-[11px] font-mono text-sand flex items-center justify-between">
            <span className="flex items-center gap-1.5">
              <span className="w-2 h-2 rounded-full bg-emerald-400" />
              <span>ISOLATION: SECURE_SUBPROCESS_DIRECT_ARGS</span>
            </span>
            <span className="text-primary-gold font-bold">MEMORY_BUDGET: &lt;50 MB</span>
          </div>
        </div>
      </div>
    </section>
  );
}
