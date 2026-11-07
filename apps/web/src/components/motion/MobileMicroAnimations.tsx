"use client";

import { useEffect } from "react";
import gsap from "gsap";
import { useReducedMotion } from "motion/react";

export default function MobileMicroAnimations() {
  const shouldReduceMotion = useReducedMotion();

  useEffect(() => {
    // Dynamic Viewport Height fix for iOS Safari
    const setDynamicViewport = () => {
      const vh = window.innerHeight * 0.01;
      document.documentElement.style.setProperty("--vh", `${vh}px`);
    };

    setDynamicViewport();
    window.addEventListener("resize", setDynamicViewport, { passive: true });
    window.addEventListener("orientationchange", setDynamicViewport, { passive: true });

    if (shouldReduceMotion) return;

    // Apply subtle GSAP micro-interaction bounce to mobile tactical action buttons
    const isTouch = "ontouchstart" in window || navigator.maxTouchPoints > 0;
    if (!isTouch) return;

    const interactiveElements = document.querySelectorAll(
      "button, [role='button'], a.tactile-button, .tactile-press"
    );

    const handleTouchStart = (e: Event) => {
      const el = e.currentTarget as HTMLElement;
      if (!el) return;
      gsap.to(el, {
        scale: 0.96,
        duration: 0.12,
        ease: "power1.out",
        overwrite: "auto",
      });
    };

    const handleTouchEnd = (e: Event) => {
      const el = e.currentTarget as HTMLElement;
      if (!el) return;
      gsap.to(el, {
        scale: 1,
        duration: 0.22,
        ease: "back.out(2)",
        overwrite: "auto",
      });
    };

    interactiveElements.forEach((el) => {
      el.addEventListener("touchstart", handleTouchStart, { passive: true });
      el.addEventListener("touchend", handleTouchEnd, { passive: true });
      el.addEventListener("touchcancel", handleTouchEnd, { passive: true });
    });

    return () => {
      window.removeEventListener("resize", setDynamicViewport);
      window.removeEventListener("orientationchange", setDynamicViewport);
      interactiveElements.forEach((el) => {
        el.removeEventListener("touchstart", handleTouchStart);
        el.removeEventListener("touchend", handleTouchEnd);
        el.removeEventListener("touchcancel", handleTouchEnd);
      });
    };
  }, [shouldReduceMotion]);

  return null;
}

