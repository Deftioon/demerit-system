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
  submit?: string; // For general submission errors
}

interface User {
  id: string;
  email: string;
  username?: string;
  firstName?: string;
  lastName?: string;
}

interface AuthFormProps {
  onAuthSuccess: (user: User) => void;
}

const AuthForm: React.FC<AuthFormProps> = ({ onAuthSuccess }) => {
  console.log("Logging in...");
  const [mode, setMode] = useState<AuthMode>("login");
  const [showPassword, setShowPassword] = useState(false);
  const [isLoading, setIsLoading] = useState(false);

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

    if (!formData.email) {
      newErrors.email = "Email is required";
    } else if (!/\S+@\S+\.\S+/.test(formData.email)) {
      newErrors.email = "Email is invalid";
    }

    if (!formData.password) {
      newErrors.password = "Password is required";
    } else if (formData.password.length < 8) {
      newErrors.password = "Password must be at least 8 characters";
    }

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

  const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
    // Fix the logging
    console.log("Sending:", JSON.stringify(formData, null, 2));

    e.preventDefault();
    if (!validateForm()) return;

    setIsLoading(true);
    setErrors({});

    try {
      // When sending login request, only send required fields
      const requestData =
        mode === "login"
          ? {
              email: formData.email,
              password: formData.password,
            }
          : {
              email: formData.email,
              password: formData.password,
              username: formData.username,
              first_name: formData.firstName, // Match backend field names
              last_name: formData.lastName, // Match backend field names
            };

      console.log("Request data:", JSON.stringify(requestData, null, 2));

      const endpoint = mode === "login" ? "/login" : "/register";
      const response = await fetch(`http://localhost:8080${endpoint}`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify(requestData), // Send requestData instead of formData
        credentials: "include",
      });

      // Log the raw response
      console.log("Raw response:", response);

      // Log the response text
      const responseText = await response.text();
      console.log("Response text:", responseText);

      // Try to parse the JSON
      let data;
      try {
        data = JSON.parse(responseText);
        console.log("Parsed data:", data);
      } catch (e) {
        console.error("Failed to parse JSON:", e);
      }

      if (!response.ok) {
        throw new Error(data?.message || "Authentication failed");
      }

      // Call the success handler with the user data
      if (data?.user) {
        onAuthSuccess(data.user);
      } else {
        throw new Error("No user data received");
      }
    } catch (error) {
      console.error("Error details:", error);
      setErrors({
        submit: error instanceof Error ? error.message : "An error occurred",
      });
    } finally {
      setIsLoading(false);
    }
  };

  const switchMode = (newMode: AuthMode) => {
    setMode(newMode);
    setErrors({});
  };

  return (
    <div className="auth-container">
      <div className="auth-header">
        <button
          className={`auth-toggle ${mode === "login" ? "active" : ""}`}
          onClick={() => switchMode("login")}
          disabled={isLoading}
        >
          Login
        </button>
        <button
          className={`auth-toggle ${mode === "register" ? "active" : ""}`}
          onClick={() => switchMode("register")}
          disabled={isLoading}
        >
          Register
        </button>
      </div>

      <Form onSubmit={handleSubmit} className="auth-form">
        <h2>{mode === "login" ? "Welcome Back" : "Create Account"}</h2>

        {errors.submit && <div className="error-banner">{errors.submit}</div>}

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
                disabled={isLoading}
              />
              <FormInput
                label="Last Name"
                name="lastName"
                value={formData.lastName || ""}
                onChange={handleChange}
                placeholder="Enter last name"
                error={errors.lastName}
                required
                disabled={isLoading}
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
              disabled={isLoading}
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
          disabled={isLoading}
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
            disabled={isLoading}
          />
          <button
            type="button"
            className="toggle-password"
            onClick={() => setShowPassword(!showPassword)}
            disabled={isLoading}
          >
            {showPassword ? "👁️" : "👁️‍🗨️"}
          </button>
        </div>

        <FormButton type="submit" disabled={isLoading}>
          {isLoading ? "Loading..." : mode === "login" ? "Login" : "Register"}
        </FormButton>

        {mode === "login" && (
          <button
            type="button"
            className="forgot-password"
            disabled={isLoading}
          >
            Forgot Password?
          </button>
        )}
      </Form>
    </div>
  );
};

export default AuthForm;
