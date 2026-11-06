export default function TroubleshootingDocPage() {
  return (
    <div className="space-y-8">
      <div>
        <span className="font-mono text-xs text-flamingo font-semibold">DIAGNOSTICS & DOCTOR</span>
        <h1 className="font-editorial text-4xl text-ivory tracking-tight mt-1 mb-3">
          Troubleshooting & Doctor
        </h1>
        <p className="text-secondary-text text-sm sm:text-base leading-relaxed">
          How to diagnose platform, agent, and toolchain issues with aek doctor.
        </p>
      </div>

      <div className="space-y-8 text-sand/90 text-sm leading-relaxed">
        <section className="p-6 rounded-2xl ceramic-card border border-espresso-border space-y-3">
          <h2 className="font-editorial text-2xl text-ivory">Running the Environment Doctor</h2>
          <p className="text-xs text-sand/80">
            AEK includes a non-destructive doctor command that verifies directory paths, permissions, agent readiness, and compiler presence:
          </p>
          <div className="p-4 rounded-xl bg-espresso-deep border border-espresso-border font-mono text-xs text-custard">
            $ aek doctor
          </div>
        </section>

        <section className="p-6 rounded-2xl ceramic-card border border-espresso-border space-y-3">
          <h2 className="font-editorial text-2xl text-ivory">Common Resolutions</h2>
          <div className="space-y-4 text-xs font-mono">
            <div className="p-4 rounded-xl bg-espresso-deep border border-espresso-border space-y-1">
              <span className="text-flamingo font-bold block">Issue: Missing PATH environment variable</span>
              <span className="text-sand/80 font-sans block">
                On Windows, run in PowerShell: <code className="text-custard">$env:Path += &quot;;$env:LOCALAPPDATA\Programs\aek\bin&quot;</code>. On Linux, ensure <code className="text-custard">~/.local/bin</code> is in your shell profile.
              </span>
            </div>
            <div className="p-4 rounded-xl bg-espresso-deep border border-espresso-border space-y-1">
              <span className="text-flamingo font-bold block">Issue: Rule size limit exceeded</span>
              <span className="text-sand/80 font-sans block">
                Rule files larger than 24 KB are rejected to protect LLM context windows. Split large rules into modular skill runbooks.
              </span>
            </div>
          </div>
        </section>
      </div>
    </div>
  );
}

