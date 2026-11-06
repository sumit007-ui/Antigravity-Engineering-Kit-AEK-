"use client";

import React, { useState } from "react";
import { Terminal, Shield, Layers, Code2, Cpu, CheckCircle, Boxes, Download, FileText, ArrowDown } from "lucide-react";
import { cn } from "@/lib/utils";

interface ArchitectureNode {
  id: string;
  crate: string;
  name: string;
  description: string;
  invariants: string;
  icon: React.ElementType;
}

const NODES: ArchitectureNode[] = [
  {
    id: "cli",
    crate: "crates/aek-cli",
    name: "AEK Command-Line Interface",
    description: "User-facing command parser powered by Clap v4 with structured diagnostic logging and exit-code determinism.",
    invariants: "Exit codes: 0 = success, 1 = failure, 2 = usage error. Supports --json and --dry-run across mutating commands.",
    icon: Terminal,
  },
  {
    id: "agents",
    crate: "crates/aek-agents",
    name: "Multi-Agent Compatibility Engine",
    description: "Extensible adapter trait translating rules, skills, and MCP configurations into native agent formats.",
    invariants: "SHA-256 ownership tracking via .aek/managed-agents.toml with KeepExisting, ReplaceManaged, and Skip strategies.",
    icon: Boxes,
  },
  {
    id: "rules",
    crate: "crates/aek-rules",
    name: "Engineering Rules Engine",
    description: "Tiered precedence resolution (Global -> Workspace -> Project) with markdown frontmatter validation.",
    invariants: "24 KB size bounds per rule; strict token budgeting to prevent LLM context saturation.",
    icon: FileText,
  },
  {
    id: "skills",
    crate: "crates/aek-skills",
    name: "Agent Skills Engine",
    description: "15 specialized engineering runbooks with progressive disclosure and project-aware auto-discovery scoring.",
    invariants: "AST and manifest heuristics; zero LLM token cost for skill discovery.",
    icon: Code2,
  },
  {
    id: "mcp",
    crate: "crates/aek-mcp",
    name: "Model Context Protocol Client",
    description: "JSON-RPC 2.0 client engine over Stdio and Streamable HTTP with 5 standard verified integration presets.",
    invariants: "Request timeout supervision, process isolation, and permission boundary sandboxing.",
    icon: Layers,
  },
  {
    id: "validation",
    crate: "crates/aek-validation",
    name: "Validation & DAG Planner",
    description: "Standardized 8-stage verification pipeline with incremental Git change detection and quality gates.",
    invariants: "Strict DAG execution order (Format -> Lint -> TypeCheck -> Build -> Test -> Security).",
    icon: CheckCircle,
  },
  {
    id: "distribution",
    crate: "crates/aek-distribution",
    name: "Cross-Platform Distribution",
    description: "User-space installation paths, release manifest verification, atomic in-place updates, and rollback archives.",
    invariants: "Cryptographic SHA-256 checksum verification; zero root/administrator privileges required.",
    icon: Download,
  },
  {
    id: "core",
    crate: "crates/aek-core",
    name: "Core Process Execution & Safety",
    description: "Subprocess execution engine passing direct argument vectors without shell evaluation, plus secret redaction.",
    invariants: "Direct argument-array execution; zero cmd.exe /c or /bin/sh string evaluation.",
    icon: Shield,
  },
  {
    id: "platform",
    crate: "crates/aek-platform",
    name: "Platform Abstraction & Paths",
    description: "Cross-platform directory resolution (Windows, Linux, macOS) and strict path traversal defense.",
    invariants: "Strict is_safe_subpath validation protecting against parent directory escape attacks.",
    icon: Cpu,
  },
];

