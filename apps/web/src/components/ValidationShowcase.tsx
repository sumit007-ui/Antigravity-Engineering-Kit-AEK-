"use client";

import React, { useState } from "react";
import { CheckCircle2, AlertTriangle, XCircle, Clock, ShieldCheck, Terminal } from "lucide-react";
import { cn } from "@/lib/utils";

interface ValidationStep {
  name: string;
  stage: string;
  status: "PASS" | "WARN" | "FAIL" | "SKIP";
  duration: string;
  details: string;
}

interface LanguageValidationPreset {
  id: string;
  name: string;
  steps: ValidationStep[];
  overallDuration: string;
}

const PRESETS: LanguageValidationPreset[] = [
  {
    id: "rust",
    name: "Rust (Cargo)",
    overallDuration: "238 ms",
    steps: [
      { name: "rustfmt", stage: "Formatting", status: "PASS", duration: "24 ms", details: "All 16 workspace crates formatted with standard rustfmt" },
      { name: "clippy", stage: "Static Analysis", status: "PASS", duration: "112 ms", details: "Zero compiler warnings with -D warnings enforced" },
      { name: "cargo-check", stage: "Compilation", status: "PASS", duration: "48 ms", details: "AST type verification and dependency tree intact" },
      { name: "cargo-test", stage: "Unit & Integration", status: "PASS", duration: "42 ms", details: "166 unit, integration, and security tests passed" },
      { name: "cargo-audit", stage: "Security Audit", status: "PASS", duration: "12 ms", details: "0 known vulnerabilities in dependency graph" },
    ]
  },
  {
    id: "python",
    name: "Python (uv / ruff)",
    overallDuration: "192 ms",
    steps: [
      { name: "ruff format", stage: "Formatting", status: "PASS", duration: "18 ms", details: "Checked 42 files across source directory" },
      { name: "ruff check", stage: "Static Analysis", status: "PASS", duration: "32 ms", details: "0 lint infractions or deprecated imports" },
      { name: "mypy", stage: "Type Checking", status: "PASS", duration: "74 ms", details: "Strict typing satisfied across modules" },
      { name: "pytest", stage: "Unit Testing", status: "PASS", duration: "52 ms", details: "28 test cases executed with 100% pass rate" },
      { name: "pip-audit", stage: "Dependency Audit", status: "PASS", duration: "16 ms", details: "PyPI advisory database verified" },
    ]
  },
  {
    id: "typescript",
    name: "TypeScript (pnpm / biome)",
    overallDuration: "164 ms",
    steps: [
      { name: "prettier", stage: "Formatting", status: "PASS", duration: "14 ms", details: "Code formatting conforms to repo style rules" },
      { name: "eslint", stage: "Linting", status: "PASS", duration: "38 ms", details: "Strict ruleset verified with zero warnings" },
      { name: "tsc --noEmit", stage: "Type Checking", status: "PASS", duration: "64 ms", details: "TypeScript type graph verified cleanly" },
      { name: "vitest", stage: "Testing", status: "PASS", duration: "34 ms", details: "Component and utility unit suites green" },
      { name: "npm audit", stage: "Security Scan", status: "PASS", duration: "14 ms", details: "0 high or critical advisory findings" },
    ]
  },
  {
    id: "go",
    name: "Go (go toolchain)",
    overallDuration: "148 ms",
    steps: [
      { name: "gofmt", stage: "Formatting", status: "PASS", duration: "12 ms", details: "Standard gofmt indentation and imports checked" },
      { name: "go vet", stage: "Static Analysis", status: "PASS", duration: "28 ms", details: "Heuristic diagnostics reported 0 issues" },
      { name: "golangci-lint", stage: "Linter Suite", status: "PASS", duration: "54 ms", details: "Enabled linters passed with 0 errors" },
      { name: "go test", stage: "Testing", status: "PASS", duration: "38 ms", details: "Package tests executed in 38 ms" },
      { name: "govulncheck", stage: "Security Audit", status: "PASS", duration: "16 ms", details: "Go vulnerability database verified" },
    ]
  }
];

