import "./styles/reset.css";
import React, { useCallback, useMemo, useState } from "react";
import { createRoot } from "react-dom/client";
import CodeViewer from "./CodeViewer";
import Computer from "./Computer";

import {
  make_computer as makeComputer,
  get_formatted_ram,
  tick,
} from "../../web-emulator/pkg/web_emulator";

const rom = new Int16Array(32768);
const computer = makeComputer(rom);

function getElementById(id: string): HTMLElement {
  const element = document.getElementById(id);
  if (element === null) {
    throw new Error(`failed to find element with id ${id}`);
  }
  return element;
}

function App() {
  const [wordDisplayBaseIdx, setWordDisplayBaseIdx] = useState(0);
  const [programCounter, setProgramCounter] = useState(0);

  const ram = useMemo(() => {
    const ramString = get_formatted_ram(computer.ram, wordDisplayBaseIdx);
    return ramString.split(/(?=\n)/);
  }, [wordDisplayBaseIdx, programCounter]);

  const handleTick = useCallback(() => {
    tick(computer);
    setProgramCounter(computer.cpu.pc);
  }, []);

  return (
    <div style={{ display: "flex" }}>
      <CodeViewer onTick={handleTick} programCounter={programCounter} />
      <Computer
        wordDisplayBaseIdx={wordDisplayBaseIdx}
        ram={ram}
        onWordDisplayBaseIdxChange={(idx) => setWordDisplayBaseIdx(idx)}
      />
    </div>
  );
}

const root = createRoot(getElementById("root"));
root.render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);
