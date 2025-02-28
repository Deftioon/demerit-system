import React, { useState, useEffect } from "react";
import "./DemeritHistory.css";

interface StudentDetailModalProps {
  studentId: number;
  studentName: string;
  onClose: () => void;
}

interface DemeritDetail {
  demerit_id: number;
  category_name: string;
  points: number;
  teacher_name: string;
  description: string;
  date_issued: string;
}

export const StudentDetailModal: React.FC<StudentDetailModalProps> = ({
  studentId,
  studentName,
  onClose,
}) => {
  const [demerits, setDemerits] = useState<DemeritDetail[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const fetchStudentDemerits = async () => {
      try {
        const response = await fetch(
          `http://localhost:8080/student_demerits/${studentId}`,
        );

        if (!response.ok) {
          throw new Error("Failed to fetch student demerits");
        }

        const data = await response.json();
        setDemerits(data);
      } catch (err) {
        setError(err instanceof Error ? err.message : "An error occurred");
      } finally {
        setLoading(false);
      }
    };

    fetchStudentDemerits();
  }, [studentId]);

  return (
    <div className="modal-overlay">
      <div className="modal-content demerit-history">
        <button className="floating-close-button" onClick={onClose}>
          Close
        </button>

        <div className="history-header">
          <h2>{studentName}'s Demerit History</h2>
        </div>

        {loading ? (
          <div>Loading...</div>
        ) : error ? (
          <div className="error">{error}</div>
        ) : demerits.length > 0 ? (
          <table className="history-table">
            <thead>
              <tr>
                <th>Date</th>
                <th>Category</th>
                <th>Points</th>
                <th>Teacher</th>
                <th>Description</th>
              </tr>
            </thead>
            <tbody>
              {demerits.map((demerit) => (
                <tr key={demerit.demerit_id}>
                  <td>{new Date(demerit.date_issued).toLocaleDateString()}</td>
                  <td>{demerit.category_name}</td>
                  <td>{demerit.points}</td>
                  <td>{demerit.teacher_name}</td>
                  <td>{demerit.description}</td>
                </tr>
              ))}
            </tbody>
          </table>
        ) : (
          <div className="no-records">
            No demerit records found for this student
          </div>
        )}

        <button onClick={onClose} className="close-button">
          Close
        </button>
      </div>
    </div>
  );
};
