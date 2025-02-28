import { useUser } from "../contexts/UserContext";
import "./ParentDashboard.css";
import { ParentChildSummary } from "../components/ParentChildSummary";

export const ParentDashboard = () => {
  const { user } = useUser();

  return (
    <div>
      <h1>Parent Dashboard</h1>
      <p className="Welcome">Welcome, {user?.firstName || user?.email}!</p>
      <ParentChildSummary />
    </div>
  );
};
