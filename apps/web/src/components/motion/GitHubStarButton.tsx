"use client";

import { useState } from "react";
import { motion, useReducedMotion } from "motion/react";
import { Github, Star, Sparkles } from "lucide-react";
import { GITHUB_REPO_URL } from "@/lib/utils";

interface GitHubStarButtonProps {
  stars?: string;
  className?: string;
}

export default function GitHubStarButton({
  stars = "1.2k",
  className = "",
}: GitHubStarButtonProps) {
  const [isHovered, setIsHovered] = useState(false);
  const shouldReduceMotion = useReducedMotion();

  return (
    <motion.a
      href={GITHUB_REPO_URL}
      target="_blank"
      rel="noopener noreferrer"
      aria-label="GitHub Repository and Star"
      onHoverStart={() => setIsHovered(true)}
      onHoverEnd={() => setIsHovered(false)}
      whileHover={shouldReduceMotion ? {} : { scale: 1.03, y: -1 }}
      whileTap={shouldReduceMotion ? {} : { scale: 0.97 }}
      transition={{ type: "spring", stiffness: 400, damping: 25 }}
      className={`relative inline-flex items-center gap-2 px-3 py-2 rounded-xl border-2 border-primary-gold/50 bg-space-surface/90 hover:border-primary-gold text-xs font-mono font-bold text-paper-beige shadow-hard-sm cursor-pointer select-none min-h-[44px] overflow-hidden group transition-colors ${className}`}
    >
      {/* Ambient glowing background aura on hover */}
      <motion.div
        className="absolute inset-0 bg-gradient-to-r from-warm-orange/15 via-primary-gold/15 to-transparent pointer-events-none"
        initial={{ opacity: 0 }}
        animate={{ opacity: isHovered ? 1 : 0 }}
        transition={{ duration: 0.25 }}
      />

      {/* GitHub Octocat Icon */}
      <div className="relative flex items-center justify-center">
        <Github className="w-4 h-4 text-paper-beige group-hover:text-primary-gold transition-colors duration-200" />
      </div>

      <span className="hidden xl:inline text-sand/80 group-hover:text-paper-beige transition-colors duration-200">
        GitHub
      </span>

      {/* Star Badge with Interactive Motion Physics */}
      <div className="relative flex items-center gap-1.5 pl-2 border-l border-space-border/80">
        <motion.div
          animate={
            shouldReduceMotion
              ? {}
              : isHovered
              ? { rotate: [0, -15, 20, 0], scale: [1, 1.25, 1.35, 1.2] }
              : { rotate: 0, scale: [1, 1.08, 1] }
          }
          transition={
            isHovered
              ? { duration: 0.45, ease: "easeOut" }
              : { repeat: Infinity, duration: 2.8, ease: "easeInOut" }
          }
          className="relative flex items-center justify-center"
        >
          {/* Subtle star glow backdrop */}
          <div className="absolute inset-0 rounded-full bg-primary-gold/30 blur-xs scale-150 opacity-0 group-hover:opacity-100 transition-opacity duration-300" />
          <Star className="w-3.5 h-3.5 text-primary-gold fill-primary-gold" />
        </motion.div>

        <span className="tabular-nums font-mono text-[11px] font-bold text-primary-gold tracking-tight">
          ★ {stars}
        </span>

        {/* Micro sparkle burst on hover */}
        {!shouldReduceMotion && isHovered && (
          <motion.span
            initial={{ scale: 0, opacity: 0, x: -2 }}
            animate={{ scale: 1, opacity: 1, x: 0 }}
            exit={{ scale: 0, opacity: 0 }}
            className="text-warm-orange"
          >
            <Sparkles className="w-3 h-3 text-warm-orange" />
          </motion.span>
        )}
      </div>

      {/* Corner Pixel Highlight Accent */}
      <div className="absolute top-0 right-0 w-1.5 h-1.5 bg-primary-gold/60 pointer-events-none" />
    </motion.a>
  );
}
