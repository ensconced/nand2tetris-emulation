import "./styles/reset.css";
import React, { useCallback, useMemo, useState } from "react";
import { createRoot } from "react-dom/client";
import CodeViewer from "./CodeViewer";
import Computer from "./Computer";
import computer from "./computer-setup";

function getElementById(id: string): HTMLElement {
  const element = document.getElementById(id);
  if (element === null) {
    throw new Error(`failed to find element with id ${id}`);
  }
  return element;
}

function App() {
  const [programCounter, setProgramCounter] = useState(0);

  const updateProgramCounter = useCallback((programCounter: number) => {
    setProgramCounter(programCounter);
  }, []);

  const ram = useMemo(() => {
    return computer.ram;
  }, [programCounter]);

  return (
    <div style={{ display: "flex" }}>
      <CodeViewer
        setProgramCounter={updateProgramCounter}
        programCounter={programCounter}
      />
      <Computer ram={ram} />
    </div>
  );
}

const root = createRoot(getElementById("root"));
root.render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);
