import "./styles/reset.css";
import React from "react";
import { createRoot } from "react-dom/client";
import CodeViewer from "./CodeViewer";
import Computer from "./Computer";

function getElementById(id: string): HTMLElement {
  const element = document.getElementById(id);
  if (element === null) {
    throw new Error(`failed to find element with id ${id}`);
  }
  return element;
}

function App() {
  return (
    <div style={{ display: "flex" }}>
      <CodeViewer />
      <Computer />
    </div>
  );
}

const root = createRoot(getElementById("root"));
root.render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);
