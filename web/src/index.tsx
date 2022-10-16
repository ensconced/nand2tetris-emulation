import "./styles/reset.css";
import React from "react";
import { createRoot } from "react-dom/client";
import CodeViewer from "./CodeViewer";

import {
  make_computer as makeComputer,
  tick,
} from "../../web-emulator/pkg/web_emulator";

const rom = new Int16Array(32768);
const computer = makeComputer(rom);
tick(computer);
console.log(computer.cpu.pc);
tick(computer);
console.log(computer.cpu.pc);
tick(computer);
console.log(computer.cpu.pc);

function getElementById(id: string): HTMLElement {
  const element = document.getElementById(id);
  if (element === null) {
    throw new Error(`failed to find element with id ${id}`);
  }
  return element;
}

function Computer() {
  console.log(computer);
  return <div>hello i am the ram</div>;
}

function App() {
  return (
    <div style={{ display: "flex" }}>
      <CodeViewer />;
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
