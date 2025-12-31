import { Link, useLocation } from 'react-router-dom';
import { motion } from 'framer-motion';
import { MdDashboard, MdShowChart, MdAnalytics, MdAccountBalanceWallet } from 'react-icons/md';
import { WalletButton } from '@/components/wallet';
import { ConnectionStatus } from '@/components/common';

const navLinks = [
  { path: '/', label: 'Dashboard', icon: MdDashboard },
  { path: '/strategies', label: 'Strategies', icon: MdShowChart },
  { path: '/analytics', label: 'Analytics', icon: MdAnalytics },
  { path: '/portfolio', label: 'Portfolio', icon: MdAccountBalanceWallet },
];

export const Navbar = () => {
  const location = useLocation();

  return (
    <nav className="sticky top-0 z-40 glass border-b border-gray-200">
      <div className="max-w-7xl mx-auto px-4">
        <div className="flex items-center justify-between h-20">
          {/* Logo */}
          <Link to="/" className="flex items-center gap-3">
            <div className="w-12 h-12 rounded-xl bg-gradient-to-br from-primary-500 to-accent-500 flex items-center justify-center">
              <span className="text-2xl font-bold text-white font-display">CV</span>
            </div>
            <div className="hidden md:block">
              <h1 className="text-xl font-bold text-gray-900 font-display">CasperVault</h1>
              <p className="text-xs text-gray-500">Cross-Chain DeFi</p>
            </div>
          </Link>

          {/* Navigation Links */}
          <div className="hidden md:flex items-center gap-2">
            {navLinks.map((link) => {
              const Icon = link.icon;
              const isActive = location.pathname === link.path;

              return (
                <motion.div key={link.path} whileHover={{ scale: 1.05 }} whileTap={{ scale: 0.95 }}>
                  <Link
                    to={link.path}
                    className={`
                      flex items-center gap-2 px-4 py-2 rounded-xl font-medium transition-all
                      ${
                        isActive
                          ? 'bg-primary-500 text-white shadow-md'
                          : 'text-gray-600 hover:bg-gray-100 hover:text-gray-900'
                      }
                    `}
                  >
                    <Icon className="w-5 h-5" />
                    <span>{link.label}</span>
                  </Link>
                </motion.div>
              );
            })}
          </div>

          {/* Wallet Button & Status */}
          <div className="flex items-center gap-4">
            <ConnectionStatus />
            <WalletButton />
          </div>
        </div>
      </div>
    </nav>
  );
};
