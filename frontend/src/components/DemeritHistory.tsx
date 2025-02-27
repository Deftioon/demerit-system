import React, { useState, useEffect } from "react";
import "./DemeritHistory.css";

interface DemeritHistoryProps {
  onClose: () => void;
}

interface DemeritRecord {
  demerit_id: number;
  student_name: string;
  category_name: string;
  points: number;
  teacher_name: string;
  description: string;
  date_issued: string;
}

export const DemeritHistory: React.FC<DemeritHistoryProps> = ({ onClose }) => {
  const [records, setRecords] = useState<DemeritRecord[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [searchTerm, setSearchTerm] = useState("");

  useEffect(() => {
    const fetchDemeritHistory = async () => {
      try {
        const response = await fetch("http://localhost:8080/demerit_history", {
          credentials: "include",
        });

        if (!response.ok) {
          throw new Error("Failed to fetch demerit history");
        }

        const data = await response.json();
        setRecords(data);
      } catch (err) {
        setError(err instanceof Error ? err.message : "An error occurred");
      } finally {
        setLoading(false);
      }
    };

    fetchDemeritHistory();
  }, []);

  const filteredRecords = records.filter(
    (record) =>
      record.student_name.toLowerCase().includes(searchTerm.toLowerCase()) ||
      record.teacher_name.toLowerCase().includes(searchTerm.toLowerCase()) ||
      record.category_name.toLowerCase().includes(searchTerm.toLowerCase()),
  );

  if (loading)
    return (
      <div className="modal-overlay">
        <div className="modal-content demerit-history">
          <button className="floating-close-button" onClick={onClose}>
            Close
          </button>
          <h2>Demerit History</h2>
          <div>Loading...</div>
        </div>
      </div>
    );

  if (error)
    return (
      <div className="modal-overlay">
        <div className="modal-content demerit-history">
          <button className="floating-close-button" onClick={onClose}>
            Close
          </button>
          <h2>Demerit History</h2>
          <div className="error">{error}</div>
          <button onClick={onClose} className="close-button">
            Close
          </button>
        </div>
      </div>
    );

  return (
    <div className="modal-overlay">
      <div className="modal-content demerit-history">
        <button className="floating-close-button" onClick={onClose}>
          Close
        </button>
        <div className="history-header">
          <h2>Demerit History</h2>
          <div className="search-container">
            <input
              type="text"
              placeholder="Search by student, teacher, or category..."
              value={searchTerm}
              onChange={(e) => setSearchTerm(e.target.value)}
              className="search-input"
            />
          </div>
        </div>

        {filteredRecords.length > 0 ? (
          <table className="history-table">
            <thead>
              <tr>
                <th>ID</th>
                <th>Student</th>
                <th>Category</th>
                <th>Points</th>
                <th>Teacher</th>
                <th>Description</th>
                <th>Date</th>
              </tr>
            </thead>
            <tbody>
              {filteredRecords.map((record) => (
                <tr key={record.demerit_id}>
                  <td>{record.demerit_id}</td>
                  <td>{record.student_name}</td>
                  <td>{record.category_name}</td>
                  <td>{record.points}</td>
                  <td>{record.teacher_name}</td>
                  <td>{record.description}</td>
                  <td>{new Date(record.date_issued).toLocaleDateString()}</td>
                </tr>
              ))}
            </tbody>
          </table>
        ) : (
          <div className="no-records">
            {searchTerm
              ? "No matching records found"
              : "No demerit records found"}
          </div>
        )}

        <button onClick={onClose} className="close-button">
          Close
        </button>
      </div>
    </div>
  );
};
