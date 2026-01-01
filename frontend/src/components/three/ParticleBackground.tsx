// @ts-nocheck
/// <reference types="@react-three/fiber" />
import { useEffect, useRef, useMemo } from 'react';
import { Canvas, useFrame } from '@react-three/fiber';
import { Points, PointMaterial } from '@react-three/drei';
import * as THREE from 'three';

interface ParticlesProps {
  count?: number;
}

const Particles = ({ count = 3000 }: ParticlesProps) => {
  const ref = useRef<THREE.Points>(null);
  const mousePos = useRef({ x: 0, y: 0, prevX: 0, prevY: 0 });
  const mouseVelocity = useRef({ x: 0, y: 0 });
  const originalPositions = useRef<Float32Array | null>(null);
  
  // Generate random particle positions
  const positions = useMemo(() => {
    const positions = new Float32Array(count * 3);
    
    for (let i = 0; i < count; i++) {
      // Spread particles in a larger space
      positions[i * 3] = (Math.random() - 0.5) * 80;
      positions[i * 3 + 1] = (Math.random() - 0.5) * 80;
      positions[i * 3 + 2] = (Math.random() - 0.5) * 80;
    }
    
    originalPositions.current = new Float32Array(positions);
    return positions;
  }, [count]);
  
  // Mouse move handler with velocity tracking
  useEffect(() => {
    const handleMouseMove = (event: MouseEvent) => {
      const newX = (event.clientX / window.innerWidth) * 2 - 1;
      const newY = -(event.clientY / window.innerHeight) * 2 + 1;
      
      mouseVelocity.current = {
        x: newX - mousePos.current.x,
        y: newY - mousePos.current.y
      };
      
      mousePos.current = {
        x: newX,
        y: newY,
        prevX: mousePos.current.x,
        prevY: mousePos.current.y
      };
    };
    
    window.addEventListener('mousemove', handleMouseMove);
    return () => window.removeEventListener('mousemove', handleMouseMove);
  }, []);
  
  // Animate particles with mouse interaction
  useFrame((state) => {
    if (!ref.current || !originalPositions.current) return;
    
    const time = state.clock.getElapsedTime();
    
    // Very gentle automatic rotation
    ref.current.rotation.x = Math.sin(time * 0.05) * 0.1;
    ref.current.rotation.y = time * 0.02;
    
    // Subtle global mouse interaction
    ref.current.rotation.x += mousePos.current.y * 0.02;
    ref.current.rotation.y += mousePos.current.x * 0.02;
    
    // Individual particle reactions to mouse
    const positions = ref.current.geometry.attributes.position.array as Float32Array;
    const mouse3D = new THREE.Vector3(
      mousePos.current.x * 40,
      mousePos.current.y * 40,
      20
    );
    
    for (let i = 0; i < count; i++) {
      const i3 = i * 3;
      const originalX = originalPositions.current[i3];
      const originalY = originalPositions.current[i3 + 1];
      const originalZ = originalPositions.current[i3 + 2];
      
      // Each particle has its own phase for organic motion
      const phase = originalX * 0.1 + originalZ * 0.1;
      const floatX = Math.cos(time * 0.2 + phase) * 0.5;
      const floatY = Math.sin(time * 0.3 + phase) * 0.5;
      
      // Calculate distance from mouse (in 2D screen space)
      const particlePos = new THREE.Vector3(positions[i3], positions[i3 + 1], positions[i3 + 2]);
      const distance = particlePos.distanceTo(mouse3D);
      
      // Push particles away from cursor
      if (distance < 15) {
        const force = (15 - distance) / 15;
        const direction = particlePos.clone().sub(mouse3D).normalize();
        
        positions[i3] += direction.x * force * 0.5;
        positions[i3 + 1] += direction.y * force * 0.5;
        positions[i3 + 2] += direction.z * force * 0.3;
      } else {
        // Gradually return to original position
        positions[i3] += (originalX + floatX - positions[i3]) * 0.02;
        positions[i3 + 1] += (originalY + floatY - positions[i3 + 1]) * 0.02;
        positions[i3 + 2] += (originalZ - positions[i3 + 2]) * 0.02;
      }
    }
    
    ref.current.geometry.attributes.position.needsUpdate = true;
  });
  
  return (
    <Points ref={ref} positions={positions} stride={3} frustumCulled={false}>
      <PointMaterial
        transparent
        color="#0ea5e9"
        size={0.5}
        sizeAttenuation
        depthWrite={false}
        opacity={0.4}
        blending={THREE.AdditiveBlending}
      />
    </Points>
  );
};

// Connection lines between particles
const ParticleConnections = () => {
  const lineSegmentsRef = useRef<THREE.LineSegments | null>(null);
  
  useFrame((state) => {
    if (!lineSegmentsRef.current) return;
    
    const time = state.clock.getElapsedTime();
    lineSegmentsRef.current.rotation.y = time * 0.015;
  });

  const lineSegments = useMemo(() => {
    const geometry = new THREE.BufferGeometry();
    const lineCount = 200;
    const positions = new Float32Array(lineCount * 6);
    
    for (let i = 0; i < lineCount; i++) {
      const i6 = i * 6;
      // Random line endpoints
      positions[i6] = (Math.random() - 0.5) * 60;
      positions[i6 + 1] = (Math.random() - 0.5) * 60;
      positions[i6 + 2] = (Math.random() - 0.5) * 60;
      positions[i6 + 3] = (Math.random() - 0.5) * 60;
      positions[i6 + 4] = (Math.random() - 0.5) * 60;
      positions[i6 + 5] = (Math.random() - 0.5) * 60;
    }
    
    geometry.setAttribute('position', new THREE.BufferAttribute(positions, 3));
    
    const material = new THREE.LineBasicMaterial({
      color: '#38bdf8',
      transparent: true,
      opacity: 0.08,
      blending: THREE.AdditiveBlending
    });
    
    const mesh = new THREE.LineSegments(geometry, material);
    return mesh;
  }, []);

  useEffect(() => {
    if (lineSegmentsRef.current) {
      lineSegmentsRef.current = lineSegments;
    }
  }, [lineSegments]);

  return <primitive ref={lineSegmentsRef} object={lineSegments} />;
};

export const ParticleBackground = () => {
  return (
    <div className="fixed inset-0 -z-10 bg-gradient-to-b from-white via-primary-50/20 to-accent-50/20">
      <Canvas
        camera={{ position: [0, 0, 30], fov: 75 }}
        style={{ background: 'transparent', cursor: 'none' }}
        dpr={[1, 1.5]}
        gl={{ alpha: true, antialias: true }}
      >
        <Particles count={3000} />
        <ParticleConnections />
      </Canvas>
      
      {/* Gradient overlays */}
      <div className="absolute inset-0 bg-gradient-to-b from-transparent via-transparent to-white/50 pointer-events-none" />
      
      {/* Custom cursor */}
      <style>{`
        .particle-container {
          cursor: none !important;
        }
        .particle-container * {
          cursor: none !important;
        }
      `}</style>
    </div>
  );
};
