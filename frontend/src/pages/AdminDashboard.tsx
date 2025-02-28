import { CsvUploader } from "../components/CsvUploader";
import { useState } from "react";
import { useUser } from "../contexts/UserContext";
import { AdminDataTable } from "../components/AdminDataTable";

export const AdminDashboard = () => {
  const { user } = useUser();
  const [uploadSuccess, setUploadSuccess] = useState(false);
  const [uploadError, setUploadError] = useState<string | null>(null);

  const handleUploadSuccess = (response: any) => {
    console.log("Upload success:", response);
    setUploadSuccess(true);
    setUploadError(null);

    // Clear success message after 3 seconds
    setTimeout(() => setUploadSuccess(false), 3000);
  };

  const handleUploadError = (error: Error) => {
    console.error("Upload error:", error);
    setUploadError(error.message);
    setUploadSuccess(false);
  };

  return (
    <div>
      <h1>Admin Dashboard</h1>
      <p>
        Welcome, {user?.firstName} {user?.lastName}
      </p>

      <div className="admin-dashboard-section">
        <CsvUploader
          onUploadSuccess={handleUploadSuccess}
          onUploadError={handleUploadError}
        />
        {uploadSuccess && (
          <div className="success-notification">
            CSV file uploaded successfully!
          </div>
        )}
      </div>

      <AdminDataTable />
    </div>
  );
};
