import React from "react";

interface SectionHeaderProps {
  coordinates?: string;
  category?: string;
  sectionCode?: string;
  sectionNumber?: string;
  sectionNum?: string;
  sectionTag?: string;
  badgePill?: string;
  title: string;
  subtitle?: string;
  align?: "left" | "center";
  className?: string;
}

export default function SectionHeader({
  coordinates = "15°29'N 73°49'E",
  category = "AEK_MISSION_HUD",
  sectionCode = "SEC_02",
  sectionNumber,
  sectionNum,
  sectionTag,
  badgePill,
  title,
  subtitle,
  align = "center",
  className = "",
}: SectionHeaderProps) {
  const isCenter = align === "center";
  const num = sectionNumber || sectionNum;

  return (
    <div className={`mb-12 sm:mb-16 ${isCenter ? "text-center mx-auto max-w-3xl" : "text-left max-w-3xl"} ${className}`}>
      {/* 1. Tracked Monospace Classified Eyebrow */}
      <div className="label text-[11px] text-muted tracking-widest flex items-center gap-2 mb-2 justify-center">
        <span className="text-accent-primary font-bold">✦</span>
        <span>{coordinates}</span>
        <span className="text-border">•</span>
        <span>{category}</span>
        <span className="text-border">•</span>
        <span className="text-accent-gold font-bold">{sectionCode}</span>
      </div>

      {/* 2. Sub-label with § mark and optional badge */}
      {(num || sectionTag || badgePill) && (
        <div className="flex items-center gap-2 justify-center mb-3">
          <span className="text-xs font-mono font-bold uppercase tracking-wider text-accent-secondary">
            §{num || "00"} — {sectionTag || "PROTOCOL"}
          </span>
          {badgePill && (
            <span className="px-2 py-0.5 rounded text-[10px] font-mono font-bold bg-surface-card border border-border text-accent-gold uppercase">
              {badgePill}
            </span>
          )}
        </div>
      )}

      {/* 3. High-Impact H2 Headline */}
      <h2 className="font-display text-3xl sm:text-4xl md:text-5xl font-black text-fg tracking-tight leading-[1.08] mb-3">
        {title}
      </h2>

      {/* 4. Single-Sentence Subtitle */}
      {subtitle && (
        <p className="text-sm sm:text-base text-muted font-normal leading-relaxed max-w-2xl mx-auto">
          {subtitle}
        </p>
      )}
    </div>
  );
}

