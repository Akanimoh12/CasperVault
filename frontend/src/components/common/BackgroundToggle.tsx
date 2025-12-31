import { BsStars } from 'react-icons/bs';
import { motion } from 'framer-motion';
import { useUIStore } from '../../store/uiStore';

export const BackgroundToggle = () => {
  const { backgroundEnabled, toggleBackground } = useUIStore();
  
  return (
    <motion.button
      whileHover={{ scale: 1.05 }}
      whileTap={{ scale: 0.95 }}
      onClick={toggleBackground}
      className="fixed bottom-8 right-8 z-50 p-4 bg-white rounded-full shadow-lg border border-gray-200 hover:shadow-xl transition-shadow"
      title={backgroundEnabled ? 'Disable effects' : 'Enable effects'}
      aria-label={backgroundEnabled ? 'Disable background effects' : 'Enable background effects'}
    >
      <BsStars 
        className={`text-2xl transition-colors ${
          backgroundEnabled ? 'text-primary-500' : 'text-gray-400'
        }`} 
      />
    </motion.button>
  );
};
