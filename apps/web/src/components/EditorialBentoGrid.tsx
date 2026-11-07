"use client";

import Link from "next/link";
import { ArrowUpRight } from "lucide-react";
import Domain3DIcon, { DomainIconType } from "@/components/three/Domain3DIcon";
import SectionHeader from "@/components/primitives/SectionHeader";
import { PixelCard, LockedCard } from "@/components/primitives/PixelCard";

interface FeatureCardItem {
  id: string;
  type: DomainIconType;
  title: string;
  category: string;
  description: string;
  command: string;
  badge: string;
  link: string;
  colSpan: string;
}

const FEATURES: FeatureCardItem[] = [
  {
    id: "rules",
    type: "rules",
    title: "Global & Project Engineering Rules",
    category: "STANDARDIZED PROTOCOLS",
    description:
      "Deterministic 3-tier precedence resolution (Global → Workspace → Project). Enforces 24 KB memory bounds, token budgeting, and zero hallucinated requirements.",
    command: "aek rules list --audit",
    badge: "PRECISION ENGINE",
    link: "/features#rules",
    colSpan: "lg:col-span-7",
  },
  {
    id: "skills",
    type: "skills",
    title: "15 Specialized Agent Skills",
    category: "PROGRESSIVE DISCLOSURE",
    description:
      "Curated engineering runbooks across Rust, Go, C++, Zig, Python, TypeScript, Flutter, and ROS 2 robotics. Automatically recommended based on repository manifest detection.",
    command: "aek skills discover",
    badge: "DISCOVERY ENGINE",
    link: "/features#skills",
    colSpan: "lg:col-span-5",
  },
  {
    id: "multi-agent",
    type: "multi-agent",
    title: "Universal Multi-Agent Compatibility",
    category: "AGENT SYNERGY",
    description:
      "One engineering kit that speaks native formats: Google Antigravity, Anthropic Claude Code, Cursor IDE, and OpenCode. Safe synchronization with rollback archives.",
    command: "aek agent sync --all",
    badge: "CROSS-PLATFORM",
    link: "/agents",
    colSpan: "lg:col-span-5",
  },
  {
    id: "mcp",
    type: "mcp",
    title: "Model Context Protocol Client & Presets",
    category: "STANDARDIZED TOOLS",
    description:
      "Full JSON-RPC 2.0 client engine supporting Stdio and HTTP transports. Pre-configured adapters for Context7, GitHub, Playwright, Sequential Thinking, and Filesystem.",
    command: "aek mcp list --json",
    badge: "MCP CLIENT",
    link: "/features#mcp",
    colSpan: "lg:col-span-7",
  },
  {
    id: "validation",
    type: "validation",
    title: "8-Stage Quality Gate Verification",
    category: "DETERMINISTIC GATES",
    description:
      "Automated verification DAG executing formatting, static linting, type-checking, and test suites with incremental Git diff filtering and SARIF / JSON reporting.",
    command: "aek validate --strict",
    badge: "DAG RUNNER",
    link: "/features#validation",
    colSpan: "lg:col-span-6",
  },
  {
    id: "security",
    type: "security",
    title: "Multi-Scanner Security Orchestration",
    category: "SECURITY ASSURANCE",
    description:
      "Deep static analysis combining Semgrep SAST, Trivy vulnerability auditing, and ecosystem package dependency checkers with normalized CVSS severity scoring.",
    command: "aek security audit",
    badge: "LEAST PRIVILEGE",
    link: "/features#security",
    colSpan: "lg:col-span-6",
  },
];

export default function EditorialBentoGrid() {
  return (
    <section className="py-28 px-4 sm:px-6 lg:px-8 bg-space-black border-t border-space-border relative overflow-hidden">
      <div className="max-w-content mx-auto relative z-10">
        <SectionHeader
          sectionNum="03"
          coordinates="15°29'N 73°49'E • SEC_03"
          title="Precision engineering by architectural design."
          subtitle="Sixteen interconnected Rust crates working in harmony to replace fragile prompt engineering with durable systems infrastructure."
        />

        {/* Asymmetric Bento Grid */}
        <div className="grid grid-cols-1 lg:grid-cols-12 gap-6 mt-12">
          {FEATURES.map((feat) => (
            <div key={feat.id} className={feat.colSpan}>
              <PixelCard className="h-full flex flex-col justify-between">
                <div>
                  <div className="flex items-start justify-between mb-6">
                    <Domain3DIcon type={feat.type} size="md" />
                    <span className="text-[10px] font-mono tracking-wider px-2.5 py-1 rounded-md bg-space-deep border border-space-border text-primary-gold">
                      {feat.badge}
                    </span>
                  </div>

                  <p className="text-[11px] font-mono tracking-widest text-warm-orange uppercase font-semibold mb-1">
                    {feat.category}
                  </p>
                  <h3 className="font-editorial text-2xl sm:text-3xl text-paper-beige mb-3 font-medium">
                    {feat.title}
                  </h3>
                  <p className="text-sm text-sand/80 leading-relaxed mb-6">
                    {feat.description}
                  </p>
                </div>

                <div className="pt-4 border-t border-space-border/80 flex items-center justify-between">
                  <code className="text-xs font-mono text-primary-gold bg-space-deep px-3 py-1.5 rounded-lg border border-space-border">
                    $ {feat.command}
                  </code>
                  <Link
                    href={feat.link}
                    className="inline-flex items-center gap-1 text-xs font-medium text-warm-orange hover:text-paper-beige transition-colors font-mono"
                  >
                    <span>Details</span>
                    <ArrowUpRight className="w-3.5 h-3.5" />
                  </Link>
                </div>
              </PixelCard>
            </div>
          ))}

          {/* Honest Empty States (UX Goal 5): Locked / Coming Soon Modules */}
          <div className="lg:col-span-6">
            <LockedCard
              id="cloud"
              title="CRATE: aek-cloud"
              status="CLASSIFIED // Q3 ROADMAP"
              category="AUTONOMOUS CLOUD ORCHESTRATION"
            />
          </div>

          <div className="lg:col-span-6">
            <LockedCard
              id="eval"
              title="CRATE: aek-eval"
              status="CLASSIFIED // IN RESEARCH"
              category="PROMPTFOO & RED-TEAMING HARNESS"
            />
          </div>
        </div>
      </div>
    </section>
  );
}
