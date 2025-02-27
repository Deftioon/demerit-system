import React, { useState, useEffect } from "react";
import { useUser } from "../contexts/UserContext";
import "./DataTable.css";
import { DataRecord } from "../types/demerit";
interface Column {
  key: string;
  header: string;
}

interface DataTableProps {
  title: string;
}

// Define interfaces for different types of data records

const DataTable: React.FC<DataTableProps> = ({ title }) => {
  const { user } = useUser();
  const [data, setData] = useState<DataRecord[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [columns, setColumns] = useState<Column[]>([]);

  // Define columns based on user type
  useEffect(() => {
    if (user?.userType === "teacher") {
      setColumns([
        { key: "student_name", header: "Student Name" },
        { key: "category", header: "Category" },
        { key: "points", header: "Points" },
        { key: "date_issued", header: "Date Issued" },
      ]);
    } else if (user?.userType === "student") {
      setColumns([
        { key: "category", header: "Category" },
        { key: "points", header: "Points" },
        { key: "teacher_name", header: "Issued By" },
        { key: "date_issued", header: "Date" },
      ]);
    } else if (user?.userType === "parent") {
      setColumns([
        { key: "student_name", header: "Child Name" },
        { key: "category", header: "Category" },
        { key: "points", header: "Points" },
        { key: "teacher_name", header: "Teacher" },
        { key: "date_issued", header: "Date" },
      ]);
    }
  }, [user?.userType]);

  const fetchData = async () => {
    setLoading(true);
    setError(null);
    try {
      const endpoint = `http://localhost:8080/${user?.userType}_data`;
      const response = await fetch(endpoint, {
        credentials: "include",
      });

      if (!response.ok) {
        throw new Error("Failed to fetch data");
      }

      const jsonData: DataRecord[] = await response.json();
      setData(jsonData);
    } catch (err) {
      setError(err instanceof Error ? err.message : "An error occurred");
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchData();
  }, [user?.userType]);

  if (loading) {
    return <div className="loading">Loading...</div>;
  }

  if (error) {
    return <div className="error">{error}</div>;
  }

  return (
    <div className="data-table-container">
      <div className="data-table-header">
        <h2>{title}</h2>
        <button onClick={fetchData} className="refresh-button">
          Refresh
        </button>
      </div>
      <table className="data-table">
        <thead>
          <tr>
            {columns.map((column) => (
              <th key={column.key}>{column.header}</th>
            ))}
          </tr>
        </thead>
        <tbody>
          {data.map((row, index) => (
            <tr key={index}>
              {columns.map((column) => (
                <td key={`${index}-${column.key}`}>
                  {row[column.key as keyof DataRecord]}
                </td>
              ))}
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
};

export default DataTable;
