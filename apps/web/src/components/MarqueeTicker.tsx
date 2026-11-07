"use client";

import Image from "next/image";
import { Cpu, Terminal, Zap, Shield, Flame } from "lucide-react";
import MarqueeTrack, { MarqueeItem } from "@/components/primitives/MarqueeTrack";
import AgentLogo from "@/components/AgentLogo";

const TICKER_ITEMS: MarqueeItem[] = [
  {
    icon: (
      <Image
        src="/icons/symbol.png"
        alt="AEK Emblem"
        width={18}
        height={18}
        className="w-4 h-4 object-contain filter drop-shadow-[0_0_6px_rgba(244,102,27,0.5)]"
      />
    ),
    label: "ANTIGRAVITY ENGINEERING KIT",
    tag: "AEK v0.1.0",
    glowColor: "#F4661B",
  },
  {
    icon: <AgentLogo agent="antigravity" className="w-4 h-4" size={16} />,
    label: "GOOGLE ANTIGRAVITY ENGINE",
    tag: "SUB-MILLISECOND",
    glowColor: "#4285F4",
  },
  {
    icon: <AgentLogo agent="claude" className="w-4 h-4" size={16} />,
    label: "CLAUDE CODE ENGINEERING",
    tag: "UNIFIED SYNCS",
    glowColor: "#D97757",
  },
  {
    icon: <AgentLogo agent="cursor" className="w-4 h-4" size={16} />,
    label: "CURSOR AI MDC COMPLIANT",
    tag: "ISOMETRIC CORE",
    glowColor: "#A78BFA",
  },
  {
    icon: <AgentLogo agent="opencode" className="w-4 h-4" size={16} />,
    label: "OPENCODE AGENT ADAPTER",
    tag: "NATIVE COMPLIANT",
    glowColor: "#10B981",
  },
  {
    icon: <Cpu className="w-4 h-4 text-[#F74C00]" />,
    label: "16 RUST WORKSPACE CRATES",
    tag: "ZERO CRASH",
    glowColor: "#F74C00",
  },
  {
    icon: <Terminal className="w-4 h-4 text-[#FFD36B]" />,
    label: "MODEL CONTEXT PROTOCOL (MCP)",
    tag: "LIVE INTEGRATION",
    glowColor: "#FFD36B",
  },
  {
    icon: <Zap className="w-4 h-4 text-[#C084FC]" />,
    label: "DSPY DECLARATIVE PROMPT OPTIMIZER",
    tag: "DSPY v2.5",
    glowColor: "#C084FC",
  },
  {
    icon: <Shield className="w-4 h-4 text-[#22C55E]" />,
    label: "100% PASSING CI WORKFLOW",
    tag: "TEST DRIVEN",
    glowColor: "#22C55E",
  },
];

export default function MarqueeTicker() {
  return <MarqueeTrack items={TICKER_ITEMS} pauseOnHover={true} />;
}
