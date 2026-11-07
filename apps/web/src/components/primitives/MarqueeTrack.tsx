"use client";

import React from "react";

export interface MarqueeItem {
  label: string;
  tag?: string;
  icon?: React.ReactNode;
}

interface MarqueeTrackProps {
  items?: MarqueeItem[];
  reverse?: boolean;
  className?: string;
  speed?: number;
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
      className={`relative w-full overflow-hidden bg-space-black border-y-2 border-primary-gold/40 py-3 select-none ${
        pauseOnHover ? "hover:[&>div]:[animation-play-state:paused]" : ""
      } ${className}`}
      aria-label="Scrolling announcements"
    >
      <div
        className={`flex w-max animate-marquee gap-8 items-center ${
          reverse ? "direction-reverse" : ""
        }`}
      >
        {/* Render 3 identical sets to ensure a 100% seamless infinite loop */}
        {[...items, ...items, ...items].map((item, idx) => (
          <div key={idx} className="inline-flex items-center gap-3 text-xs font-mono font-bold tracking-widest text-paper-beige">
            {item.icon && <span className="text-primary-gold shrink-0">{item.icon}</span>}
            <span className="uppercase">{item.label}</span>
            {item.tag && (
              <span className="px-1.5 py-0.5 rounded text-[10px] bg-space-surface border border-space-border text-primary-gold font-bold">
                {item.tag}
              </span>
            )}
            <span className="text-warm-orange font-black">★</span>
          </div>
        ))}
      </div>
    </div>
  );
}
