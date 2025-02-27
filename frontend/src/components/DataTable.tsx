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
  refreshTrigger?: number;
}

// Define interfaces for different types of data records

const DataTable: React.FC<DataTableProps> = ({ title, refreshTrigger = 0 }) => {
  const { user } = useUser();
  const [data, setData] = useState<DataRecord[]>([]);
  const [filteredData, setFilteredData] = useState<DataRecord[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [columns, setColumns] = useState<Column[]>([]);
  const [searchTerm, setSearchTerm] = useState("");

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
      console.log(`Fetching data from: ${endpoint}`);

      const response = await fetch(endpoint, {
        credentials: "include",
      });

      if (!response.ok) {
        throw new Error("Failed to fetch data");
      }

      const jsonData: DataRecord[] = await response.json();
      console.log("Received data:", jsonData);

      setData(jsonData);
      setFilteredData(jsonData);
    } catch (err) {
      console.error("Error fetching data:", err);
      setError(err instanceof Error ? err.message : "An error occurred");
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    if (searchTerm.trim() === "") {
      setFilteredData(data);
    } else {
      const term = searchTerm.toLowerCase();
      const filtered = data.filter((record) => {
        // Search in all string fields of the record
        return Object.entries(record).some(([key, value]) => {
          // Only search in string values
          if (typeof value === "string") {
            return value.toLowerCase().includes(term);
          }
          return false;
        });
      });
      setFilteredData(filtered);
    }
  }, [data, searchTerm]);

  useEffect(() => {
    fetchData();
  }, [user?.userType, refreshTrigger]);

  if (loading) {
    return <div className="loading">Loading...</div>;
  }

  if (error) {
    return <div className="error">{error}</div>;
  }

  return (
    <div className="data-table-container">
      <div className="table-header">
        <h2>{title}</h2>
        <div className="table-controls">
          <div className="search-container">
            <input
              type="text"
              placeholder="Search records..."
              value={searchTerm}
              onChange={(e) => setSearchTerm(e.target.value)}
              className="search-input"
            />
          </div>
          <button onClick={fetchData} className="refresh-button">
            Refresh
          </button>
        </div>
      </div>

      {/* Show helpful message when data is empty */}
      {filteredData.length === 0 && !loading && !error && (
        <div className="no-data">
          {searchTerm
            ? "No matching records found"
            : "No records available yet"}
        </div>
      )}

      {filteredData.length > 0 && (
        <table className="data-table">
          <thead>
            <tr>
              {columns.map((column) => (
                <th key={column.key}>{column.header}</th>
              ))}
            </tr>
          </thead>
          <tbody>
            {filteredData.map((row, index) => (
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
      )}
    </div>
  );
};

export default DataTable;
