"use client";

import { useState } from "react";
import { Copy, Check, Terminal as TerminalIcon, Sparkles } from "lucide-react";

interface TerminalTab {
  id: string;
  label: string;
  command: string;
  output: string[];
}

const TABS: TerminalTab[] = [
  {
    id: "init",
    label: "Initialize Workspace",
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
    label: "Sync Coding Agents",
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
    label: "Quality Gate DAG",
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
    label: "MCP Discovery",
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
    <section className="py-28 px-4 sm:px-6 lg:px-8 bg-espresso-deep border-t border-espresso-border relative">
      <div className="max-w-content mx-auto">
        <div className="text-center max-w-3xl mx-auto mb-14">
          <div className="inline-flex items-center gap-2 px-3 py-1 rounded-full border border-flamingo/40 bg-brandy/20 text-xs font-mono text-custard mb-3">
            ✦ COMMAND LINE INTERFACE
          </div>
          <h2 className="font-editorial text-4xl sm:text-5xl text-ivory tracking-tight mb-4">
            Deterministic control at your <span className="italic text-gradient-editorial">fingertips.</span>
          </h2>
          <p className="text-secondary-text text-sm sm:text-base">
            Safe subprocess execution with direct argument arrays. Zero shell injection vulnerabilities.
          </p>
        </div>

        {/* Terminal Window Frame */}
        <div className="max-w-4xl mx-auto rounded-3xl overflow-hidden border border-espresso-border bg-espresso-deep shadow-tactile-hover">
          {/* Top Bar with traffic lights & command switcher */}
          <div className="bg-espresso-surface px-4 py-3 border-b border-espresso-border flex flex-col sm:flex-row items-center justify-between gap-4">
            <div className="flex items-center gap-2 self-start sm:self-center">
              <span className="w-3 h-3 rounded-full bg-brandy border border-flamingo/40" />
              <span className="w-3 h-3 rounded-full bg-sand border border-custard/40" />
              <span className="w-3 h-3 rounded-full bg-emerald-500/80 border border-emerald-400/40" />
              <span className="font-mono text-xs text-muted-text ml-2">aek-terminal ~ v0.1.0</span>
            </div>

            {/* Tabs */}
            <div className="flex items-center gap-1 overflow-x-auto w-full sm:w-auto pb-1 sm:pb-0">
              {TABS.map((tab, idx) => (
                <button
                  key={tab.id}
                  onClick={() => setActiveTab(idx)}
                  className={`px-3 py-1 text-xs font-mono rounded-lg transition-colors whitespace-nowrap ${
                    activeTab === idx
                      ? "bg-brandy text-ivory border border-flamingo/50"
                      : "text-sand/70 hover:text-ivory hover:bg-espresso-elevated"
                  }`}
                >
                  {tab.label}
                </button>
              ))}
            </div>
          </div>

          {/* Terminal Command Line */}
          <div className="p-6 bg-espresso-deep/95 font-mono text-xs sm:text-sm">
            <div className="flex items-center justify-between pb-4 border-b border-espresso-border/60 mb-6">
              <div className="flex items-center gap-2 text-custard">
                <span className="text-flamingo font-bold">$</span>
                <span className="text-ivory font-medium select-all">{current.command}</span>
              </div>
              <button
                onClick={handleCopy}
                className="flex items-center gap-1.5 px-3 py-1 rounded-lg bg-espresso-surface hover:bg-espresso-elevated border border-espresso-border text-xs text-sand hover:text-ivory transition-colors"
                aria-label="Copy command"
              >
                {copied ? (
                  <>
                    <Check className="w-3.5 h-3.5 text-emerald-400" />
                    <span className="text-emerald-400">Copied</span>
                  </>
                ) : (
                  <>
                    <Copy className="w-3.5 h-3.5 text-muted-text" />
                    <span>Copy</span>
                  </>
                )}
              </button>
            </div>

            {/* Terminal Output */}
            <div className="space-y-2 text-sand/90 min-h-[180px]">
              {current.output.map((line, i) => {
                const isPass = line.includes("[PASS") || line.includes("[SUCCESS");
                const isInfo = line.includes("[INFO");
                const isDag = line.includes("[DAG");

                return (
                  <div key={i} className="leading-relaxed flex items-start gap-2">
                    <span className="text-muted-text select-none">{">"}</span>
                    <span
                      className={
                        isPass
                          ? "text-emerald-400 font-medium"
                          : isInfo
                          ? "text-custard"
                          : isDag
                          ? "text-flamingo"
                          : "text-sand/80"
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
          <div className="bg-espresso-surface px-6 py-2.5 border-t border-espresso-border text-[11px] font-mono text-muted-text flex items-center justify-between">
            <span>ISOLATION: SECURE_SUBPROCESS_DIRECT_ARGS</span>
            <span className="text-custard">MEMORY: 18.4 MB</span>
          </div>
        </div>
      </div>
    </section>
  );
}
