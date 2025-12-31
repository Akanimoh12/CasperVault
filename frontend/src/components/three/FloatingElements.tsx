/// <reference types="@react-three/fiber" />
import { Canvas } from '@react-three/fiber';
import { Float, MeshDistortMaterial } from '@react-three/drei';

interface FloatingShapeProps {
  position: [number, number, number];
  color: string;
}

const FloatingShape = ({ position, color }: FloatingShapeProps) => {
  return (
    <Float
      speed={2}
      rotationIntensity={0.5}
      floatIntensity={1}
      floatingRange={[-0.5, 0.5]}
    >
      {/* @ts-expect-error - Three.js types not properly recognized by TypeScript */}
      <mesh position={position}>
        {/* @ts-expect-error - Three.js types not properly recognized by TypeScript */}
        <icosahedronGeometry args={[1, 1]} />
        <MeshDistortMaterial
          color={color}
          opacity={0.1}
          transparent
          distort={0.3}
          speed={2}
        />
        {/* @ts-expect-error - Three.js types not properly recognized by TypeScript */}
      </mesh>
    </Float>
  );
};

export const FloatingElements = () => {
  return (
    <div className="fixed inset-0 -z-10 pointer-events-none">
      <Canvas 
        camera={{ position: [0, 0, 8], fov: 60 }}
        dpr={[1, 2]} // Limit pixel ratio for performance
      >
        {/* @ts-expect-error - Three.js types not properly recognized by TypeScript */}
        <ambientLight intensity={0.5} />
        <FloatingShape position={[-3, 2, -2]} color="#0ea5e9" />
        <FloatingShape position={[3, -2, -3]} color="#d946ef" />
        <FloatingShape position={[0, 2, -4]} color="#10b981" />
      </Canvas>
    </div>
  );
};
