import React, { useState, useEffect } from "react";
import "./DataTable.css"; // Reuse the data table styles
import { StudentDetailModal } from "./StudentDetailModal";

interface StudentDemeritSummary {
  student_id: number;
  student_name: string;
  total_points: number;
  recent_demerit: string | null;
  grade_level: number | null;
  class_section: string | null;
}

interface StudentDemeritSummaryProps {
  refreshTrigger?: number;
}

export const StudentDemeritSummary: React.FC<StudentDemeritSummaryProps> = ({
  refreshTrigger = 0,
}) => {
  const [summaries, setSummaries] = useState<StudentDemeritSummary[]>([]);
  const [filteredSummaries, setFilteredSummaries] = useState<
    StudentDemeritSummary[]
  >([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [searchTerm, setSearchTerm] = useState("");

  const [selectedStudent, setSelectedStudent] = useState<{
    id: number;
    name: string;
  } | null>(null);

  const fetchData = async () => {
    setLoading(true);
    setError(null);
    try {
      const response = await fetch(
        "http://localhost:8080/student_demerit_summary",
        {
          credentials: "include",
        },
      );

      if (!response.ok) {
        throw new Error("Failed to fetch data");
      }

      const data = await response.json();
      setSummaries(data);
      setFilteredSummaries(data);
    } catch (err) {
      setError(err instanceof Error ? err.message : "An error occurred");
    } finally {
      setLoading(false);
    }
  };

  // Apply search filter
  useEffect(() => {
    if (searchTerm.trim() === "") {
      setFilteredSummaries(summaries);
    } else {
      const term = searchTerm.toLowerCase();
      setFilteredSummaries(
        summaries.filter(
          (summary) =>
            summary.student_name.toLowerCase().includes(term) ||
            (summary.class_section &&
              summary.class_section.toLowerCase().includes(term)) ||
            (summary.recent_demerit &&
              summary.recent_demerit.toLowerCase().includes(term)),
        ),
      );
    }
  }, [summaries, searchTerm]);

  // Initial data fetch and refresh
  useEffect(() => {
    fetchData();
  }, [refreshTrigger]);

  // Define class for high demerit points
  const getDemeritClass = (points: number) => {
    if (points >= 12) return "very-high-demerits";
    if (points >= 6) return "high-demerits";
    if (points >= 3) return "medium-demerits";
    return "good-demerits";
  };

  const handleRowClick = (summary: StudentDemeritSummary) => {
    setSelectedStudent({
      id: summary.student_id,
      name: summary.student_name,
    });
  };

  if (loading) return <div className="loading">Loading...</div>;
  if (error) return <div className="error">{error}</div>;

  return (
    <div className="data-table-container">
      <div className="table-header">
        <h2>Student Demerit Summary</h2>
        <div className="table-controls">
          <div className="search-container">
            <input
              type="text"
              placeholder="Search students..."
              value={searchTerm}
              onChange={(e) => setSearchTerm(e.target.value)}
              className="search-input"
            />
          </div>
          <button onClick={fetchData} className="refresh-button">
            Refresh
          </button>
        </div>
      </div>

      {filteredSummaries.length > 0 ? (
        <table className="data-table">
          <thead>
            <tr>
              <th>ID</th>
              <th>Student Name</th>
              <th>Grade</th>
              <th>Class</th>
              <th>Total Points</th>
              <th>Most Recent Issue</th>
            </tr>
          </thead>
          <tbody>
            {filteredSummaries.map((summary) => (
              <tr
                key={summary.student_id}
                onClick={() => handleRowClick(summary)}
                style={{ cursor: "pointer" }}
              >
                <td>{summary.student_id}</td>
                <td>{summary.student_name}</td>
                <td>{summary.grade_level || "-"}</td>
                <td>{summary.class_section || "-"}</td>
                <td className={getDemeritClass(summary.total_points)}>
                  {summary.total_points}
                </td>
                <td>{summary.recent_demerit || "-"}</td>
              </tr>
            ))}
          </tbody>
        </table>
      ) : (
        <div className="no-data">
          {searchTerm
            ? "No matching students found"
            : "No demerit records available"}
        </div>
      )}

      {selectedStudent && (
        <StudentDetailModal
          studentId={selectedStudent.id}
          studentName={selectedStudent.name}
          onClose={() => setSelectedStudent(null)}
        />
      )}
    </div>
  );
};
