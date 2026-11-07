"use client";

import { Zap, Terminal, Shield, Cpu, Flame, Layers, Box, Code2 } from "lucide-react";
import MarqueeTrack, { MarqueeItem } from "@/components/primitives/MarqueeTrack";

const TICKER_ITEMS: MarqueeItem[] = [
  { icon: <Cpu className="w-3.5 h-3.5" />, label: "16 RUST WORKSPACE CRATES", tag: "ZERO CRASH" },
  { icon: <Terminal className="w-3.5 h-3.5" />, label: "MODEL CONTEXT PROTOCOL (MCP)", tag: "LIVE INTEGRATION" },
  { icon: <Zap className="w-3.5 h-3.5" />, label: "DSPY DECLARATIVE PROMPT OPTIMIZER", tag: "DSPY v2.5" },
  { icon: <Flame className="w-3.5 h-3.5" />, label: "GOOGLE ANTIGRAVITY ENGINE", tag: "SUB-MILLISECOND" },
  { icon: <Code2 className="w-3.5 h-3.5" />, label: "CLAUDE CODE & CURSOR MDC", tag: "UNIFIED SYNCS" },
  { icon: <Shield className="w-3.5 h-3.5" />, label: "100% PASSING CI WORKFLOW", tag: "TEST DRIVEN" },
  { icon: <Layers className="w-3.5 h-3.5" />, label: "PROGRESSIVE DISCLOSURE ARCHITECTURE", tag: "15 SKILLS" },
  { icon: <Box className="w-3.5 h-3.5" />, label: "OPENCODE AGENT ADAPTER", tag: "NATIVE COMPLIANT" },
];

export default function MarqueeTicker() {
  return (
    <MarqueeTrack items={TICKER_ITEMS} pauseOnHover={true} />
  );
}
