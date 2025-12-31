import { BrowserRouter, Routes, Route } from 'react-router-dom';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { Layout } from '@/components/layout/Layout';
import { Landing, Dashboard, Strategies, Analytics, Portfolio } from '@/pages';
import { useEffect } from 'react';

// Create React Query client
const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      refetchOnWindowFocus: false,
      retry: 1,
      staleTime: 30000, // 30 seconds
    },
  },
});

function App() {
  // Debug: Check if Casper Wallet extension is loaded
  useEffect(() => {
    const checkWallet = () => {
      console.log('=== CASPER WALLET DEBUG ===');
      console.log('window.casperlabsHelper:', window.casperlabsHelper);
      console.log('window.CasperWalletProvider:', window.CasperWalletProvider);
      console.log('window.csprclick:', window.csprclick);
      
      if (window.casperlabsHelper) {
        console.log('✅ Casper Wallet extension detected on page load');
        console.log('Type:', typeof window.casperlabsHelper);
        console.log('Methods:', Object.keys(window.casperlabsHelper));
      } else if (window.CasperWalletProvider) {
        console.log('✅ CasperWalletProvider detected');
      } else if (window.csprclick) {
        console.log('✅ CSPR.click detected');
      } else {
        console.warn('⚠️ No Casper Wallet extension detected on initial load.');
        console.warn('Extension may still be loading. Will retry when connecting...');
        console.warn('Available window properties:', Object.keys(window).filter(k => k.toLowerCase().includes('casper')));
      }
    };
    
    // Check immediately
    checkWallet();
    
    // Also check after delays (for slower extension loading in Brave)
    setTimeout(checkWallet, 500);
    setTimeout(checkWallet, 1000);
    setTimeout(checkWallet, 2000);
  }, []);

  return (
    <QueryClientProvider client={queryClient}>
      <BrowserRouter>
        <Routes>
          {/* Landing page without layout */}
          <Route path="/" element={<Landing />} />
          
          {/* Dashboard routes with layout */}
          <Route element={<Layout />}>
            <Route path="/dashboard" element={<Dashboard />} />
            <Route path="/strategies" element={<Strategies />} />
            <Route path="/analytics" element={<Analytics />} />
            <Route path="/portfolio" element={<Portfolio />} />
          </Route>
        </Routes>
      </BrowserRouter>
    </QueryClientProvider>
  );
}

export default App;
