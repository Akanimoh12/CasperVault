/**
 * Three.js Background Components Usage Examples
 * 
 * These components add visual depth to the white-themed UI without
 * overwhelming the content. They can be toggled on/off by the user.
 */

import { ParticleBackground } from '../components/three/ParticleBackground';
import { FloatingElements } from '../components/three/FloatingElements';
import { BackgroundToggle } from '../components/common/BackgroundToggle';
import { useUIStore } from '../store/uiStore';

/**
 * Example 1: Basic Usage with Particle Background
 * 
 * Add the ParticleBackground to your layout component.
 * It will render behind all content with -z-10.
 */
export const LayoutWithParticles = () => {
  const { backgroundEnabled } = useUIStore();
  
  return (
    <div className="min-h-screen">
      {/* Particle background (renders behind everything) */}
      {backgroundEnabled && <ParticleBackground />}
      
      {/* Your content */}
      <main className="relative z-10">
        <h1>Welcome to CasperVault</h1>
        {/* ... */}
      </main>
      
      {/* Toggle button (fixed position) */}
      <BackgroundToggle />
    </div>
  );
};

/**
 * Example 2: Using Floating Elements Instead
 * 
 * FloatingElements provides a different visual effect
 * with geometric shapes instead of particles.
 */
export const LayoutWithFloating = () => {
  const { backgroundEnabled } = useUIStore();
  
  return (
    <div className="min-h-screen">
      {backgroundEnabled && <FloatingElements />}
      
      <main className="relative z-10">
        {/* Your content */}
      </main>
      
      <BackgroundToggle />
    </div>
  );
};

/**
 * Example 3: Combining Both Effects
 * 
 * You can layer both particle and floating effects
 * for maximum visual impact (use sparingly).
 */
export const LayoutWithBothEffects = () => {
  const { backgroundEnabled } = useUIStore();
  
  return (
    <div className="min-h-screen">
      {backgroundEnabled && (
        <>
          <ParticleBackground />
          <FloatingElements />
        </>
      )}
      
      <main className="relative z-10">
        {/* Your content */}
      </main>
      
      <BackgroundToggle />
    </div>
  );
};

/**
 * Example 4: Conditional Background Based on Route
 * 
 * Show effects only on specific pages.
 */
export const ConditionalBackground = ({ showEffects }: { showEffects: boolean }) => {
  const { backgroundEnabled } = useUIStore();
  
  return (
    <div className="min-h-screen">
      {backgroundEnabled && showEffects && <ParticleBackground />}
      
      <main className="relative z-10">
        {/* Your content */}
      </main>
      
      <BackgroundToggle />
    </div>
  );
};

/**
 * Performance Tips:
 * 
 * 1. Particle Count: Default is 1000. Reduce for better performance:
 *    <ParticleBackground count={500} />
 * 
 * 2. DPR (Device Pixel Ratio): Already optimized to [1, 2] for performance
 * 
 * 3. Mobile: Consider disabling on mobile devices:
 *    const isMobile = window.innerWidth < 768;
 *    {!isMobile && backgroundEnabled && <ParticleBackground />}
 * 
 * 4. Toggle State: Persisted in localStorage via Zustand, so user
 *    preference is remembered across sessions
 */
