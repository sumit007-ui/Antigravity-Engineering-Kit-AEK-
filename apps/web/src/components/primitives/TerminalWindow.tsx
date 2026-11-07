"use client";

import React, { useState } from "react";
import { Terminal, Copy, Check } from "lucide-react";

interface TerminalWindowProps {
  title?: string;
  command: string;
  output?: string[];
  statusRow?: { label: string; value: string; color?: string };
  className?: string;
}

export default function TerminalWindow({
  title = "SYS_TERMINAL: READY",
  command,
  output = [],
  statusRow,
  className = "",
}: TerminalWindowProps) {
  const [copied, setCopied] = useState(false);

  const handleCopy = () => {
    navigator.clipboard.writeText(command);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div
      className={`rounded-lg border-2 border-border bg-black shadow-hard-md overflow-hidden font-mono text-xs ${className}`}
    >
      {/* Window Chrome Header */}
      <div className="bg-surface-elevated px-4 py-2.5 border-b border-border flex items-center justify-between text-muted select-none">
        <div className="flex items-center gap-2">
          <div className="flex items-center gap-1.5">
            <span className="w-2.5 h-2.5 rounded-full bg-danger/80" />
            <span className="w-2.5 h-2.5 rounded-full bg-accent-gold/80" />
            <span className="w-2.5 h-2.5 rounded-full bg-success/80" />
          </div>
          <span className="text-[11px] font-bold text-fg ml-2 flex items-center gap-1">
            <Terminal className="w-3.5 h-3.5 text-accent-primary" />
            <span>{title}</span>
          </span>
        </div>

        <div className="flex items-center gap-3">
          <span className="text-[10px] text-muted hidden sm:inline">RUST_SUBPROCESS</span>
          <button
            onClick={handleCopy}
            className="p-1.5 rounded hover:bg-surface-card hover:text-fg transition-colors flex items-center gap-1 text-[11px] text-accent-gold"
            title="Copy command"
            aria-label="Copy terminal command"
          >
            {copied ? (
              <>
                <Check className="w-3.5 h-3.5 text-success" />
                <span className="text-success text-[10px]">COPIED</span>
              </>
            ) : (
              <>
                <Copy className="w-3.5 h-3.5" />
                <span className="text-[10px]">COPY</span>
              </>
            )}
          </button>
        </div>
      </div>

      {/* Terminal Body */}
      <div className="p-5 space-y-3">
        <div className="flex items-center gap-2 text-accent-gold font-bold">
          <span className="text-accent-primary select-none">$</span>
          <span>{command}</span>
          <span className="w-2 h-4 bg-accent-primary animate-radar inline-block ml-1" />
        </div>

        {output.length > 0 && (
          <div className="space-y-1 text-muted text-[11px] pt-1 border-t border-border/50">
            {output.map((line, idx) => (
              <p key={idx} className="leading-relaxed">
                {line}
              </p>
            ))}
          </div>
        )}

        {statusRow && (
          <div className="pt-2 border-t border-border/40 flex items-center justify-between text-[11px]">
            <span className="text-muted tracking-widest uppercase">
              {statusRow.label}:
            </span>
            <span className="font-bold text-success flex items-center gap-1.5">
              <span className="w-1.5 h-1.5 rounded-full bg-success animate-radar" />
              <span>{statusRow.value}</span>
            </span>
          </div>
        )}
      </div>
    </div>
  );
}