export default function ValidationShowcase() {
  const [activePreset, setActivePreset] = useState<LanguageValidationPreset>(PRESETS[0]);

  return (
    <section className="py-20 bg-surface/30 border-t border-border/50 relative">
      <div className="max-w-[1280px] mx-auto px-4 sm:px-6 lg:px-8">
        {/* Section Header */}
        <div className="text-center max-w-3xl mx-auto mb-16">
          <div className="inline-flex items-center gap-2 px-3 py-1 rounded-full bg-surface-elevated border border-border text-xs font-mono text-accent mb-4">
            QUALITY GATES
          </div>
          <h2 className="text-3xl sm:text-4xl lg:text-5xl font-extrabold text-primary-text tracking-tight mb-4">
            Ship with confidence.
          </h2>
          <p className="text-secondary-text text-base sm:text-lg">
            Standardized 8-stage verification pipeline with incremental Git change detection and configurable quality thresholds.
          </p>
        </div>

        {/* Language Tabs */}
        <div className="flex items-center justify-center gap-2 mb-8 overflow-x-auto pb-2">
          {PRESETS.map((preset) => (
            <button
              key={preset.id}
              onClick={() => setActivePreset(preset)}
              className={cn(
                "px-4 py-2 text-xs font-mono rounded-xl border transition-all focus:outline-none focus-visible:ring-2 focus-visible:ring-accent",
                activePreset.id === preset.id
                  ? "bg-surface-elevated text-accent border-accent shadow-md shadow-accent/10 font-bold"
                  : "bg-surface border-border text-secondary-text hover:text-primary-text hover:bg-surface-elevated/40"
              )}
            >
              {preset.name}
            </button>
          ))}
        </div>

        {/* Dashboard Display Card */}
        <div className="max-w-4xl mx-auto rounded-2xl bg-surface border border-border shadow-2xl overflow-hidden">
          {/* Dashboard Header Bar */}
          <div className="p-4 sm:p-5 bg-surface-elevated border-b border-border flex flex-col sm:flex-row sm:items-center justify-between gap-3 text-xs font-mono">
            <div className="flex items-center gap-3">
              <span className="flex items-center gap-1.5 px-2.5 py-1 rounded-md bg-status-success/10 text-status-success border border-status-success/20 font-bold">
                <CheckCircle2 className="w-3.5 h-3.5" />
                OVERALL STATUS: PASSED
              </span>
              <span className="text-secondary-text">Pipeline: 8-Stage DAG</span>
            </div>
            <div className="flex items-center gap-4 text-secondary-text">
              <span className="flex items-center gap-1">
                <Clock className="w-3.5 h-3.5" /> Total Duration: {activePreset.overallDuration}
              </span>
              <span className="text-status-success font-semibold">0 Warnings</span>
            </div>
          </div>

          {/* Validation Steps Table */}
          <div className="p-4 sm:p-6 space-y-3 font-mono text-xs">
            {activePreset.steps.map((step, idx) => (
              <div
                key={idx}
                className="flex flex-col sm:flex-row sm:items-center justify-between p-3 rounded-xl bg-[#08090D] border border-border/80 gap-2 hover:border-border transition-colors"
              >
                <div className="flex items-center gap-3">
                  <span className="px-2 py-0.5 rounded text-[10px] bg-status-success/10 text-status-success border border-status-success/20 font-bold">
                    {step.status}
                  </span>
                  <div>
                    <span className="text-primary-text font-bold mr-2">{step.name}</span>
                    <span className="text-secondary-text/70 text-[11px] font-sans">({step.stage})</span>
                  </div>
                </div>

                <div className="flex items-center justify-between sm:justify-end gap-4 text-secondary-text text-[11px]">
                  <span className="hidden md:inline-block text-secondary-text/80">{step.details}</span>
                  <span className="text-secondary-text font-mono font-medium">{step.duration}</span>
                </div>
              </div>
            ))}
          </div>

          {/* Dashboard Footer */}
          <div className="p-3.5 bg-surface-elevated/40 border-t border-border flex items-center justify-between text-[11px] font-mono text-secondary-text">
            <span>Command: <code className="text-accent">aek validate --incremental</code></span>
            <span>Incremental Git detection: 100% AST verified</span>
          </div>
        </div>
      </div>
    </section>
  );
}

