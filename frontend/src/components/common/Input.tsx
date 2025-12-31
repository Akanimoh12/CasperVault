import { type InputHTMLAttributes, type ReactNode } from 'react';

interface InputProps extends InputHTMLAttributes<HTMLInputElement> {
  label?: string;
  error?: string;
  helper?: string;
  icon?: ReactNode;
}

export const Input = ({
  label,
  error,
  helper,
  icon,
  className = '',
  ...props
}: InputProps) => {
  return (
    <div className="w-full">
      {label && (
        <label className="block text-sm font-medium text-gray-700 mb-2">
          {label}
        </label>
      )}
      
      <div className="relative">
        {icon && (
          <div className="absolute left-4 top-1/2 -translate-y-1/2 text-gray-400">
            {icon}
          </div>
        )}
        
        <input
          className={`input ${icon ? 'pl-12' : ''} ${error ? 'border-danger-500 focus:border-danger-500 focus:ring-danger-100' : ''} ${className}`}
          {...props}
        />
      </div>
      
      {error && (
        <p className="mt-2 text-sm text-danger-600">{error}</p>
      )}
      
      {helper && !error && (
        <p className="mt-2 text-sm text-gray-500">{helper}</p>
      )}
    </div>
  );
};
