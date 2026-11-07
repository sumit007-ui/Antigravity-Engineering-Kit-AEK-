"use client";

import React from "react";

export interface MarqueeItem {
  label: string;
  tag?: string;
  icon?: React.ReactNode;
  glowColor?: string; // Signature color to glow on hover
}

interface MarqueeTrackProps {
  items: MarqueeItem[];
  reverse?: boolean;
  className?: string;
  pauseOnHover?: boolean;
}

export default function MarqueeTrack({
  items = [],
  reverse = false,
  className = "",
  pauseOnHover = true,
}: MarqueeTrackProps) {
  if (!items || items.length === 0) {
    return null;
  }

  return (
    <div
      className={`relative w-full overflow-hidden bg-space-black border-y-2 border-primary-gold/40 py-2.5 select-none ${
        pauseOnHover ? "hover:[&>div]:[animation-play-state:paused]" : ""
      } ${className}`}
      aria-label="Scrolling announcements"
    >
      <div
        className={`flex w-max animate-marquee gap-6 items-center ${
          reverse ? "direction-reverse" : ""
        }`}
      >
        {/* Render 3 identical sets to ensure a 100% seamless infinite loop */}
        {[...items, ...items, ...items].map((item, idx) => {
          const glow = item.glowColor || "#FFD36B";
          return (
            <div
              key={idx}
              className="group/item relative inline-flex items-center gap-2.5 px-3 py-1.5 rounded-lg border border-transparent transition-all duration-200 cursor-pointer shrink-0 text-xs font-mono font-bold tracking-widest text-paper-beige hover:bg-space-surface/90"
              onMouseEnter={(e) => {
                const el = e.currentTarget;
                el.style.borderColor = glow;
                el.style.boxShadow = `0 0 16px ${glow}40`;
              }}
              onMouseLeave={(e) => {
                const el = e.currentTarget;
                el.style.borderColor = "transparent";
                el.style.boxShadow = "none";
              }}
            >
              {/* Official Icon with Dynamic Color Glow on Hover */}
              {item.icon && (
                <span
                  className="shrink-0 transition-all duration-200 group-hover/item:scale-115 flex items-center justify-center"
                  style={{
                    filter: "drop-shadow(0 0 0px transparent)",
                  }}
                  onMouseEnter={(e) => {
                    (e.currentTarget as HTMLElement).style.filter = `drop-shadow(0 0 10px ${glow})`;
                  }}
                  onMouseLeave={(e) => {
                    (e.currentTarget as HTMLElement).style.filter = "none";
                  }}
                >
                  {item.icon}
                </span>
              )}

              {/* Item Label with Hover Transition */}
              <span
                className="uppercase transition-colors duration-200 group-hover/item:text-fg"
              >
                {item.label}
              </span>

              {/* Tag Badge with Matching Accent Border */}
              {item.tag && (
                <span
                  className="px-1.5 py-0.5 rounded text-[10px] bg-space-surface border font-bold transition-all duration-200"
                  style={{
                    borderColor: `${glow}60`,
                    color: glow,
                  }}
                >
                  {item.tag}
                </span>
              )}

              <span className="text-warm-orange font-black text-[10px] ml-1">★</span>
            </div>
          );
        })}
      </div>
    </div>
  );
}
