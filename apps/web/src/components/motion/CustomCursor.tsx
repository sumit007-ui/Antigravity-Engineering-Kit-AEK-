"use client";

import { useEffect, useState } from "react";
import { motion, useMotionValue, useSpring } from "motion/react";

export default function CustomCursor() {
  const [enabled, setEnabled] = useState(false);
  const [isHovered, setIsHovered] = useState(false);

  const cursorX = useMotionValue(-100);
  const cursorY = useMotionValue(-100);

  const springConfig = { damping: 24, stiffness: 220, mass: 0.5 };
  const smoothX = useSpring(cursorX, springConfig);
  const smoothY = useSpring(cursorY, springConfig);

  useEffect(() => {
    // Strictly disable custom cursor on mobile, touch, and low-pointer devices
    const isMobile =
      typeof window !== "undefined" &&
      (window.innerWidth < 768 ||
        window.matchMedia("(hover: none)").matches ||
        navigator.maxTouchPoints > 0);

    const hasFinePointer = typeof window !== "undefined" && window.matchMedia("(pointer: fine)").matches;
    const prefersReducedMotion =
      typeof window !== "undefined" && window.matchMedia("(prefers-reduced-motion: reduce)").matches;

    if (isMobile || !hasFinePointer || prefersReducedMotion) {
      return;
    }

    setEnabled(true);

    const handleMouseMove = (e: MouseEvent) => {
      cursorX.set(e.clientX);
      cursorY.set(e.clientY);
    };

    const handleMouseOver = (e: MouseEvent) => {
      const target = e.target as HTMLElement | null;
      if (!target) return;
      const isInteractive = target.closest("a, button, [role='button'], input, textarea, select");
      setIsHovered(!!isInteractive);
    };

    window.addEventListener("mousemove", handleMouseMove, { passive: true });
    document.addEventListener("mouseover", handleMouseOver, { passive: true });

    return () => {
      window.removeEventListener("mousemove", handleMouseMove);
      document.removeEventListener("mouseover", handleMouseOver);
    };
  }, [cursorX, cursorY]);

  if (!enabled) return null;

  return (
    <motion.div
      className="hidden md:block fixed top-0 left-0 pointer-events-none z-[9999] rounded-full border border-flamingo/60 mix-blend-screen"
      style={{
        x: smoothX,
        y: smoothY,
        translateX: "-50%",
        translateY: "-50%",
      }}
      animate={{
        width: isHovered ? 48 : 20,
        height: isHovered ? 48 : 20,
        backgroundColor: isHovered ? "rgba(217, 128, 141, 0.15)" : "rgba(243, 220, 188, 0.1)",
        borderColor: isHovered ? "#D9808D" : "rgba(243, 220, 188, 0.5)",
      }}
      transition={{ type: "spring", damping: 20, stiffness: 250 }}
    />
  );
}

