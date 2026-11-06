"use client";

import React, { useState } from "react";
import { Terminal, Copy, Check, ChevronRight } from "lucide-react";
import { cn } from "@/lib/utils";

interface CliCommandExample {
  id: string;
  category: string;
  cmd: string;
  description: string;
  output: string;
}

const COMMANDS: CliCommandExample[] = [
  {
    id: "doctor",
    category: "Diagnostics",
    cmd: "aek doctor",
    description: "Audit host operating system, compilers, Git, runtimes, and Google Antigravity readiness in one command.",
    output: `============================================================
     Antigravity Engineering Kit (AEK) Doctor (v0.1.0)
============================================================
Platform:  windows (x86_64)

[Platform]
  [PASS] Operating System          - windows
  [PASS] CPU Architecture          - x86_64
[Core Toolchain]
  [PASS] rustc                     - rustc 1.91.0
  [PASS] cargo                     - cargo 1.91.0
  [PASS] git                       - git version 2.49.0
[Configuration]
  [PASS] AEK Configuration         - Validated at .aek/config.toml
[Environment]
  [PASS] Google Antigravity        - CLI: agy.exe, MCP: mcp_config.json

Summary: 9 checks: 9 passed, 0 warnings, 0 failed.
Status:  HEALTHY - Environment is ready.`
  },
  {
    id: "skills",
    category: "Agent Skills",
    cmd: "aek skills discover",
    description: "Scan repository manifests to calculate confidence scores and recommend tailored engineering skills.",
    output: `AEK Project-Aware Skill Discovery
============================================================
Inspecting project files in '.'...
Found: Cargo.toml (Rust workspace detected)

Recommended Skills (1 found with confidence >= 70%):
  [1] rust-systems-engineer (Confidence: 100%)
      Guides development, memory safety, and cross-compilation.

To install this skill:
  aek skills install rust-systems-engineer --workspace .`
  },
  {
    id: "mcp",
    category: "MCP Framework",
    cmd: "aek mcp list",
    description: "Inspect registered Model Context Protocol servers, transport types, and sandbox permissions.",
    output: `Available MCP Servers (5 registered):
---------------------------------------------------------------------------
Server ID              Transport  Status     Description
---------------------------------------------------------------------------
context7               http       enabled    Fetch library documentation
filesystem             stdio      enabled    Controlled read & write access
github                 stdio      enabled    Access GitHub repos & issues
playwright             stdio      enabled    Automate browser interactions
sequential-thinking    stdio      enabled    Dynamic structured reasoning
---------------------------------------------------------------------------`
  },
  {
    id: "agent-detect",
    category: "Multi-Agent",
    cmd: "aek agent detect",
    description: "Non-destructively detect installed coding assistants across standard PATHs and platform configs.",
    output: `AEK AI Coding Agent Discovery Engine
======================================================================
Agent: Google Antigravity
  Detected:   [PASS] Active installation identified (agy.exe)
  Evidence:   Found Antigravity dir: ~/.gemini/antigravity
----------------------------------------------------------------------
Agent: Claude Code
  Detected:   [PASS] Active installation identified
  Evidence:   Found Claude Code home: ~/.claude
----------------------------------------------------------------------
Agent: Cursor
  Detected:   [PASS] Active installation identified (cursor executable)
  Evidence:   Found Cursor user dir: ~/.cursor
----------------------------------------------------------------------
Agent: OpenCode
  Detected:   [PASS] Active installation identified (opencode executable)
======================================================================`
  },
  {
    id: "security",
    category: "Security",
    cmd: "aek security scan",
    description: "Run multi-scanner security scans (Semgrep SAST, Trivy CVE scan, package dependency audits).",
    output: `AEK Multi-Scanner Security Orchestration
============================================================
Scanner: Semgrep SAST        [PASS] 0 security infractions
Scanner: Trivy CVE scan      [PASS] 0 critical vulnerabilities
Scanner: Cargo Audit         [PASS] 0 dependency advisories

Security Policy Evaluation:
  Fail on Critical: true
  Fail on High:     true
Result: PASSED - Security gates satisfied.`
  }
];

