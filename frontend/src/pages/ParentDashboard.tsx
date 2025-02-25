import { useUser } from "../contexts/UserContext";

export const ParentDashboard = () => {
  const { user } = useUser();
  return (
    <div>
      <h1>Parent Dashboard</h1>
      <p>Welcome, {user?.firstName}!</p>
    </div>
  );
};
