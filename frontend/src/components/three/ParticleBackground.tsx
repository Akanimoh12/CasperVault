/// <reference types="@react-three/fiber" />
import { useEffect, useRef, useMemo } from 'react';
import { Canvas, useFrame } from '@react-three/fiber';
import { Points, PointMaterial } from '@react-three/drei';
import * as THREE from 'three';

interface ParticlesProps {
  count?: number;
}

const Particles = ({ count = 1000 }: ParticlesProps) => {
  const ref = useRef<THREE.Points>(null);
  const mousePos = useRef({ x: 0, y: 0 });
  
  // Generate random particle positions (memoized for performance)
  const positions = useMemo(() => {
    const positions = new Float32Array(count * 3);
    for (let i = 0; i < count; i++) {
      positions[i * 3] = (Math.random() - 0.5) * 50; // x
      positions[i * 3 + 1] = (Math.random() - 0.5) * 50; // y
      positions[i * 3 + 2] = (Math.random() - 0.5) * 50; // z
    }
    return positions;
  }, [count]);
  
  // Mouse move handler
  useEffect(() => {
    const handleMouseMove = (event: MouseEvent) => {
      mousePos.current = {
        x: (event.clientX / window.innerWidth) * 2 - 1,
        y: -(event.clientY / window.innerHeight) * 2 + 1,
      };
    };
    
    window.addEventListener('mousemove', handleMouseMove);
    return () => window.removeEventListener('mousemove', handleMouseMove);
  }, []);
  
  // Animate particles
  useFrame((state) => {
    if (!ref.current) return;
    
    const time = state.clock.getElapsedTime();
    
    // Gentle rotation
    ref.current.rotation.x = time * 0.05;
    ref.current.rotation.y = time * 0.075;
    
    // Mouse interaction (subtle)
    ref.current.rotation.x += mousePos.current.y * 0.05;
    ref.current.rotation.y += mousePos.current.x * 0.05;
    
    // Wave motion
    const positions = ref.current.geometry.attributes.position.array as Float32Array;
    for (let i = 0; i < count; i++) {
      const i3 = i * 3;
      positions[i3 + 1] += Math.sin(time + positions[i3]) * 0.001;
    }
    ref.current.geometry.attributes.position.needsUpdate = true;
  });
  
  return (
    <Points ref={ref} positions={positions} stride={3} frustumCulled={false}>
      <PointMaterial
        transparent
        color="#0ea5e9" // Primary blue
        size={0.05}
        sizeAttenuation
        depthWrite={false}
        opacity={0.3}
      />
    </Points>
  );
};

export const ParticleBackground = () => {
  return (
    <div className="fixed inset-0 -z-10 bg-gradient-to-br from-white via-blue-50/30 to-purple-50/30">
      <Canvas
        camera={{ position: [0, 0, 10], fov: 75 }}
        style={{ background: 'transparent' }}
        dpr={[1, 2]} // Limit pixel ratio for performance
      >
        {/* @ts-expect-error - Three.js types not properly recognized by TypeScript */}
        <ambientLight intensity={0.5} />
        <Particles count={1000} />
      </Canvas>
    </div>
  );
};
