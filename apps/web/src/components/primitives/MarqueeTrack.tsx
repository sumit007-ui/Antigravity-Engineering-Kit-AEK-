"use client";

import React from "react";

interface MarqueeTrackProps {
  items: Array<{ label: string; tag?: string; icon?: React.ReactNode }>;
  reverse?: boolean;
  className?: string;
}

export default function MarqueeTrack({ items, reverse = false, className = "" }: MarqueeTrackProps) {
  return (
    <div
      className={`relative w-full overflow-hidden bg-black border-y-2 border-border py-3 select-none ${className}`}
      aria-label="Scrolling announcements"
    >
      <div
        className={`flex w-max animate-marquee gap-8 items-center ${
          reverse ? "direction-reverse" : ""
        }`}
      >
        {/* Render 3 identical sets to ensure a 100% seamless infinite loop */}
        {[...items, ...items, ...items].map((item, idx) => (
          <div key={idx} className="inline-flex items-center gap-3 text-xs font-mono font-bold tracking-widest text-fg">
            {item.icon && <span className="text-accent-primary shrink-0">{item.icon}</span>}
            <span className="uppercase">{item.label}</span>
            {item.tag && (
              <span className="px-1.5 py-0.5 rounded text-[10px] bg-surface-card border border-border text-accent-gold font-bold">
                {item.tag}
              </span>
            )}
            <span className="text-accent-primary font-black">★</span>
          </div>
        ))}
      </div>
    </div>
  );
}
