"use client";

import { motion } from "motion/react";
import { cn } from "@/lib/utils";

export type DomainIconType =
  | "rules"
  | "skills"
  | "mcp"
  | "languages"
  | "prompt"
  | "validation"
  | "security"
  | "multi-agent";

interface Domain3DIconProps {
  type: DomainIconType;
  className?: string;
  size?: "sm" | "md" | "lg";
}

export default function Domain3DIcon({
  type,
  className,
  size = "md",
}: Domain3DIconProps) {
  const sizeMap = {
    sm: "w-10 h-10",
    md: "w-14 h-14",
    lg: "w-20 h-20",
  };

  const renderVisual = () => {
    switch (type) {
      case "rules":
        // Layered precision plates with gold/custard ceramic finish
        return (
          <div className="relative w-full h-full flex items-center justify-center">
            {/* Base plate */}
            <div className="absolute w-4/5 h-4/5 rounded-lg bg-brandy/70 border border-brandy-light shadow-tactile transform rotate-6 translate-y-1" />
            {/* Middle plate */}
            <div className="absolute w-4/5 h-4/5 rounded-lg bg-espresso-surface border border-custard/30 transform -rotate-3" />
            {/* Top ceramic plate */}
            <div className="absolute w-4/5 h-4/5 rounded-lg bg-gradient-to-br from-custard/90 to-sand border border-custard shadow-ceramic flex items-center justify-center">
              <div className="w-2.5 h-2.5 rounded-full bg-brandy" />
            </div>
          </div>
        );

      case "skills":
        // Interlocking modular blocks
        return (
          <div className="relative w-full h-full flex items-center justify-center">
            <div className="w-1/2 h-1/2 rounded-md bg-gradient-to-br from-flamingo to-flamingo-hover border border-flamingo-light shadow-tactile transform -translate-x-1 -translate-y-1" />
            <div className="w-1/2 h-1/2 rounded-md bg-gradient-to-br from-brandy to-espresso-surface border border-brandy-light shadow-tactile transform translate-x-1 translate-y-1" />
            <div className="absolute w-1/3 h-1/3 rounded-full bg-custard border border-ivory shadow-glow" />
          </div>
        );

      case "mcp":
        // Connected ports / nodes with conduit lines
        return (
          <div className="relative w-full h-full flex items-center justify-center">
            <svg viewBox="0 0 48 48" className="w-full h-full drop-shadow-md" fill="none">
              <path
                d="M12 24H36M24 12V36M15 15L33 33M15 33L33 15"
                stroke="#382C2D"
                strokeWidth="2.5"
                strokeLinecap="round"
              />
              <circle cx="24" cy="24" r="6" fill="#98B4D4" stroke="#FFF9F0" strokeWidth="2" />
              <circle cx="12" cy="24" r="3.5" fill="#F8D878" stroke="#382C2D" strokeWidth="1.5" />
              <circle cx="36" cy="24" r="3.5" fill="#F8D878" stroke="#382C2D" strokeWidth="1.5" />
              <circle cx="24" cy="12" r="3.5" fill="#F8D878" stroke="#382C2D" strokeWidth="1.5" />
              <circle cx="24" cy="36" r="3.5" fill="#F8D878" stroke="#382C2D" strokeWidth="1.5" />
            </svg>
          </div>
        );

      case "languages":
        // Mechanical multi-tool object / precision calipers
        return (
          <div className="relative w-full h-full flex items-center justify-center">
            <div className="w-4/5 h-4/5 rounded-full border-2 border-brandy/80 flex items-center justify-center">
              <div className="w-3/5 h-3/5 rounded-md border border-flamingo bg-espresso-surface transform rotate-45 flex items-center justify-center">
                <div className="w-2 h-2 rounded-full bg-custard" />
              </div>
              <div className="absolute top-1 w-1.5 h-3 bg-flamingo rounded-full" />
              <div className="absolute bottom-1 w-1.5 h-3 bg-brandy-light rounded-full" />
            </div>
          </div>
        );

      case "prompt":
        // Sculptural folded origami / faceted crystalline form
        return (
          <div className="relative w-full h-full flex items-center justify-center">
            <div className="w-3/4 h-3/4 rounded-xl bg-gradient-to-tr from-brandy via-flamingo to-custard p-0.5 shadow-tactile transform rotate-12">
              <div className="w-full h-full rounded-[10px] bg-espresso-deep/90 flex items-center justify-center">
                <span className="font-editorial text-lg italic text-custard font-bold">π</span>
              </div>
            </div>
          </div>
        );

      case "validation":
        // Precision check mechanism / micrometer gauge
        return (
          <div className="relative w-full h-full flex items-center justify-center">
            <div className="w-4/5 h-4/5 rounded-full bg-espresso-surface border-2 border-custard/50 shadow-tactile flex items-center justify-center">
              <div className="w-3/5 h-3/5 rounded-full border border-dashed border-flamingo flex items-center justify-center">
                <div className="w-3 h-3 rounded-full bg-emerald-500 shadow-glow" />
              </div>
            </div>
          </div>
        );

      case "security":
        // Protective geometric shield
        return (
          <div className="relative w-full h-full flex items-center justify-center">
            <div className="w-4/5 h-5/6 rounded-t-2xl rounded-b-[28px] bg-gradient-to-b from-brandy via-espresso-surface to-brandy-dark border border-flamingo/50 shadow-tactile flex items-center justify-center">
              <div className="w-3 h-4 rounded-t-sm rounded-b-md border border-custard bg-flamingo/40" />
            </div>
          </div>
        );

      case "multi-agent":
        // Connected modular satellites
        return (
          <div className="relative w-full h-full flex items-center justify-center">
            <div className="w-5 h-5 rounded-full bg-custard border-2 border-brandy shadow-glow flex items-center justify-center">
              <div className="w-2 h-2 rounded-full bg-brandy" />
            </div>
            <div className="absolute top-1 left-2 w-3.5 h-3.5 rounded-full bg-flamingo border border-ivory" />
            <div className="absolute bottom-1 right-2 w-3.5 h-3.5 rounded-full bg-sand border border-brandy" />
            <div className="absolute top-2 right-1 w-3 h-3 rounded-full bg-dustyRose border border-espresso" />
          </div>
        );
    }
  };

  return (
    <motion.div
      whileHover={{ scale: 1.08, rotate: 2 }}
      transition={{ type: "spring", stiffness: 300, damping: 15 }}
      className={cn(
        "relative rounded-xl p-2 flex items-center justify-center bg-espresso-surface/80 border border-espresso-border shadow-ceramic select-none",
        sizeMap[size],
        className
      )}
      aria-hidden="true"
    >
      {renderVisual()}
    </motion.div>
  );
}