export default function ArchitectureDiagram() {
  const [activeNode, setActiveNode] = useState<ArchitectureNode>(NODES[0]);

  return (
    <section className="py-20 bg-background relative border-t border-border/50">
      <div className="max-w-[1280px] mx-auto px-4 sm:px-6 lg:px-8">
        {/* Section Header */}
        <div className="text-center max-w-3xl mx-auto mb-16">
          <div className="inline-flex items-center gap-2 px-3 py-1 rounded-full bg-surface-elevated border border-border text-xs font-mono text-accent mb-4">
            SYSTEMS ARCHITECTURE
          </div>
          <h2 className="text-3xl sm:text-4xl lg:text-5xl font-extrabold text-primary-text tracking-tight mb-4">
            Engineered in Rust. Built for reliability.
          </h2>
          <p className="text-secondary-text text-base sm:text-lg">
            A modular, cross-platform architecture designed for predictable configuration, secure execution, and extensible developer workflows.
          </p>
        </div>

        {/* Interactive Architecture Layout */}
        <div className="grid grid-cols-1 lg:grid-cols-12 gap-8 items-start">
          {/* Left Diagram Nodes */}
          <div className="lg:col-span-7 space-y-3">
            <div className="text-xs font-mono text-secondary-text mb-2">
              Select an architectural layer to inspect crate boundaries:
            </div>

            {/* Top Tier: CLI */}
            <button
              onClick={() => setActiveNode(NODES[0])}
              className={cn(
                "w-full p-4 rounded-xl text-left border transition-all flex items-center justify-between",
                activeNode.id === "cli"
                  ? "bg-surface-elevated border-accent shadow-md shadow-accent/10"
                  : "bg-surface border-border hover:bg-surface-elevated/40"
              )}
            >
              <div className="flex items-center gap-3">
                <Terminal className="w-5 h-5 text-accent" />
                <div>
                  <span className="font-bold text-sm text-primary-text">{NODES[0].name}</span>
                  <span className="block text-xs font-mono text-secondary-text">{NODES[0].crate}</span>
                </div>
              </div>
              <span className="text-xs font-mono px-2 py-0.5 rounded bg-surface border border-border text-accent">
                User Layer
              </span>
            </button>

            {/* Flow Indicator */}
            <div className="flex justify-center my-1 text-secondary-text/40">
              <ArrowDown className="w-4 h-4" />
            </div>

            {/* Middle Tier: Domain Engines Grid */}
            <div className="grid grid-cols-1 sm:grid-cols-2 gap-3">
              {NODES.slice(1, 7).map((node) => {
                const Icon = node.icon;
                const isSelected = activeNode.id === node.id;
                return (
                  <button
                    key={node.id}
                    onClick={() => setActiveNode(node)}
                    className={cn(
                      "p-3.5 rounded-xl text-left border transition-all flex flex-col justify-between",
                      isSelected
                        ? "bg-surface-elevated border-accent shadow-md shadow-accent/10"
                        : "bg-surface border-border hover:bg-surface-elevated/40"
                    )}
                  >
                    <div className="flex items-center gap-2 mb-2">
                      <Icon className={cn("w-4 h-4", isSelected ? "text-accent" : "text-secondary-text")} />
                      <span className="font-bold text-xs text-primary-text truncate">{node.name}</span>
                    </div>
                    <span className="text-[10px] font-mono text-secondary-text/80 truncate">{node.crate}</span>
                  </button>
                );
              })}
            </div>

            {/* Flow Indicator */}
            <div className="flex justify-center my-1 text-secondary-text/40">
              <ArrowDown className="w-4 h-4" />
            </div>

            {/* Bottom Tier: Core & Platform Foundation */}
            <div className="grid grid-cols-1 sm:grid-cols-2 gap-3">
              {NODES.slice(7).map((node) => {
                const Icon = node.icon;
                const isSelected = activeNode.id === node.id;
                return (
                  <button
                    key={node.id}
                    onClick={() => setActiveNode(node)}
                    className={cn(
                      "p-4 rounded-xl text-left border transition-all flex items-center justify-between",
                      isSelected
                        ? "bg-surface-elevated border-accent shadow-md shadow-accent/10"
                        : "bg-surface border-border hover:bg-surface-elevated/40"
                    )}
                  >
                    <div className="flex items-center gap-3">
                      <Icon className="w-4 h-4 text-status-success" />
                      <div>
                        <span className="font-bold text-xs text-primary-text">{node.name}</span>
                        <span className="block text-[11px] font-mono text-secondary-text">{node.crate}</span>
                      </div>
                    </div>
                    <span className="text-[10px] font-mono px-2 py-0.5 rounded bg-surface border border-border text-status-success">
                      Foundation
                    </span>
                  </button>
                );
              })}
            </div>
          </div>

          {/* Right Column: Node Inspector Details */}
          <div className="lg:col-span-5 p-6 rounded-2xl bg-surface border border-border shadow-xl">
            <div className="flex items-center gap-3 mb-4 pb-4 border-b border-border">
              <div className="w-10 h-10 rounded-xl bg-surface-elevated border border-border flex items-center justify-center text-accent">
                <activeNode.icon className="w-5 h-5" />
              </div>
              <div>
                <h3 className="text-base font-bold text-primary-text">{activeNode.name}</h3>
                <span className="text-xs font-mono text-secondary-text">{activeNode.crate}</span>
              </div>
            </div>

            <div className="space-y-4 text-xs">
              <div>
                <h4 className="font-mono text-[11px] uppercase tracking-wider text-secondary-text mb-1.5">
                  Crate Responsibility
                </h4>
                <p className="text-primary-text/90 leading-relaxed bg-[#08090D] p-3 rounded-lg border border-border/80">
                  {activeNode.description}
                </p>
              </div>

              <div>
                <h4 className="font-mono text-[11px] uppercase tracking-wider text-accent mb-1.5">
                  Engineering Invariants
                </h4>
                <p className="text-secondary-text leading-relaxed bg-[#08090D] p-3 rounded-lg border border-border/80">
                  {activeNode.invariants}
                </p>
              </div>

              <div className="pt-2">
                <span className="inline-block text-[11px] font-mono text-secondary-text/70">
                  Dependencies: Unidirectional strictly enforced. Zero circular crate references.
                </span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>
  );
}

