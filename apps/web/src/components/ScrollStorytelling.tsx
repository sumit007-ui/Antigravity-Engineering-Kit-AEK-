"use client";

import { useState } from "react";
import { motion, AnimatePresence } from "motion/react";
import { Layers, Puzzle, Cpu, Network, CheckCircle2, Bot, ArrowRight } from "lucide-react";
import Domain3DIcon from "@/components/three/Domain3DIcon";

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
    <section className="relative py-28 px-4 sm:px-6 lg:px-8 border-t border-espresso-border bg-espresso-deep overflow-hidden">
      {/* Background ambient radial glow */}
      <div className="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[700px] h-[700px] rounded-full bg-radial-gradient opacity-30 pointer-events-none" />

      <div className="max-w-content mx-auto">
        <div className="text-center max-w-3xl mx-auto mb-16">
          <div className="inline-flex items-center gap-2 px-3.5 py-1.5 rounded-full border border-flamingo/40 bg-brandy/20 text-xs font-semibold text-custard tracking-widest uppercase mb-4">
            System Evolution
          </div>
          <h2 className="font-editorial text-4xl sm:text-5xl lg:text-6xl text-ivory tracking-tight mb-4">
            From fragmented tools to <br className="hidden sm:inline" />
            <span className="italic text-gradient-editorial">one engineering system.</span>
          </h2>
          <p className="text-secondary-text text-base sm:text-lg">
            How AEK unifies disconnected AI tools into a single, high-performance developer foundation.
          </p>
        </div>

        {/* Interactive Scene Controller */}
        <div className="grid grid-cols-1 lg:grid-cols-12 gap-8 items-center">
          {/* Left Narrative Stepper */}
          <div className="lg:col-span-5 space-y-4">
            {scenes.map((scene, idx) => {
              const isActive = activeScene === idx;
              return (
                <button
                  key={scene.id}
                  onClick={() => setActiveScene(idx)}
                  className={`w-full text-left p-6 rounded-2xl transition-all duration-300 border ${
                    isActive
                      ? "bg-espresso-surface border-flamingo shadow-tactile glow-flamingo"
                      : "bg-espresso/40 border-espresso-border hover:border-custard/30 hover:bg-espresso-surface/60"
                  }`}
                  aria-pressed={isActive}
                >
                  <div className="flex items-center justify-between mb-2">
                    <span
                      className={`text-xs font-mono font-bold tracking-wider ${
                        isActive ? "text-flamingo" : "text-muted-text"
                      }`}
                    >
                      {scene.tag}
                    </span>
                    {isActive && (
                      <span className="w-2 h-2 rounded-full bg-flamingo animate-ping" />
                    )}
                  </div>
                  <h3
                    className={`font-editorial text-xl sm:text-2xl mb-1 ${
                      isActive ? "text-ivory" : "text-custard/80"
                    }`}
                  >
                    {scene.title}
                  </h3>
                  <p className="text-xs sm:text-sm text-sand/80 font-medium mb-2">
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
          <div className="lg:col-span-7 h-[460px] sm:h-[520px] rounded-3xl ceramic-card relative p-8 flex items-center justify-center overflow-hidden border border-espresso-border">
            {/* Fine coordinate markers */}
            <div className="absolute top-4 left-4 font-mono text-[11px] text-muted-text">
              SYS::SCENE_0{activeScene + 1}
            </div>
            <div className="absolute top-4 right-4 font-mono text-[11px] text-muted-text">
              ORCHESTRATION_CORE
            </div>
            <div className="absolute bottom-4 left-4 font-mono text-[11px] text-muted-text">
              RUST_2024::SAFE_SUBPROCESS
            </div>

            <AnimatePresence mode="wait">
              {activeScene === 0 && (
                <motion.div
                  key="scene-1"
                  initial={{ opacity: 0, scale: 0.9 }}
                  animate={{ opacity: 1, scale: 1 }}
                  exit={{ opacity: 0, scale: 0.9 }}
                  transition={{ duration: 0.5 }}
                  className="w-full h-full relative flex items-center justify-center"
                >
                  <div className="absolute -top-4 -left-2 p-4 rounded-xl bg-espresso-elevated/90 border border-red-500/30 text-xs text-red-200">
                    ⚠️ Conflicting .cursorrules
                  </div>
                  <div className="absolute top-8 right-0 p-4 rounded-xl bg-espresso-elevated/90 border border-amber-500/30 text-xs text-amber-200">
                    ⚠️ Ad-hoc MCP scripts
                  </div>
                  <div className="absolute -bottom-2 left-6 p-4 rounded-xl bg-espresso-elevated/90 border border-red-500/30 text-xs text-red-200">
                    ⚠️ Unchecked prompt drift
                  </div>
                  <div className="absolute bottom-10 right-4 p-4 rounded-xl bg-espresso-elevated/90 border border-amber-500/30 text-xs text-amber-200">
                    ⚠️ Broken shell escapes
                  </div>

                  {/* Scattered drifting modules */}
                  <div className="grid grid-cols-2 gap-10">
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
                  initial={{ opacity: 0, scale: 0.8 }}
                  animate={{ opacity: 1, scale: 1 }}
                  exit={{ opacity: 0, scale: 0.8 }}
                  transition={{ duration: 0.5 }}
                  className="w-full h-full relative flex items-center justify-center"
                >
                  {/* Inward converging arrows & nodes */}
                  <div className="relative w-64 h-64 rounded-full border border-dashed border-flamingo/40 flex items-center justify-center animate-spin-slow">
                    <div className="absolute -top-5"><Domain3DIcon type="rules" size="sm" /></div>
                    <div className="absolute -bottom-5"><Domain3DIcon type="skills" size="sm" /></div>
                    <div className="absolute -left-5"><Domain3DIcon type="mcp" size="sm" /></div>
                    <div className="absolute -right-5"><Domain3DIcon type="languages" size="sm" /></div>
                  </div>

                  <div className="absolute w-28 h-28 rounded-2xl bg-gradient-to-tr from-brandy to-flamingo border border-custard flex flex-col items-center justify-center shadow-tactile glow-flamingo">
                    <Cpu className="w-8 h-8 text-ivory mb-1" />
                    <span className="font-editorial text-sm font-bold text-ivory">AEK Core</span>
                  </div>
                </motion.div>
              )}

              {activeScene === 2 && (
                <motion.div
                  key="scene-3"
                  initial={{ opacity: 0, scale: 0.9 }}
                  animate={{ opacity: 1, scale: 1 }}
                  exit={{ opacity: 0, scale: 0.9 }}
                  transition={{ duration: 0.5 }}
                  className="w-full h-full flex flex-col items-center justify-center text-center p-6"
                >
                  <div className="w-32 h-32 rounded-3xl bg-gradient-to-br from-brandy via-espresso-surface to-brandy-dark border-2 border-custard flex items-center justify-center shadow-tactile-hover glow-brandy mb-6">
                    <div className="text-center">
                      <span className="font-editorial text-3xl font-bold text-ivory">AEK</span>
                      <span className="block text-[10px] font-mono text-custard tracking-widest">
                        v0.1.0 RUST
                      </span>
                    </div>
                  </div>

                  <div className="flex flex-wrap gap-2 justify-center max-w-md">
                    <span className="px-3 py-1 rounded-full text-xs font-mono bg-brandy/30 border border-brandy-light text-custard">
                      ✓ Precedence Engine
                    </span>
                    <span className="px-3 py-1 rounded-full text-xs font-mono bg-brandy/30 border border-brandy-light text-custard">
                      ✓ SHA-256 Tracking
                    </span>
                    <span className="px-3 py-1 rounded-full text-xs font-mono bg-brandy/30 border border-brandy-light text-custard">
                      ✓ 8-Stage DAG Quality
                    </span>
                    <span className="px-3 py-1 rounded-full text-xs font-mono bg-brandy/30 border border-brandy-light text-custard">
                      ✓ Zero Telemetry
                    </span>
                  </div>
                </motion.div>
              )}

              {activeScene === 3 && (
                <motion.div
                  key="scene-4"
                  initial={{ opacity: 0, scale: 0.9 }}
                  animate={{ opacity: 1, scale: 1 }}
                  exit={{ opacity: 0, scale: 0.9 }}
                  transition={{ duration: 0.5 }}
                  className="w-full h-full flex flex-col items-center justify-center"
                >
                  <div className="w-20 h-20 rounded-2xl bg-brandy border border-custard flex items-center justify-center shadow-tactile mb-8">
                    <span className="font-editorial font-bold text-xl text-ivory">AEK</span>
                  </div>

                  <div className="grid grid-cols-2 sm:grid-cols-4 gap-4 w-full max-w-lg">
                    {[
                      { name: "Antigravity", color: "border-sky-500/40" },
                      { name: "Claude Code", color: "border-amber-500/40" },
                      { name: "Cursor", color: "border-purple-500/40" },
                      { name: "OpenCode", color: "border-emerald-500/40" },
                    ].map((agent) => (
                      <div
                        key={agent.name}
                        className={`p-3 rounded-xl bg-espresso-surface border ${agent.color} text-center`}
                      >
                        <Bot className="w-5 h-5 mx-auto mb-1 text-custard" />
                        <span className="text-xs font-medium text-ivory block">
                          {agent.name}
                        </span>
                        <span className="text-[10px] text-flamingo font-mono">
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

