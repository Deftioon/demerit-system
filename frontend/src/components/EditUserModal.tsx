import React, { useState } from "react";
import { AdminUserRecord } from "../types/demerit";
import { Form, FormInput, FormButton } from "./Form";
// import "./EditUserModal.css";

//TODO: The Edit User does not edit the database.

interface EditUserModalProps {
  user: AdminUserRecord;
  onClose: () => void;
  onSave: (updatedUser: AdminUserRecord) => Promise<void>;
}

export const EditUserModal: React.FC<EditUserModalProps> = ({
  user,
  onClose,
  onSave,
}) => {
  const [formData, setFormData] = useState<AdminUserRecord>({
    ...user,
    // Ensure these required fields are always present
    total_demerits: user.total_demerits || 0,
    created_at: user.created_at || new Date().toISOString(),
  });
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleChange = (
    e: React.ChangeEvent<HTMLInputElement | HTMLSelectElement>,
  ) => {
    const { name, value } = e.target;

    // Special handling for numeric fields
    if (name === "grade_level") {
      // For grade_level, we'll store as a number in the state
      setFormData((prev) => ({
        ...prev,
        [name]: value === "" ? null : Number(value),
      }));
    } else {
      // For other fields, keep as is
      setFormData((prev) => ({
        ...prev,
        [name]: value,
      }));
    }
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);
    setError(null);
    try {
      // Create a copy of the form data with proper type conversions
      const updatedData = {
        ...formData,
        grade_level:
          formData.user_type === "student"
            ? Number(formData.grade_level)
            : null,
        class_section:
          formData.user_type === "student" ? formData.class_section : null,
        total_demerits: formData.total_demerits || 0,
        created_at: formData.created_at || new Date().toISOString(),
      };

      console.log("Submitting user update:", updatedData);
      await onSave(updatedData);
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed to update user");
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="modal-overlay">
      <div className="modal-content">
        <h2>Edit User</h2>
        <Form onSubmit={handleSubmit} className="edit-user-form">
          <FormInput
            label="First Name"
            name="first_name"
            value={formData.first_name}
            onChange={handleChange}
            required
          />
          <FormInput
            label="Last Name"
            name="last_name"
            value={formData.last_name}
            onChange={handleChange}
            required
          />
          <FormInput
            label="Username"
            name="username"
            value={formData.username}
            onChange={handleChange}
            required
          />
          <FormInput
            label="Email"
            type="email"
            name="email"
            value={formData.email}
            onChange={handleChange}
            required
          />
          <label>Role</label>
          <select
            name="user_type"
            value={formData.user_type}
            onChange={handleChange}
            required
          >
            <option value="admin">Admin</option>
            <option value="teacher">Teacher</option>
            <option value="student">Student</option>
            <option value="parent">Parent</option>
          </select>
          {formData.user_type === "student" && (
            <>
              <FormInput
                label="Grade Level"
                type="number"
                name="grade_level"
                value={formData.grade_level ?? ""}
                onChange={handleChange}
                required
              />
              <FormInput
                label="Class Section"
                name="class_section"
                value={formData.class_section ?? ""}
                onChange={handleChange}
                required
              />
            </>
          )}
          {error && <div className="error-message">{error}</div>}
          <div className="form-buttons">
            <FormButton type="submit" disabled={loading}>
              {loading ? "Saving..." : "Save"}
            </FormButton>
            <FormButton type="button" onClick={onClose}>
              Cancel
            </FormButton>
          </div>
        </Form>
      </div>
    </div>
  );
};
