import { type ReactNode } from 'react';

interface BadgeProps {
  variant?: 'success' | 'warning' | 'danger' | 'default';
  children: ReactNode;
  className?: string;
}

export const Badge = ({
  variant = 'default',
  children,
  className = '',
}: BadgeProps) => {
  const variantClasses = {
    success: 'badge-success',
    warning: 'badge-warning',
    danger: 'badge-danger',
    default: 'bg-gray-100 text-gray-700',
  };
  
  return (
    <span className={`badge ${variantClasses[variant]} ${className}`}>
      {children}
    </span>
  );
};
