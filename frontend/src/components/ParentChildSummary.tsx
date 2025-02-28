import React, { useState, useEffect } from "react";
import "./DataTable.css";
import { StudentDetailModal } from "./StudentDetailModal";
import { useUser } from "../contexts/UserContext";

interface ChildSummary {
  student_id: number;
  student_name: string;
  total_points: number;
  recent_demerit: string | null;
  grade_level: number | null;
  class_section: string | null;
}

export const ParentChildSummary: React.FC = () => {
  const { user } = useUser();
  const [children, setChildren] = useState<ChildSummary[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [selectedChild, setSelectedChild] = useState<{
    id: number;
    name: string;
  } | null>(null);

  const fetchChildrenData = async () => {
    setLoading(true);
    setError(null);
    try {
      // Convert user ID from string to number
      const userId = parseInt(user?.id || "0");

      // Make POST request with user ID
      const response = await fetch(
        "http://localhost:8080/parent_children_summary",
        {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify({ user_id: userId }),
        },
      );

      if (!response.ok) {
        throw new Error("Failed to fetch children data");
      }

      const data = await response.json();
      setChildren(data);
    } catch (err) {
      setError(err instanceof Error ? err.message : "An error occurred");
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    if (user?.id) {
      fetchChildrenData();
    }
  }, [user?.id]);

  // Define class for demerit points severity
  const getDemeritClass = (points: number) => {
    if (points >= 12) return "very-high-demerits";
    if (points >= 6) return "high-demerits";
    if (points >= 3) return "medium-demerits";
    return "good-demerits";
  };

  const handleRowClick = (child: ChildSummary) => {
    setSelectedChild({
      id: child.student_id,
      name: child.student_name,
    });
  };

  if (loading) return <div className="loading">Loading...</div>;
  if (error) return <div className="error">{error}</div>;

  return (
    <div className="data-table-container">
      <div className="table-header">
        <h2>My Children's Demerit Summary</h2>
        <div className="table-controls">
          <button onClick={fetchChildrenData} className="refresh-button">
            Refresh
          </button>
        </div>
      </div>

      {children.length > 0 ? (
        <table className="data-table">
          <thead>
            <tr>
              <th>Child Name</th>
              <th>Grade</th>
              <th>Class</th>
              <th>Total Points</th>
              <th>Most Recent Issue</th>
            </tr>
          </thead>
          <tbody>
            {children.map((child) => (
              <tr
                key={child.student_id}
                onClick={() => handleRowClick(child)}
                style={{ cursor: "pointer" }}
              >
                <td>{child.student_name}</td>
                <td>{child.grade_level || "-"}</td>
                <td>{child.class_section || "-"}</td>
                <td className={getDemeritClass(child.total_points)}>
                  {child.total_points}
                </td>
                <td>{child.recent_demerit || "None"}</td>
              </tr>
            ))}
          </tbody>
        </table>
      ) : (
        <div className="no-data">No children or demerit records found</div>
      )}

      {selectedChild && (
        <StudentDetailModal
          studentId={selectedChild.id}
          studentName={selectedChild.name}
          onClose={() => setSelectedChild(null)}
        />
      )}
    </div>
  );
};
