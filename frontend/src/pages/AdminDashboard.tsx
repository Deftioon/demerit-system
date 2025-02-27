import { useUser } from "../contexts/UserContext";
import { AdminDataTable } from "../components/AdminDataTable";

export const AdminDashboard = () => {
  const { user } = useUser();

  return (
    <div>
      <h1>Admin Dashboard</h1>
      <p>
        Welcome, {user?.firstName} {user?.lastName}
      </p>
      <AdminDataTable />
    </div>
  );
};
