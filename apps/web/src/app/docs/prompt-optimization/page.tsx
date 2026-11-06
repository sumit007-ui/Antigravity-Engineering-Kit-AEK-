export default function PromptOptimizationDocPage() {
  return (
    <div className="space-y-8">
      <div>
        <span className="font-mono text-xs text-flamingo font-semibold">PROMPT ENGINEERING</span>
        <h1 className="font-editorial text-4xl text-ivory tracking-tight mt-1 mb-3">
          Prompt Optimization & DSPy
        </h1>
        <p className="text-secondary-text text-sm sm:text-base leading-relaxed">
          Structured prompt templates, Stanford DSPy declarative signatures, and Promptfoo evaluation suites.
        </p>
      </div>

      <div className="space-y-8 text-sand/90 text-sm leading-relaxed">
        <section className="p-6 rounded-2xl ceramic-card border border-espresso-border space-y-3">
          <h2 className="font-editorial text-2xl text-ivory">Declarative DSPy Signatures</h2>
          <p className="text-xs text-sand/80">
            AEK translates loose markdown instructions into typed, declarative DSPy signatures for self-optimizing pipelines:
          </p>
          <div className="p-4 rounded-xl bg-espresso-deep border border-espresso-border font-mono text-xs text-custard">
            $ aek prompt export-dspy --signature CodeReview
          </div>
        </section>

        <section className="p-6 rounded-2xl ceramic-card border border-espresso-border space-y-3">
          <h2 className="font-editorial text-2xl text-ivory">Automated Promptfoo Test Suites</h2>
          <p className="text-xs text-sand/80">
            Generate and run behavioral regression assertions (`llm-rubric`, `is-json`, `contains`) before deploying agent prompts:
          </p>
          <div className="p-4 rounded-xl bg-espresso-deep border border-espresso-border font-mono text-xs text-custard">
            $ aek prompt eval --config promptfoo.yaml
          </div>
        </section>
      </div>
    </div>
  );
}

