"use client";

import React from "react";
import { ArrowUpRight } from "lucide-react";

export type ButtonVariant = "primary" | "secondary" | "gold" | "terminal";

interface TactileButtonProps extends React.ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: ButtonVariant;
  external?: boolean;
  icon?: React.ReactNode;
  children: React.ReactNode;
}

export default function TactileButton({
  variant = "primary",
  external = false,
  icon,
  children,
  className = "",
  ...props
}: TactileButtonProps) {
  const baseStyles =
    "inline-flex items-center justify-center gap-2 px-5 py-2.5 text-xs font-mono font-bold uppercase tracking-wider rounded border-2 min-h-[44px] cursor-pointer select-none transition-all duration-80 tactile-press focus-visible:outline-2 focus-visible:outline-offset-2";

  let variantStyles = "";
  switch (variant) {
    case "primary":
      variantStyles =
        "bg-accent-primary text-black border-black shadow-hard-md hover:brightness-110 active:shadow-none focus-visible:outline-accent-primary";
      break;
    case "gold":
      variantStyles =
        "bg-accent-gold text-black border-black shadow-hard-md hover:brightness-110 active:shadow-none focus-visible:outline-accent-gold";
      break;
    case "secondary":
      variantStyles =
        "bg-surface-card text-fg border-border hover:border-accent-primary shadow-hard-md active:shadow-none focus-visible:outline-accent-primary";
      break;
    case "terminal":
      variantStyles =
        "bg-black text-accent-secondary border-border hover:border-accent-secondary shadow-hard-sm active:shadow-none font-mono text-[11px] focus-visible:outline-accent-secondary";
      break;
  }

  return (
    <button className={`${baseStyles} ${variantStyles} ${className}`} {...props}>
      {icon && <span className="shrink-0">{icon}</span>}
      <span>{children}</span>
      {external && <ArrowUpRight className="w-3.5 h-3.5 shrink-0" />}
    </button>
  );
}
