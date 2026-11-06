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
        {/* v0.1.0 Release Card */}
        <div className="rounded-3xl ceramic-card p-8 border border-espresso-border space-y-6">
          <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4 pb-4 border-b border-espresso-border">
            <div>
              <span className="font-mono text-xs text-flamingo font-semibold">INITIAL PUBLIC RELEASE</span>
              <h2 className="font-editorial text-3xl text-ivory mt-1">v{LATEST_VERSION} Stable Launch</h2>
            </div>
            <a
              href={`${GITHUB_REPO_URL}/releases/tag/v${LATEST_VERSION}`}
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
