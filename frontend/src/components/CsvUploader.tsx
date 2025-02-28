import React, { useState, useRef } from "react";
import "./CsvUploader.css";

interface CsvUploaderProps {
  onUploadSuccess?: (response: any) => void;
  onUploadError?: (error: Error) => void;
}

export const CsvUploader: React.FC<CsvUploaderProps> = ({
  onUploadSuccess,
  onUploadError,
}) => {
  const [isUploading, setIsUploading] = useState(false);
  const fileInputRef = useRef<HTMLInputElement>(null);

  const handleButtonClick = () => {
    // Trigger the hidden file input click
    if (fileInputRef.current) {
      fileInputRef.current.click();
    }
  };

  const handleFileChange = async (e: React.ChangeEvent<HTMLInputElement>) => {
    const selectedFile = e.target.files?.[0];
    if (!selectedFile) return;

    // Check if it's a CSV file
    if (!selectedFile.name.toLowerCase().endsWith(".csv")) {
      alert("Please select a CSV file");
      return;
    }

    // Automatically start upload when file is selected
    setIsUploading(true);

    const formData = new FormData();
    formData.append("file", selectedFile);

    try {
      const response = await fetch("http://localhost:8080/upload_csv", {
        method: "POST",
        body: formData,
        credentials: "include",
      });

      if (!response.ok) {
        const data = await response.json();
        throw new Error(data.message || "Upload failed");
      }

      const data = await response.json();
      alert("CSV file uploaded successfully!");
      if (onUploadSuccess) {
        onUploadSuccess(data);
      }
    } catch (err) {
      const error = err as Error;
      alert(`Upload failed: ${error.message}`);
      if (onUploadError) {
        onUploadError(error);
      }
    } finally {
      setIsUploading(false);
      // Reset the file input
      if (fileInputRef.current) {
        fileInputRef.current.value = "";
      }
    }
  };

  return (
    <>
      <button
        className="upload-csv-button"
        onClick={handleButtonClick}
        disabled={isUploading}
      >
        {isUploading ? "Uploading..." : "Upload CSV"}
      </button>
      <input
        ref={fileInputRef}
        type="file"
        accept=".csv"
        onChange={handleFileChange}
        style={{ display: "none" }}
      />
    </>
  );
};
