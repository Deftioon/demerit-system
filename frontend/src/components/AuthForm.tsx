import React, { useState } from "react";
import { Form, FormInput, FormButton } from "./Form";
import "./AuthForm.css";

type AuthMode = "login" | "register";

interface AuthFormData {
  email: string;
  password: string;
  username?: string;
  firstName?: string;
  lastName?: string;
}

interface AuthFormErrors {
  email?: string;
  password?: string;
  username?: string;
  firstName?: string;
  lastName?: string;
}

const AuthForm: React.FC = () => {
  const [mode, setMode] = useState<AuthMode>("login");
  const [showPassword, setShowPassword] = useState(false);

  const [formData, setFormData] = useState<AuthFormData>({
    email: "",
    password: "",
    username: "",
    firstName: "",
    lastName: "",
  });

  const [errors, setErrors] = useState<AuthFormErrors>({});

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const { name, value } = e.target;
    setFormData((prev) => ({
      ...prev,
      [name]: value,
    }));
    // Clear error when user starts typing
    if (errors[name as keyof AuthFormErrors]) {
      setErrors((prev) => ({
        ...prev,
        [name]: undefined,
      }));
    }
  };

  const validateForm = (): boolean => {
    const newErrors: AuthFormErrors = {};

    // Email validation
    if (!formData.email) {
      newErrors.email = "Email is required";
    } else if (!/\S+@\S+\.\S+/.test(formData.email)) {
      newErrors.email = "Email is invalid";
    }

    // Password validation
    if (!formData.password) {
      newErrors.password = "Password is required";
    } else if (formData.password.length < 8) {
      newErrors.password = "Password must be at least 8 characters";
    }

    // Additional validations for registration
    if (mode === "register") {
      if (!formData.username) {
        newErrors.username = "Username is required";
      }
      if (!formData.firstName) {
        newErrors.firstName = "First name is required";
      }
      if (!formData.lastName) {
        newErrors.lastName = "Last name is required";
      }
    }

    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };

  const handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    if (validateForm()) {
      console.log(`${mode} submitted:`, formData);
      // Handle authentication logic here
    }
  };

  const switchMode = (newMode: AuthMode) => {
    setMode(newMode);
    setErrors({});
  };

  return (
    <div className="auth-container">
      {/* Auth Header */}
      <div className="auth-header">
        <button
          className={`auth-toggle ${mode === "login" ? "active" : ""}`}
          onClick={() => switchMode("login")}
        >
          Login
        </button>
        <button
          className={`auth-toggle ${mode === "register" ? "active" : ""}`}
          onClick={() => switchMode("register")}
        >
          Register
        </button>
      </div>

      {/* Auth Form */}
      <Form onSubmit={handleSubmit} className="auth-form">
        <h2>{mode === "login" ? "Welcome Back" : "Create Account"}</h2>

        {mode === "register" && (
          <>
            <div className="name-fields">
              <FormInput
                label="First Name"
                name="firstName"
                value={formData.firstName || ""}
                onChange={handleChange}
                placeholder="Enter first name"
                error={errors.firstName}
                required
              />
              <FormInput
                label="Last Name"
                name="lastName"
                value={formData.lastName || ""}
                onChange={handleChange}
                placeholder="Enter last name"
                error={errors.lastName}
                required
              />
            </div>

            <FormInput
              label="Username"
              name="username"
              value={formData.username || ""}
              onChange={handleChange}
              placeholder="Choose a username"
              error={errors.username}
              required
            />
          </>
        )}

        <FormInput
          label="Email"
          type="email"
          name="email"
          value={formData.email}
          onChange={handleChange}
          placeholder="Enter your email"
          error={errors.email}
          required
        />

        <div className="password-input-wrapper">
          <FormInput
            label="Password"
            type={showPassword ? "text" : "password"}
            name="password"
            value={formData.password}
            onChange={handleChange}
            placeholder="Enter your password"
            error={errors.password}
            required
          />
          <button
            type="button"
            className="toggle-password"
            onClick={() => setShowPassword(!showPassword)}
          >
            {showPassword ? "👁️" : "👁️‍🗨️"}
          </button>
        </div>

        <FormButton type="submit">
          {mode === "login" ? "Login" : "Register"}
        </FormButton>

        {mode === "login" && (
          <button type="button" className="forgot-password">
            Forgot Password?
          </button>
        )}
      </Form>
    </div>
  );
};

export default AuthForm;
