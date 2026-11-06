"use client";

import { useEffect, useRef, useState } from "react";
import * as THREE from "three";

export default function HeroArtifact() {
  const containerRef = useRef<HTMLDivElement>(null);
  const [webGlSupported, setWebGlSupported] = useState(true);

  useEffect(() => {
    const container = containerRef.current;
    if (!container) return;

    // Check for prefers-reduced-motion
    const prefersReducedMotion = window.matchMedia("(prefers-reduced-motion: reduce)").matches;

    // Check WebGL support
    try {
      const canvas = document.createElement("canvas");
      const gl = canvas.getContext("webgl") || canvas.getContext("experimental-webgl");
      if (!gl) {
        setWebGlSupported(false);
        return;
      }
    } catch {
      setWebGlSupported(false);
      return;
    }

    const width = container.clientWidth || 460;
    const height = container.clientHeight || 460;

    const scene = new THREE.Scene();
    const camera = new THREE.PerspectiveCamera(40, width / height, 0.1, 100);
    camera.position.set(0, 0, 7.5);

    const renderer = new THREE.WebGLRenderer({
      alpha: true,
      antialias: true,
      powerPreference: "high-performance",
    });
    renderer.setSize(width, height);
    renderer.setPixelRatio(Math.min(window.devicePixelRatio, 2));
    renderer.toneMapping = THREE.ACESFilmicToneMapping;
    renderer.toneMappingExposure = 1.1;

    container.appendChild(renderer.domElement);

    // Group for the entire AEK sculptural artifact
    const artifactGroup = new THREE.Group();
    scene.add(artifactGroup);

    // Materials: Pantone Chocolate Plum (#382C2D), Popcorn Yellow (#F8D878), Cerulean Blue (#98B4D4)
    const plumMaterial = new THREE.MeshPhysicalMaterial({
      color: 0x382c2d,
      roughness: 0.35,
      metalness: 0.65,
      clearcoat: 0.4,
      clearcoatRoughness: 0.2,
    });

    const popcornMaterial = new THREE.MeshStandardMaterial({
      color: 0xf8d878,
      roughness: 0.28,
      metalness: 0.15,
    });

    const ceruleanAccentMaterial = new THREE.MeshStandardMaterial({
      color: 0x98b4d4,
      roughness: 0.25,
      metalness: 0.45,
      emissive: 0x1b2838,
    });

    // 1. Center Precision Outer Ring (Machined Chocolate Plum)
    const ringGeo = new THREE.TorusGeometry(2.0, 0.12, 32, 100);
    const ringMesh = new THREE.Mesh(ringGeo, plumMaterial);
    artifactGroup.add(ringMesh);

    // 2. Inner Interlocking Popcorn Octahedron
    const coreGeo = new THREE.OctahedronGeometry(1.25, 0);
    const coreMesh = new THREE.Mesh(coreGeo, popcornMaterial);
    artifactGroup.add(coreMesh);

    // 3. Four Precision Coordinate Nodes (Cerulean Blue Accents)
    const nodeGeo = new THREE.SphereGeometry(0.18, 24, 24);
    const nodePositions = [
      [2.0, 0, 0],
      [-2.0, 0, 0],
      [0, 2.0, 0],
      [0, -2.0, 0],
      [0, 0, 1.4],
      [0, 0, -1.4],
    ];

    nodePositions.forEach(([x, y, z]) => {
      const nodeMesh = new THREE.Mesh(nodeGeo, ceruleanAccentMaterial);
      nodeMesh.position.set(x, y, z);
      artifactGroup.add(nodeMesh);
    });

    // 4. Floating Precision Orbital Gyro Ring (Cerulean)
    const gyroGeo = new THREE.TorusGeometry(2.4, 0.04, 16, 80);
    const gyroMesh = new THREE.Mesh(gyroGeo, ceruleanAccentMaterial);
    gyroMesh.rotation.x = Math.PI / 3;
    gyroMesh.rotation.y = Math.PI / 6;
    artifactGroup.add(gyroMesh);

    // Lighting
    const ambientLight = new THREE.AmbientLight(0xfff9f0, 1.3);
    scene.add(ambientLight);

    const dirLight1 = new THREE.DirectionalLight(0xf8d878, 2.4);
    dirLight1.position.set(4, 5, 6);
    scene.add(dirLight1);

    const dirLight2 = new THREE.DirectionalLight(0x98b4d4, 1.8);
    dirLight2.position.set(-5, -4, 3);
    scene.add(dirLight2);

    const rimLight = new THREE.PointLight(0xffffff, 2.0, 15);
    rimLight.position.set(0, 0, -4);
    scene.add(rimLight);

    // Pointer parallax tracking
    let targetX = 0;
    let targetY = 0;

    const onPointerMove = (e: MouseEvent) => {
      const rect = container.getBoundingClientRect();
      const x = ((e.clientX - rect.left) / rect.width) * 2 - 1;
      const y = -(((e.clientY - rect.top) / rect.height) * 2 - 1);
      targetX = x * 0.6;
      targetY = y * 0.6;
    };

    window.addEventListener("mousemove", onPointerMove, { passive: true });

    // Render loop
    let animationFrameId: number;
    let clock = new THREE.Clock();

    const animate = () => {
      animationFrameId = requestAnimationFrame(animate);

      const delta = clock.getDelta();

      if (!prefersReducedMotion) {
        // Slow autonomous rotation
        artifactGroup.rotation.y += delta * 0.45;
        artifactGroup.rotation.x += delta * 0.22;
        gyroMesh.rotation.z += delta * 0.6;
      }

      // Smooth pointer damping
      artifactGroup.rotation.y += (targetX - artifactGroup.rotation.y * 0.1) * 0.05;
      artifactGroup.rotation.x += (targetY - artifactGroup.rotation.x * 0.1) * 0.05;

      renderer.render(scene, camera);
    };

    animate();

    const handleResize = () => {
      if (!container) return;
      const w = container.clientWidth;
      const h = container.clientHeight;
      camera.aspect = w / h;
      camera.updateProjectionMatrix();
      renderer.setSize(w, h);
    };

    window.addEventListener("resize", handleResize);

    return () => {
      cancelAnimationFrame(animationFrameId);
      window.removeEventListener("mousemove", onPointerMove);
      window.removeEventListener("resize", handleResize);
      if (container && renderer.domElement) {
        container.removeChild(renderer.domElement);
      }
      renderer.dispose();
      ringGeo.dispose();
      coreGeo.dispose();
      nodeGeo.dispose();
      gyroGeo.dispose();
      plumMaterial.dispose();
      popcornMaterial.dispose();
      ceruleanAccentMaterial.dispose();
    };
  }, []);

  if (!webGlSupported) {
    // Elegant fallback SVG centerpiece
    return (
      <div className="w-full h-full flex items-center justify-center p-8">
        <div className="relative w-64 h-64 rounded-full border-2 border-brandy/60 flex items-center justify-center glow-flamingo bg-espresso/50">
          <div className="w-44 h-44 rounded-full border border-dashed border-custard/40 flex items-center justify-center animate-spin-slow">
            <div className="w-24 h-24 rounded-2xl bg-gradient-to-tr from-brandy to-flamingo transform rotate-45 flex items-center justify-center shadow-tactile">
              <span className="font-editorial text-2xl font-bold text-ivory">AEK</span>
            </div>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div
      ref={containerRef}
      className="relative w-full h-[400px] md:h-[480px] lg:h-[540px] flex items-center justify-center pointer-events-auto cursor-grab active:cursor-grabbing"
      aria-label="Interactive 3D AEK Sculptural Artifact"
    />
  );
}

