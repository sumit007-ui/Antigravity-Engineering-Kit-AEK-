"use client";

import { useState } from "react";
import SectionHeader from "@/components/primitives/SectionHeader";
import { PixelCard } from "@/components/primitives/PixelCard";

interface NodeDetail {
  id: string;
  name: string;
  crate: string;
  role: string;
  description: string;
  invariants: string[];
}

const NODES: NodeDetail[] = [
  {
    id: "cli",
    name: "CLI Shell & UX",
    crate: "crates/aek-cli",
    role: "User-facing Command Interface",
    description: "Clap-driven CLI binary executing subcommands with sub-millisecond dispatch and formatted terminal UI.",
    invariants: ["Exit code 0 on success", "Structured JSON/SARIF output flags", "Direct argument array dispatch"],
  },
  {
    id: "core",
    name: "Core Execution Engine",
    crate: "crates/aek-core",
    role: "Subprocess & Diagnostic Runner",
    description: "Secure process supervisor with timeouts, memory limits, and isolated environment variables.",
    invariants: ["Zero shell evaluation (no cmd.exe /c)", "Pre-allocated buffer caps", "Deterministic diagnostic codes"],
  },
  {
    id: "agents",
    name: "Multi-Agent Adapters",
    crate: "crates/aek-agents",
    role: "Universal Agent Synchronization",
    description: "Translates abstract engineering rules, skills, and MCP tools into Antigravity, Claude Code, Cursor, and OpenCode configs.",
    invariants: ["SHA-256 ownership verification", "Safe rollback backups before write", "Non-destructive conflict strategies"],
  },
  {
    id: "mcp",
    name: "MCP Client & Integrations",
    crate: "crates/aek-mcp",
    role: "Model Context Protocol Engine",
    description: "High-throughput JSON-RPC 2.0 client supporting Stdio and HTTP transports with verified server adapters.",
    invariants: ["Process lifecycle supervision", "Least-privilege permission tagging", "Strict error code mapping"],
  },
  {
    id: "validation",
    name: "8-Stage Quality Gates",
    crate: "crates/aek-validation",
    role: "Verification Pipeline Planner",
    description: "DAG planner executing formatters, linters, type checks, and test suites with incremental Git diff filtering.",
    invariants: ["Topological dependency order", "Fail-fast execution option", "Machine-readable SARIF output"],
  },
  {
    id: "security",
    name: "Security Scanner Orchestrator",
    crate: "crates/aek-security",
    role: "SAST & Dependency Auditing",
    description: "Unified security runner combining Semgrep, Trivy, and native package manager vulnerability auditors.",
    invariants: ["CVSS normalized scoring", "Deterministic exit gates", "Zero credential persistence"],
  },
];

export default function ArchitectureVisualization() {
  const [activeNode, setActiveNode] = useState<string>("cli");
  const selected = NODES.find((n) => n.id === activeNode) || NODES[0];

  return (
    <section className="py-28 px-4 sm:px-6 lg:px-8 bg-space-black border-t border-space-border relative">
      <div className="max-w-content mx-auto">
        <SectionHeader
          sectionNum="06"
          sectionCode="SEC_06"
          title="Sixteen decoupled crates. One cohesive architecture."
          subtitle="Inspect the foundational crates powering AEK's memory safety, deterministic execution, and agent synchronization."
          align="center"
        />

        {/* Interactive Architecture Canvas */}
        <div className="grid grid-cols-1 lg:grid-cols-12 gap-8 items-center mt-12">
          {/* Node Grid */}
          <div className="lg:col-span-7 grid grid-cols-1 sm:grid-cols-2 gap-4">
            {NODES.map((node) => {
              const isSelected = activeNode === node.id;
              return (
                <button
                  key={node.id}
                  onClick={() => setActiveNode(node.id)}
                  className={`p-5 rounded-2xl text-left transition-all border-2 ${
                    isSelected
                      ? "bg-space-surface border-warm-orange shadow-hard-md"
                      : "bg-space-deep/80 border-space-border hover:border-primary-gold/40 shadow-hard-sm"
                  }`}
                >
                  <span className="font-mono text-[11px] text-primary-gold font-bold block mb-1">
                    {node.crate}
                  </span>
                  <h3 className="font-editorial text-xl text-paper-beige mb-1 font-bold">{node.name}</h3>
                  <p className="text-xs text-sand/70">{node.role}</p>
                </button>
              );
            })}
          </div>

          {/* Node Inspector Panel */}
          <div className="lg:col-span-5 rounded-3xl bg-space-deep p-7 border-2 border-primary-gold/40 shadow-hard-md">
            <div className="flex items-center justify-between pb-4 border-b border-space-border mb-5">
              <div>
                <span className="font-mono text-xs text-warm-orange font-bold">{selected.crate}</span>
                <h3 className="font-editorial text-2xl text-paper-beige mt-0.5 font-bold">{selected.name}</h3>
              </div>
              <span className="px-2.5 py-1 rounded-full bg-space-surface border border-primary-gold/40 text-[10px] font-mono text-primary-gold font-bold">
                CRATE SPEC
              </span>
            </div>

            <p className="text-xs sm:text-sm text-sand/90 leading-relaxed mb-6 font-mono">
              {selected.description}
            </p>

            <div>
              <h4 className="text-xs font-mono font-bold uppercase tracking-wider text-primary-gold mb-3">
                Architectural Invariants
              </h4>
              <ul className="space-y-2.5">
                {selected.invariants.map((inv, i) => (
                  <li key={i} className="flex items-start gap-2 text-xs text-sand/90">
                    <span className="text-warm-orange font-bold">✓</span>
                    <span>{inv}</span>
                  </li>
                ))}
              </ul>
            </div>
          </div>
        </div>
      </div>
    </section>
  );
}
