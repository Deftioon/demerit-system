import React, { ReactNode } from "react";
import "./Form.css";

interface FormProps {
  onSubmit: (e: React.FormEvent<HTMLFormElement>) => void;
  children: ReactNode;
  className?: string;
}

interface FormInputProps {
  label?: string;
  type?: "text" | "email" | "password" | "number" | "tel" | "date";
  name: string;
  value: string | number;
  onChange: (e: React.ChangeEvent<HTMLInputElement>) => void;
  placeholder?: string;
  required?: boolean;
  error?: string;
  className?: string;
}

interface FormButtonProps {
  type?: "submit" | "button" | "reset";
  onClick?: (e: React.MouseEvent<HTMLButtonElement>) => void;
  children: ReactNode;
  disabled?: boolean;
  className?: string;
}

export const Form: React.FC<FormProps> = ({
  onSubmit,
  children,
  className,
}) => {
  return (
    <form onSubmit={onSubmit} className={`form-container ${className || ""}`}>
      {children}
    </form>
  );
};

export const FormInput: React.FC<FormInputProps> = ({
  label,
  type = "text",
  name,
  value,
  onChange,
  placeholder,
  required = false,
  error,
  className,
}) => {
  return (
    <div className="form-group">
      {label && (
        <label htmlFor={name}>
          {label}
          {required && <span className="required">*</span>}
        </label>
      )}
      <input
        id={name}
        type={type}
        name={name}
        value={value}
        onChange={onChange}
        placeholder={placeholder}
        required={required}
        className={`form-input ${error ? "error" : ""} ${className || ""}`}
      />
      {error && <span className="error-message">{error}</span>}
    </div>
  );
};

export const FormButton: React.FC<FormButtonProps> = ({
  type = "submit",
  onClick,
  children,
  disabled = false,
  className,
}) => {
  return (
    <button
      type={type}
      onClick={onClick}
      disabled={disabled}
      className={`form-button ${className || ""}`}
    >
      {children}
    </button>
  );
};
