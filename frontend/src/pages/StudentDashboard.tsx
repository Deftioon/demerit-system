import DataTable from "../components/DataTable";
import { useUser } from "../contexts/UserContext";

export const StudentDashboard = () => {
  const { user } = useUser();
  return (
    <div>
      <h1>Student Dashboard</h1>
      <p>Welcome, {user?.firstName}!</p>
      <DataTable title="My Demerits" />
    </div>
  );
};
