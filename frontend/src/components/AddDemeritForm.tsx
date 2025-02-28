import React, { useState, useEffect, useRef } from "react";
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
  const [filteredStudents, setFilteredStudents] = useState<Student[]>([]);
  const [studentSearchTerm, setStudentSearchTerm] = useState("");
  const [isDropdownOpen, setIsDropdownOpen] = useState(false);
  const [selectedStudent, setSelectedStudent] = useState<Student | null>(null);
  const dropdownRef = useRef<HTMLDivElement>(null);
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

  useEffect(() => {
    // Filter students based on search term
    if (studentSearchTerm) {
      const filtered = students.filter((student) =>
        student.name.toLowerCase().includes(studentSearchTerm.toLowerCase()),
      );
      setFilteredStudents(filtered);
    } else {
      setFilteredStudents(students);
    }
  }, [studentSearchTerm, students]);

  // Close dropdown when clicking outside
  useEffect(() => {
    function handleClickOutside(event: MouseEvent) {
      if (
        dropdownRef.current &&
        !dropdownRef.current.contains(event.target as Node)
      ) {
        setIsDropdownOpen(false);
      }
    }
    document.addEventListener("mousedown", handleClickOutside);
    return () => document.removeEventListener("mousedown", handleClickOutside);
  }, []);

  const handleStudentSearchChange = (
    e: React.ChangeEvent<HTMLInputElement>,
  ) => {
    setStudentSearchTerm(e.target.value);
    setIsDropdownOpen(true);
  };

  const handleStudentSelect = (student: Student) => {
    setSelectedStudent(student);
    setFormData((prev) => ({ ...prev, student_id: student.id.toString() }));
    setStudentSearchTerm(student.name);
    setIsDropdownOpen(false);
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      const submitData = {
        student_id: parseInt(formData.student_id, 10),
        category_id: parseInt(formData.category_id, 10),
        points: parseInt(String(formData.points), 10),
        description: formData.description,
      };

      setLoading(true);
      onSubmit(submitData);
      onClose();
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

  const highlightMatch = (text: string, query: string) => {
    if (!query) return text;

    const regex = new RegExp(`(${query})`, "gi");
    const parts = text.split(regex);

    return (
      <>
        {parts.map((part, i) =>
          regex.test(part) ? (
            <span key={i} className="match-highlight">
              {part}
            </span>
          ) : (
            part
          ),
        )}
      </>
    );
  };

  if (loading) return <div>Loading...</div>;
  if (error) return <div>Error: {error}</div>;

  return (
    <div className="modal-overlay">
      <div className="modal-content">
        <button className="floating-close-button" onClick={onClose}>
          Close
        </button>

        <h2>Add New Demerit</h2>
        <Form onSubmit={handleSubmit} className="add-demerit-form">
          <div className="form-group">
            <label htmlFor="student-search">Student</label>
            <div
              className={`searchable-dropdown ${isDropdownOpen ? "active" : ""}`}
              ref={dropdownRef}
            >
              <input
                type="text"
                id="student-search"
                className="search-input"
                placeholder="Search for a student..."
                value={studentSearchTerm}
                onChange={handleStudentSearchChange}
                onClick={() => setIsDropdownOpen(true)}
                autoComplete="off"
              />
              {isDropdownOpen && (
                <ul className="dropdown-list">
                  {filteredStudents.length > 0 ? (
                    filteredStudents.map((student) => (
                      <li
                        key={student.id}
                        onClick={() => handleStudentSelect(student)}
                        className={
                          selectedStudent?.id === student.id ? "selected" : ""
                        }
                      >
                        {highlightMatch(student.name, studentSearchTerm)}
                      </li>
                    ))
                  ) : (
                    <li className="no-results">No students found</li>
                  )}
                </ul>
              )}
            </div>
            {!formData.student_id && (
              <div className="form-error">Please select a student</div>
            )}
          </div>

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
            <FormButton type="submit" disabled={!formData.student_id}>
              Add Demerit
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
