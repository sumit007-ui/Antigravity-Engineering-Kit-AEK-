"use client";

import { useEffect, useRef, useState } from "react";
import { useReducedMotion } from "motion/react";

interface Bot {
  x: number;
  y: number;
  vx: number;
  vy: number;
  angle: number;
  size: number;
  walkCycle: number;
  speed: number;
  type: "walker" | "drone";
  color: string;
  glow: string;
  idleTimer: number;
  targetX: number;
  targetY: number;
}

interface Particle {
  x: number;
  y: number;
  vx: number;
  vy: number;
  life: number;
  maxLife: number;
  color: string;
  size: number;
}

export default function AutonomousAgentSwarm() {
  const canvasRef = useRef<HTMLCanvasElement | null>(null);
  const containerRef = useRef<HTMLDivElement | null>(null);
  const shouldReduceMotion = useReducedMotion();
  const mouseRef = useRef<{ x: number | null; y: number | null; active: boolean }>({
    x: null,
    y: null,
    active: false,
  });

  useEffect(() => {
    const canvas = canvasRef.current;
    const container = containerRef.current;
    if (!canvas || !container) return;

    let ctx: CanvasRenderingContext2D | null = null;
    try {
      ctx = canvas.getContext("2d");
    } catch {
      return;
    }
    if (!ctx) return;

    let animId: number;
    let width = 0;
    let height = 0;

    const resize = () => {
      const rect = container.getBoundingClientRect();
      const dpr = Math.min(window.devicePixelRatio || 1, 2);
      width = rect.width;
      height = rect.height;
      canvas.width = width * dpr;
      canvas.height = height * dpr;
      canvas.style.width = `${width}px`;
      canvas.style.height = `${height}px`;
      ctx.scale(dpr, dpr);
    };

    resize();
    window.addEventListener("resize", resize);

    // Spawn 14 autonomous intelligent agent micro-bots
    const botColors = [
      { color: "#FFD36B", glow: "rgba(255, 211, 107, 0.85)" },
      { color: "#F4661B", glow: "rgba(244, 102, 27, 0.85)" },
      { color: "#FF8A3D", glow: "rgba(255, 138, 61, 0.85)" },
      { color: "#4285F4", glow: "rgba(66, 133, 244, 0.85)" },
    ];

    const bots: Bot[] = Array.from({ length: 14 }).map((_, i) => {
      const palette = botColors[i % botColors.length];
      return {
        x: Math.random() * (width || 800),
        y: Math.random() * (height || 500),
        vx: (Math.random() - 0.5) * 0.8,
        vy: (Math.random() - 0.5) * 0.8,
        angle: Math.random() * Math.PI * 2,
        size: 10 + Math.random() * 4,
        walkCycle: Math.random() * 100,
        speed: 1.2 + Math.random() * 0.9,
        type: i % 3 === 0 ? "drone" : "walker",
        color: palette.color,
        glow: palette.glow,
        idleTimer: Math.random() * 120,
        targetX: Math.random() * (width || 800),
        targetY: Math.random() * (height || 500),
      };
    });

    const particles: Particle[] = [];

    // Track mouse interaction relative to section container
    const handleMouseMove = (e: MouseEvent) => {
      const rect = container.getBoundingClientRect();
      mouseRef.current.x = e.clientX - rect.left;
      mouseRef.current.y = e.clientY - rect.top;
      mouseRef.current.active = true;
    };

    const handleMouseEnter = (e: MouseEvent) => {
      const rect = container.getBoundingClientRect();
      mouseRef.current.x = e.clientX - rect.left;
      mouseRef.current.y = e.clientY - rect.top;
      mouseRef.current.active = true;
    };

    const handleMouseLeave = () => {
      mouseRef.current.active = false;
      mouseRef.current.x = null;
      mouseRef.current.y = null;
    };

    const handleTouchMove = (e: TouchEvent) => {
      if (e.touches.length > 0) {
        const rect = container.getBoundingClientRect();
        mouseRef.current.x = e.touches[0].clientX - rect.left;
        mouseRef.current.y = e.touches[0].clientY - rect.top;
        mouseRef.current.active = true;
      }
    };

    container.addEventListener("mousemove", handleMouseMove);
    container.addEventListener("mouseenter", handleMouseEnter);
    container.addEventListener("mouseleave", handleMouseLeave);
    container.addEventListener("touchmove", handleTouchMove, { passive: true });
    container.addEventListener("touchend", handleMouseLeave);

    let reticleAngle = 0;

    // Simulation render loop
    const render = () => {
      ctx.clearRect(0, 0, width, height);

      const mouse = mouseRef.current;
      reticleAngle += 0.02;

      // Draw interactive tactical cursor reticle if active
      if (mouse.active && mouse.x !== null && mouse.y !== null) {
        ctx.save();
        ctx.translate(mouse.x, mouse.y);

        // Ambient targeting ring
        ctx.strokeStyle = "rgba(255, 211, 107, 0.35)";
        ctx.lineWidth = 1;
        ctx.setLineDash([4, 4]);
        ctx.beginPath();
        ctx.arc(0, 0, 26, 0, Math.PI * 2);
        ctx.stroke();

        // Rotating outer radar ticks
        ctx.save();
        ctx.rotate(reticleAngle);
        ctx.strokeStyle = "rgba(244, 102, 27, 0.6)";
        ctx.setLineDash([]);
        ctx.lineWidth = 1.5;
        for (let a = 0; a < 4; a++) {
          ctx.beginPath();
          ctx.arc(0, 0, 32, a * Math.PI * 0.5 - 0.2, a * Math.PI * 0.5 + 0.2);
          ctx.stroke();
        }
        ctx.restore();

        // Center beacon pin
        ctx.fillStyle = "#FFD36B";
        ctx.beginPath();
        ctx.arc(0, 0, 2.5, 0, Math.PI * 2);
        ctx.fill();

        ctx.restore();
      }

      // Draw faint tactical mesh lines between nearby bots
      for (let i = 0; i < bots.length; i++) {
        for (let j = i + 1; j < bots.length; j++) {
          const dx = bots[i].x - bots[j].x;
          const dy = bots[i].y - bots[j].y;
          const dist = Math.sqrt(dx * dx + dy * dy);
          if (dist < 110) {
            const alpha = (1 - dist / 110) * 0.22;
            ctx.strokeStyle = `rgba(255, 211, 107, ${alpha})`;
            ctx.lineWidth = 0.8;
            ctx.setLineDash([2, 4]);
            ctx.beginPath();
            ctx.moveTo(bots[i].x, bots[i].y);
            ctx.lineTo(bots[j].x, bots[j].y);
            ctx.stroke();
          }
        }
      }

      // Update and render each robot
      bots.forEach((bot) => {
        let destX = bot.targetX;
        let destY = bot.targetY;

        // If cursor is active in section, bots gravitate towards cursor!
        if (mouse.active && mouse.x !== null && mouse.y !== null) {
          const mdx = mouse.x - bot.x;
          const mdy = mouse.y - bot.y;
          const mDist = Math.sqrt(mdx * mdx + mdy * mdy);

          // Orbit around cursor at personal radius
          const orbitRadius = 45 + (bot.size * 2);
          if (mDist > orbitRadius) {
            destX = mouse.x;
            destY = mouse.y;
          } else {
            // Orbit inquisitively around cursor
            const angleFromMouse = Math.atan2(bot.y - mouse.y, bot.x - mouse.x) + 0.04;
            destX = mouse.x + Math.cos(angleFromMouse) * orbitRadius;
            destY = mouse.y + Math.sin(angleFromMouse) * orbitRadius;

            // Occasional spark particles when close
            if (Math.random() < 0.08) {
              particles.push({
                x: bot.x,
                y: bot.y,
                vx: (Math.random() - 0.5) * 1.5,
                vy: (Math.random() - 0.5) * 1.5,
                life: 1,
                maxLife: 20 + Math.random() * 15,
                color: bot.color,
                size: 1.5,
              });
            }
          }

          // Draw scanning laser beam from closest bots
          if (mDist < 180 && Math.random() < 0.2) {
            ctx.strokeStyle = bot.glow;
            ctx.lineWidth = 0.75;
            ctx.setLineDash([3, 5]);
            ctx.beginPath();
            ctx.moveTo(bot.x, bot.y);
            ctx.lineTo(mouse.x, mouse.y);
            ctx.stroke();
          }
        } else {
          // Autonomous roaming patrols
          bot.idleTimer--;
          if (bot.idleTimer <= 0) {
            bot.targetX = 40 + Math.random() * (width - 80);
            bot.targetY = 40 + Math.random() * (height - 80);
            bot.idleTimer = 80 + Math.random() * 160;
          }
        }

        // Steer towards target destination
        const dx = destX - bot.x;
        const dy = destY - bot.y;
        const dist = Math.sqrt(dx * dx + dy * dy);

        if (!shouldReduceMotion && dist > 4) {
          const targetAngle = Math.atan2(dy, dx);
          // Smooth angular interpolation
          let angleDiff = targetAngle - bot.angle;
          while (angleDiff < -Math.PI) angleDiff += Math.PI * 2;
          while (angleDiff > Math.PI) angleDiff -= Math.PI * 2;
          bot.angle += angleDiff * 0.12;

          const currentSpeed = mouse.active ? bot.speed * 1.4 : bot.speed * 0.7;
          bot.vx = Math.cos(bot.angle) * currentSpeed;
          bot.vy = Math.sin(bot.angle) * currentSpeed;

          bot.x += bot.vx;
          bot.y += bot.vy;
          bot.walkCycle += currentSpeed * 0.35;
        }

        // Keep inside bounds
        bot.x = Math.max(15, Math.min(width - 15, bot.x));
        bot.y = Math.max(15, Math.min(height - 15, bot.y));

        // RENDER CYBER ROBOT
        ctx.save();
        ctx.translate(bot.x, bot.y);
        ctx.rotate(bot.angle);

        ctx.setLineDash([]);

        if (bot.type === "walker") {
          // 4 Animated Jointed Cyber Legs
          ctx.strokeStyle = "rgba(255, 211, 107, 0.7)";
          ctx.lineWidth = 1.2;
          ctx.lineCap = "round";

          const legSwing = Math.sin(bot.walkCycle) * 4;

          // Front-Left leg
          ctx.beginPath();
          ctx.moveTo(-2, -bot.size * 0.4);
          ctx.lineTo(-bot.size * 0.6, -bot.size * 0.8 + legSwing);
          ctx.lineTo(-bot.size * 0.9, -bot.size * 0.6 + legSwing);
          ctx.stroke();

          // Front-Right leg
          ctx.beginPath();
          ctx.moveTo(-2, bot.size * 0.4);
          ctx.lineTo(-bot.size * 0.6, bot.size * 0.8 - legSwing);
          ctx.lineTo(-bot.size * 0.9, bot.size * 0.6 - legSwing);
          ctx.stroke();

          // Rear-Left leg
          ctx.beginPath();
          ctx.moveTo(3, -bot.size * 0.4);
          ctx.lineTo(bot.size * 0.6, -bot.size * 0.8 - legSwing);
          ctx.lineTo(bot.size * 0.9, -bot.size * 0.6 - legSwing);
          ctx.stroke();

          // Rear-Right leg
          ctx.beginPath();
          ctx.moveTo(3, bot.size * 0.4);
          ctx.lineTo(bot.size * 0.6, bot.size * 0.8 + legSwing);
          ctx.lineTo(bot.size * 0.9, bot.size * 0.6 + legSwing);
          ctx.stroke();

          // Hexagonal Cyber Carapace / Body
          ctx.fillStyle = "#120D24";
          ctx.strokeStyle = bot.color;
          ctx.lineWidth = 1.5;
          ctx.beginPath();
          ctx.moveTo(bot.size * 0.7, 0);
          ctx.lineTo(bot.size * 0.25, -bot.size * 0.55);
          ctx.lineTo(-bot.size * 0.55, -bot.size * 0.45);
          ctx.lineTo(-bot.size * 0.7, 0);
          ctx.lineTo(-bot.size * 0.55, bot.size * 0.45);
          ctx.lineTo(bot.size * 0.25, bot.size * 0.55);
          ctx.closePath();
          ctx.fill();
          ctx.stroke();

          // Glowing Optical Sensor Eye
          ctx.fillStyle = bot.color;
          ctx.shadowColor = bot.glow;
          ctx.shadowBlur = 8;
          ctx.beginPath();
          ctx.arc(bot.size * 0.35, 0, 2.2, 0, Math.PI * 2);
          ctx.fill();
          ctx.shadowBlur = 0;
        } else {
          // Drone Type: Levitating Triangle Core with Flickering Thruster Jets
          ctx.fillStyle = "#0A0815";
          ctx.strokeStyle = bot.color;
          ctx.lineWidth = 1.5;

          ctx.beginPath();
          ctx.moveTo(bot.size * 0.8, 0);
          ctx.lineTo(-bot.size * 0.6, -bot.size * 0.6);
          ctx.lineTo(-bot.size * 0.3, 0);
          ctx.lineTo(-bot.size * 0.6, bot.size * 0.6);
          ctx.closePath();
          ctx.fill();
          ctx.stroke();

          // Dual Ion Thrusters on rear
          const flameSize = 3 + Math.random() * 4;
          ctx.fillStyle = "#FF8A3D";
          ctx.beginPath();
          ctx.moveTo(-bot.size * 0.6, -bot.size * 0.3);
          ctx.lineTo(-bot.size * 0.6 - flameSize, -bot.size * 0.3);
          ctx.stroke();

          ctx.beginPath();
          ctx.moveTo(-bot.size * 0.6, bot.size * 0.3);
          ctx.lineTo(-bot.size * 0.6 - flameSize, bot.size * 0.3);
          ctx.stroke();

          // Drone Optical Radar Eye
          ctx.fillStyle = "#FFFFFF";
          ctx.shadowColor = bot.glow;
          ctx.shadowBlur = 10;
          ctx.beginPath();
          ctx.arc(bot.size * 0.2, 0, 2.5, 0, Math.PI * 2);
          ctx.fill();
          ctx.shadowBlur = 0;
        }

        ctx.restore();
      });

      // Render micro spark particles
      for (let p = particles.length - 1; p >= 0; p--) {
        const pt = particles[p];
        pt.x += pt.vx;
        pt.y += pt.vy;
        pt.life++;

        const alpha = 1 - pt.life / pt.maxLife;
        if (alpha <= 0) {
          particles.splice(p, 1);
          continue;
        }

        ctx.fillStyle = pt.color;
        ctx.globalAlpha = alpha;
        ctx.beginPath();
        ctx.arc(pt.x, pt.y, pt.size, 0, Math.PI * 2);
        ctx.fill();
        ctx.globalAlpha = 1;
      }

      animId = requestAnimationFrame(render);
    };

    animId = requestAnimationFrame(render);

    return () => {
      cancelAnimationFrame(animId);
      window.removeEventListener("resize", resize);
      container.removeEventListener("mousemove", handleMouseMove);
      container.removeEventListener("mouseenter", handleMouseEnter);
      container.removeEventListener("mouseleave", handleMouseLeave);
      container.removeEventListener("touchmove", handleTouchMove);
      container.removeEventListener("touchend", handleMouseLeave);
    };
  }, [shouldReduceMotion]);

  return (
    <div
      ref={containerRef}
      className="absolute inset-0 pointer-events-auto overflow-hidden z-0"
      aria-hidden="true"
    >
      <canvas
        ref={canvasRef}
        className="w-full h-full block"
      />
    </div>
  );
}
