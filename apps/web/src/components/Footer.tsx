import Link from "next/link";
import { Github, ArrowUpRight, Shield } from "lucide-react";
import { GITHUB_REPO_URL, LATEST_VERSION } from "@/lib/utils";
import TactileButton from "@/components/primitives/TactileButton";

export default function Footer() {
  return (
    <footer className="w-full bg-space-black border-t-2 border-space-border pt-20 pb-12 px-4 sm:px-6 lg:px-8 relative overflow-hidden">
      {/* Decorative fine tactile grid */}
      <div className="absolute inset-0 bg-tactile-grid opacity-15 pointer-events-none" />

      <div className="max-w-content mx-auto relative z-10">
        {/* Large memorable statement */}
        <div className="pb-16 border-b-2 border-space-border/80 text-center sm:text-left flex flex-col md:flex-row md:items-end justify-between gap-8">
          <div>
            <div className="inline-flex items-center gap-2 px-3 py-1 rounded-full border border-primary-gold/40 bg-space-surface text-xs font-mono text-primary-gold mb-4 font-bold shadow-hard-sm">
              <span>✦ AEK AUTONOMOUS ENGINEERING KIT</span>
            </div>
            <h2 className="font-editorial text-4xl sm:text-6xl lg:text-7xl text-paper-beige tracking-tight font-bold">
              Build beyond the <span className="italic text-gradient-editorial">ordinary.</span>
            </h2>
            <p className="font-editorial text-xl sm:text-2xl text-primary-gold/90 mt-2">
              AEK — Antigravity Engineering Kit
            </p>
          </div>

          <div className="flex flex-col sm:flex-row items-center gap-3">
            <Link href="/download" className="w-full sm:w-auto">
              <TactileButton variant="gold" size="md" className="w-full">
                Get AEK v{LATEST_VERSION}
              </TactileButton>
            </Link>
            <a
              href={GITHUB_REPO_URL}
              target="_blank"
              rel="noopener noreferrer"
              className="w-full sm:w-auto"
            >
              <TactileButton variant="secondary" size="md" className="w-full flex items-center justify-center gap-2 font-mono">
                <Github className="w-4 h-4" />
                <span>Star on GitHub</span>
              </TactileButton>
            </a>
          </div>
        </div>

        {/* 4-column structured links */}
        <div className="grid grid-cols-2 md:grid-cols-4 gap-8 py-14 border-b-2 border-space-border/60">
          {/* Engineering Engines */}
          <div>
            <h3 className="text-xs font-mono font-bold uppercase tracking-wider text-primary-gold mb-4">
              Core Architecture
            </h3>
            <ul className="space-y-2.5 text-xs text-sand/80 font-mono">
              <li><Link href="/features#rules" className="hover:text-warm-orange transition-colors">Global Rules Engine</Link></li>
              <li><Link href="/features#skills" className="hover:text-warm-orange transition-colors">15 Specialized Skills</Link></li>
              <li><Link href="/features#mcp" className="hover:text-warm-orange transition-colors">Model Context Protocol</Link></li>
              <li><Link href="/features#toolchains" className="hover:text-warm-orange transition-colors">Language Toolchains</Link></li>
              <li><Link href="/features#validation" className="hover:text-warm-orange transition-colors">8-Stage Quality Gates</Link></li>
            </ul>
          </div>

          {/* AI Coding Agents */}
          <div>
            <h3 className="text-xs font-mono font-bold uppercase tracking-wider text-primary-gold mb-4">
              Supported Agents
            </h3>
            <ul className="space-y-2.5 text-xs text-sand/80 font-mono">
              <li><Link href="/agents#antigravity" className="hover:text-warm-orange transition-colors">Google Gemini Antigravity</Link></li>
              <li><Link href="/agents#claude" className="hover:text-warm-orange transition-colors">Anthropic Claude Code</Link></li>
              <li><Link href="/agents#cursor" className="hover:text-warm-orange transition-colors">Cursor IDE</Link></li>
              <li><Link href="/agents#opencode" className="hover:text-warm-orange transition-colors">OpenCode</Link></li>
              <li><Link href="/docs/multi-agent" className="hover:text-warm-orange transition-colors">Sync Protocol Spec</Link></li>
            </ul>
          </div>

          {/* Documentation */}
          <div>
            <h3 className="text-xs font-mono font-bold uppercase tracking-wider text-primary-gold mb-4">
              Documentation
            </h3>
            <ul className="space-y-2.5 text-xs text-sand/80 font-mono">
              <li><Link href="/docs/getting-started" className="hover:text-warm-orange transition-colors">Quickstart Guide</Link></li>
              <li><Link href="/docs/installation" className="hover:text-warm-orange transition-colors">Installation (Win/Linux)</Link></li>
              <li><Link href="/docs/cli-reference" className="hover:text-warm-orange transition-colors">CLI Command Reference</Link></li>
              <li><Link href="/docs/security" className="hover:text-warm-orange transition-colors">Security Invariants</Link></li>
              <li><Link href="/docs/troubleshooting" className="hover:text-warm-orange transition-colors">Troubleshooting</Link></li>
            </ul>
          </div>

          {/* Open Source & Community */}
          <div>
            <h3 className="text-xs font-mono font-bold uppercase tracking-wider text-primary-gold mb-4">
              Open Source
            </h3>
            <ul className="space-y-2.5 text-xs text-sand/80 font-mono">
              <li><a href={`${GITHUB_REPO_URL}/releases/tag/v${LATEST_VERSION}`} target="_blank" rel="noopener noreferrer" className="hover:text-warm-orange flex items-center gap-1 transition-colors">Release v{LATEST_VERSION} <ArrowUpRight className="w-3 h-3 text-sand/60" /></a></li>
              <li><Link href="/roadmap" className="hover:text-warm-orange transition-colors">Roadmap & Phases</Link></li>
              <li><Link href="/changelog" className="hover:text-warm-orange transition-colors">Changelog</Link></li>
              <li><a href={`${GITHUB_REPO_URL}/blob/main/LICENSE-APACHE`} target="_blank" rel="noopener noreferrer" className="hover:text-warm-orange flex items-center gap-1 transition-colors">Apache-2.0 / MIT <ArrowUpRight className="w-3 h-3 text-sand/60" /></a></li>
              <li><Link href="/about" className="hover:text-warm-orange transition-colors">About & Rationale</Link></li>
            </ul>
          </div>
        </div>

        {/* UX Goal 7: Trust Center Row */}
        <div className="py-6 border-b-2 border-space-border/60 flex flex-wrap items-center justify-between gap-4 text-xs font-mono">
          <div className="flex items-center gap-2 text-primary-gold font-bold">
            <Shield className="w-4 h-4 text-primary-gold" />
            <span>AEK TRUST CENTER:</span>
            <span className="text-sand/80 font-normal">LEAST-PRIVILEGE • DETERMINISTIC GATES • CRYPTOGRAPHIC PROOFS</span>
          </div>
          <div className="flex items-center gap-6 text-sand/80">
            <Link href="/privacy" className="hover:text-primary-gold transition-colors">Privacy Policy</Link>
            <Link href="/terms" className="hover:text-primary-gold transition-colors">Terms of Service</Link>
            <Link href="/docs/security" className="hover:text-primary-gold transition-colors">Security Audit</Link>
            <a href={`${GITHUB_REPO_URL}/blob/main/CODE_OF_CONDUCT.md`} target="_blank" rel="noopener noreferrer" className="hover:text-primary-gold transition-colors">Code of Conduct</a>
          </div>
        </div>

        {/* Operational Bottom Metadata */}
        <div className="pt-8 flex flex-col sm:flex-row items-center justify-between gap-4 text-xs text-sand/70 font-mono">
          <div className="flex items-center gap-2">
            <span className="w-2 h-2 rounded-full bg-emerald-500 animate-pulse" />
            <span className="text-paper-beige font-bold">v{LATEST_VERSION} Stable Release Active</span>
            <span className="text-space-border">•</span>
            <span>Zero Remote Telemetry</span>
          </div>

          <div className="flex items-center gap-6">
            <span>© 2026 Antigravity Engineering Kit Contributors</span>
          </div>
        </div>
      </div>
    </footer>
  );
}
