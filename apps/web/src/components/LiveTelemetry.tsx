"use client";

import Link from "next/link";
import { ArrowUpRight, GitCommit } from "lucide-react";
import { GITHUB_REPO_URL } from "@/lib/utils";
import { StatCard } from "@/components/primitives/PixelCard";

const METRICS = [
  {
    tag: "01/CRATES",
    label: "RUST WORKSPACE",
    value: "16 CRATES",
    detail: "Zero-cost abstractions",
    color: "gold" as const,
  },
  {
    tag: "02/CI",
    label: "GITHUB ACTIONS CI",
    value: "100% PASSING",
    detail: "Ubuntu • Windows • macOS",
    color: "orange" as const,
  },
  {
    tag: "03/LATENCY",
    label: "CLI EXECUTION",
    value: "< 1.2ms COLD BOOT",
    detail: "Sub-millisecond latency",
    color: "gold" as const,
  },
  {
    tag: "04/AGENTS",
    label: "AGENT COVERAGE",
    value: "4 UNIFIED AGENTS",
    detail: "AGY • Claude • Cursor • OpenCode",
    color: "orange" as const,
  },
];

export default function LiveTelemetry() {
  return (
    <section className="w-full max-w-content mx-auto px-4 sm:px-6 lg:px-8 -mt-6 mb-16 relative z-20">
      <div className="rounded-2xl border-2 border-primary-gold/40 bg-space-deep/95 p-6 backdrop-blur-xl shadow-hard-md relative overflow-hidden">
        {/* Top Status Banner */}
        <div className="flex flex-wrap items-center justify-between gap-4 pb-5 border-b border-space-border/80">
          <div className="flex items-center gap-3">
            <span className="relative flex h-3 w-3">
              <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-emerald-400 opacity-75" />
              <span className="relative inline-flex rounded-full h-3 w-3 bg-emerald-500" />
            </span>
            <span className="text-xs font-mono font-bold tracking-widest uppercase text-paper-beige">
              LIVE SYSTEM TELEMETRY
            </span>
            <span className="px-2 py-0.5 rounded text-[10px] font-mono font-semibold bg-emerald-500/10 text-emerald-400 border border-emerald-500/30">
              OPERATIONAL
            </span>
          </div>

          <div className="flex items-center gap-4 text-xs font-mono text-sand">
            <span className="flex items-center gap-1.5">
              <GitCommit className="w-3.5 h-3.5 text-primary-gold" />
              <span>MAIN BRANCH</span>
            </span>
            <span className="text-space-border">•</span>
            <Link
              href={GITHUB_REPO_URL}
              target="_blank"
              rel="noreferrer"
              className="inline-flex items-center gap-1 text-primary-gold hover:underline font-bold"
            >
              <span>AUDIT ON GITHUB</span>
              <ArrowUpRight className="w-3.5 h-3.5" />
            </Link>
          </div>
        </div>

        {/* 4-Column Neo-Brutalist Stat Cards Grid */}
        <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4 pt-6">
          {METRICS.map((metric, i) => (
            <StatCard
              key={i}
              tag={metric.tag}
              caption={metric.label}
              value={metric.value}
              subCaption={metric.detail}
              highlight={metric.color === "gold"}
            />
          ))}
        </div>
      </div>
    </section>
  );
}
