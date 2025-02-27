import React, { useState, useEffect } from "react";
import { Form, FormInput, FormButton } from "./Form";
import "./AddDemeritForm.css";

interface AddDemeritFormProps {
  onSubmit: (demerit: NewDemeritRecord) => void;
  onClose: () => void;
}

export interface NewDemeritRecord {
  student_id: number;
  category_id: number;
  points: number;
  description: string;
}

interface Student {
  id: number;
  name: string;
}

interface Category {
  id: number;
  name: string;
  default_points: number;
}

export const AddDemeritForm: React.FC<AddDemeritFormProps> = ({
  onSubmit,
  onClose,
}) => {
  const [formData, setFormData] = useState<{
    student_id: string;
    category_id: string;
    points: number;
    description: string;
  }>({
    student_id: "",
    category_id: "",
    points: 1,
    description: "",
  });

  const [students, setStudents] = useState<Student[]>([]);
  const [categories, setCategories] = useState<Category[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const fetchData = async () => {
      try {
        const [studentsRes, categoriesRes] = await Promise.all([
          fetch("http://localhost:8080/students", { credentials: "include" }),
          fetch("http://localhost:8080/demerit-categories", {
            credentials: "include",
          }),
        ]);

        if (!studentsRes.ok || !categoriesRes.ok) {
          throw new Error("Failed to fetch data");
        }

        const [studentsData, categoriesData] = await Promise.all([
          studentsRes.json(),
          categoriesRes.json(),
        ]);

        setStudents(studentsData);
        setCategories(categoriesData);
      } catch (err) {
        setError(err instanceof Error ? err.message : "An error occurred");
      } finally {
        setLoading(false);
      }
    };

    fetchData();
  }, []);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      // Convert string values to numbers before sending
      const submitData = {
        student_id: parseInt(formData.student_id, 10),
        category_id: parseInt(formData.category_id, 10),
        points: formData.points,
        description: formData.description,
      };

      const response = await fetch("http://localhost:8080/add_demerit", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        credentials: "include",
        body: JSON.stringify(submitData), // Send the converted data
      });

      if (!response.ok) {
        const errorData = await response.json();
        throw new Error(errorData.message || "Failed to add demerit");
      }

      onSubmit(submitData);
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed to add demerit");
    }
  };

  const handleChange = (
    e: React.ChangeEvent<HTMLInputElement | HTMLSelectElement>,
  ) => {
    const { name, value } = e.target;
    setFormData((prev) => ({
      ...prev,
      [name]: value,
    }));
  };

  if (loading) return <div>Loading...</div>;
  if (error) return <div>Error: {error}</div>;

  return (
    <div className="modal-overlay">
      <div className="modal-content">
        <h2>Add New Demerit</h2>
        <Form onSubmit={handleSubmit} className="add-demerit-form">
          <select
            name="student_id"
            value={formData.student_id}
            onChange={handleChange}
            required
          >
            <option value="">Select Student</option>
            {students.map((student) => (
              <option key={student.id} value={student.id}>
                {student.name}
              </option>
            ))}
          </select>

          <select
            name="category_id"
            value={formData.category_id}
            onChange={handleChange}
            required
          >
            <option value="">Select Category</option>
            {categories.map((category) => (
              <option key={category.id} value={category.id}>
                {category.name} ({category.default_points} points)
              </option>
            ))}
          </select>

          <FormInput
            label="Points"
            type="number"
            name="points"
            value={formData.points}
            onChange={handleChange}
            min={1}
            max={5}
            required
          />

          <FormInput
            label="Description"
            type="text"
            name="description"
            value={formData.description}
            onChange={handleChange}
            placeholder="Enter description"
            required
          />

          <div className="form-buttons">
            <FormButton type="submit">Add Demerit</FormButton>
            <FormButton type="button" onClick={onClose}>
              Cancel
            </FormButton>
          </div>
        </Form>
      </div>
    </div>
  );
};
