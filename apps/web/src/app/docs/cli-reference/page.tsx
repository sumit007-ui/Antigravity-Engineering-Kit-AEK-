export default function CliReferenceDocPage() {
  const COMMANDS = [
    { cmd: "aek init", desc: "Initialize AEK in the current workspace with an engineering profile." },
    { cmd: "aek agent sync", desc: "Synchronize rules, skills, and tools across configured coding agents." },
    { cmd: "aek agent profile", desc: "Apply or inspect pre-configured multi-agent profiles." },
    { cmd: "aek rules list", desc: "List registered engineering rules and audit 24 KB size constraints." },
    { cmd: "aek skills list", desc: "List operational agent skills or discover skills for the current repo." },
    { cmd: "aek mcp list", desc: "List available MCP servers and inspect live runtime status." },
    { cmd: "aek mcp export", desc: "Export MCP tool definitions to an agent's native format." },
    { cmd: "aek language doctor", desc: "Run non-destructive diagnostics on compilers and linters." },
    { cmd: "aek validate", desc: "Execute 8-stage verification pipeline with quality gates." },
    { cmd: "aek security audit", desc: "Run Semgrep, Trivy, and package dependency vulnerability scans." },
    { cmd: "aek doctor", desc: "Audit complete AEK platform readiness, directories, and agents." },
    { cmd: "aek update", desc: "Check for and perform atomic in-place binary update." },
    { cmd: "aek rollback", desc: "Restore the previous binary version from local backup archive." },
    { cmd: "aek backup / restore", desc: "Create timestamped configuration safety copies or restore from backup." },
  ];

  return (
    <div className="space-y-8">
      <div>
        <span className="font-mono text-xs text-flamingo font-semibold">COMMAND REFERENCE</span>
        <h1 className="font-editorial text-4xl text-ivory tracking-tight mt-1 mb-3">
          CLI Command Reference
        </h1>
        <p className="text-secondary-text text-sm sm:text-base leading-relaxed">
          Comprehensive manual for all aek command-line subcommands and flags.
        </p>
      </div>

      <div className="space-y-4">
        {COMMANDS.map((c) => (
          <div
            key={c.cmd}
            className="p-5 rounded-2xl ceramic-card border border-espresso-border flex flex-col sm:flex-row sm:items-center justify-between gap-3 font-mono text-xs"
          >
            <code className="text-custard font-bold text-sm bg-espresso-deep px-3 py-1.5 rounded-lg border border-espresso-border">
              {c.cmd}
            </code>
            <span className="text-sand/80 font-sans text-xs">{c.desc}</span>
          </div>
        ))}
      </div>
    </div>
  );
}