export default function CliShowcase() {
  const [selected, setSelected] = useState<CliCommandExample>(COMMANDS[0]);
  const [copied, setCopied] = useState(false);

  const handleCopy = () => {
    navigator.clipboard.writeText(selected.cmd);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <section className="py-20 bg-background relative border-t border-border/50">
      <div className="max-w-[1280px] mx-auto px-4 sm:px-6 lg:px-8">
        {/* Section Header */}
        <div className="text-center max-w-3xl mx-auto mb-16">
          <div className="inline-flex items-center gap-2 px-3 py-1 rounded-full bg-surface-elevated border border-border text-xs font-mono text-accent mb-4">
            ERGONOMIC TOOLING
          </div>
          <h2 className="text-3xl sm:text-4xl lg:text-5xl font-extrabold text-primary-text tracking-tight mb-4">
            One CLI. Complete control.
          </h2>
          <p className="text-secondary-text text-base sm:text-lg">
            A cohesive command suite designed for speed, predictable exit codes, and automated CI/CD integration.
          </p>
        </div>

        {/* Split Showcase Layout */}
        <div className="grid grid-cols-1 lg:grid-cols-12 gap-8 items-start">
          {/* Left Column: Command List */}
          <div className="lg:col-span-5 space-y-2.5">
            {COMMANDS.map((c) => {
              const isSelected = selected.id === c.id;
              return (
                <button
                  key={c.id}
                  onClick={() => setSelected(c)}
                  className={cn(
                    "w-full p-4 rounded-xl text-left border transition-all flex flex-col gap-1 focus:outline-none focus-visible:ring-2 focus-visible:ring-accent",
                    isSelected
                      ? "bg-surface-elevated border-accent shadow-md shadow-accent/10"
                      : "bg-surface border-border hover:border-border/80 hover:bg-surface-elevated/40"
                  )}
                >
                  <div className="flex items-center justify-between">
                    <span className="font-mono text-xs font-bold text-accent">{c.cmd}</span>
                    <span className="text-[10px] font-mono px-2 py-0.5 rounded bg-surface border border-border text-secondary-text">
                      {c.category}
                    </span>
                  </div>
                  <p className="text-xs text-secondary-text line-clamp-2 leading-relaxed">
                    {c.description}
                  </p>
                </button>
              );
            })}
          </div>

          {/* Right Column: Interactive Terminal Preview */}
          <div className="lg:col-span-7 rounded-2xl bg-[#101218] border border-border shadow-2xl overflow-hidden">
            {/* Header */}
            <div className="bg-[#171A22] border-b border-border px-4 py-3 flex items-center justify-between">
              <div className="flex items-center gap-2">
                <span className="w-3 h-3 rounded-full bg-[#EF4444]/80 inline-block"></span>
                <span className="w-3 h-3 rounded-full bg-[#F59E0B]/80 inline-block"></span>
                <span className="w-3 h-3 rounded-full bg-[#22C55E]/80 inline-block"></span>
                <span className="ml-3 font-mono text-xs text-secondary-text">
                  $ {selected.cmd}
                </span>
              </div>
              <button
                onClick={handleCopy}
                className="flex items-center gap-1.5 px-2.5 py-1 rounded bg-[#101218] border border-border text-secondary-text hover:text-primary-text text-xs font-mono transition-colors"
              >
                {copied ? (
                  <>
                    <Check className="w-3.5 h-3.5 text-status-success" />
                    <span className="text-status-success">Copied</span>
                  </>
                ) : (
                  <>
                    <Copy className="w-3.5 h-3.5" />
                    <span>Copy</span>
                  </>
                )}
              </button>
            </div>

            {/* Output */}
            <div className="p-5 font-mono text-xs bg-[#08090D] overflow-x-auto text-secondary-text whitespace-pre leading-relaxed min-h-[300px]">
              {selected.output}
            </div>

            {/* Sub-footer */}
            <div className="p-3 bg-[#171A22]/50 border-t border-border flex items-center justify-between text-[11px] font-mono text-secondary-text/70">
              <span>Output: Deterministic UTF-8 Text / JSON</span>
              <span>Execution Time: ~20–60 ms</span>
            </div>
          </div>
        </div>
      </div>
    </section>
  );
}

