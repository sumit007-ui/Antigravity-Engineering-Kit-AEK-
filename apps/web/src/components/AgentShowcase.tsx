"use client";

import { useState } from "react";
import Link from "next/link";
import { Check, ArrowRight, ShieldCheck, FileCode, Layers } from "lucide-react";
import TiltCard from "@/components/motion/TiltCard";
import AgentLogo, { AgentId } from "@/components/AgentLogo";

interface AgentInfo {
  id: string;
  name: string;
  provider: string;
  accent: string;
  status: string;
  rulesConfig: string;
  mcpConfig: string;
  skillsConfig: string;
  previewFile: string;
  codePreview: string;
}

const AGENTS: AgentInfo[] = [
  {
    id: "antigravity",
    name: "Google Gemini Antigravity",
    provider: "Google DeepMind Ecosystem",
    accent: "text-sky-400",
    status: "100% Native Architecture",
    rulesConfig: "~/.gemini/antigravity/rules/ & workspace .gemini/rules/",
    mcpConfig: "~/.gemini/antigravity/mcp/<server>/mcp.json",
    skillsConfig: "~/.gemini/config/skills/<skill>/SKILL.md",
    previewFile: ".gemini/rules/aek-engineering.md",
    codePreview: `---
trigger: always_on
description: Mandatory TDD, Graphify knowledge graph, and ruthless precision invariants.
---
# Antigravity Engineering Invariants
1. Test-Driven Development (RED -> GREEN -> REFACTOR)
2. Direct argument subprocess execution
3. Token-budgeted rule size bounds (<= 24 KB)`,
  },
  {
    id: "claude",
    name: "Anthropic Claude Code",
    provider: "Claude Developer Platform",
    accent: "text-amber-400",
    status: "Consolidated Translation",
    rulesConfig: "./CLAUDE.md & ~/.claude/CLAUDE.md",
    mcpConfig: ".claude/mcp.json (Standard Stdio/HTTP schema)",
    skillsConfig: ".claude/skills/ or CLAUDE.md progressive sections",
    previewFile: "CLAUDE.md",
    codePreview: `# Claude Code Engineering Rules (AEK Managed)
<!-- aek:managed sha256=9f83... -->
## Quality Gates
- Run cargo test --workspace before all code changes
- Refuse deprecated compiler flags and undocumented APIs
- Maintain zero secret exposure across all diffs`,
  },
  {
    id: "cursor",
    name: "Cursor AI IDE",
    provider: "Anysphere Platform",
    accent: "text-purple-400",
    status: "Modular MDC Translation",
    rulesConfig: ".cursor/rules/*.mdc & .cursorrules",
    mcpConfig: ".cursor/mcp.json",
    skillsConfig: ".cursor/skills/",
    previewFile: ".cursor/rules/aek-invariants.mdc",
    codePreview: `---
description: AEK Universal Engineering Invariants
globs: ["**/*"]
alwaysApply: true
---
# Cursor Engineering Invariants
- Enforce strict static analysis and zero type warnings
- Apply deterministic formatting before committing
- Never execute unchecked shell commands`,
  },
  {
    id: "opencode",
    name: "OpenCode",
    provider: "OpenCode Developer System",
    accent: "text-emerald-400",
    status: "AGENTS.md & Native Config",
    rulesConfig: "./AGENTS.md & .opencode/config.json",
    mcpConfig: ".opencode/config.json mcpServers block",
    skillsConfig: ".opencode/skills/",
    previewFile: "AGENTS.md",
    codePreview: `# OpenCode Agent Architecture (AEK Synchronized)
## Global Invariants
- Execution Mode: ruthless_precision
- Safety Bounds: Memory limit < 50 MB idle
- Continuous Verification: 8-stage verification pipeline`,
  },
];

