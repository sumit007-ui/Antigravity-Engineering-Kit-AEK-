"use client";

import { useState, useEffect } from "react";
import { motion, useReducedMotion } from "motion/react";
import { Github, Star, Sparkles } from "lucide-react";
import { GITHUB_REPO_URL } from "@/lib/utils";

interface GitHubStarButtonProps {
  stars?: string;
  className?: string;
}

export default function GitHubStarButton({
  stars: initialStars,
  className = "",
}: GitHubStarButtonProps) {
  const [stars, setStars] = useState<string | number | null>(initialStars ?? null);
  const [isHovered, setIsHovered] = useState(false);
  const shouldReduceMotion = useReducedMotion();

  // Fetch live star count directly from GitHub API
  useEffect(() => {
    let isMounted = true;

    async function fetchStars() {
      try {
        const response = await fetch(
          "https://api.github.com/repos/sumit007-ui/Antigravity-Engineering-Kit-AEK-",
          {
            headers: { Accept: "application/vnd.github.v3+json" },
            next: { revalidate: 300 },
          }
        );

        if (response.ok) {
          const data = await response.json();
          if (isMounted && typeof data.stargazers_count === "number") {
            const count = data.stargazers_count;
            const formatted =
              count >= 1000 ? `${(count / 1000).toFixed(1)}k` : `${count}`;
            setStars(formatted);
          }
        }
      } catch {
        // Fallback gracefully without breaking UI
        if (isMounted && !stars) {
          setStars(initialStars ?? "5");
        }
      }
    }

    fetchStars();

    return () => {
      isMounted = false;
    };
  }, [initialStars]);

  return (
    <motion.a
      href={GITHUB_REPO_URL}
      target="_blank"
      rel="noopener noreferrer"
      aria-label="GitHub Repository and Star"
      onHoverStart={() => setIsHovered(true)}
      onHoverEnd={() => setIsHovered(false)}
      whileHover={shouldReduceMotion ? {} : { scale: 1.02, y: -1 }}
      whileTap={shouldReduceMotion ? {} : { scale: 0.98 }}
      transition={{ type: "spring", stiffness: 400, damping: 25 }}
      className={`relative inline-flex items-center gap-1.5 sm:gap-2 px-2.5 sm:px-3 py-1.5 sm:py-2 rounded-lg border-2 transition-all duration-200 select-none cursor-pointer overflow-hidden group min-h-[40px] sm:min-h-[44px] ${
        isHovered
          ? "border-[#FFD36B] bg-space-surface/95 shadow-[0_0_16px_rgba(255,211,107,0.22)]"
          : "border-space-border/90 bg-space-surface/80 hover:border-sand/40"
      } ${className}`}
    >
      {/* Golden Ambient Aura on Hover */}
      <motion.div
        className="absolute inset-0 bg-gradient-to-r from-[#FFD36B]/15 via-[#FFC15E]/10 to-transparent pointer-events-none"
        initial={{ opacity: 0 }}
        animate={{ opacity: isHovered ? 1 : 0 }}
        transition={{ duration: 0.2 }}
      />

      {/* GitHub Octocat Icon */}
      <div className="relative flex items-center justify-center">
        <Github
          className={`w-4 h-4 transition-colors duration-200 ${
            isHovered ? "text-[#FFD36B]" : "text-paper-beige/80 group-hover:text-paper-beige"
          }`}
        />
      </div>

      <span
        className={`hidden xl:inline text-xs font-mono font-medium transition-colors duration-200 ${
          isHovered ? "text-paper-beige font-semibold" : "text-sand/80"
        }`}
      >
        GitHub
      </span>

      {/* Star Section with Interactive Golden Physics */}
      <div className="relative flex items-center gap-1.5 pl-1.5 sm:pl-2 border-l border-space-border/60">
        <motion.div
          animate={
            shouldReduceMotion
              ? {}
              : isHovered
              ? { rotate: [0, -18, 22, 0], scale: [1, 1.25, 1.35, 1.2] }
              : { rotate: 0, scale: 1 }
          }
          transition={{ duration: 0.4, ease: "easeOut" }}
          className="relative flex items-center justify-center"
        >
          {/* Radiant Golden Glow Backdrop when Hovered */}
          <div
            className={`absolute inset-0 rounded-full blur-[6px] transition-opacity duration-300 pointer-events-none ${
              isHovered
                ? "bg-[#FFD36B]/60 opacity-100 scale-150"
                : "bg-transparent opacity-0"
            }`}
          />

          <Star
            className={`w-3.5 h-3.5 transition-all duration-200 ${
              isHovered
                ? "text-[#FFD36B] fill-[#FFD36B] drop-shadow-[0_0_8px_rgba(255,211,107,0.95)]"
                : "text-sand/60 fill-none group-hover:text-[#FFD36B]/70"
            }`}
          />
        </motion.div>

        {/* Live Star Count Display */}
        <span
          className={`tabular-nums font-mono text-[11px] tracking-tight transition-colors duration-200 ${
            isHovered
              ? "text-[#FFD36B] font-bold drop-shadow-[0_0_6px_rgba(255,211,107,0.6)]"
              : "text-paper-beige/90 font-medium"
          }`}
        >
          {stars !== null ? (
            `★ ${stars}`
          ) : (
            <span className="inline-block w-4 h-2 bg-space-border/60 animate-pulse rounded-xs" />
          )}
        </span>

        {/* Micro Golden Sparkle Burst on Hover */}
        {!shouldReduceMotion && isHovered && (
          <motion.span
            initial={{ scale: 0, opacity: 0, x: -3 }}
            animate={{ scale: 1, opacity: 1, x: 0 }}
            exit={{ scale: 0, opacity: 0 }}
            className="text-[#FFC15E]"
          >
            <Sparkles className="w-3 h-3 text-[#FFD36B]" />
          </motion.span>
        )}
      </div>

      {/* Retro Pixel Corner Accent */}
      <div
        className={`absolute top-0 right-0 w-1.5 h-1.5 pointer-events-none transition-colors duration-200 ${
          isHovered ? "bg-[#FFD36B]" : "bg-space-border"
        }`}
      />
    </motion.a>
  );
}
