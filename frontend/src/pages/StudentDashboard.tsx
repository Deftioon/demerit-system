import { useState, useEffect } from "react";
import { useUser } from "../contexts/UserContext";
import { StudentDashboardSummary } from "../components/StudentDashboardSummary";

export const StudentDashboard = () => {
  const { user } = useUser();

  if (!user) {
    return (
      <div className="error">
        <h2>Authentication Error</h2>
        <p>You are not logged in. Please return to the login page.</p>
      </div>
    );
  }

  return (
    <div>
      <h1>Student Dashboard</h1>
      <p>Welcome, {user?.firstName || user?.username || user?.email}!</p>
      <StudentDashboardSummary />
    </div>
  );
};
