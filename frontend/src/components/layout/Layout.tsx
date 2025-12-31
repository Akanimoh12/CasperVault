import { Outlet } from 'react-router-dom';
import { Toaster } from 'react-hot-toast';
import { Navbar } from './Navbar';
import { Footer } from './Footer';
import { ParticleBackground } from '@/components/three';
import { useUIStore } from '@/store/uiStore';

export const Layout = () => {
  const { backgroundEnabled } = useUIStore();

  return (
    <div className="min-h-screen flex flex-col">
      {/* Three.js Background */}
      {backgroundEnabled && <ParticleBackground />}

      {/* Navbar */}
      <Navbar />

      {/* Main Content */}
      <main className="flex-1 flex items-center justify-center">
        <div className="max-w-7xl w-full mx-auto px-4 py-8">
          <Outlet />
        </div>
      </main>

      {/* Footer */}
      <Footer />

      {/* Toast Notifications */}
      <Toaster
        position="bottom-right"
        toastOptions={{
          className: 'bg-white shadow-lg border border-gray-200',
          style: {
            background: '#ffffff',
            color: '#111827',
          },
          success: {
            iconTheme: {
              primary: '#10b981',
              secondary: '#ffffff',
            },
          },
          error: {
            iconTheme: {
              primary: '#ef4444',
              secondary: '#ffffff',
            },
          },
        }}
      />
    </div>
  );
};
