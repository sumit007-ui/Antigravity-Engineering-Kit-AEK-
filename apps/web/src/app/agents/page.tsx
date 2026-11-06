import AgentShowcase from "@/components/AgentShowcase";
import { Bot, CheckCircle2, ShieldCheck, ArrowRight } from "lucide-react";
import Link from "next/link";

export default function AgentsPage() {
  return (
    <div className="py-20 px-4 sm:px-6 lg:px-8 max-w-content mx-auto">
      <div className="text-center max-w-3xl mx-auto mb-16">
        <div className="inline-flex items-center gap-2 px-3.5 py-1.5 rounded-full border border-flamingo/40 bg-brandy/20 text-xs font-mono text-custard mb-4">
          ✦ MULTI-AGENT SYNC ENGINE
        </div>
        <h1 className="font-editorial text-4xl sm:text-6xl text-ivory tracking-tight mb-4">
          Universal <span className="italic text-gradient-editorial">Agent Ecosystem.</span>
        </h1>
        <p className="text-secondary-text text-base sm:text-lg">
          Maintain a single source of truth for your rules, skills, and tools. Sync natively to Google Antigravity, Claude Code, Cursor, and OpenCode.
        </p>
      </div>

      <AgentShowcase />

      {/* Synchronization Strategies Card */}
      <div className="mt-16 rounded-3xl ceramic-card p-8 border border-espresso-border max-w-4xl mx-auto space-y-6">
        <h3 className="font-editorial text-2xl text-ivory">Conflict-Safe Synchronization Strategies</h3>
        <p className="text-xs sm:text-sm text-sand/80 leading-relaxed">
          AEK records cryptographic SHA-256 hashes inside <code className="text-custard">.aek/managed-agents.toml</code> to track managed blocks and preserve user-authored modifications.
        </p>

        <div className="grid grid-cols-1 md:grid-cols-2 gap-4 text-xs font-mono">
          <div className="p-4 rounded-xl bg-espresso-deep border border-espresso-border">
            <span className="text-flamingo font-bold block mb-1">ReplaceManaged (Default)</span>
            <span className="text-sand/80">Updates only files and comment sections managed by AEK. Preserves your custom user configurations completely.</span>
          </div>
          <div className="p-4 rounded-xl bg-espresso-deep border border-espresso-border">
            <span className="text-flamingo font-bold block mb-1">KeepExisting</span>
            <span className="text-sand/80">Skips any configuration file where user modifications or pre-existing configs are detected.</span>
          </div>
        </div>
      </div>
    </div>
  );
}
