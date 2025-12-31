import { type ReactNode } from 'react';
import { motion } from 'framer-motion';

interface CardProps {
  title?: string;
  subtitle?: string;
  icon?: ReactNode;
  children: ReactNode;
  className?: string;
  hover?: boolean;
}

export const Card = ({
  title,
  subtitle,
  icon,
  children,
  className = '',
  hover = true,
}: CardProps) => {
  return (
    <motion.div
      initial={{ opacity: 0, y: 20 }}
      animate={{ opacity: 1, y: 0 }}
      className={`card ${hover ? 'hover:shadow-hover' : ''} ${className}`}
    >
      {(title || icon) && (
        <div className="flex items-center justify-between mb-4">
          <div>
            {title && (
              <h3 className="text-lg font-bold text-gray-900">{title}</h3>
            )}
            {subtitle && (
              <p className="text-sm text-gray-500 mt-1">{subtitle}</p>
            )}
          </div>
          {icon && <div className="text-primary-500">{icon}</div>}
        </div>
      )}
      {children}
    </motion.div>
  );
};
