import React from "react";
import AuthForm from "./components/AuthForm";
import Clock from "./components/Clock";
import "./App.css";

function onAuthSuccess() {
  return;
}

function App() {
  return (
    <>
      <Clock />
      <div className="main">
        <div className="title">
          <h1>Demerit System</h1>
          <h3>DSI Demerit Point Management</h3>
        </div>
        <div className="app">
          <AuthForm onAuthSuccess={onAuthSuccess} />
        </div>
      </div>
    </>
  );
}

export default App;
