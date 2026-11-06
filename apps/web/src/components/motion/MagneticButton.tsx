"use client";

import { useRef, useState } from "react";
import { motion } from "motion/react";
import { cn } from "@/lib/utils";

interface MagneticButtonProps {
  children: React.ReactNode;
  className?: string;
  variant?: "primary" | "secondary" | "outline" | "ghost";
  onClick?: React.MouseEventHandler<HTMLButtonElement>;
  type?: "button" | "submit" | "reset";
  disabled?: boolean;
}

export default function MagneticButton({
  children,
  className,
  variant = "primary",
  onClick,
  type = "button",
  disabled = false,
}: MagneticButtonProps) {
  const ref = useRef<HTMLButtonElement>(null);
  const [position, setPosition] = useState({ x: 0, y: 0 });

  const handleMouseMove = (e: React.MouseEvent<HTMLButtonElement>) => {
    if (!ref.current) return;
    const { clientX, clientY } = e;
    const { left, top, width, height } = ref.current.getBoundingClientRect();
    const centerX = left + width / 2;
    const centerY = top + height / 2;
    const distanceX = (clientX - centerX) * 0.25;
    const distanceY = (clientY - centerY) * 0.25;
    setPosition({ x: distanceX, y: distanceY });
  };

  const handleMouseLeave = () => {
    setPosition({ x: 0, y: 0 });
  };

  const variantStyles = {
    primary:
      "bg-gradient-to-r from-brandy to-brandy-hover text-ivory border border-flamingo/30 hover:border-flamingo shadow-tactile hover:shadow-tactile-hover hover:from-brandy-hover hover:to-brandy-light",
    secondary:
      "bg-espresso-surface text-custard border border-espresso-border hover:border-flamingo/50 hover:bg-espresso-elevated",
    outline:
      "bg-transparent text-ivory border border-espresso-border hover:border-flamingo hover:text-flamingo",
    ghost:
      "bg-transparent text-secondary-text hover:text-ivory hover:bg-espresso/50",
  };

  return (
    <motion.button
      ref={ref}
      type={type}
      disabled={disabled}
      onMouseMove={handleMouseMove}
      onMouseLeave={handleMouseLeave}
      animate={{ x: position.x, y: position.y }}
      transition={{ type: "spring", stiffness: 200, damping: 18, mass: 0.2 }}
      whileTap={{ scale: 0.97 }}
      onClick={onClick}
      className={cn(
        "relative inline-flex items-center justify-center font-medium transition-colors rounded-xl px-5 py-2.5 text-sm active:scale-95 disabled:opacity-50 disabled:pointer-events-none",
        variantStyles[variant],
        className
      )}
    >
      {children}
    </motion.button>
  );
}
