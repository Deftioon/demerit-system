import { useUser } from "../contexts/UserContext";
import "./ParentDashboard.css";
import DataTable from "../components/DataTable";

export const ParentDashboard = () => {
  const { user } = useUser();
  return (
    <div>
      <h1>Parent Dashboard</h1>
      <p>Welcome, {user?.email}!</p>
      <DataTable title="Child's Demerits" />
    </div>
  );
};
