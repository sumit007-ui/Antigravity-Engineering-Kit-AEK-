"use client";

import React from "react";
import { ArrowUpRight } from "lucide-react";

export type ButtonVariant = "primary" | "secondary" | "gold" | "terminal";
export type ButtonSize = "sm" | "md" | "lg";

interface TactileButtonProps extends React.ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: ButtonVariant;
  size?: ButtonSize;
  hotkey?: string;
  external?: boolean;
  icon?: React.ReactNode;
  children: React.ReactNode;
}

export default function TactileButton({
  variant = "primary",
  size = "md",
  hotkey,
  external = false,
  icon,
  children,
  className = "",
  ...props
}: TactileButtonProps) {
  const sizeStyles =
    size === "sm"
      ? "px-3.5 py-2 text-[11px] min-h-[44px]"
      : size === "lg"
      ? "px-6 py-3.5 text-sm min-h-[48px]"
      : "px-5 py-2.5 text-xs min-h-[44px]";

  const baseStyles =
    "inline-flex items-center justify-center gap-2 font-mono font-bold uppercase tracking-wider rounded border-2 cursor-pointer select-none transition-all duration-80 tactile-press focus-visible:outline-2 focus-visible:outline-offset-2";

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
        "bg-black text-accent-gold border-border hover:border-accent-gold font-mono shadow-hard-sm active:shadow-none focus-visible:outline-accent-gold";
      break;
  }

  return (
    <button className={`${baseStyles} ${sizeStyles} ${variantStyles} ${className}`} {...props}>
      {icon && <span className="shrink-0">{icon}</span>}
      {hotkey && (
        <span className="px-1.5 py-0.5 rounded bg-black/40 text-[10px] font-mono border border-current opacity-80 mr-0.5 select-none">
          [{hotkey}]
        </span>
      )}
      <span>{children}</span>
      {external && <ArrowUpRight className="w-3.5 h-3.5 shrink-0" />}
    </button>
  );
}
