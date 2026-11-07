"use client";

import { useEffect, useState } from "react";

const SCRAMBLE_CHARS = "01#%&*+-=?@[]^_{}~";

interface DecryptTextProps {
  text: string;
  speed?: number;
  className?: string;
}

export default function DecryptText({
  text,
  speed = 30,
  className = "",
}: DecryptTextProps) {
  const [displayText, setDisplayText] = useState(text);

  useEffect(() => {
    if (typeof window === "undefined") return;

    const prefersReducedMotion = window.matchMedia("(prefers-reduced-motion: reduce)").matches;
    if (prefersReducedMotion) {
      setDisplayText(text);
      return;
    }

    let iteration = 0;
    const totalIterations = text.length;

    const interval = setInterval(() => {
      setDisplayText(
        text
          .split("")
          .map((char, index) => {
            if (char === " " || index < iteration) {
              return text[index];
            }
            return SCRAMBLE_CHARS[Math.floor(Math.random() * SCRAMBLE_CHARS.length)];
          })
          .join("")
      );

      iteration += 1 / 2;

      if (iteration >= totalIterations) {
        clearInterval(interval);
        setDisplayText(text);
      }
    }, speed);

    return () => clearInterval(interval);
  }, [text, speed]);

  return <span className={className}>{displayText}</span>;
}
