"use client";

import { motion } from "motion/react";
import { Zap, Terminal, Shield, Cpu, Flame, Layers, Box, Code2 } from "lucide-react";

const TICKER_ITEMS = [
  { icon: Cpu, label: "16 RUST WORKSPACE CRATES", tag: "ZERO CRASH" },
  { icon: Terminal, label: "MODEL CONTEXT PROTOCOL (MCP)", tag: "LIVE INTEGRATION" },
  { icon: Zap, label: "DSPY DECLARATIVE PROMPT OPTIMIZER", tag: "DSPY v2.5" },
  { icon: Flame, label: "GOOGLE ANTIGRAVITY ENGINE", tag: "SUB-MILLISECOND" },
  { icon: Code2, label: "CLAUDE CODE & CURSOR MDC", tag: "UNIFIED SYNCS" },
  { icon: Shield, label: "100% PASSING CI WORKFLOW", tag: "TEST DRIVEN" },
  { icon: Layers, label: "PROGRESSIVE DISCLOSURE ARCHITECTURE", tag: "15 SKILLS" },
  { icon: Box, label: "OPENCODE AGENT ADAPTER", tag: "NATIVE COMPLIANT" },
];

export default function MarqueeTicker() {
  return (
    <div className="relative w-full overflow-hidden bg-plum border-y-2 border-popcorn/30 py-3 select-none">
      {/* Subtle background glow */}
      <div className="absolute inset-0 bg-gradient-to-r from-plum-deep via-transparent to-plum-deep z-10 pointer-events-none" />

      <div className="flex w-max animate-marquee gap-8 items-center">
        {[...TICKER_ITEMS, ...TICKER_ITEMS, ...TICKER_ITEMS].map((item, index) => {
          const Icon = item.icon;
          return (
            <div
              key={index}
              className="inline-flex items-center gap-3 px-4 py-1.5 rounded-lg border border-popcorn/20 bg-plum-surface/80 text-xs font-mono tracking-wider shadow-[2px_2px_0px_rgba(248,216,120,0.15)] group hover:border-popcorn transition-colors"
            >
              <Icon className="w-3.5 h-3.5 text-popcorn" />
              <span className="text-ivory font-bold">{item.label}</span>
              <span className="px-1.5 py-0.5 rounded text-[10px] bg-popcorn/15 text-popcorn font-semibold border border-popcorn/30">
                {item.tag}
              </span>
              <span className="text-cerulean font-bold">★</span>
            </div>
          );
        })}
      </div>
    </div>
  );
}
