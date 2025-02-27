import { useState } from "react";
import { useUser } from "../contexts/UserContext";
import DataTable from "../components/DataTable";
import { AddDemeritForm, NewDemeritRecord } from "../components/AddDemeritForm";
import "./TeacherDashboard.css";

export const TeacherDashboard = () => {
  const { user } = useUser();
  const [showAddDemerit, setShowAddDemerit] = useState(false);

  const handleAddDemerit = async (demerit: NewDemeritRecord) => {
    try {
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
      // You could add a refresh mechanism here
      window.location.reload(); // Simple way to refresh data
    } catch (error) {
      console.error("Error adding demerit:", error);
      alert(
        `Failed to add demerit: ${error instanceof Error ? error.message : "Unknown error"}`,
      );
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
      </div>

      <DataTable title="Demerits" />

      {showAddDemerit && (
        <AddDemeritForm
          onSubmit={handleAddDemerit}
          onClose={() => setShowAddDemerit(false)}
        />
      )}
    </div>
  );
};
