"use client";

import { motion } from "motion/react";
import Link from "next/link";
import { ArrowRight, Terminal, Shield, Download, Sparkles, CheckCircle2 } from "lucide-react";
import { GITHUB_REPO_URL, LATEST_VERSION } from "@/lib/utils";
import HeroArtifact from "@/components/three/HeroArtifact";
import MagneticButton from "@/components/motion/MagneticButton";

export default function Hero() {
  return (
    <section className="relative min-h-[92vh] flex items-center justify-center pt-8 pb-20 px-4 sm:px-6 lg:px-8 overflow-hidden bg-espresso-deep">
      {/* Background ambient lighting */}
      <div className="absolute top-0 left-1/2 -translate-x-1/2 w-[800px] h-[500px] bg-radial-gradient opacity-40 pointer-events-none" />
      <div className="absolute inset-0 bg-tactile-grid opacity-20 pointer-events-none" />

      <div className="max-w-content w-full mx-auto relative z-10 grid grid-cols-1 lg:grid-cols-12 gap-12 items-center">
        {/* Left Column: Editorial Headline & Actions */}
        <div className="lg:col-span-7 flex flex-col items-center lg:items-start text-center lg:text-left">
          {/* Eyebrow Pill */}
          <motion.div
            initial={{ opacity: 0, y: -10 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.5 }}
            className="inline-flex flex-wrap items-center gap-2 px-4 py-1.5 rounded-full border-2 border-popcorn/50 bg-plum-surface/90 text-xs font-mono text-popcorn mb-6 shadow-[3px_3px_0px_rgba(248,216,120,0.2)]"
          >
            <span className="w-2 h-2 rounded-full bg-emerald-400 animate-pulse" />
            <span className="font-bold">AEK v{LATEST_VERSION} RUST ENGINE</span>
            <span className="text-plum-border">•</span>
            <span className="text-cerulean font-semibold">AUTONOMOUS AGENTS UNIFIED</span>
            <span className="text-plum-border">•</span>
            <span className="text-ivory font-bold">100% OPEN SOURCE</span>
          </motion.div>

          {/* Primary Editorial Headline */}
          <motion.h1
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.7, delay: 0.1 }}
            className="font-editorial text-5xl sm:text-7xl lg:text-8xl text-ivory tracking-tight leading-[1.02] mb-6"
          >
            One Engineering Kit. <br />
            <span className="italic font-normal text-gradient-editorial">
              Autonomous Precision.
            </span>
          </motion.h1>

          {/* Secondary Subtitle */}
          <motion.p
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.7, delay: 0.2 }}
            className="text-lg sm:text-xl text-sand/90 font-normal max-w-2xl leading-relaxed mb-4"
          >
            The high-performance Rust foundation built for developers who ship software with autonomous AI agents.
          </motion.p>

          <motion.p
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.7, delay: 0.25 }}
            className="text-xs sm:text-sm text-secondary-text max-w-xl mb-8"
          >
            Unify global engineering rules, 15 specialized skills, Model Context Protocol (MCP), and automated test-driven validation with <strong className="text-popcorn font-bold">ruthless precision</strong> across Antigravity, Claude Code, Cursor, and OpenCode.
          </motion.p>

          {/* Action CTAs - Neo-Brutalist Tactile Buttons */}
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.6, delay: 0.35 }}
            className="flex flex-wrap items-center justify-center lg:justify-start gap-4 mb-10"
          >
            <Link href="/download">
              <button className="px-6 py-3.5 text-sm font-bold bg-popcorn text-plum-deep rounded-xl border-2 border-popcorn shadow-[4px_4px_0px_#241B1C] hover:brightness-105 active:translate-x-[2px] active:translate-y-[2px] active:shadow-none transition-all flex items-center gap-2 cursor-pointer">
                <Download className="w-4 h-4 text-plum-deep" />
                <span>Get AEK v{LATEST_VERSION}</span>
              </button>
            </Link>

            <Link href="/docs/getting-started">
              <button className="px-6 py-3.5 text-sm font-semibold bg-plum-surface text-ivory rounded-xl border-2 border-cerulean/50 shadow-[4px_4px_0px_rgba(152,180,212,0.2)] hover:border-cerulean hover:text-popcorn active:translate-x-[2px] active:translate-y-[2px] active:shadow-none transition-all flex items-center gap-2 cursor-pointer">
                <span>Quickstart Guide</span>
                <ArrowRight className="w-4 h-4 text-cerulean" />
              </button>
            </Link>

            <a
              href={GITHUB_REPO_URL}
              target="_blank"
              rel="noopener noreferrer"
              className="px-4 py-3.5 text-xs font-mono text-sand hover:text-ivory border-2 border-plum-border rounded-xl bg-plum/40 hover:border-popcorn/60 shadow-[3px_3px_0px_rgba(0,0,0,0.4)] active:translate-x-[2px] active:translate-y-[2px] active:shadow-none transition-all flex items-center gap-2"
            >
              <Terminal className="w-4 h-4 text-popcorn" />
              <span>cargo install aek-cli</span>
            </a>
          </motion.div>

          {/* Verified Quality Badges */}
          <motion.div
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            transition={{ duration: 0.8, delay: 0.5 }}
            className="flex flex-wrap items-center justify-center lg:justify-start gap-6 text-xs text-sand/80 pt-6 border-t border-espresso-border/60"
          >
            <div className="flex items-center gap-2">
              <CheckCircle2 className="w-4 h-4 text-emerald-400" />
              <span>Zero Shell-Injection Risk</span>
            </div>
            <div className="flex items-center gap-2">
              <CheckCircle2 className="w-4 h-4 text-emerald-400" />
              <span>Sub-50 MB Memory Footprint</span>
            </div>
            <div className="flex items-center gap-2">
              <CheckCircle2 className="w-4 h-4 text-emerald-400" />
              <span>Cryptographic Rollbacks</span>
            </div>
          </motion.div>
        </div>

        {/* Right Column: 3D AEK Sculptural Artifact */}
        <div className="lg:col-span-5 relative flex items-center justify-center">
          <div className="relative w-full max-w-[500px]">
            {/* Ambient backdrop glow */}
            <div className="absolute inset-0 rounded-full bg-gradient-to-tr from-brandy/30 via-flamingo/20 to-custard/10 blur-3xl" />
            
            {/* 3D Scene */}
            <HeroArtifact />

            {/* Dimensional floating caption */}
            <div className="absolute bottom-2 left-1/2 -translate-x-1/2 px-4 py-1.5 rounded-full bg-espresso-surface/90 border border-espresso-border text-[11px] font-mono text-custard backdrop-blur-md shadow-tactile whitespace-nowrap">
              ✦ Precision-Machined Monogram Artifact
            </div>
          </div>
        </div>
      </div>
    </section>
  );
}
