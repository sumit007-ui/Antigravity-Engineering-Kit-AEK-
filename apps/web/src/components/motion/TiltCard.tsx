"use client";

import { useRef, useState } from "react";
import { motion } from "motion/react";
import { cn } from "@/lib/utils";

interface TiltCardProps {
  children: React.ReactNode;
  className?: string;
  glowColor?: "flamingo" | "brandy" | "custard";
}

export default function TiltCard({
  children,
  className,
  glowColor = "flamingo",
}: TiltCardProps) {
  const cardRef = useRef<HTMLDivElement>(null);
  const [rotateX, setRotateX] = useState(0);
  const [rotateY, setRotateY] = useState(0);
  const [spotlight, setSpotlight] = useState({ x: 50, y: 50, opacity: 0 });

  const handleMouseMove = (e: React.MouseEvent<HTMLDivElement>) => {
    if (!cardRef.current) return;
    const rect = cardRef.current.getBoundingClientRect();
    const x = e.clientX - rect.left;
    const y = e.clientY - rect.top;

    const centerX = rect.width / 2;
    const centerY = rect.height / 2;

    const rX = ((y - centerY) / centerY) * -7;
    const rY = ((x - centerX) / centerX) * 7;

    setRotateX(rX);
    setRotateY(rY);
    setSpotlight({
      x: (x / rect.width) * 100,
      y: (y / rect.height) * 100,
      opacity: 1,
    });
  };

  const handleMouseLeave = () => {
    setRotateX(0);
    setRotateY(0);
    setSpotlight((prev) => ({ ...prev, opacity: 0 }));
  };

  const glowStyles = {
    flamingo: "rgba(217, 128, 141, 0.18)",
    brandy: "rgba(115, 56, 45, 0.25)",
    custard: "rgba(243, 220, 188, 0.18)",
  };

  return (
    <motion.div
      ref={cardRef}
      onMouseMove={handleMouseMove}
      onMouseLeave={handleMouseLeave}
      animate={{
        rotateX,
        rotateY,
      }}
      transition={{ type: "spring", stiffness: 260, damping: 20 }}
      style={{
        transformStyle: "preserve-3d",
      }}
      className={cn(
        "relative rounded-2xl ceramic-card p-6 overflow-hidden transition-all duration-300",
        className
      )}
    >
      {/* Specular pointer tracking highlight */}
      <div
        className="pointer-events-none absolute -inset-px transition-opacity duration-300 rounded-2xl"
        style={{
          opacity: spotlight.opacity,
          background: `radial-gradient(400px circle at ${spotlight.x}% ${spotlight.y}%, ${glowStyles[glowColor]}, transparent 70%)`,
        }}
      />
      <div className="relative z-10" style={{ transform: "translateZ(12px)" }}>
        {children}
      </div>
    </motion.div>
  );
}

