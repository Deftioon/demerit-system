import React, { useState, useEffect } from "react";
import "./ParentChildManager.css";

interface Student {
  id: number;
  name: string;
}

interface Parent {
  parent_id: number;
  name: string;
  user_id: number;
}

export const ParentChildManager: React.FC = () => {
  const [parents, setParents] = useState<Parent[]>([]);
  const [students, setStudents] = useState<Student[]>([]);
  const [selectedParent, setSelectedParent] = useState<number | null>(null);
  const [selectedStudent, setSelectedStudent] = useState<number | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [success, setSuccess] = useState<string | null>(null);

  useEffect(() => {
    const fetchData = async () => {
      try {
        const [parentsRes, studentsRes] = await Promise.all([
          fetch("http://localhost:8080/parents", { credentials: "include" }),
          fetch("http://localhost:8080/students", { credentials: "include" }),
        ]);

        if (!parentsRes.ok || !studentsRes.ok) {
          throw new Error("Failed to fetch data");
        }

        const [parentsData, studentsData] = await Promise.all([
          parentsRes.json(),
          studentsRes.json(),
        ]);

        setParents(parentsData);
        setStudents(studentsData);
      } catch (err) {
        setError(err instanceof Error ? err.message : "An error occurred");
      } finally {
        setLoading(false);
      }
    };

    fetchData();
  }, []);

  const handleAddRelationship = async () => {
    if (!selectedParent || !selectedStudent) {
      setError("Please select both a parent and a student");
      return;
    }

    try {
      const response = await fetch("http://localhost:8080/add_parent_student", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        credentials: "include",
        body: JSON.stringify({
          parent_id: selectedParent,
          student_id: selectedStudent,
        }),
      });

      if (!response.ok) {
        const errorData = await response.json();
        throw new Error(errorData.message || "Failed to add relationship");
      }

      setSuccess("Parent-child relationship added successfully");
      // Reset selection
      setSelectedParent(null);
      setSelectedStudent(null);
      
      setTimeout(() => setSuccess(null), 3000);
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed to add relationship");
    }
  };

  if (loading) return <div>Loading...</div>;
  if (error) return <div className="error-message">{error}</div>;

  return (
    <div className="parent-child-manager">
      <h2>Manage Parent-Child Relationships</h2>
      
      {success && <div className="success-message">{success}</div>}
      
      <div className="relationship-form">
        <div className="form-group">
          <label>Select Parent:</label>
          <select
            value={selectedParent || ""}
            onChange={(e) => setSelectedParent(Number(e.target.value))}
          >
            <option value="">Choose a parent</option>
            {parents.map((parent) => (
              <option key={parent.parent_id} value={parent.parent_id}>
                {parent.name}
              </option>
            ))}
          </select>
        </div>
        
        <div className="form-group">
          <label>Select Student:</label>
          <select
            value={selectedStudent || ""}
            onChange={(e) => setSelectedStudent(Number(e.target.value))}
          >
            <option value="">Choose a student</option>
            {students.map((student) => (
              <option key={student.id} value={student.id}>
                {student.name}
              </option>
            ))}
          </select>
        </div>
        
        <button onClick={handleAddRelationship} className="add-button">
          Assign Student to Parent
        </button>
      </div>
    </div>
  );
};