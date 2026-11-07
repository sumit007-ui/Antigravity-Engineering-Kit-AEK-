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
  coordinates,
  category = "SYSTEM_ARCHITECTURE",
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
  const num = sectionNumber || sectionNum || "00";

  return (
    <div className={`mb-12 sm:mb-16 ${isCenter ? "text-center mx-auto max-w-3xl" : "text-left max-w-3xl"} ${className}`}>
      {/* 1. Tracked Monospace Clean Eyebrow */}
      <div className={`label text-[11px] text-muted tracking-widest flex items-center gap-2 mb-2 ${isCenter ? "justify-center" : "justify-start"}`}>
        <span className="text-accent-primary font-bold">✦</span>
        <span className="text-accent-gold font-bold font-mono">§{num}</span>
        <span className="text-border">•</span>
        <span className="font-mono uppercase">{sectionTag || category}</span>
        <span className="text-border">•</span>
        <span className="text-accent-primary font-mono font-bold">{sectionCode}</span>
      </div>

      {/* 2. Primary Headline (Editorial Style) */}
      <h2 className="font-display text-3xl sm:text-4xl md:text-5xl font-black text-fg tracking-tight leading-[1.08] mb-3">
        {title}
      </h2>

      {/* 3. Subtitle / Summary Line */}
      {subtitle && (
        <p className="text-sm sm:text-base text-muted font-normal leading-relaxed max-w-2xl mx-auto">
          {subtitle}
        </p>
      )}

      {/* 4. Optional Extra Status Badge */}
      {badgePill && (
        <div className={`mt-4 inline-flex items-center gap-2 px-3 py-1 rounded-full border border-border bg-surface-card text-xs font-mono ${isCenter ? "mx-auto" : ""}`}>
          <span className="w-1.5 h-1.5 rounded-full bg-accent-gold animate-pulse" />
          <span className="text-fg font-medium">{badgePill}</span>
        </div>
      )}
    </div>
  );
}
