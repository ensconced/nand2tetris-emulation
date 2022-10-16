import "./styles/reset.css";
import React from "react";
import { createRoot } from "react-dom/client";
import CodeViewer from "./CodeViewer";

import {
  make_computer as makeComputer,
  get_ram_copy,
  tick,
} from "../../web-emulator/pkg/web_emulator";

const rom = new Int16Array(32768);
const computer = makeComputer(rom);
tick(computer);
tick(computer);
tick(computer);
const ram = get_ram_copy(computer.ram);

function getElementById(id: string): HTMLElement {
  const element = document.getElementById(id);
  if (element === null) {
    throw new Error(`failed to find element with id ${id}`);
  }
  return element;
}

function Computer() {
  return (
    <div className="panel-container">
      <code className="code-panel">{ram.join("\n")}</code>
    </div>
  );
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
