export default function SecurityDocPage() {
  return (
    <div className="space-y-8">
      <div>
        <span className="font-mono text-xs text-flamingo font-semibold">SECURITY & AUDITING</span>
        <h1 className="font-editorial text-4xl text-ivory tracking-tight mt-1 mb-3">
          Security Scanners & Invariants
        </h1>
        <p className="text-secondary-text text-sm sm:text-base leading-relaxed">
          Semgrep SAST, Trivy container and misconfiguration scanning, and dependency vulnerability auditing.
        </p>
      </div>

      <div className="space-y-8 text-sand/90 text-sm leading-relaxed">
        <section className="p-6 rounded-2xl ceramic-card border border-espresso-border space-y-3">
          <h2 className="font-editorial text-2xl text-ivory">Core Security Invariants</h2>
          <ul className="space-y-2 text-xs font-mono text-custard pl-4 border-l-2 border-brandy-light">
            <li>• Safe Subprocess: Uses direct argument arrays (Command::new(cmd).args([...])), avoiding any shell interpretation.</li>
            <li>• Path Traversal Defense: Every path is verified with is_safe_subpath to prevent directory escapes.</li>
            <li>• Zero Credential Exposure: Tokens and secrets are sanitized from logs and error diagnostics.</li>
            <li>• Strict Containment: External tools execute in unprivileged sandboxes with pre-allocated buffer limits.</li>
          </ul>
        </section>

        <section className="p-6 rounded-2xl ceramic-card border border-espresso-border space-y-3">
          <h2 className="font-editorial text-2xl text-ivory">Running Security Audits</h2>
          <div className="p-4 rounded-xl bg-espresso-deep border border-espresso-border font-mono text-xs text-custard space-y-2">
            <div>
              <span className="text-muted-text"># Run full security audit suite:</span>
              <div>$ aek security audit</div>
            </div>
            <div className="pt-2">
              <span className="text-muted-text"># Run doctor to inspect scanner availability:</span>
              <div>$ aek security doctor</div>
            </div>
          </div>
        </section>
      </div>
    </div>
  );
}

