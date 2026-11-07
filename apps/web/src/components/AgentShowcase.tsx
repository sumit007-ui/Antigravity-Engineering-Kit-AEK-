"use client";

import { useState } from "react";
import Link from "next/link";
import { ArrowRight, ShieldCheck, FileCode } from "lucide-react";
import SectionHeader from "@/components/primitives/SectionHeader";
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
    <section className="py-20 sm:py-28 px-3.5 sm:px-6 lg:px-8 bg-space-deep border-t border-space-border relative overflow-hidden w-full max-w-full">
      <div className="max-w-content mx-auto w-full min-w-0">
        <SectionHeader
          sectionNum="05"
          sectionCode="SEC_05"
          title="One kit. Every AI coding agent."
          subtitle="AEK translates your engineering rules, skills, and MCP tools natively into the target agent's exact configuration dialect."
          align="center"
        />

        {/* 4 Agent Selection Cards with Official Logos */}
        <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-3 sm:gap-4 mb-8 mt-10 sm:mt-12 w-full min-w-0">
          {AGENTS.map((agent) => {
            const isSelected = selectedAgent === agent.id;
            return (
              <button
                key={agent.id}
                onClick={() => setSelectedAgent(agent.id)}
                className={`p-4 sm:p-5 rounded-2xl text-left transition-all border-2 group cursor-pointer min-w-0 ${
                  isSelected
                    ? "bg-space-surface border-warm-orange shadow-hard-md"
                    : "bg-space-black/80 border-space-border/80 hover:border-primary-gold/60 shadow-hard-sm"
                }`}
              >
                <div className="flex items-center justify-between mb-3">
                  <div className="w-10 h-10 rounded-xl bg-space-deep border border-space-border flex items-center justify-center group-hover:border-primary-gold/50 transition-colors">
                    <AgentLogo agent={agent.id as AgentId} size={22} className="grayscale group-hover:grayscale-0 transition-all duration-160 group-hover:scale-110" />
                  </div>
                  {isSelected ? (
                    <span className="px-2 py-0.5 rounded text-[10px] font-mono font-bold bg-primary-gold text-space-black uppercase">
                      ACTIVE
                    </span>
                  ) : (
                    <span className="w-2 h-2 rounded-full bg-space-border group-hover:bg-warm-orange transition-colors" />
                  )}
                </div>
                <h3 className="font-editorial text-base sm:text-lg text-paper-beige mb-0.5 font-bold truncate">{agent.name}</h3>
                <p className="text-[11px] font-mono text-sand/70 mb-2 truncate">{agent.provider}</p>
                <div className="text-[11px] font-mono text-emerald-400 flex items-center gap-1">
                  <ShieldCheck className="w-3.5 h-3.5 shrink-0" />
                  <span className="truncate">{agent.status}</span>
                </div>
              </button>
            );
          })}
        </div>

        {/* Detailed Inspector Card */}
        <div className="rounded-2xl sm:rounded-3xl border-2 border-primary-gold/40 bg-space-black p-4 sm:p-6 lg:p-8 shadow-hard-md grid grid-cols-1 lg:grid-cols-12 gap-6 sm:gap-8 items-center overflow-hidden w-full min-w-0 max-w-full">
          {/* Left Details */}
          <div className="lg:col-span-6 space-y-5 sm:space-y-6 overflow-hidden w-full min-w-0">
            <div>
              <span className="text-xs font-mono font-bold uppercase tracking-wider text-primary-gold">
                ✦ NATIVE TRANSLATION MATRIX
              </span>
              <div className="flex items-center gap-2.5 sm:gap-3 mt-2 mb-2 min-w-0">
                <div className="w-9 h-9 sm:w-10 sm:h-10 rounded-xl bg-space-surface border border-space-border flex items-center justify-center shrink-0">
                  <AgentLogo agent={active.id as AgentId} size={22} />
                </div>
                <h3 className="font-editorial text-2xl sm:text-3xl text-paper-beige font-bold truncate min-w-0">
                  {active.name}
                </h3>
              </div>
              <p className="text-xs font-mono text-warm-orange font-semibold truncate">{active.provider}</p>
            </div>

            <div className="space-y-2.5 sm:space-y-3 text-xs w-full min-w-0">
              <div className="p-3 sm:p-3.5 rounded-xl bg-space-surface border border-space-border overflow-hidden min-w-0">
                <span className="font-mono text-primary-gold text-[11px] sm:text-xs font-bold block mb-1">RULES CONFIGURATION:</span>
                <span className="font-mono text-sand text-[11px] sm:text-xs break-all block leading-relaxed">{active.rulesConfig}</span>
              </div>
              <div className="p-3 sm:p-3.5 rounded-xl bg-space-surface border border-space-border overflow-hidden min-w-0">
                <span className="font-mono text-primary-gold text-[11px] sm:text-xs font-bold block mb-1">MCP INTEGRATION PATH:</span>
                <span className="font-mono text-sand text-[11px] sm:text-xs break-all block leading-relaxed">{active.mcpConfig}</span>
              </div>
              <div className="p-3 sm:p-3.5 rounded-xl bg-space-surface border border-space-border overflow-hidden min-w-0">
                <span className="font-mono text-primary-gold text-[11px] sm:text-xs font-bold block mb-1">SKILLS RUNBOOKS:</span>
                <span className="font-mono text-sand text-[11px] sm:text-xs break-all block leading-relaxed">{active.skillsConfig}</span>
              </div>
            </div>

            <div className="pt-2">
              <Link
                href="/agents"
                className="inline-flex items-center gap-2 text-xs font-semibold text-primary-gold hover:text-paper-beige font-mono"
              >
                <span>View Full Agent Capability Matrix</span>
                <ArrowRight className="w-3.5 h-3.5 text-warm-orange shrink-0" />
              </Link>
            </div>
          </div>

          {/* Right Live Configuration Preview */}
          <div className="lg:col-span-6 rounded-2xl overflow-hidden border-2 border-space-border bg-space-deep shadow-hard-sm w-full min-w-0 max-w-full">
            <div className="bg-space-surface px-4 py-2.5 border-b border-space-border flex items-center justify-between text-xs font-mono text-sand min-w-0">
              <span className="flex items-center gap-1.5 text-primary-gold font-bold min-w-0 truncate">
                <FileCode className="w-3.5 h-3.5 text-warm-orange shrink-0" />
                <span className="truncate">{active.previewFile}</span>
              </span>
              <span className="text-[10px] text-sand/60 shrink-0">AUTOGENERATED BY AEK</span>
            </div>
            <pre className="p-4 sm:p-5 font-mono text-[11px] sm:text-xs text-sand/90 overflow-x-auto leading-relaxed max-w-full">
              <code>{active.codePreview}</code>
            </pre>
          </div>
        </div>
      </div>
    </section>
  );
}
