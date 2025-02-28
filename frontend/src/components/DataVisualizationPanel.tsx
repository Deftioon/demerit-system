import React, { useState, useEffect } from "react";
import "./DataVisualizationPanel.css";
import { Bar, Pie, Line } from "react-chartjs-2";
import {
  Chart as ChartJS,
  CategoryScale,
  LinearScale,
  BarElement,
  Title,
  Tooltip,
  Legend,
  ArcElement,
  LineElement,
  PointElement,
  TimeScale,
} from "chart.js";
import "chartjs-adapter-date-fns";

// Register Chart.js components
ChartJS.register(
  CategoryScale,
  LinearScale,
  BarElement,
  ArcElement,
  LineElement,
  PointElement,
  TimeScale,
  Title,
  Tooltip,
  Legend,
);

interface DemeritTimePoint {
  date: string;
  count: number;
}

interface DataVisualizationPanelProps {
  onClose: () => void;
}

interface DemeritCategoryCount {
  category_name: string;
  count: number;
}

interface GradeDemeritCount {
  grade: number;
  count: number;
}

interface DemeritDistribution {
  categories: DemeritCategoryCount[];
  grades: GradeDemeritCount[];
}

export const DataVisualizationPanel: React.FC<DataVisualizationPanelProps> = ({
  onClose,
}) => {
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [categoryData, setCategoryData] = useState<DemeritCategoryCount[]>([]);
  const [gradeData, setGradeData] = useState<GradeDemeritCount[]>([]);
  const [trendData, setTrendData] = useState<DemeritTimePoint[]>([]);

  useEffect(() => {
    // Fetch data for the visualizations
    const fetchData = async () => {
      try {
        setLoading(true);
        setError(null);

        // Fetch both distribution and trend data
        const [distributionResponse, trendResponse] = await Promise.all([
          fetch("http://localhost:8080/demerit_distribution", {
            credentials: "include",
          }),
          fetch("http://localhost:8080/demerit_trend", {
            credentials: "include",
          }),
        ]);

        if (!distributionResponse.ok) {
          throw new Error("Failed to fetch demerit distribution data");
        }

        if (!trendResponse.ok) {
          throw new Error("Failed to fetch demerit trend data");
        }

        const distributionData = await distributionResponse.json();
        const trendData = await trendResponse.json();

        setCategoryData(distributionData.categories);
        setGradeData(distributionData.grades);
        setTrendData(trendData);

        setLoading(false);
      } catch (err) {
        console.error("Error fetching visualization data:", err);
        setError(err instanceof Error ? err.message : "An error occurred");
        setLoading(false);
      }
    };

    fetchData();
  }, []);

  // For the bar chart (grade distribution)
  const gradeChartData = {
    labels: gradeData.map((item) => `Grade ${item.grade}`),
    datasets: [
      {
        label: "Demerit Points by Grade Level",
        data: gradeData.map((item) => item.count),
        backgroundColor: "rgba(75, 192, 192, 0.8)",
        borderColor: "rgba(75, 192, 192, 1)",
        borderWidth: 1,
      },
    ],
  };

  // For the pie chart (category distribution)
  const categoryChartData = {
    labels: categoryData.map((item) => item.category_name),
    datasets: [
      {
        label: "Demerits by Category",
        data: categoryData.map((item) => item.count),
        backgroundColor: [
          "rgba(255, 99, 132, 0.8)",
          "rgba(54, 162, 235, 0.8)",
          "rgba(255, 206, 86, 0.8)",
          "rgba(75, 192, 192, 0.8)",
          "rgba(153, 102, 255, 0.8)",
          "rgba(255, 159, 64, 0.8)",
          "rgba(201, 203, 207, 0.8)", // Additional colors for more categories
          "rgba(255, 159, 64, 0.8)",
          "rgba(153, 102, 255, 0.8)",
          "rgba(75, 192, 192, 0.8)",
        ],
        borderColor: [
          "rgba(255, 99, 132, 1)",
          "rgba(54, 162, 235, 1)",
          "rgba(255, 206, 86, 1)",
          "rgba(75, 192, 192, 1)",
          "rgba(153, 102, 255, 1)",
          "rgba(255, 159, 64, 1)",
          "rgba(201, 203, 207, 1)",
          "rgba(255, 159, 64, 1)",
          "rgba(153, 102, 255, 1)",
          "rgba(75, 192, 192, 1)",
        ],
        borderWidth: 1,
      },
    ],
  };

  const trendChartData = {
    labels: trendData.map((item) => item.date),
    datasets: [
      {
        label: "Demerits Over Time",
        data: trendData.map((item) => item.count),
        borderColor: "rgba(153, 102, 255, 1)",
        backgroundColor: "rgba(153, 102, 255, 0.2)",
        borderWidth: 2,
        fill: true,
        tension: 0.4,
        pointBackgroundColor: "rgba(153, 102, 255, 1)",
        pointRadius: 3,
        pointHoverRadius: 5,
      },
    ],
  };

  const lineOptions = {
    responsive: true,
    maintainAspectRatio: false,
    plugins: {
      legend: {
        position: "top" as const,
        labels: {
          color: "#fff",
        },
      },
      title: {
        display: true,
        text: "Demerit Points Trend Over Time",
        color: "#fff",
      },
      tooltip: {
        callbacks: {
          title: function (tooltipItems: any) {
            // Format the date for the tooltip
            return new Date(tooltipItems[0].label).toLocaleDateString();
          },
        },
      },
    },
    scales: {
      y: {
        beginAtZero: true,
        ticks: {
          color: "#fff",
          precision: 0,
        },
        grid: {
          color: "rgba(255, 255, 255, 0.1)",
        },
        title: {
          display: true,
          text: "Number of Demerits",
          color: "#fff",
        },
      },
      x: {
        ticks: {
          color: "#fff",
        },
        grid: {
          color: "rgba(255, 255, 255, 0.1)",
        },
        title: {
          display: true,
          text: "Date",
          color: "#fff",
        },
      },
    },
  };

  const barOptions = {
    responsive: true,
    maintainAspectRatio: false,
    plugins: {
      legend: {
        position: "top" as const,
        labels: {
          color: "#fff", // Make legend text white
        },
      },
      title: {
        display: true,
        text: "Demerit Distribution by Grade Level",
        color: "#fff", // Make title text white
      },
    },
    scales: {
      y: {
        beginAtZero: true,
        ticks: {
          color: "#fff", // Make y-axis labels white
        },
        grid: {
          color: "rgba(255, 255, 255, 0.1)", // Subtle grid lines
        },
      },
      x: {
        ticks: {
          color: "#fff", // Make x-axis labels white
        },
        grid: {
          color: "rgba(255, 255, 255, 0.1)", // Subtle grid lines
        },
      },
    },
  };

  const pieOptions = {
    responsive: true,
    maintainAspectRatio: false,
    plugins: {
      legend: {
        position: "right" as const,
        labels: {
          color: "#fff", // Make legend text white
          padding: 20,
          font: {
            size: 12,
          },
        },
      },
      title: {
        display: true,
        text: "Demerit Categories Distribution",
        color: "#fff", // Make title text white
      },
    },
  };

  return (
    <div className="modal-overlay">
      <div className="modal-content visualization-panel">
        <button className="floating-close-button" onClick={onClose}>
          Close
        </button>

        <h2>Demerit Data Visualization</h2>

        {loading ? (
          <div className="loading-container">
            <div className="loading-spinner"></div>
            <p>Loading data...</p>
          </div>
        ) : error ? (
          <div className="error-message">
            <p>Error: {error}</p>
            <button
              onClick={() => window.location.reload()}
              className="retry-button"
            >
              Retry
            </button>
          </div>
        ) : (
          <div className="charts-container">
            {gradeData.length > 0 ? (
              <div className="chart-wrapper">
                <h3>Demerits by Grade Level</h3>
                <div className="chart">
                  <Bar data={gradeChartData} options={barOptions} />
                </div>
              </div>
            ) : (
              <div className="chart-wrapper empty-chart">
                <h3>Demerits by Grade Level</h3>
                <p className="no-data-message">No grade data available</p>
              </div>
            )}

            {categoryData.length > 0 ? (
              <div className="chart-wrapper">
                <h3>Demerits by Category</h3>
                <div className="chart pie-chart">
                  <Pie data={categoryChartData} options={pieOptions} />
                </div>
              </div>
            ) : (
              <div className="chart-wrapper empty-chart">
                <h3>Demerits by Category</h3>
                <p className="no-data-message">No category data available</p>
              </div>
            )}

            {trendData.length > 0 ? (
              <div className="chart-wrapper full-width">
                <h3>Demerit Points Trend Over Time</h3>
                <div className="chart">
                  <Line data={trendChartData} options={lineOptions} />
                </div>
              </div>
            ) : (
              <div className="chart-wrapper full-width empty-chart">
                <h3>Demerit Points Trend Over Time</h3>
                <p className="no-data-message">No trend data available</p>
              </div>
            )}
          </div>
        )}

        <button onClick={onClose} className="close-button">
          Close
        </button>
      </div>
    </div>
  );
};
