/// <reference types="vite/client" />

// Extend global JSX namespace to include Three.js elements
declare global {
  namespace JSX {
    interface IntrinsicElements {
      ambientLight: any;
      pointLight: any;
      mesh: any;
      icosahedronGeometry: any;
      sphereGeometry: any;
      boxGeometry: any;
      group: any;
      points: any;
      pointsMaterial: any;
      bufferGeometry: any;
      bufferAttribute: any;
    }
  }

  // Casper Wallet extension types
  interface Window {
    // Casper Wallet (formerly CasperLabs Signer)
    casperlabsHelper?: {
      requestConnection: () => Promise<void>;
      disconnectFromSite: () => Promise<void>;
      getActivePublicKey: () => Promise<string>;
      isConnected: () => Promise<boolean>;
      sign: (deploy: any, publicKey: string) => Promise<any>;
    };
    // Alternative Casper Wallet API
    CasperWalletProvider?: any;
    // CSPR.click API
    csprclick?: any;
  }
}

export {};
