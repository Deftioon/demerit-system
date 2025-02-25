import { useUser } from "../contexts/UserContext";

export const TeacherDashboard = () => {
  const { user } = useUser();
  return (
    <div>
      <h1>Teacher Dashboard</h1>
      <p>Welcome, {user?.username}</p>
    </div>
  );
};
