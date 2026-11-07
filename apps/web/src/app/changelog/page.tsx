import { LATEST_VERSION, GITHUB_REPO_URL } from "@/lib/utils";
import { ArrowUpRight, Sparkles, CheckCircle2 } from "lucide-react";

export default function ChangelogPage() {
  return (
    <div className="py-20 px-4 sm:px-6 lg:px-8 max-w-content mx-auto">
      <div className="text-center max-w-3xl mx-auto mb-16">
        <div className="inline-flex items-center gap-2 px-3.5 py-1.5 rounded-full border border-flamingo/40 bg-brandy/20 text-xs font-mono text-custard mb-4">
          ✦ RELEASE LOGS
        </div>
        <h1 className="font-editorial text-4xl sm:text-6xl text-ivory tracking-tight mb-4">
          Changelog & <span className="italic text-gradient-editorial">Releases.</span>
        </h1>
        <p className="text-secondary-text text-base sm:text-lg">
          Continuous improvements, security audits, and version history.
        </p>
      </div>

      <div className="max-w-3xl mx-auto space-y-12">
        {/* v0.2.0 Major Feature & Audit Release */}
        <div className="rounded-3xl ceramic-card p-8 border border-primary-gold/40 space-y-6 shadow-hard-md">
          <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4 pb-4 border-b border-espresso-border">
            <div>
              <span className="font-mono text-xs text-primary-gold font-bold">LATEST PRODUCTION RELEASE</span>
              <h2 className="font-editorial text-3xl text-ivory mt-1">v0.2.0 Cryptographic Integrity & Mobile Excellence</h2>
            </div>
            <a
              href={`${GITHUB_REPO_URL}/releases/tag/v0.2.0`}
              target="_blank"
              rel="noopener noreferrer"
              className="inline-flex items-center gap-1.5 text-xs font-mono text-custard hover:text-ivory px-3 py-1.5 rounded-xl bg-espresso-deep border border-espresso-border"
            >
              <span>GitHub Release Tag</span>
              <ArrowUpRight className="w-3.5 h-3.5 text-warm-orange" />
            </a>
          </div>

          <div className="space-y-4 text-xs sm:text-sm text-sand/90 leading-relaxed">
            <p>
              Major release introducing cryptographic binary integrity verification, authentic manifest resolution, transparent deterministic prompt engineering compilation, native macOS Apple Silicon/Intel support, and a responsive mobile UX overhaul.
            </p>

            <h3 className="font-mono text-xs font-bold uppercase tracking-wider text-custard pt-2">
              Key Capabilities Delivered in v0.2.0
            </h3>
            <ul className="space-y-2 pl-4 border-l-2 border-primary-gold/60 font-mono text-xs text-sand">
              <li>• <strong>Cryptographic Verification:</strong> Real constant-time SHA-256 integrity verification (`aek install verify --expected-hash &lt;SHA256&gt;`) against installation manifests.</li>
              <li>• <strong>Authentic Manifest Resolution:</strong> `aek update` calculates live file hashes and binary dimensions from official release manifests.</li>
              <li>• <strong>Deterministic Structural Prompt Compiler:</strong> 5-pillar Prompt Optimizer engine with transparent Stanford DSPy bridge classification.</li>
              <li>• <strong>macOS Native Packaging:</strong> Multi-platform release automation for Apple Silicon (aarch64) and Intel (x86_64).</li>
              <li>• <strong>Mobile UX Perfection:</strong> Zero-collision tactical HUD scenes, adaptive filepath formatting, and 1-tap companion dismiss.</li>
              <li>• <strong>Automated Frontend CI Gate:</strong> Continuous integration now builds and validates the Next.js production bundle alongside Rust crates.</li>
            </ul>
          </div>
        </div>

        {/* v0.1.0 Release Card */}
        <div className="rounded-3xl ceramic-card p-8 border border-espresso-border space-y-6 opacity-85">
          <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4 pb-4 border-b border-espresso-border">
            <div>
              <span className="font-mono text-xs text-sand/60 font-semibold">INITIAL PUBLIC RELEASE</span>
              <h2 className="font-editorial text-2xl text-ivory mt-1">v0.1.0 Stable Launch</h2>
            </div>
            <a
              href={`${GITHUB_REPO_URL}/releases/tag/v0.1.0`}
              target="_blank"
              rel="noopener noreferrer"
              className="inline-flex items-center gap-1.5 text-xs font-mono text-custard hover:text-ivory px-3 py-1.5 rounded-xl bg-espresso-deep border border-espresso-border"
            >
              <span>GitHub Release Tag</span>
              <ArrowUpRight className="w-3.5 h-3.5 text-flamingo" />
            </a>
          </div>

          <div className="space-y-4 text-xs sm:text-sm text-sand/90 leading-relaxed">
            <p>
              Initial public release of the Antigravity Engineering Kit (AEK), establishing a high-performance Rust foundation across 16 modular crates.
            </p>

            <h3 className="font-mono text-xs font-bold uppercase tracking-wider text-custard pt-2">
              Key Capabilities Delivered
            </h3>
            <ul className="space-y-2 pl-4 border-l-2 border-brandy-light font-mono text-xs text-sand">
              <li>• Universal Multi-Agent Compatibility (Antigravity, Claude Code, Cursor, OpenCode).</li>
              <li>• Model Context Protocol JSON-RPC 2.0 client engine with 5 standard verified presets.</li>
              <li>• Tiered engineering rules precedence engine with 24 KB memory budget.</li>
              <li>• 15 progressive-disclosure engineering skills runbooks.</li>
              <li>• 8-stage verification DAG quality gates with Git incremental change detection.</li>
              <li>• Multi-scanner security orchestration (Semgrep, Trivy, cargo-audit).</li>
              <li>• Cross-platform user-level binary installation for Windows and Linux.</li>
            </ul>
          </div>
        </div>
      </div>
    </div>
  );
}
