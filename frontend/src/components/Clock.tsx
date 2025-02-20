import React, { useState, useEffect } from "react";
import "./Clock.css";

interface TimeResponse {
  current_time: string;
}

function Clock() {
  const [time, setTime] = useState<string>("");

  useEffect(() => {
    fetchTime();
    // Update time every second
    const interval = setInterval(() => {
      fetchTime();
    }, 1000);

    // Cleanup interval on component unmount
    return () => clearInterval(interval);
  }, []);

  const fetchTime = async () => {
    try {
      const response = await fetch("http://localhost:8080/time");
      const data: TimeResponse = await response.json();
      setTime(data.current_time);
    } catch (error) {
      console.error("Error fetching time: ", error);
    }
  };

  const formatTime = (time: string) => {
    const date = new Date(time);
    return date.toLocaleTimeString("en-UK", {
      hour: "2-digit",
      minute: "2-digit",
      second: "2-digit",
    });
  };

  return <div className="clock">{formatTime(time)}</div>;
}

export default Clock;
