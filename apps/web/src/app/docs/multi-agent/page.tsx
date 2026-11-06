export default function MultiAgentDocPage() {
  return (
    <div className="space-y-8">
      <div>
        <span className="font-mono text-xs text-flamingo font-semibold">MULTI-AGENT SYNC</span>
        <h1 className="font-editorial text-4xl text-ivory tracking-tight mt-1 mb-3">
          Multi-Agent Synchronization
        </h1>
        <p className="text-secondary-text text-sm sm:text-base leading-relaxed">
          Native translation into Google Antigravity, Claude Code, Cursor, and OpenCode.
        </p>
      </div>

      <div className="space-y-8 text-sand/90 text-sm leading-relaxed">
        <section className="p-6 rounded-2xl ceramic-card border border-espresso-border space-y-3">
          <h2 className="font-editorial text-2xl text-ivory">Supported Agent Targets</h2>
          <div className="space-y-3 font-mono text-xs text-custard pt-2">
            <div className="p-3 rounded-xl bg-espresso-deep border border-espresso-border">
              <span className="text-flamingo font-bold">Google Antigravity:</span> ~/.gemini/antigravity/rules/, skills, and mcp configs.
            </div>
            <div className="p-3 rounded-xl bg-espresso-deep border border-espresso-border">
              <span className="text-flamingo font-bold">Claude Code:</span> Consolidated CLAUDE.md, .claude/mcp.json, and settings.
            </div>
            <div className="p-3 rounded-xl bg-espresso-deep border border-espresso-border">
              <span className="text-flamingo font-bold">Cursor:</span> Modular .cursor/rules/*.mdc, .cursorrules, and .cursor/mcp.json.
            </div>
            <div className="p-3 rounded-xl bg-espresso-deep border border-espresso-border">
              <span className="text-flamingo font-bold">OpenCode:</span> AGENTS.md, .opencode/config.json, and skills directory.
            </div>
          </div>
        </section>

        <section className="p-6 rounded-2xl ceramic-card border border-espresso-border space-y-3">
          <h2 className="font-editorial text-2xl text-ivory">Sync Command & Conflict Resolution</h2>
          <div className="p-4 rounded-xl bg-espresso-deep border border-espresso-border font-mono text-xs text-custard space-y-2">
            <div>$ aek agent sync --all --strategy replace-managed</div>
            <div className="text-[11px] text-sand/70 pt-2">
              Cryptographic hashes in <code className="text-custard">.aek/managed-agents.toml</code> ensure user additions are never lost.
            </div>
          </div>
        </section>
      </div>
    </div>
  );
}

