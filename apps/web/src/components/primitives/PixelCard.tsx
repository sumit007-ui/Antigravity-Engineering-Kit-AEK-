"use client";

import React, { useState } from "react";
import { Lock, Bell, Check } from "lucide-react";

export function PixelCard({
  children,
  className = "",
  onClick,
}: {
  children: React.ReactNode;
  className?: string;
  onClick?: () => void;
}) {
  return (
    <div
      onClick={onClick}
      className={`rounded-lg border-2 border-border bg-surface-card p-6 shadow-hard-md transition-all duration-160 hover:-translate-x-0.5 hover:-translate-y-0.5 hover:shadow-[6px_6px_0px_#050505] hover:border-accent-primary/60 relative overflow-hidden ${className}`}
    >
      {children}
    </div>
  );
}

export function StatCard({
  tag,
  value,
  caption,
  subCaption,
  highlight = false,
}: {
  tag: string;
  value: string;
  caption: string;
  subCaption?: string;
  highlight?: boolean;
}) {
  return (
    <PixelCard className={`flex flex-col justify-between ${highlight ? "border-accent-primary" : ""}`}>
      <div>
        <div className="flex items-center justify-between mb-4">
          <span className="text-[11px] font-mono font-bold text-accent-gold uppercase tracking-wider">
            [{tag}]
          </span>
          <span className="w-1.5 h-1.5 rounded-full bg-accent-primary animate-radar" />
        </div>
        <div className="stat text-4xl sm:text-5xl text-fg mb-2">
          {value}
        </div>
      </div>
      <div>
        <p className="text-xs font-mono font-bold text-fg uppercase tracking-wide">
          {caption}
        </p>
        {subCaption && (
          <p className="text-[10px] font-mono text-muted uppercase tracking-wider mt-0.5">
            {subCaption}
          </p>
        )}
      </div>
    </PixelCard>
  );
}

export function LockedCard({
  id,
  title,
  status = "ENCRYPTED",
  category = "CLASSIFIED SPEC",
}: {
  id: string;
  title: string;
  status?: string;
  category?: string;
}) {
  const [notified, setNotified] = useState(false);

  return (
    <div className="rounded-lg border-2 border-dashed border-border bg-surface-base p-6 shadow-hard-sm relative flex flex-col justify-between min-h-[220px]">
      <div>
        <div className="flex items-center justify-between mb-3 text-[11px] font-mono">
          <span className="text-muted tracking-widest uppercase">
            [{category}]
          </span>
          <span className="inline-flex items-center gap-1 text-accent-primary font-bold">
            <Lock className="w-3 h-3" />
            <span>{status}</span>
          </span>
        </div>

        <h3 className="font-display text-xl font-bold text-fg/80 mb-2">
          {title}
        </h3>

        <div className="p-2.5 rounded bg-black/60 border border-border/80 font-mono text-[10px] text-muted tracking-wider mb-4">
          <span className="text-accent-gold font-bold">SEALED</span> • REVEAL AT ZERO HOUR
        </div>
      </div>

      <button
        onClick={() => setNotified(true)}
        className={`w-full py-2 px-3 text-xs font-mono font-bold uppercase rounded border-2 transition-all flex items-center justify-center gap-2 select-none min-h-[44px] ${
          notified
            ? "bg-emerald-950 text-emerald-400 border-emerald-600"
            : "bg-surface-elevated text-fg border-border hover:border-accent-primary active:scale-95 shadow-hard-sm"
        }`}
      >
        {notified ? (
          <>
            <Check className="w-3.5 h-3.5" />
            <span>REGISTERED FOR UNLOCK</span>
          </>
        ) : (
          <>
            <Bell className="w-3.5 h-3.5 text-accent-primary" />
            <span>NOTIFY ME</span>
          </>
        )}
      </button>
    </div>
  );
}
