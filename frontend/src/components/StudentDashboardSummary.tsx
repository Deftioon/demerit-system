import React, { useState, useEffect } from "react";
import { useUser } from "../contexts/UserContext";
import "./DataTable.css";

interface DemeritDetail {
  demerit_id: number;
  category_name: string;
  points: number;
  teacher_name: string;
  description: string;
  date_issued: string;
}

interface StudentSummary {
  total_points: number;
  recent_demerit: string | null;
  grade_level: number | null;
  class_section: string | null;
}

export const StudentDashboardSummary: React.FC = () => {
  const { user } = useUser();
  const [demerits, setDemerits] = useState<DemeritDetail[]>([]);
  const [summary, setSummary] = useState<StudentSummary | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const fetchStudentData = async () => {
      // Make sure we have a user
      if (!user || !user.id) {
        setError("User not authenticated");
        setLoading(false);
        return;
      }

      setLoading(true);
      try {
        // Convert user ID from string to number
        const userId = parseInt(user.id);

        // First, get the student's information using POST
        const studentResponse = await fetch(
          "http://localhost:8080/my_student_info",
          {
            method: "POST",
            headers: {
              "Content-Type": "application/json",
            },
            body: JSON.stringify({ user_id: userId }),
            credentials: "include",
          },
        );

        if (!studentResponse.ok) {
          throw new Error("Failed to fetch student info");
        }

        const studentInfo = await studentResponse.json();

        // Then fetch the detailed demerit history using POST
        const demeritResponse = await fetch(
          "http://localhost:8080/my_demerits",
          {
            method: "POST",
            headers: {
              "Content-Type": "application/json",
            },
            body: JSON.stringify({ user_id: userId }),
            credentials: "include",
          },
        );

        if (!demeritResponse.ok) {
          throw new Error("Failed to fetch demerit records");
        }

        const demeritData = await demeritResponse.json();
        setDemerits(demeritData);

        // Calculate summary data
        const totalPoints = demeritData.reduce(
          (sum: number, demerit: DemeritDetail) => sum + demerit.points,
          0,
        );
        const recentDemerit =
          demeritData.length > 0 ? demeritData[0].category_name : null;

        setSummary({
          total_points: totalPoints,
          recent_demerit: recentDemerit,
          grade_level: studentInfo.grade_level,
          class_section: studentInfo.class_section,
        });
      } catch (err) {
        console.error("Error fetching student data:", err);
        setError(err instanceof Error ? err.message : "An error occurred");
      } finally {
        setLoading(false);
      }
    };

    fetchStudentData();
  }, [user?.id]);
  // Define class for high demerit points
  const getDemeritClass = (points: number) => {
    if (points >= 12) return "very-high-demerits";
    if (points >= 6) return "high-demerits";
    if (points >= 3) return "medium-demerits";
    return "good-demerits";
  };

  if (loading) return <div className="loading">Loading...</div>;
  if (error) return <div className="error">{error}</div>;

  return (
    <div className="data-table-container">
      <div className="table-header">
        <h2>My Demerit Summary</h2>
      </div>

      {/* Summary row - sticky at the top */}
      <div className="student-summary-card">
        <div className="summary-title">Current Status</div>
        <div className="summary-details">
          <div className="summary-item">
            <span>Grade/Class:</span>
            <span>
              {summary?.grade_level || "-"} {summary?.class_section || ""}
            </span>
          </div>
          <div className="summary-item">
            <span>Total Demerits:</span>
            <span className={`${getDemeritClass(summary?.total_points || 0)}`}>
              {summary?.total_points || 0}
            </span>
          </div>
          <div className="summary-item">
            <span>Recent Issue:</span>
            <span>{summary?.recent_demerit || "None"}</span>
          </div>
        </div>
      </div>

      {/* Detailed table */}
      <h3>Demerit History</h3>
      {demerits.length > 0 ? (
        <table className="data-table">
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
        <div className="no-data">No demerit records found</div>
      )}
    </div>
  );
};
