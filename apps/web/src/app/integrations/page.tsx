import Link from "next/link";
import { ArrowUpRight, CheckCircle2, ShieldCheck, Terminal, Cpu } from "lucide-react";
import TiltCard from "@/components/motion/TiltCard";

const INTEGRATIONS = [
  {
    id: "context7",
    name: "Context7 MCP Server",
    type: "Documentation & Library Indexer",
    transport: "Stdio (Dynamic Query)",
    status: "Verified Preset",
    description: "Enables agents to query up-to-date, real-world library documentation and API schemas on demand.",
    command: "aek mcp show context7",
  },
  {
    id: "github",
    name: "GitHub Official MCP Server",
    type: "Code Hosting & PR Management",
    transport: "Stdio (Token Auth)",
    status: "Verified Preset",
    description: "Provides repository management, branch creation, pull request reviews, and issue tracking.",
    command: "aek mcp show github",
  },
  {
    id: "playwright",
    name: "Playwright Automation MCP",
    type: "Headless Browser Automation",
    transport: "Stdio (Process Isolation)",
    status: "Verified Preset",
    description: "Empowers coding agents to take screenshots, inspect DOM elements, and automate end-to-end tests.",
    command: "aek mcp show playwright",
  },
  {
    id: "sequential-thinking",
    name: "Sequential Thinking MCP",
    type: "Dynamic Reasoning Engine",
    transport: "Stdio (Stateful)",
    status: "Verified Preset",
    description: "Step-by-step cognitive reasoning loop for complex algorithmic problem solving and architectural design.",
    command: "aek mcp show sequential-thinking",
  },
  {
    id: "filesystem",
    name: "Filesystem Sandboxed MCP",
    type: "Contained Disk Operations",
    transport: "Stdio (Scoped)",
    status: "Verified Preset",
    description: "Secure, path-traversal-defended filesystem read/write operations confined to authorized workspace directories.",
    command: "aek mcp show filesystem",
  },
];

export default function IntegrationsPage() {
  return (
    <div className="py-20 px-4 sm:px-6 lg:px-8 max-w-content mx-auto">
      <div className="text-center max-w-3xl mx-auto mb-16">
        <div className="inline-flex items-center gap-2 px-3.5 py-1.5 rounded-full border border-flamingo/40 bg-brandy/20 text-xs font-mono text-custard mb-4">
          ✦ MODEL CONTEXT PROTOCOL
        </div>
        <h1 className="font-editorial text-4xl sm:text-6xl text-ivory tracking-tight mb-4">
          Standard <span className="italic text-gradient-editorial">MCP Integrations.</span>
        </h1>
        <p className="text-secondary-text text-base sm:text-lg">
          Connect your coding agents to verified Model Context Protocol tools with zero configuration drift.
        </p>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 mb-16">
        {INTEGRATIONS.map((integ) => (
          <TiltCard key={integ.id} glowColor="flamingo" className="flex flex-col justify-between">
            <div>
              <div className="flex items-center justify-between mb-4">
                <span className="text-[10px] font-mono uppercase px-2.5 py-1 rounded bg-brandy/40 border border-brandy-light text-custard">
                  {integ.type}
                </span>
                <span className="text-[10px] font-mono text-emerald-400">✓ {integ.status}</span>
              </div>

              <h3 className="font-editorial text-2xl text-ivory mb-2">{integ.name}</h3>
              <p className="text-xs text-sand/80 leading-relaxed mb-6">{integ.description}</p>
            </div>

            <div className="pt-4 border-t border-espresso-border/80">
              <span className="text-[10px] font-mono text-sand/60 block mb-1">TRANSPORT: {integ.transport}</span>
              <code className="text-xs font-mono text-custard bg-espresso-deep px-3 py-1.5 rounded-lg border border-espresso-border block truncate">
                $ {integ.command}
              </code>
            </div>
          </TiltCard>
        ))}
      </div>

      <div className="rounded-3xl ceramic-card p-8 border border-espresso-border text-center max-w-3xl mx-auto">
        <h3 className="font-editorial text-2xl text-ivory mb-2">Export to any AI agent with one command</h3>
        <p className="text-xs sm:text-sm text-sand/80 mb-6">
          Synchronize your verified MCP servers directly into Google Antigravity, Claude Code, Cursor, or OpenCode configuration files.
        </p>
        <code className="inline-block px-4 py-2 rounded-xl bg-espresso-deep border border-espresso-border font-mono text-xs text-custard">
          $ aek mcp export --agent all
        </code>
      </div>
    </div>
  );
}
