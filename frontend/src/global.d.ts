import '@react-three/fiber';

declare module '@react-three/fiber' {
  interface ThreeElements {
    ambientLight: object;
    pointLight: object;
    mesh: object;
    icosahedronGeometry: object;
    sphereGeometry: object;
    boxGeometry: object;
    group: object;
  }
}
