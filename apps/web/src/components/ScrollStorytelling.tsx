"use client";

import { useState } from "react";
import { motion, AnimatePresence } from "motion/react";
import { Cpu, Bot } from "lucide-react";
import Domain3DIcon from "@/components/three/Domain3DIcon";
import SectionHeader from "@/components/primitives/SectionHeader";
import AgentLogo from "@/components/AgentLogo";

const scenes = [
  {
    id: 1,
    title: "1. The Fragmented Reality",
    subtitle: "Isolated prompts, ad-hoc configs, brittle MCP scripts",
    description:
      "Modern development teams manage conflicting rule files, unvalidated MCP definitions, and disparate setup commands across every different AI assistant.",
    tag: "DISCONNECTED WORKFLOWS",
  },
  {
    id: 2,
    title: "2. Precision Convergence",
    subtitle: "Consolidating 8 engineering domains into one Rust core",
    description:
      "Rules, Skills, Toolchains, Prompt templates, and Verification pipelines converge into AEK's memory-safe, sub-millisecond execution engine.",
    tag: "UNIFIED RUST CORE",
  },
  {
    id: 3,
    title: "3. The Unified Engine",
    subtitle: "Cryptographic state, atomic rollbacks, standardized gates",
    description:
      "AEK resolves precedence (Global → Workspace → Project), audits security policies, discovers MCP tools, and runs strict quality gates.",
    tag: "ONE SPECIFICATION",
  },
  {
    id: 4,
    title: "4. Multi-Agent Synthesis",
    subtitle: "Native translation into Antigravity, Claude Code, Cursor & OpenCode",
    description:
      "Write once in AEK. Sync cleanly to any agent's native format with SHA-256 state tracking and conflict-free zero-overwrite guarantees.",
    tag: "UNIVERSAL HARMONY",
  },
];

