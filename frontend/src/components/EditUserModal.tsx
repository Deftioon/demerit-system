import React, { useState, useEffect } from "react";
import { AdminUserRecord } from "../types/demerit";
import { Form, FormInput, FormButton } from "./Form";
import "./EditUserModal.css";

interface EditUserModalProps {
  user: AdminUserRecord;
  onClose: () => void;
  onSave: (updatedUser: AdminUserRecord) => Promise<void>;
}

interface StudentOption {
  id: number;
  name: string;
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
    children: user.children || [],
  });

  const [availableStudents, setAvailableStudents] = useState<StudentOption[]>(
    [],
  );
  const [selectedStudents, setSelectedStudents] = useState<StudentOption[]>(
    user.children?.map((child) => ({ id: child.id, name: child.name })) || [],
  );
  const [loading, setLoading] = useState(false);
  const [loadingStudents, setLoadingStudents] = useState(false);
  const [error, setError] = useState<string | null>(null);

  // Fetch all students when editing a parent
  useEffect(() => {
    if (user.user_type === "parent" || formData.user_type === "parent") {
      fetchStudents();
    }
  }, [user.user_type, formData.user_type]);

  const fetchStudents = async () => {
    setLoadingStudents(true);
    try {
      const response = await fetch("http://localhost:8080/students", {
        credentials: "include",
      });
      if (!response.ok) throw new Error("Failed to fetch students");

      const students = await response.json();
      setAvailableStudents(students);

      // If we have children data, mark them as selected
      if (user.children && user.children.length > 0) {
        const childrenIds = user.children.map((child) => child.id);
        const selectedStudents = students.filter((student: StudentOption) =>
          childrenIds.includes(student.id),
        );
        setSelectedStudents(selectedStudents);
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed to load students");
    } finally {
      setLoadingStudents(false);
    }
  };

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

      // If changing user type to parent, fetch students
      if (
        name === "user_type" &&
        value === "parent" &&
        !loadingStudents &&
        availableStudents.length === 0
      ) {
        fetchStudents();
      }
    }
  };

  const handleStudentSelect = (e: React.ChangeEvent<HTMLSelectElement>) => {
    const selectedOptions = Array.from(e.target.selectedOptions);
    const selectedStudentIds = selectedOptions.map((option) =>
      parseInt(option.value),
    );

    const newSelectedStudents = availableStudents.filter((student) =>
      selectedStudentIds.includes(student.id),
    );

    setSelectedStudents(newSelectedStudents);

    // Update formData with new children
    setFormData((prev) => ({
      ...prev,
      children: newSelectedStudents.map((student) => ({
        id: student.id,
        name: student.name,
      })),
    }));
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
        children: formData.user_type === "parent" ? formData.children : [],
      };

      console.log("Submitting user update:", updatedData);
      await onSave(updatedData);

      // After successful save, add parent-student relationships if needed
      if (formData.user_type === "parent" && selectedStudents.length > 0) {
        await saveParentStudentRelationships(
          formData.user_id,
          selectedStudents,
        );
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed to update user");
    } finally {
      setLoading(false);
    }
  };

  const saveParentStudentRelationships = async (
    userId: number,
    students: StudentOption[],
  ) => {
    // First we need the parent_id
    try {
      // Fetch the parent's ID from the database
      const parentResponse = await fetch(`http://localhost:8080/parents`, {
        credentials: "include",
      });
      if (!parentResponse.ok) throw new Error("Failed to fetch parent info");

      const parents = await parentResponse.json();
      const parent = parents.find((p: any) => p.user_id === userId);

      if (!parent) {
        throw new Error("Parent record not found");
      }

      // Add relationships for each selected student
      for (const student of students) {
        await fetch("http://localhost:8080/add_parent_student", {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          credentials: "include",
          body: JSON.stringify({
            parent_id: parent.parent_id,
            student_id: student.id,
          }),
        });
      }
    } catch (err) {
      console.error("Error saving parent-student relationships:", err);
      // We don't want to block the overall success just because of this
    }
  };

  return (
    <div className="modal-overlay">
      <div className="modal-content">
        <button className="floating-close-button" onClick={onClose}>
          Close
        </button>
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

          {formData.user_type === "parent" && (
            <div className="form-group">
              <label htmlFor="children">Associated Students</label>
              {loadingStudents ? (
                <div className="loading-indicator">Loading students...</div>
              ) : (
                <select
                  multiple
                  id="children"
                  className="student-select"
                  value={selectedStudents.map((s) => s.id.toString())}
                  onChange={handleStudentSelect}
                  size={5}
                >
                  {availableStudents.map((student) => (
                    <option key={student.id} value={student.id}>
                      {student.name}
                    </option>
                  ))}
                </select>
              )}
              <div className="help-text">
                Hold Ctrl/Cmd to select multiple students
              </div>
            </div>
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
