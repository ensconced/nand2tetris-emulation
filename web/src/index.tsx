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

import data from "../debug-output.json";
import { CompilerResult } from "../bindings/CompilerResult";

const compilerResult = data as CompilerResult;
const {
  assembly_result: { instructions },
} = compilerResult;

const rom = new Uint16Array(instructions);
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
    const ram = get_formatted_ram(computer.ram);
    const result: string[] = [];
    ram.forEach((word) => {
      result.push(
        word.toString(wordDisplayBaseIdx === 0 ? 2 : 10).padStart(16, "0")
      );
    });
    return result;
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
