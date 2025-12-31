import { Link } from 'react-router-dom';
import { FaTwitter, FaGithub, FaDiscord, FaTelegram } from 'react-icons/fa';

const productLinks = [
  { label: 'Dashboard', path: '/' },
  { label: 'Strategies', path: '/strategies' },
  { label: 'Analytics', path: '/analytics' },
];

const resourceLinks = [
  { label: 'Documentation', href: '#' },
  { label: 'API', href: '#' },
  { label: 'Security', href: '#' },
];

const socialLinks = [
  { icon: FaTwitter, href: 'https://twitter.com', label: 'Twitter' },
  { icon: FaGithub, href: 'https://github.com', label: 'GitHub' },
  { icon: FaDiscord, href: 'https://discord.com', label: 'Discord' },
  { icon: FaTelegram, href: 'https://telegram.org', label: 'Telegram' },
];

export const Footer = () => {
  return (
    <footer className="border-t border-gray-200 bg-white mt-auto">
      <div className="max-w-7xl mx-auto px-4 py-12">
        <div className="grid grid-cols-1 md:grid-cols-4 gap-8">
          {/* Brand Section */}
          <div className="space-y-4">
            <div className="flex items-center gap-3">
              <div className="w-10 h-10 rounded-xl bg-gradient-to-br from-primary-500 to-accent-500 flex items-center justify-center">
                <span className="text-lg font-bold text-white font-display">CV</span>
              </div>
              <h3 className="text-lg font-bold text-gray-900 font-display">CasperVault</h3>
            </div>
            <p className="text-sm text-gray-600">
              Cross-chain DeFi aggregator on Casper Network. Maximize your yields with automated
              strategies.
            </p>
          </div>

          {/* Product Links */}
          <div>
            <h4 className="font-semibold text-gray-900 mb-4">Product</h4>
            <ul className="space-y-2">
              {productLinks.map((link) => (
                <li key={link.path}>
                  <Link
                    to={link.path}
                    className="text-sm text-gray-600 hover:text-primary-500 transition-colors"
                  >
                    {link.label}
                  </Link>
                </li>
              ))}
            </ul>
          </div>

          {/* Resource Links */}
          <div>
            <h4 className="font-semibold text-gray-900 mb-4">Resources</h4>
            <ul className="space-y-2">
              {resourceLinks.map((link) => (
                <li key={link.label}>
                  <a
                    href={link.href}
                    className="text-sm text-gray-600 hover:text-primary-500 transition-colors"
                  >
                    {link.label}
                  </a>
                </li>
              ))}
            </ul>
          </div>

          {/* Community / Social Links */}
          <div>
            <h4 className="font-semibold text-gray-900 mb-4">Community</h4>
            <div className="flex items-center gap-3">
              {socialLinks.map((social) => {
                const Icon = social.icon;
                return (
                  <a
                    key={social.label}
                    href={social.href}
                    target="_blank"
                    rel="noopener noreferrer"
                    className="w-10 h-10 rounded-xl bg-gray-100 hover:bg-primary-500 hover:text-white text-gray-600 flex items-center justify-center transition-all"
                    aria-label={social.label}
                  >
                    <Icon className="w-5 h-5" />
                  </a>
                );
              })}
            </div>
          </div>
        </div>

        {/* Copyright */}
        <div className="mt-12 pt-8 border-t border-gray-200">
          <p className="text-sm text-center text-gray-500">
            © 2026 CasperVault. Built for Casper Hackathon 2026.
          </p>
        </div>
      </div>
    </footer>
  );
};
