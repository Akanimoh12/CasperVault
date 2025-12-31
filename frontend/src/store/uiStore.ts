import { create } from 'zustand';
import { persist } from 'zustand/middleware';

interface UIState {
  backgroundEnabled: boolean;
  sidebarOpen: boolean;
  theme: 'light' | 'dark';
  
  toggleBackground: () => void;
  setBackgroundEnabled: (enabled: boolean) => void;
  toggleSidebar: () => void;
  setTheme: (theme: 'light' | 'dark') => void;
}

export const useUIStore = create<UIState>()(
  persist(
    (set) => ({
      backgroundEnabled: true,
      sidebarOpen: false,
      theme: 'light',
      
      toggleBackground: () =>
        set((state) => ({ backgroundEnabled: !state.backgroundEnabled })),
      
      setBackgroundEnabled: (enabled: boolean) =>
        set({ backgroundEnabled: enabled }),
      
      toggleSidebar: () =>
        set((state) => ({ sidebarOpen: !state.sidebarOpen })),
      
      setTheme: (theme: 'light' | 'dark') =>
        set({ theme }),
    }),
    {
      name: 'ui-storage',
      partialize: (state) => ({
        backgroundEnabled: state.backgroundEnabled,
        theme: state.theme,
      }),
    }
  )
);
