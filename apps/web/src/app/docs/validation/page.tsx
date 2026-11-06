export default function ValidationDocPage() {
  return (
    <div className="space-y-8">
      <div>
        <span className="font-mono text-xs text-flamingo font-semibold">QUALITY ASSURANCE</span>
        <h1 className="font-editorial text-4xl text-ivory tracking-tight mt-1 mb-3">
          8-Stage Validation Pipeline
        </h1>
        <p className="text-secondary-text text-sm sm:text-base leading-relaxed">
          Topological DAG verification, incremental git filtering, and OASIS SARIF reporting.
        </p>
      </div>

      <div className="space-y-8 text-sand/90 text-sm leading-relaxed">
        <section className="p-6 rounded-2xl ceramic-card border border-espresso-border space-y-3">
          <h2 className="font-editorial text-2xl text-ivory">The Eight Verification Stages</h2>
          <div className="grid grid-cols-1 sm:grid-cols-2 gap-3 font-mono text-xs text-custard pt-2">
            <div className="p-3 rounded-lg bg-espresso-deep border border-espresso-border">Stage 1: Code Formatting</div>
            <div className="p-3 rounded-lg bg-espresso-deep border border-espresso-border">Stage 2: Static Linting</div>
            <div className="p-3 rounded-lg bg-espresso-deep border border-espresso-border">Stage 3: Type Consistency</div>
            <div className="p-3 rounded-lg bg-espresso-deep border border-espresso-border">Stage 4: Compilation / Build</div>
            <div className="p-3 rounded-lg bg-espresso-deep border border-espresso-border">Stage 5: Unit & Integration Tests</div>
            <div className="p-3 rounded-lg bg-espresso-deep border border-espresso-border">Stage 6: Memory & Footprint</div>
            <div className="p-3 rounded-lg bg-espresso-deep border border-espresso-border">Stage 7: Security Audit Gate</div>
            <div className="p-3 rounded-lg bg-espresso-deep border border-espresso-border">Stage 8: SARIF Report Generation</div>
          </div>
        </section>

        <section className="p-6 rounded-2xl ceramic-card border border-espresso-border space-y-3">
          <h2 className="font-editorial text-2xl text-ivory">Executing Quality Gates</h2>
          <div className="p-4 rounded-xl bg-espresso-deep border border-espresso-border font-mono text-xs text-custard space-y-2">
            <div>
              <span className="text-muted-text"># Full verification suite with strict quality gates:</span>
              <div>$ aek validate --strict</div>
            </div>
            <div className="pt-2">
              <span className="text-muted-text"># Fast incremental verification on modified files only:</span>
              <div>$ aek validate --incremental</div>
            </div>
          </div>
        </section>
      </div>
    </div>
  );
}

