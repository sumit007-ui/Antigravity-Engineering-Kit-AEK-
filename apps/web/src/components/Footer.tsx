import Link from "next/link";
import { Github, Terminal, Shield, ArrowUpRight, Heart } from "lucide-react";
import { GITHUB_REPO_URL, LATEST_VERSION } from "@/lib/utils";

export default function Footer() {
  return (
    <footer className="w-full bg-espresso-deep border-t border-espresso-border pt-20 pb-12 px-4 sm:px-6 lg:px-8 relative overflow-hidden">
      {/* Decorative fine tactile grid */}
      <div className="absolute inset-0 bg-tactile-grid opacity-15 pointer-events-none" />

      <div className="max-w-content mx-auto relative z-10">
        {/* Large memorable statement */}
        <div className="pb-16 border-b border-espresso-border/80 text-center sm:text-left flex flex-col md:flex-row md:items-end justify-between gap-8">
          <div>
            <p className="text-xs font-mono font-bold uppercase tracking-widest text-flamingo mb-3">
              THE NEXT GENERATION OF AGENTIC ENGINEERING
            </p>
            <h2 className="font-editorial text-4xl sm:text-6xl lg:text-7xl text-ivory tracking-tight">
              Build beyond the <span className="italic text-gradient-editorial">ordinary.</span>
            </h2>
            <p className="font-editorial text-xl sm:text-2xl text-custard/80 mt-2">
              AEK — Antigravity Engineering Kit
            </p>
          </div>

          <div className="flex flex-col sm:flex-row items-center gap-3">
            <Link
              href="/download"
              className="w-full sm:w-auto px-6 py-3 rounded-xl bg-brandy hover:bg-brandy-hover border border-flamingo/40 text-ivory text-xs font-semibold tracking-wide text-center transition-all shadow-tactile"
            >
              Get AEK v{LATEST_VERSION}
            </Link>
            <a
              href={GITHUB_REPO_URL}
              target="_blank"
              rel="noopener noreferrer"
              className="w-full sm:w-auto px-6 py-3 rounded-xl bg-espresso-surface hover:bg-espresso-elevated border border-espresso-border text-custard text-xs font-mono flex items-center justify-center gap-2 transition-all"
            >
              <Github className="w-4 h-4" />
              <span>Star on GitHub</span>
            </a>
          </div>
        </div>

        {/* 4-column structured links */}
        <div className="grid grid-cols-2 md:grid-cols-4 gap-8 py-14 border-b border-espresso-border/60">
          {/* Engineering Engines */}
          <div>
            <h3 className="text-xs font-mono font-bold uppercase tracking-wider text-custard mb-4">
              Core Architecture
            </h3>
            <ul className="space-y-2.5 text-xs text-sand/80">
              <li><Link href="/features#rules" className="hover:text-flamingo transition-colors">Global Rules Engine</Link></li>
              <li><Link href="/features#skills" className="hover:text-flamingo transition-colors">15 Specialized Skills</Link></li>
              <li><Link href="/features#mcp" className="hover:text-flamingo transition-colors">Model Context Protocol</Link></li>
              <li><Link href="/features#toolchains" className="hover:text-flamingo transition-colors">Language Toolchains</Link></li>
              <li><Link href="/features#validation" className="hover:text-flamingo transition-colors">8-Stage Quality Gates</Link></li>
            </ul>
          </div>

          {/* AI Coding Agents */}
          <div>
            <h3 className="text-xs font-mono font-bold uppercase tracking-wider text-custard mb-4">
              Supported Agents
            </h3>
            <ul className="space-y-2.5 text-xs text-sand/80">
              <li><Link href="/agents#antigravity" className="hover:text-flamingo transition-colors">Google Gemini Antigravity</Link></li>
              <li><Link href="/agents#claude" className="hover:text-flamingo transition-colors">Anthropic Claude Code</Link></li>
              <li><Link href="/agents#cursor" className="hover:text-flamingo transition-colors">Cursor IDE</Link></li>
              <li><Link href="/agents#opencode" className="hover:text-flamingo transition-colors">OpenCode</Link></li>
              <li><Link href="/docs/multi-agent" className="hover:text-flamingo transition-colors">Sync Protocol Spec</Link></li>
            </ul>
          </div>

          {/* Documentation */}
          <div>
            <h3 className="text-xs font-mono font-bold uppercase tracking-wider text-custard mb-4">
              Documentation
            </h3>
            <ul className="space-y-2.5 text-xs text-sand/80">
              <li><Link href="/docs/getting-started" className="hover:text-flamingo transition-colors">Quickstart Guide</Link></li>
              <li><Link href="/docs/installation" className="hover:text-flamingo transition-colors">Installation (Win/Linux)</Link></li>
              <li><Link href="/docs/cli-reference" className="hover:text-flamingo transition-colors">CLI Command Reference</Link></li>
              <li><Link href="/docs/security" className="hover:text-flamingo transition-colors">Security Invariants</Link></li>
              <li><Link href="/docs/troubleshooting" className="hover:text-flamingo transition-colors">Troubleshooting</Link></li>
            </ul>
          </div>

          {/* Open Source & Community */}
          <div>
            <h3 className="text-xs font-mono font-bold uppercase tracking-wider text-custard mb-4">
              Open Source
            </h3>
            <ul className="space-y-2.5 text-xs text-sand/80">
              <li><a href={`${GITHUB_REPO_URL}/releases/tag/v${LATEST_VERSION}`} target="_blank" rel="noopener noreferrer" className="hover:text-flamingo flex items-center gap-1 transition-colors">Release v{LATEST_VERSION} <ArrowUpRight className="w-3 h-3 text-muted-text" /></a></li>
              <li><Link href="/roadmap" className="hover:text-flamingo transition-colors">Roadmap & Phases</Link></li>
              <li><Link href="/changelog" className="hover:text-flamingo transition-colors">Changelog</Link></li>
              <li><a href={`${GITHUB_REPO_URL}/blob/main/LICENSE-APACHE`} target="_blank" rel="noopener noreferrer" className="hover:text-flamingo flex items-center gap-1 transition-colors">Apache-2.0 / MIT <ArrowUpRight className="w-3 h-3 text-muted-text" /></a></li>
              <li><Link href="/about" className="hover:text-flamingo transition-colors">About & Rationale</Link></li>
            </ul>
          </div>
        </div>

        {/* Operational Bottom Metadata */}
        <div className="pt-8 flex flex-col sm:flex-row items-center justify-between gap-4 text-xs text-muted-text">
          <div className="flex items-center gap-2">
            <span className="w-2 h-2 rounded-full bg-emerald-500 animate-pulse" />
            <span className="font-mono text-sand">v{LATEST_VERSION} Stable Release Active</span>
            <span className="text-espresso-border">•</span>
            <span>Zero Remote Telemetry</span>
          </div>

          <div className="flex items-center gap-6">
            <Link href="/privacy" className="hover:text-sand transition-colors">Privacy Policy</Link>
            <Link href="/terms" className="hover:text-sand transition-colors">Terms of Use</Link>
            <span>© 2026 Antigravity Engineering Kit Contributors</span>
          </div>
        </div>
      </div>
    </footer>
  );
}
