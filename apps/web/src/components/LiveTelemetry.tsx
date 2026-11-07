"use client";

import { motion } from "motion/react";
import { Activity, ShieldCheck, Cpu, Terminal, ArrowUpRight, GitCommit } from "lucide-react";
import Link from "next/link";
import { GITHUB_REPO_URL } from "@/lib/utils";

const METRICS = [
  {
    label: "RUST WORKSPACE",
    value: "16 CRATES",
    detail: "Zero-cost abstractions",
    color: "popcorn",
  },
  {
    label: "GITHUB ACTIONS CI",
    value: "100% PASSING",
    detail: "Ubuntu • Windows • macOS",
    color: "cerulean",
  },
  {
    label: "CLI EXECUTION",
    value: "< 1.2ms COLD BOOT",
    detail: "Sub-millisecond latency",
    color: "popcorn",
  },
  {
    label: "AGENT COVERAGE",
    value: "4 UNIFIED AGENTS",
    detail: "AGY • Claude • Cursor • OpenCode",
    color: "cerulean",
  },
];

export default function LiveTelemetry() {
  return (
    <div className="w-full max-w-content mx-auto px-4 sm:px-6 lg:px-8 -mt-6 mb-16 relative z-20">
      <div className="rounded-2xl border-2 border-popcorn/30 bg-plum-deep/95 p-6 backdrop-blur-xl shadow-[6px_6px_0px_#382C2D] relative overflow-hidden">
        {/* Top Status Banner */}
        <div className="flex flex-wrap items-center justify-between gap-4 pb-5 border-b border-plum-border/60">
          <div className="flex items-center gap-3">
            <span className="relative flex h-3 w-3">
              <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-emerald-400 opacity-75" />
              <span className="relative inline-flex rounded-full h-3 w-3 bg-emerald-500" />
            </span>
            <span className="text-xs font-mono font-bold tracking-widest uppercase text-ivory">
              LIVE SYSTEM TELEMETRY
            </span>
            <span className="px-2 py-0.5 rounded text-[10px] font-mono font-semibold bg-emerald-500/10 text-emerald-400 border border-emerald-500/30">
              OPERATIONAL
            </span>
          </div>

          <div className="flex items-center gap-4 text-xs font-mono text-sand">
            <span className="flex items-center gap-1.5">
              <GitCommit className="w-3.5 h-3.5 text-popcorn" />
              <span>MAIN BRANCH</span>
            </span>
            <span className="text-plum-border">•</span>
            <Link
              href={GITHUB_REPO_URL}
              target="_blank"
              rel="noreferrer"
              className="inline-flex items-center gap-1 text-popcorn hover:underline font-bold"
            >
              <span>AUDIT ON GITHUB</span>
              <ArrowUpRight className="w-3.5 h-3.5" />
            </Link>
          </div>
        </div>

        {/* 4-Column Neo-Brutalist Metrics Grid */}
        <div className="grid grid-cols-2 md:grid-cols-4 gap-4 pt-6">
          {METRICS.map((metric, i) => (
            <div
              key={i}
              className="p-4 rounded-xl border border-plum-border bg-plum/50 hover:border-popcorn/50 transition-all hover:-translate-y-0.5 shadow-[2px_2px_0px_rgba(0,0,0,0.3)] group"
            >
              <span className="block text-[11px] font-mono text-sand/70 tracking-wider mb-1">
                {metric.label}
              </span>
              <span className="block text-xl sm:text-2xl font-black font-mono tracking-tight text-ivory mb-1 group-hover:text-popcorn transition-colors">
                {metric.value}
              </span>
              <span className="block text-[11px] text-popcorn/90 font-medium">
                {metric.detail}
              </span>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}
