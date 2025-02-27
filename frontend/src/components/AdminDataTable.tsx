import React, { useState, useEffect } from "react";
import "./DataTable.css";
import type { AdminUserRecord } from "../types/demerit";
import { EditUserModal } from "./EditUserModal";

export const AdminDataTable: React.FC = () => {
  const [selectedUser, setSelectedUser] = useState<AdminUserRecord | null>(
    null,
  );
  const [showEditModal, setShowEditModal] = useState(false);
  const openEditModal = (user: AdminUserRecord) => {
    setSelectedUser(user);
    setShowEditModal(true);
  };
  const [users, setUsers] = useState<AdminUserRecord[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [filter, setFilter] = useState<string>("all");

  const [isUpdating, setIsUpdating] = useState<number | null>(null);
  const userTypes = ["admin", "teacher", "student", "parent"];

  const handleRoleChange = async (userId: number, newRole: string) => {
    setIsUpdating(userId);
    try {
      // Find the current user data
      const user = users.find((u) => u.user_id === userId);
      if (!user) {
        throw new Error("User not found");
      }

      console.log("Found user:", user);

      // Create the update payload with minimal required fields
      const updatePayload = {
        user_id: userId,
        username: user.username,
        email: user.email,
        user_type: newRole,
        first_name: user.first_name,
        last_name: user.last_name,
        total_demerits: user.total_demerits || 0,
        created_at: user.created_at || new Date().toISOString(), // Add the created_at field
        grade_level: user.grade_level,
        class_section: user.class_section,
      };

      console.log("Sending update payload:", updatePayload);

      const response = await fetch("http://localhost:8080/update_user", {
        method: "PUT",
        headers: {
          "Content-Type": "application/json",
        },
        credentials: "include",
        body: JSON.stringify(updatePayload),
      });

      // Read the response text first
      const responseText = await response.text();
      console.log("Raw response:", responseText);

      // Then parse it if not empty
      let data;
      if (responseText) {
        try {
          data = JSON.parse(responseText);
          console.log("Parsed response:", data);
        } catch (e) {
          console.error("Error parsing JSON:", e);
        }
      }

      if (!response.ok) {
        throw new Error(
          data?.message || `Request failed with status ${response.status}`,
        );
      }

      console.log("Role updated successfully");
      await fetchUsers(); // Refresh the data
    } catch (err) {
      console.error("Error updating role:", err);
      setError(err instanceof Error ? err.message : "Failed to update role");
      alert(
        `Failed to update role: ${err instanceof Error ? err.message : "Unknown error"}`,
      );
    } finally {
      setIsUpdating(null);
    }
  };

  const fetchUsers = async () => {
    setLoading(true);
    try {
      // Add cache-busting
      const response = await fetch(
        `http://localhost:8080/admin_data?t=${new Date().getTime()}`,
        {
          credentials: "include",
          headers: {
            "Cache-Control": "no-cache",
            Pragma: "no-cache",
          },
        },
      );

      if (!response.ok) throw new Error("Failed to fetch data");

      const data = await response.json();
      console.log("FETCHED ADMIN DATA:", JSON.stringify(data, null, 2));
      setUsers(data);
      // // Force a complete state replacement
      // setUsers((prevUsers) => {
      //   console.log("Previous users:", prevUsers);
      //   console.log("New users:", data);
      //   return [...data];
      // });
    } catch (err) {
      setError(err instanceof Error ? err.message : "An error occurred");
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchUsers();
  }, []);

  if (loading) return <div className="loading">Loading...</div>;
  if (error) return <div className="error">{error}</div>;

  const filteredUsers =
    filter === "all"
      ? users
      : users.filter((user) => user.user_type === filter);

  const getDemeritClass = (demerits: number) => {
    return demerits >= 6 ? "high-demerits" : "";
  };

  const renderGradeLevel = (user: AdminUserRecord) => {
    console.log(
      `User ${user.user_id} (${user.user_type}) grade_level:`,
      user.grade_level,
    );

    // Only display grade level for students and make sure it's displayed properly
    return user.user_type === "student"
      ? user.grade_level !== null && user.grade_level !== undefined
        ? user.grade_level
        : "-"
      : "-";
  };

  const renderClassSection = (user: AdminUserRecord) => {
    console.log(
      `User ${user.user_id} (${user.user_type}) class_section:`,
      user.class_section,
    );

    // Only display class section for students and make sure it's displayed properly
    return user.user_type === "student"
      ? user.class_section !== null && user.class_section !== undefined
        ? user.class_section
        : "-"
      : "-";
  };

  return (
    <div className="admin-table-container">
      <div className="admin-controls">
        <h2>User Management</h2>
        <div className="filter-controls">
          <select
            value={filter}
            onChange={(e) => setFilter(e.target.value)}
            className="filter-select"
          >
            <option value="all">All Users</option>
            <option value="student">Students</option>
            <option value="teacher">Teachers</option>
            <option value="parent">Parents</option>
          </select>
          <button onClick={fetchUsers} className="refresh-button">
            ↻ Refresh
          </button>
        </div>
      </div>

      <table className="data-table">
        <thead>
          <tr>
            <th>ID</th>
            <th>Name</th>
            <th>Username</th>
            <th>Email</th>
            <th>Role</th>
            <th>Grade</th>
            <th>Class</th>
            <th>Demerits</th>
            <th>Created</th>
            <th>Children</th>
          </tr>
        </thead>
        <tbody>
          {filteredUsers.map((user) => (
            <tr
              key={user.user_id}
              onClick={() => openEditModal(user)}
              style={{ cursor: "pointer" }}
            >
              <td>{user.user_id}</td>
              <td>{`${user.first_name} ${user.last_name}`}</td>
              <td>{user.username}</td>
              <td>{user.email}</td>
              <td>
                <div className={`role-badge-container ${user.user_type}`}>
                  <div className={`role-badge ${user.user_type}`}>
                    {user.user_type}
                  </div>
                  <select
                    className="role-select"
                    value={user.user_type}
                    onChange={(e) =>
                      handleRoleChange(user.user_id, e.target.value)
                    }
                    disabled={isUpdating === user.user_id}
                  >
                    {userTypes.map((type) => (
                      <option
                        key={type}
                        value={type}
                        className={`role-option ${type}`}
                      >
                        {type}
                      </option>
                    ))}
                  </select>
                  {isUpdating === user.user_id && (
                    <span className="updating-spinner">⟳</span>
                  )}
                </div>
              </td>
              <td>{renderGradeLevel(user)}</td>
              <td>{renderClassSection(user)}</td>
              <td className={getDemeritClass(user.total_demerits)}>
                {user.total_demerits}
              </td>
              {/* TODO: Is this date handling correct? */}
              <td>{new Date(user.created_at).toLocaleDateString()}</td>
              <td>
                {user.user_type === "parent" &&
                user.children &&
                user.children.length > 0
                  ? user.children.map((child) => child.name).join(", ")
                  : "-"}
              </td>
            </tr>
          ))}
        </tbody>
      </table>

      {users.length === 0 && !loading && !error && (
        <div className="no-data">No users found</div>
      )}

      {showEditModal && selectedUser && (
        <EditUserModal
          user={selectedUser}
          onClose={() => {
            setShowEditModal(false);
            setSelectedUser(null);
          }}
          onSave={async (updatedUser) => {
            // Call an endpoint to update full user details
            const response = await fetch("http://localhost:8080/update_user", {
              method: "PUT",
              headers: {
                "Content-Type": "application/json",
              },
              credentials: "include",
              body: JSON.stringify(updatedUser),
            });
            if (!response.ok) {
              const data = await response.json();
              throw new Error(data.message || "Failed to update user");
            }
            if (response.headers.get("Content-Length") === "0") {
              throw new Error("Empty response received");
            }
            const data = await response.json();
            console.log("User updated successfully:", data);

            await fetchUsers(); // Refresh data
            setShowEditModal(false);
            setSelectedUser(null);
          }}
        />
      )}
    </div>
  );
};
