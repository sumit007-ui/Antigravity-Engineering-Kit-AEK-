import Link from "next/link";
import { ArrowRight, BookOpen, Terminal, Shield, Cpu } from "lucide-react";
import { LATEST_VERSION } from "@/lib/utils";

export default function DocsOverviewPage() {
  return (
    <div className="space-y-10">
      <div>
        <span className="font-mono text-xs text-flamingo font-semibold">DOCUMENTATION OVERVIEW</span>
        <h1 className="font-editorial text-4xl sm:text-5xl text-ivory tracking-tight mt-1 mb-4">
          Antigravity Engineering Kit Documentation
        </h1>
        <p className="text-secondary-text text-base leading-relaxed">
          Welcome to the official developer documentation for AEK v{LATEST_VERSION}. Learn how to configure, audit, and synchronize engineering rules, specialized agent skills, MCP servers, and validation quality gates across Google Antigravity, Claude Code, Cursor, and OpenCode.
        </p>
      </div>

      {/* Quick Cards Grid */}
      <div className="grid grid-cols-1 sm:grid-cols-2 gap-4">
        <Link
          href="/docs/getting-started"
          className="p-6 rounded-2xl ceramic-card border border-espresso-border hover:border-flamingo/50 transition-all group"
        >
          <BookOpen className="w-5 h-5 text-flamingo mb-3" />
          <h3 className="font-editorial text-xl text-ivory mb-1 group-hover:text-custard transition-colors">
            Quickstart Guide
          </h3>
          <p className="text-xs text-sand/80 leading-relaxed mb-4">
            Initialize an AEK workspace and configure your first AI coding agent in under 2 minutes.
          </p>
          <span className="text-xs font-semibold text-flamingo flex items-center gap-1">
            Start here <ArrowRight className="w-3.5 h-3.5" />
          </span>
        </Link>

        <Link
          href="/docs/cli-reference"
          className="p-6 rounded-2xl ceramic-card border border-espresso-border hover:border-flamingo/50 transition-all group"
        >
          <Terminal className="w-5 h-5 text-custard mb-3" />
          <h3 className="font-editorial text-xl text-ivory mb-1 group-hover:text-custard transition-colors">
            CLI Reference
          </h3>
          <p className="text-xs text-sand/80 leading-relaxed mb-4">
            Complete command-line manual covering init, agent sync, validate, mcp, security, and doctor.
          </p>
          <span className="text-xs font-semibold text-custard flex items-center gap-1">
            View commands <ArrowRight className="w-3.5 h-3.5" />
          </span>
        </Link>
      </div>

      {/* Architectural Pillars */}
      <div className="p-8 rounded-3xl ceramic-card border border-espresso-border space-y-4">
        <h2 className="font-editorial text-2xl text-ivory">Core Architectural Invariants</h2>
        <p className="text-xs sm:text-sm text-sand/80 leading-relaxed">
          AEK operates strictly in <strong>ruthless_precision</strong> mode. Every command, adapter, and rule enforcement satisfies these invariants:
        </p>
        <ul className="space-y-2 text-xs font-mono text-custard pl-4 border-l-2 border-brandy-light">
          <li>• Direct argument array subprocess execution; zero shell string concatenation.</li>
          <li>• Path traversal defenses prevent reading or writing outside authorized project bounds.</li>
          <li>• Atomic file mutation via temporary files on the same filesystem before replace.</li>
          <li>• Zero remote telemetry; zero transmission of user code or credentials.</li>
        </ul>
      </div>
    </div>
  );
}