export default function ScrollStorytelling() {
  const [activeScene, setActiveScene] = useState(0);

  return (
    <section className="relative py-28 px-4 sm:px-6 lg:px-8 border-t border-space-border bg-space-deep overflow-hidden">
      {/* Background ambient radial glow */}
      <div className="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[700px] h-[700px] rounded-full bg-radial-gradient opacity-20 pointer-events-none" />

      <div className="max-w-content mx-auto">
        <SectionHeader
          sectionNum="02"
          coordinates="15°29'N 73°49'E • SEC_02"
          title="From fragmented tools to one engineering system."
          subtitle="How AEK unifies disconnected AI tools into a single, high-performance developer foundation."
          align="center"
        />

        {/* Interactive Scene Controller */}
        <div className="grid grid-cols-1 lg:grid-cols-12 gap-8 items-center mt-12">
          {/* Left Narrative Stepper */}
          <div className="lg:col-span-5 space-y-4">
            {scenes.map((scene, idx) => {
              const isActive = activeScene === idx;
              return (
                <button
                  key={scene.id}
                  onClick={() => setActiveScene(idx)}
                  className={`w-full text-left p-6 rounded-2xl transition-all border-2 ${
                    isActive
                      ? "bg-space-surface border-warm-orange shadow-hard-md text-paper-beige"
                      : "bg-space-black/50 border-space-border/80 hover:border-primary-gold/40 hover:bg-space-surface/60 text-sand/80"
                  }`}
                  aria-pressed={isActive}
                >
                  <div className="flex items-center justify-between mb-2">
                    <span
                      className={`text-xs font-mono font-bold tracking-wider ${
                        isActive ? "text-primary-gold" : "text-sand/60"
                      }`}
                    >
                      {scene.tag}
                    </span>
                    {isActive && (
                      <span className="w-2.5 h-2.5 rounded-full bg-warm-orange animate-pulse" />
                    )}
                  </div>
                  <h3
                    className={`font-editorial text-xl sm:text-2xl mb-1 ${
                      isActive ? "text-paper-beige font-semibold" : "text-paper-beige/80"
                    }`}
                  >
                    {scene.title}
                  </h3>
                  <p className="text-xs sm:text-sm text-sand/90 font-medium mb-2">
                    {scene.subtitle}
                  </p>
                  <p className="text-xs sm:text-sm text-secondary-text leading-relaxed">
                    {scene.description}
                  </p>
                </button>
              );
            })}
          </div>

          {/* Right Visual Stage */}
          <div className="lg:col-span-7 h-[460px] sm:h-[520px] rounded-3xl bg-space-black border-2 border-primary-gold/30 relative p-8 flex items-center justify-center overflow-hidden shadow-hard-md">
            {/* Fine coordinate markers */}
            <div className="absolute top-4 left-4 font-mono text-[11px] text-primary-gold/70">
              SYS::SCENE_0{activeScene + 1}
            </div>
            <div className="absolute top-4 right-4 font-mono text-[11px] text-primary-gold/70">
              ORCHESTRATION_CORE
            </div>
            <div className="absolute bottom-4 left-4 font-mono text-[11px] text-sand/60">
              RUST_2024::SAFE_SUBPROCESS
            </div>

            <AnimatePresence mode="wait">
              {activeScene === 0 && (
                <motion.div
                  key="scene-1"
                  initial={{ opacity: 0, scale: 0.95 }}
                  animate={{ opacity: 1, scale: 1 }}
                  exit={{ opacity: 0, scale: 0.95 }}
                  transition={{ duration: 0.3 }}
                  className="w-full h-full relative flex items-center justify-center"
                >
                  <div className="absolute -top-2 -left-2 p-3 rounded-xl bg-space-surface border-2 border-red-500/40 text-xs text-red-300 font-mono shadow-hard-sm">
                    ⚠️ Conflicting .cursorrules
                  </div>
                  <div className="absolute top-8 right-0 p-3 rounded-xl bg-space-surface border-2 border-amber-500/40 text-xs text-amber-300 font-mono shadow-hard-sm">
                    ⚠️ Ad-hoc MCP scripts
                  </div>
                  <div className="absolute -bottom-2 left-6 p-3 rounded-xl bg-space-surface border-2 border-red-500/40 text-xs text-red-300 font-mono shadow-hard-sm">
                    ⚠️ Unchecked prompt drift
                  </div>
                  <div className="absolute bottom-8 right-4 p-3 rounded-xl bg-space-surface border-2 border-amber-500/40 text-xs text-amber-300 font-mono shadow-hard-sm">
                    ⚠️ Broken shell escapes
                  </div>

                  {/* Scattered drifting modules */}
                  <div className="grid grid-cols-2 gap-8">
                    <Domain3DIcon type="rules" size="lg" />
                    <Domain3DIcon type="skills" size="lg" />
                    <Domain3DIcon type="mcp" size="lg" />
                    <Domain3DIcon type="prompt" size="lg" />
                  </div>
                </motion.div>
              )}

              {activeScene === 1 && (
                <motion.div
                  key="scene-2"
                  initial={{ opacity: 0, scale: 0.95 }}
                  animate={{ opacity: 1, scale: 1 }}
                  exit={{ opacity: 0, scale: 0.95 }}
                  transition={{ duration: 0.3 }}
                  className="w-full h-full relative flex items-center justify-center"
                >
                  <div className="relative w-64 h-64 rounded-full border-2 border-dashed border-warm-orange/40 flex items-center justify-center">
                    <div className="absolute -top-5"><Domain3DIcon type="rules" size="sm" /></div>
                    <div className="absolute -bottom-5"><Domain3DIcon type="skills" size="sm" /></div>
                    <div className="absolute -left-5"><Domain3DIcon type="mcp" size="sm" /></div>
                    <div className="absolute -right-5"><Domain3DIcon type="languages" size="sm" /></div>
                  </div>

                  <div className="absolute w-32 h-32 rounded-2xl bg-space-surface border-2 border-primary-gold flex flex-col items-center justify-center shadow-hard-md">
                    <Cpu className="w-8 h-8 text-primary-gold mb-1" />
                    <span className="font-editorial text-base font-bold text-paper-beige">AEK Core</span>
                    <span className="text-[10px] font-mono text-warm-orange">Zero-Cost</span>
                  </div>
                </motion.div>
              )}

              {activeScene === 2 && (
                <motion.div
                  key="scene-3"
                  initial={{ opacity: 0, scale: 0.95 }}
                  animate={{ opacity: 1, scale: 1 }}
                  exit={{ opacity: 0, scale: 0.95 }}
                  transition={{ duration: 0.3 }}
                  className="w-full h-full flex flex-col items-center justify-center text-center p-6"
                >
                  <div className="w-36 h-36 rounded-3xl bg-space-surface border-2 border-primary-gold flex items-center justify-center shadow-hard-md mb-6">
                    <div className="text-center">
                      <span className="font-editorial text-4xl font-bold text-paper-beige">AEK</span>
                      <span className="block text-[11px] font-mono text-primary-gold tracking-widest mt-1">
                        v0.1.0 RUST
                      </span>
                    </div>
                  </div>

                  <div className="flex flex-wrap gap-2.5 justify-center max-w-md">
                    <span className="px-3 py-1 rounded-full text-xs font-mono bg-space-surface border border-primary-gold/40 text-primary-gold shadow-hard-sm">
                      ✓ Precedence Engine
                    </span>
                    <span className="px-3 py-1 rounded-full text-xs font-mono bg-space-surface border border-primary-gold/40 text-primary-gold shadow-hard-sm">
                      ✓ SHA-256 Tracking
                    </span>
                    <span className="px-3 py-1 rounded-full text-xs font-mono bg-space-surface border border-warm-orange/40 text-warm-orange shadow-hard-sm">
                      ✓ 8-Stage DAG Quality
                    </span>
                    <span className="px-3 py-1 rounded-full text-xs font-mono bg-space-surface border border-primary-gold/40 text-primary-gold shadow-hard-sm">
                      ✓ Zero Telemetry
                    </span>
                  </div>
                </motion.div>
              )}

              {activeScene === 3 && (
                <motion.div
                  key="scene-4"
                  initial={{ opacity: 0, scale: 0.95 }}
                  animate={{ opacity: 1, scale: 1 }}
                  exit={{ opacity: 0, scale: 0.95 }}
                  transition={{ duration: 0.3 }}
                  className="w-full h-full flex flex-col items-center justify-center"
                >
                  <div className="w-24 h-24 rounded-2xl bg-space-surface border-2 border-primary-gold flex items-center justify-center shadow-hard-md mb-8">
                    <span className="font-editorial font-bold text-2xl text-paper-beige">AEK</span>
                  </div>

                  <div className="grid grid-cols-2 sm:grid-cols-4 gap-4 w-full max-w-lg">
                    {[
                      { id: "antigravity" as const, name: "Antigravity", color: "border-sky-500/50" },
                      { id: "claude" as const, name: "Claude Code", color: "border-amber-500/50" },
                      { id: "cursor" as const, name: "Cursor", color: "border-purple-500/50" },
                      { id: "opencode" as const, name: "OpenCode", color: "border-emerald-500/50" },
                    ].map((agent) => (
                      <div
                        key={agent.name}
                        className={`p-3 rounded-xl bg-space-surface border-2 ${agent.color} text-center shadow-hard-sm`}
                      >
                        <div className="w-6 h-6 mx-auto mb-1.5 flex items-center justify-center">
                          <AgentLogo agent={agent.id} size={20} />
                        </div>
                        <span className="text-xs font-bold text-paper-beige block">
                          {agent.name}
                        </span>
                        <span className="text-[10px] text-primary-gold font-mono">
                          Native Sync
                        </span>
                      </div>
                    ))}
                  </div>
                </motion.div>
              )}
            </AnimatePresence>
          </div>
        </div>
      </div>
    </section>
  );
}
