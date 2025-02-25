import { BrowserRouter as Router, Routes, Route } from "react-router-dom";
import { UserProvider } from "./contexts/UserContext";
import AuthForm from "./components/AuthForm";
import { TeacherDashboard } from "./pages/TeacherDashboard";
import { StudentDashboard } from "./pages/StudentDashboard";
import { ParentDashboard } from "./pages/ParentDashboard";
import Clock from "./components/Clock";
import { ProtectedRoute } from "./components/ProtectedRoute";

function App() {
  return (
    <UserProvider>
      <Router>
        <Clock />
        <Routes>
          <Route path="/" element={<AuthForm />} />
          <Route
            path="/teacher"
            element={
              <ProtectedRoute allowedUserType="teacher">
                <TeacherDashboard />
              </ProtectedRoute>
            }
          />
          <Route
            path="/student"
            element={
              <ProtectedRoute allowedUserType="student">
                <StudentDashboard />
              </ProtectedRoute>
            }
          />
          <Route
            path="/parent"
            element={
              <ProtectedRoute allowedUserType="parent">
                <ParentDashboard />
              </ProtectedRoute>
            }
          />
        </Routes>
      </Router>
    </UserProvider>
  );
}

export default App;
