"use client";

import {
  FileText,
  Sparkles,
  Network,
  Code2,
  Terminal,
  ShieldCheck,
  CheckCircle2,
  Users2,
} from "lucide-react";
import { cn } from "@/lib/utils";

export type DomainIconType =
  | "rules"
  | "skills"
  | "mcp"
  | "languages"
  | "prompt"
  | "validation"
  | "security"
  | "multi-agent";

interface Domain3DIconProps {
  type: DomainIconType;
  className?: string;
  size?: "sm" | "md" | "lg";
}

export default function Domain3DIcon({
  type,
  className,
  size = "md",
}: Domain3DIconProps) {
  const sizeMap = {
    sm: "w-9 h-9 p-2",
    md: "w-12 h-12 p-2.5",
    lg: "w-16 h-16 p-3.5",
  };

  const iconSizes = {
    sm: "w-5 h-5",
    md: "w-6 h-6",
    lg: "w-8 h-8",
  };

  const getDomainConfig = () => {
    switch (type) {
      case "rules":
        return {
          icon: FileText,
          bg: "bg-amber-500/10 border-amber-500/40 text-primary-gold",
          label: "RULES",
        };
      case "skills":
        return {
          icon: Sparkles,
          bg: "bg-warm-orange/10 border-warm-orange/40 text-warm-orange",
          label: "SKILLS",
        };
      case "mcp":
        return {
          icon: Network,
          bg: "bg-sky-500/10 border-sky-500/40 text-sky-400",
          label: "MCP",
        };
      case "languages":
        return {
          icon: Code2,
          bg: "bg-emerald-500/10 border-emerald-500/40 text-emerald-400",
          label: "TOOLCHAINS",
        };
      case "prompt":
        return {
          icon: Terminal,
          bg: "bg-purple-500/10 border-purple-500/40 text-purple-400",
          label: "DSPY_PROMPT",
        };
      case "validation":
        return {
          icon: CheckCircle2,
          bg: "bg-emerald-500/10 border-emerald-500/40 text-emerald-300",
          label: "GATES",
        };
      case "security":
        return {
          icon: ShieldCheck,
          bg: "bg-red-500/10 border-red-500/40 text-red-400",
          label: "SECURITY",
        };
      case "multi-agent":
        return {
          icon: Users2,
          bg: "bg-primary-gold/10 border-primary-gold/40 text-primary-gold",
          label: "SYNTHESIS",
        };
    }
  };

  const config = getDomainConfig();
  const IconComponent = config.icon;

  return (
    <div
      className={cn(
        "rounded-xl border-2 flex items-center justify-center shadow-hard-sm transition-all hover:scale-105",
        config.bg,
        sizeMap[size],
        className
      )}
      title={config.label}
    >
      <IconComponent className={iconSizes[size]} />
    </div>
  );
}