export default function AgentShowcase() {
  const [selectedAgent, setSelectedAgent] = useState<string>("antigravity");
  const active = AGENTS.find((a) => a.id === selectedAgent) || AGENTS[0];

  return (
    <section className="py-28 px-4 sm:px-6 lg:px-8 bg-espresso border-t border-espresso-border relative">
      <div className="max-w-content mx-auto">
        <div className="text-center max-w-3xl mx-auto mb-16">
          <div className="inline-flex items-center gap-2 px-3 py-1 rounded-full border border-flamingo/40 bg-brandy/20 text-xs font-mono text-custard mb-3">
            ✦ UNIVERSAL COMPATIBILITY
          </div>
          <h2 className="font-editorial text-4xl sm:text-5xl lg:text-6xl text-ivory tracking-tight mb-4">
            One kit. <span className="italic text-gradient-editorial">Every AI coding agent.</span>
          </h2>
          <p className="text-secondary-text text-sm sm:text-base">
            AEK translates your engineering rules, skills, and MCP tools natively into the target agent&apos;s exact configuration dialect.
          </p>
        </div>

        {/* 4 Agent Selection Cards with Official Logos */}
        <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4 mb-8">
          {AGENTS.map((agent) => {
            const isSelected = selectedAgent === agent.id;
            return (
              <button
                key={agent.id}
                onClick={() => setSelectedAgent(agent.id)}
                className={`p-5 rounded-2xl text-left transition-all duration-200 border-2 group cursor-pointer ${
                  isSelected
                    ? "bg-plum border-popcorn shadow-[4px_4px_0px_#F8D878]"
                    : "bg-plum-deep/90 border-plum-border hover:border-cerulean/60 shadow-[3px_3px_0px_rgba(0,0,0,0.4)]"
                }`}
              >
                <div className="flex items-center justify-between mb-3">
                  <div className="w-10 h-10 rounded-xl bg-plum-surface/90 border border-plum-border flex items-center justify-center group-hover:border-popcorn/50 transition-colors">
                    <AgentLogo agent={agent.id as AgentId} size={22} className="transition-transform group-hover:scale-110" />
                  </div>
                  {isSelected ? (
                    <span className="px-2 py-0.5 rounded text-[10px] font-mono font-bold bg-popcorn text-plum-deep uppercase">
                      ACTIVE
                    </span>
                  ) : (
                    <span className="w-2 h-2 rounded-full bg-plum-border group-hover:bg-cerulean transition-colors" />
                  )}
                </div>
                <h3 className="font-editorial text-lg text-ivory mb-0.5 font-bold">{agent.name}</h3>
                <p className="text-[11px] font-mono text-sand/70 mb-2">{agent.provider}</p>
                <div className="text-[11px] font-mono text-emerald-400 flex items-center gap-1">
                  <ShieldCheck className="w-3.5 h-3.5" />
                  <span>{agent.status}</span>
                </div>
              </button>
            );
          })}
        </div>

        {/* Detailed Inspector Card */}
        <div className="rounded-3xl border-2 border-popcorn/30 bg-plum-deep/95 p-8 shadow-[6px_6px_0px_#382C2D] grid grid-cols-1 lg:grid-cols-12 gap-8 items-center">
          {/* Left Details */}
          <div className="lg:col-span-6 space-y-6">
            <div>
              <span className="text-xs font-mono font-bold uppercase tracking-wider text-popcorn">
                ✦ NATIVE TRANSLATION MATRIX
              </span>
              <div className="flex items-center gap-3 mt-2 mb-2">
                <div className="w-10 h-10 rounded-xl bg-plum border border-plum-border flex items-center justify-center">
                  <AgentLogo agent={active.id as AgentId} size={24} />
                </div>
                <h3 className="font-editorial text-3xl text-ivory font-bold">
                  {active.name}
                </h3>
              </div>
              <p className="text-xs font-mono text-cerulean font-semibold">{active.provider}</p>
            </div>

            <div className="space-y-3 text-xs">
              <div className="p-3.5 rounded-xl bg-espresso-deep border border-espresso-border">
                <span className="font-mono text-flamingo block mb-1">RULES CONFIGURATION:</span>
                <span className="font-mono text-sand">{active.rulesConfig}</span>
              </div>
              <div className="p-3.5 rounded-xl bg-espresso-deep border border-espresso-border">
                <span className="font-mono text-flamingo block mb-1">MCP INTEGRATION PATH:</span>
                <span className="font-mono text-sand">{active.mcpConfig}</span>
              </div>
              <div className="p-3.5 rounded-xl bg-espresso-deep border border-espresso-border">
                <span className="font-mono text-flamingo block mb-1">SKILLS RUNBOOKS:</span>
                <span className="font-mono text-sand">{active.skillsConfig}</span>
              </div>
            </div>

            <div className="pt-2">
              <Link
                href="/agents"
                className="inline-flex items-center gap-2 text-xs font-semibold text-custard hover:text-ivory"
              >
                <span>View Full Agent Capability Matrix</span>
                <ArrowRight className="w-3.5 h-3.5 text-flamingo" />
              </Link>
            </div>
          </div>

          {/* Right Live Configuration Preview */}
          <div className="lg:col-span-6 rounded-2xl overflow-hidden border border-espresso-border bg-espresso-deep shadow-tactile">
            <div className="bg-espresso-surface px-4 py-2.5 border-b border-espresso-border flex items-center justify-between text-xs font-mono text-sand">
              <span className="flex items-center gap-1.5 text-custard">
                <FileCode className="w-3.5 h-3.5 text-flamingo" />
                {active.previewFile}
              </span>
              <span className="text-[10px] text-muted-text">AUTOGENERATED BY AEK</span>
            </div>
            <pre className="p-5 font-mono text-xs text-sand/90 overflow-x-auto leading-relaxed">
              <code>{active.codePreview}</code>
            </pre>
          </div>
        </div>
      </div>
    </section>
  );
}
