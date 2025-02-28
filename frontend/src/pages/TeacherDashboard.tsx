import { useState } from "react";
import { useUser } from "../contexts/UserContext";
import DataTable from "../components/DataTable";
import { AddDemeritForm, NewDemeritRecord } from "../components/AddDemeritForm";
import { DemeritHistory } from "../components/DemeritHistory";
import "./TeacherDashboard.css";
import { StudentDemeritSummary } from "../components/StudentDemeritSummary";

export const TeacherDashboard = () => {
  const { user } = useUser();
  const [showAddDemerit, setShowAddDemerit] = useState(false);
  const [showDemeritHistory, setShowDemeritHistory] = useState(false);
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const [refreshTrigger, setRefreshTrigger] = useState(0);

  const handleAddDemerit = async (demerit: NewDemeritRecord) => {
    if (isSubmitting) return;

    setIsSubmitting(true);
    setError(null);

    try {
      console.log("Submitting demerit record:", demerit);

      // Make sure demerit contains number values, not strings
      const response = await fetch("http://localhost:8080/add_demerit", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        credentials: "include",
        body: JSON.stringify(demerit),
      });

      if (!response.ok) {
        const errorData = await response.json();
        throw new Error(errorData.message || "Failed to add demerit");
      }

      // Close the form and refresh the table
      setShowAddDemerit(false);

      setRefreshTrigger((prev) => prev + 1);
      // You could add a refresh mechanism here
      alert("Demerit added successfully!");
    } catch (error) {
      console.error("Error adding demerit:", error);
      alert(
        `Failed to add demerit: ${error instanceof Error ? error.message : "Unknown error"}`,
      );
    } finally {
      setIsSubmitting(false);
    }
  };

  return (
    <div>
      <h1>Teacher Dashboard</h1>
      <p>Welcome, {user?.username}</p>

      <div className="dashboard-controls">
        <button
          className="add-demerit-button"
          onClick={() => setShowAddDemerit(true)}
        >
          Add Demerit
        </button>
        <button
          className="view-history-button"
          onClick={() => setShowDemeritHistory(true)}
        >
          View Demerit History
        </button>
      </div>

      <StudentDemeritSummary refreshTrigger={refreshTrigger} />

      {showAddDemerit && (
        <AddDemeritForm
          onSubmit={handleAddDemerit}
          onClose={() => setShowAddDemerit(false)}
        />
      )}

      {showDemeritHistory && (
        <DemeritHistory onClose={() => setShowDemeritHistory(false)} />
      )}
    </div>
  );
};
