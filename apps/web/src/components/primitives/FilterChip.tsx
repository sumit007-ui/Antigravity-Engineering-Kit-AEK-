"use client";

import React from "react";

interface FilterChipProps {
  label: string;
  active: boolean;
  onClick: () => void;
  count?: number;
}

export default function FilterChip({ label, active, onClick, count }: FilterChipProps) {
  return (
    <button
      onClick={onClick}
      className={`inline-flex items-center gap-1.5 px-3.5 py-2 rounded text-xs font-mono font-bold uppercase tracking-wider border-2 min-h-[44px] transition-all select-none cursor-pointer focus-visible:outline-2 focus-visible:outline-accent-primary ${
        active
          ? "bg-accent-primary text-black border-black shadow-hard-sm"
          : "bg-surface-card text-muted hover:text-fg border-border hover:border-accent-primary/60 shadow-hard-sm"
      }`}
    >
      <span>[{label}]</span>
      {count !== undefined && (
        <span
          className={`px-1 rounded text-[10px] ${
            active ? "bg-black text-accent-primary" : "bg-surface-elevated text-muted"
          }`}
        >
          {count}
        </span>
      )}
    </button>
  );
}

