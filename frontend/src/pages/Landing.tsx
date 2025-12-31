import { Link } from 'react-router-dom';
import { motion } from 'framer-motion';
import { ParticleBackground } from '@/components/three/ParticleBackground';
import { CustomCursor } from '@/components/common/CustomCursor';
import {
    FiArrowRight,
    FiShield,
    FiTrendingUp,
    FiZap,
    FiLayers,
    FiLock,
    FiGlobe,
    FiBarChart2,
    FiGithub,
    FiTwitter,
    FiMessageCircle
} from 'react-icons/fi';

export const Landing = () => {
    const fadeIn = {
        initial: { opacity: 0, y: 20 },
        animate: { opacity: 1, y: 0 },
        transition: { duration: 0.6 }
    };

    const staggerContainer = {
        animate: {
            transition: {
                staggerChildren: 0.1
            }
        }
    };

    const features = [
        {
            icon: <FiLayers className="w-6 h-6" />,
            title: "Multi-Protocol Aggregation",
            description: "Access the best yields across Casper's entire DeFi ecosystem. Our intelligent router automatically finds and executes the most profitable strategies across lending protocols, DEXs, and staking pools."
        },
        {
            icon: <FiShield className="w-6 h-6" />,
            title: "Battle-Tested Security",
            description: "Built with Odra framework and audited smart contracts. Multi-signature controls, reentrancy guards, and rate limiters protect your assets. Non-custodial design means you're always in control."
        },
        {
            icon: <FiTrendingUp className="w-6 h-6" />,
            title: "Liquid Staking (lstCSPR)",
            description: "Stake your CSPR without locking liquidity. Receive lstCSPR tokens that accrue staking rewards while remaining tradable and usable across DeFi. Compound your yields automatically."
        },
        {
            icon: <FiZap className="w-6 h-6" />,
            title: "Gas-Optimized Execution",
            description: "Advanced batching and routing algorithms minimize transaction costs. Our strategies are designed for maximum capital efficiency with automated rebalancing and optimal fee structures."
        },
        {
            icon: <FiLock className="w-6 h-6" />,
            title: "Risk Management",
            description: "Real-time monitoring of strategy performance and risk metrics. Customizable risk profiles, impermanent loss protection, and automated circuit breakers ensure your capital stays safe."
        },
        {
            icon: <FiGlobe className="w-6 h-6" />,
            title: "Cross-Chain Ready",
            description: "Future-proof architecture designed for cross-chain yield aggregation. Bridge assets seamlessly and capture opportunities across multiple networks from a single interface."
        }
    ];



    return (
        <div className="min-h-screen bg-white text-gray-900 relative overflow-hidden">
            <ParticleBackground />
            <CustomCursor />

            {/* Navigation */}
            <nav className="fixed top-0 w-full z-50 bg-white/90 backdrop-blur-lg border-b border-gray-200">
                <div className="max-w-7xl mx-auto px-6 py-5">
                    <div className="flex items-center justify-between">
                        <motion.div
                            className="flex items-center space-x-2"
                            initial={{ opacity: 0, x: -20 }}
                            animate={{ opacity: 1, x: 0 }}
                            transition={{ duration: 0.5 }}
                        >
                            <div className="w-12 h-12 bg-gradient-to-br from-primary-500 to-accent-500 rounded-xl flex items-center justify-center shadow-lg">
                                <span className="text-white font-extrabold text-xl">CV</span>
                            </div>
                            <span className="text-2xl font-extrabold tracking-tight bg-gradient-to-r from-primary-600 to-accent-600 bg-clip-text text-transparent">CasperVault</span>
                        </motion.div>

                        <motion.div
                            className="hidden md:flex items-center space-x-10"
                            initial={{ opacity: 0 }}
                            animate={{ opacity: 1 }}
                            transition={{ delay: 0.2 }}
                        >
                            <a href="#features" className="text-gray-700 hover:text-primary-600 transition-colors text-lg font-bold">Features</a>
                            <a href="#how-it-works" className="text-gray-700 hover:text-primary-600 transition-colors text-lg font-bold">How It Works</a>
                            <a href="#security" className="text-gray-700 hover:text-primary-600 transition-colors text-lg font-bold">Security</a>
                        </motion.div>

                        <motion.div
                            initial={{ opacity: 0, x: 20 }}
                            animate={{ opacity: 1, x: 0 }}
                            transition={{ duration: 0.5 }}
                        >
                            <Link
                                to="/dashboard"
                                className="px-8 py-3 bg-gradient-to-r from-primary-500 to-accent-500 text-white rounded-xl hover:shadow-lg hover:scale-105 transition-all duration-200 flex items-center space-x-2 group text-lg font-bold"
                            >
                                <span>Launch App</span>
                                <FiArrowRight className="group-hover:translate-x-1 transition-transform" />
                            </Link>
                        </motion.div>
                    </div>
                </div>
            </nav>

            {/* Hero Section */}
            <section className="pt-32 pb-20 px-6">
                <div className="max-w-7xl mx-auto">
                    <div className="grid md:grid-cols-2 gap-12 items-center">
                        <motion.div
                            initial={{ opacity: 0, y: 30 }}
                            animate={{ opacity: 1, y: 0 }}
                            transition={{ duration: 0.8 }}
                            className="max-w-3xl"
                        >
                            <div className="inline-block px-5 py-2 bg-gradient-to-r from-primary-100 to-accent-100 rounded-full text-base font-semibold text-primary-700 mb-8">
                                Built on Casper Network
                            </div>

                            <h1 className="text-6xl md:text-7xl lg:text-8xl font-extrabold leading-[1.1] mb-8">
                                The Future of
                                <span className="block bg-gradient-to-r from-primary-600 via-accent-600 to-primary-600 bg-clip-text text-transparent mt-2">
                                    DeFi Aggregation
                                </span>
                            </h1>

                            <p className="text-2xl md:text-3xl text-gray-600 mb-12 leading-relaxed font-medium max-w-2xl">
                                Maximize yields across Casper DeFi with intelligent automation and full control.
                            </p>

                            <div className="flex flex-wrap gap-5">
                                <Link
                                    to="/dashboard"
                                    className="px-10 py-5 bg-gradient-to-r from-primary-500 to-accent-500 text-white rounded-xl hover:shadow-xl hover:scale-105 transition-all duration-200 flex items-center space-x-3 group text-xl font-bold"
                                >
                                    <span>Launch App</span>
                                    <FiArrowRight className="w-6 h-6 group-hover:translate-x-1 transition-transform" />
                                </Link>

                                <a
                                    href="#features"
                                    className="px-10 py-5 border-2 border-primary-500 text-primary-600 rounded-xl hover:bg-primary-50 transition-all duration-200 text-xl font-bold"
                                >
                                    Explore
                                </a>
                            </div>
                        </motion.div>

                        <motion.div
                            initial={{ opacity: 0, scale: 0.95 }}
                            animate={{ opacity: 1, scale: 1 }}
                            transition={{ duration: 0.8, delay: 0.2 }}
                            className="relative hidden md:block"
                        >
                            {/* Simple visual element instead of complex card */}
                            <div className="relative">
                                <div className="w-full h-[500px] bg-gradient-to-br from-primary-50 via-accent-50 to-primary-100 rounded-3xl border-2 border-primary-200 shadow-2xl overflow-hidden">
                                    {/* Decorative gradient orbs */}
                                    <div className="absolute top-20 right-20 w-64 h-64 bg-gradient-to-br from-primary-400 to-accent-400 rounded-full blur-3xl opacity-30 animate-pulse-slow" />
                                    <div className="absolute bottom-20 left-20 w-80 h-80 bg-gradient-to-br from-accent-400 to-primary-400 rounded-full blur-3xl opacity-20 animate-pulse-slow" style={{ animationDelay: '1s' }} />

                                    {/* Simple stats overlay */}
                                    <div className="absolute inset-0 flex flex-col items-center justify-center p-12">
                                        <div className="text-center space-y-8">
                                            <div>
                                                <div className="text-7xl font-extrabold bg-gradient-to-r from-primary-600 to-accent-600 bg-clip-text text-transparent mb-3">
                                                    12.5%
                                                </div>
                                                <div className="text-2xl font-bold text-gray-700">Average APY</div>
                                            </div>

                                            <div className="grid grid-cols-2 gap-8 pt-8 border-t-2 border-primary-200/50">
                                                <div>
                                                    <div className="text-4xl font-extrabold text-gray-900 mb-2">8.5%</div>
                                                    <div className="text-lg font-semibold text-gray-600">Staking</div>
                                                </div>
                                                <div>
                                                    <div className="text-4xl font-extrabold text-gray-900 mb-2">15.2%</div>
                                                    <div className="text-lg font-semibold text-gray-600">Vault</div>
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </motion.div>
                    </div>
                </div>
            </section>

            {/* Features Section */}
            <section id="features" className="py-20 px-6 bg-gray-50">
                <div className="max-w-7xl mx-auto">
                    <motion.div
                        initial={{ opacity: 0, y: 20 }}
                        whileInView={{ opacity: 1, y: 0 }}
                        viewport={{ once: true }}
                        transition={{ duration: 0.6 }}
                        className="text-center mb-16"
                    >
                        <h2 className="text-5xl md:text-6xl font-extrabold mb-6">
                            Enterprise-Grade DeFi
                        </h2>
                        <p className="text-2xl text-gray-600 max-w-2xl mx-auto font-medium">
                            Professional yield optimization with institutional security.
                        </p>
                    </motion.div>

                    <motion.div
                        variants={staggerContainer}
                        initial="initial"
                        whileInView="animate"
                        viewport={{ once: true }}
                        className="grid md:grid-cols-2 lg:grid-cols-3 gap-8"
                    >
                        {features.map((feature, index) => (
                            <motion.div
                                key={index}
                                variants={fadeIn}
                                className="bg-white rounded-2xl p-8 border border-gray-200 hover:border-primary-300 hover:shadow-xl transition-all duration-300 group"
                            >
                                <div className="w-14 h-14 bg-gradient-to-br from-primary-100 to-accent-100 rounded-xl flex items-center justify-center text-primary-600 mb-5 group-hover:from-primary-500 group-hover:to-accent-500 group-hover:text-white transition-all duration-300 group-hover:scale-110">
                                    {feature.icon}
                                </div>
                                <h3 className="text-2xl font-extrabold mb-4">{feature.title}</h3>
                                <p className="text-lg text-gray-600 leading-relaxed font-medium">{feature.description}</p>
                            </motion.div>
                        ))}
                    </motion.div>
                </div>
            </section>

            {/* How It Works */}
            <section id="how-it-works" className="py-20 px-6">
                <div className="max-w-7xl mx-auto">
                    <motion.div
                        initial={{ opacity: 0, y: 20 }}
                        whileInView={{ opacity: 1, y: 0 }}
                        viewport={{ once: true }}
                        transition={{ duration: 0.6 }}
                        className="text-center mb-16"
                    >
                        <h2 className="text-5xl md:text-6xl font-extrabold mb-6">
                            Three Simple Steps
                        </h2>
                        <p className="text-2xl text-gray-600 max-w-2xl mx-auto font-medium">
                            Start earning optimized yields today
                        </p>
                    </motion.div>

                    <div className="grid md:grid-cols-3 gap-16">
                        {[
                            {
                                step: "01",
                                title: "Connect Wallet",
                                description: "Link your Casper wallet securely. Your keys, your crypto."
                            },
                            {
                                step: "02",
                                title: "Deposit Assets",
                                description: "Choose a strategy and deposit. Smart contracts handle the rest."
                            },
                            {
                                step: "03",
                                title: "Earn Returns",
                                description: "Watch your portfolio grow with auto-compounding yields."
                            }
                        ].map((item, index) => (
                            <motion.div
                                key={index}
                                initial={{ opacity: 0, y: 30 }}
                                whileInView={{ opacity: 1, y: 0 }}
                                viewport={{ once: true }}
                                transition={{ duration: 0.6, delay: index * 0.2 }}
                                className="relative"
                            >
                                <div className="text-8xl font-extrabold bg-gradient-to-br from-primary-200 to-accent-200 bg-clip-text text-transparent mb-6">{item.step}</div>
                                <h3 className="text-3xl font-extrabold mb-4">{item.title}</h3>
                                <p className="text-xl text-gray-600 leading-relaxed font-medium">{item.description}</p>

                                {index < 2 && (
                                    <div className="hidden lg:block absolute top-16 -right-8 text-5xl text-primary-200">
                                        →
                                    </div>
                                )}
                            </motion.div>
                        ))}
                    </div>
                </div>
            </section>

            {/* Security Section */}
            <section id="security" className="py-20 px-6 bg-gray-50">
                <div className="max-w-7xl mx-auto">
                    <div className="grid md:grid-cols-2 gap-12 items-center">
                        <motion.div
                            initial={{ opacity: 0, x: -30 }}
                            whileInView={{ opacity: 1, x: 0 }}
                            viewport={{ once: true }}
                            transition={{ duration: 0.6 }}
                        >
                            <h2 className="text-5xl md:text-6xl font-extrabold mb-8">
                                Security First
                            </h2>
                            <p className="text-2xl text-gray-600 mb-10 leading-relaxed font-medium">
                                Enterprise-grade security with battle-tested smart contracts on Casper Network.
                            </p>

                            <div className="space-y-6">
                                {[
                                    {
                                        title: "Non-Custodial",
                                        description: "You maintain full control of your private keys. We never have access to your funds."
                                    },
                                    {
                                        title: "Smart Contract Audits",
                                        description: "Multiple independent security audits by leading firms ensure code quality and safety."
                                    },
                                    {
                                        title: "Real-Time Monitoring",
                                        description: "24/7 automated monitoring with circuit breakers and emergency pause functionality."
                                    },
                                    {
                                        title: "Bug Bounty Program",
                                        description: "Ongoing incentives for security researchers to identify and report vulnerabilities."
                                    }
                                ].map((item, index) => (
                                    <div key={index} className="flex items-start space-x-4">
                                        <div className="w-6 h-6 bg-gradient-to-br from-primary-500 to-accent-500 rounded-full flex items-center justify-center flex-shrink-0 mt-1 shadow-md">
                                            <svg className="w-3 h-3 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" />
                                            </svg>
                                        </div>
                                        <div>
                                            <h4 className="font-bold text-lg mb-1">{item.title}</h4>
                                            <p className="text-gextrabold text-xl mb-2">{item.title}</h4>
                                            <p className="text-gray-600 text-lg font-medium
                  </div>
                ))}
              </div>
            </motion.div>

            <motion.div
              initial={{ opacity: 0, x: 30 }}
              whileInView={{ opacity: 1, x: 0 }}
              viewport={{ once: true }}
              transition={{ duration: 0.6 }}
              className="bg-white rounded-2xl p-8 border border-gray-200 shadow-xl"
            >
                                            <div className="space-y-6">
                                                <div className="flex items-center justify-between pb-4 border-b border-gray-200">
                                                    <span className="font-bold">Security Status</span>
                                                    <span className="text-green-600 text-sm font-medium">●  All Systems Operational</span>
                                                </div>

                                                <div className="space-y-4">
                                                    {[
                                                        { label: 'Smart Contracts', status: 'Audited', percentage: 100 },
                                                        { label: 'Multisig Protection', status: 'Active', percentage: 100 },
                                                        { label: 'Rate Limiting', status: 'Enabled', percentage: 100 },
                                                        { label: 'Emergency Pause', status: 'Armed', percentage: 100 }
                                                    ].map((item, index) => (
                                                        <div key={index}>
                                                            <div className="flex items-center justify-between mb-2">
                                                                <span className="text-sm font-medium text-gray-700">{item.label}</span>
                                                                <span className="text-xs text-gray-600">{item.status}</span>
                                                            </div>
                                                            <div className="h-2 bg-gray-100 rounded-full overflow-hidden">
                                                                <motion.div
                                                                    initial={{ width: 0 }}
                                                                    whileInView={{ width: `${item.percentage}%` }}
                                                                    viewport={{ once: true }}
                                                                    transition={{ duration: 1, delay: index * 0.1 }}
                                                                    className="h-full bg-gradient-to-r from-primary-500 to-accent-500"
                                                                />
                                                            </div>
                                                        </div>
                                                    ))}
                                                </div>

                                                <div className="pt-6 border-t border-gray-200">
                                                    <div className="flex items-center space-x-2 text-sm text-gray-600">
                                                        <FiShield className="w-4 h-4" />
                                                        <span>Last audit: December 2025 by CertiK</span>
                                                    </div>
                                                </div>
                                            </div>
                                        </motion.div>
                                    </div>
        </div>
                        </section>

                        {/* CTA Section */}
                        <section className="py-20 px-6">
                            <div className="max-w-4xl mx-auto text-center">
                                <motion.div
                                    initial={{ opacity: 0, y: 20 }}
                                    whileInView={{ opacity: 1, y: 0 }}
                                    viewport={{ once: true }}
                                    transition={{ duration: 0.6 }}
                                >
                                    <h2 className="text-4xl md:text-5xl font-bold mb-6">
              Ready to Maximize 5xl md:text-6xl font-extrabold mb-8">
                                        Ready to Start Earning?
                                    </h2>
                                    <p className="text-2xl text-gray-600 mb-10 font-medium">
                                        Join the future of DeFi
                                        <Link
                                            to="/dashboard"
                                            className="inline-flex items-center space-x-3 px-12 py-6 bg-gradient-to-r from-primary-500 to-accent-500 text-white rounded-xl hover:shadow-2xl hover:scale-105 transition-all duration-200 text-2xl font-extrabold group"
                                        >
                                            <span>Launch App</span>
                                            <FiArrowRight className="w-7 h-7 group-hover:translate-x-1 transition-transform" />
                                        </Link>
                                </motion.div>
                            </div>
                        </section>

                        {/* Footer */}
                        <footer className="border-t border-gray-200 py-16 px-6">
                            <div className="max-w-7xl mx-auto">
                                <div className="grid md:grid-cols-4 gap-12 mb-12">
                                    <div>
                                        <div className="flex items-center space-x-2 mb-6">
                                            <div className="w-10 h-10 bg-gradient-to-br from-primary-500 to-accent-500 rounded-xl flex items-center justify-center shadow-md">
                                                <span className="text-white font-extrabold text-lg">CV</span>
                                            </div>
                                            <span className="text-xl font-extrabold bg-gradient-to-r from-primary-600 to-accent-600 bg-clip-text text-transparent">CasperVault</span>
                                        </div>
                                        <p className="text-base text-gray-600 font-medium">
                                            Premier DeFi aggregator on Casper Network
                                        </p>
                                    </div>

                                    <div>
                                        <h4 className="font-extrabold text-lg mb-5">Product</h4>
                                        <ul className="space-y-3 text-base text-gray-600 font-medium">
                                            <li><Link to="/dashboard" className="hover:text-primary-600 transition-colors">Dashboard</Link></li>
                                            <li><Link to="/strategies" className="hover:text-primary-600 transition-colors">Strategies</Link></li>
                                            <li><Link to="/analytics" className="hover:text-primary-600 transition-colors">Analytics</Link></li>
                                        </ul>
                                    </div>

                                    <div>
                                        <h4 className="font-extrabold text-lg mb-5">Resources</h4>
                                        <ul className="space-y-3 text-base text-gray-600 font-medium">
                                            <li><a href="#" className="hover:text-primary-600 transition-colors">Documentation</a></li>
                                            <li><a href="#" className="hover:text-primary-600 transition-colors">Audits</a></li>
                                            <li><a href="#" className="hover:text-primary-600 transition-colors">GitHub</a></li>
                                        </ul>
                                    </div>

                                    <div>
                                        <h4 className="font-extrabold text-lg mb-5">Community</h4>
                                        <ul className="space-y-3 text-base text-gray-600 font-medium">
                                            <li><a href="#" className="hover:text-primary-600 transition-colors">Twitter</a></li>
                                            <li><a href="#" className="hover:text-primary-600 transition-colors">Discord</a></li>
                                            <li><a href="#" className="hover:text-primary-600 transition-colors">Telegram</a></li>
                                        </ul>
                                    </div>
                                </div>

                                <div className="pt-8 border-t border-gray-200 flex flex-col md:flex-row justify-between items-center">
    }                                <p className="text-base text-gray-600 font-medium">
                                        © 2025 CasperVault. All rights reserved.
                                    </p>

                                    <div className="flex items-center space-x-6 mt-6 md:mt-0">
                                        <a href="#" className="text-gray-600 hover:text-primary-600 transition-colors">
                                            <FiGithub className="w-6 h-6" />
                                        </a>
                                        <a href="#" className="text-gray-600 hover:text-primary-600 transition-colors">
                                            <FiTwitter className="w-6 h-6" />
                                        </a>
                                        <a href="#" className="text-gray-600 hover:text-primary-600 transition-colors">
                                            <FiMessageCircle className="w-6 h-6" />
                                        </a>
                                    </div>
                                </div>
                            </div>
                        </footer>
                    </div>
                    );
};
