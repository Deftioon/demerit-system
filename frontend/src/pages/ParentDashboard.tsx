import { useUser } from "../contexts/UserContext";
import "./ParentDashboard.css";

export const ParentDashboard = () => {
  const { user } = useUser();
  return (
    <div>
      <h1>Parent Dashboard</h1>
      <div className="welcome">
        <h3>Welcome, {user?.email}!</h3>
      </div>
    </div>
  );
};
