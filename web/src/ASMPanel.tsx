import React, { useMemo, useState } from "react";
import { CompilerResult } from "../bindings/CompilerResult";
import data from "../debug-output.json";
import CodePanel, { InteractedInstructionIdxs } from "./CodePanel";
import { FileIdx } from "./types";
import _ from "lodash";

const compilerResult = data as CompilerResult;
const {
  vm_compiler_result: { instructions },
  assembly_result,
} = compilerResult;

const instructionsWithNewLines = instructions.map(
  (instruction) => `${instruction}\n`
);

import computer from "./computer-setup";
import {
  tick,
  tick_to_some_breakpoint,
} from "../../web-emulator/pkg/web_emulator";

interface Props {
  directlyHoveredInstructionIdx: number | undefined;
  hoveredInstructionIdxs: Set<number>;
  selectedInstructionIdxs: InteractedInstructionIdxs;
  setDirectlyHoveredInstructionIdx: (idx: number | undefined) => void;
  setDirectlySelectedInstructionIdx: (idx: number | undefined) => void;
  setDirectlySelectedToken: (idx: FileIdx | undefined) => void;
  setDirectlySelectedVMCommand: (idx: FileIdx | undefined) => void;
  programCounter: number;
  setProgramCounter: (programCounter: number) => void;
}

export default function ASMPanel({
  selectedInstructionIdxs,
  hoveredInstructionIdxs,
  setDirectlyHoveredInstructionIdx,
  setDirectlySelectedInstructionIdx,
  setDirectlySelectedToken,
  setDirectlySelectedVMCommand,
  programCounter,
  setProgramCounter,
}: Props) {
  const filename = "asm";
  const hoveredItemIdxs = { filename, idxs: hoveredInstructionIdxs };

  const selectedItemIdxs = useMemo(() => {
    return selectedInstructionIdxs && { ...selectedInstructionIdxs, filename };
  }, [selectedInstructionIdxs]);

  const currentASMInstructionIdx = useMemo(() => {
    const asmInstructionIdx =
      assembly_result.sourcemap.machine_code_to_asm[programCounter];
    if (asmInstructionIdx === undefined) {
      throw new Error("failed to find current ASM instruction");
    }
    return asmInstructionIdx;
  }, [programCounter]);

  const [breakpoints, setBreakpoints] = useState<Record<number, boolean>>({});

  return (
    <div className="panel-container">
      <fieldset>
        <button
          onClick={() => {
            tick(computer);
            setProgramCounter(computer.cpu.pc);
          }}
        >
          tick
        </button>
        <button
          onClick={() => {
            tick_to_some_breakpoint(
              computer,
              new Uint16Array(
                Object.entries(breakpoints)
                  .filter(([, on]) => on)
                  .map(
                    ([idx]) =>
                      assembly_result.sourcemap.asm_to_machine_code[
                        parseInt(idx, 10)
                      ]!
                  )
              )
            );
            setProgramCounter(computer.cpu.pc);
          }}
        >
          play
        </button>
      </fieldset>
      <CodePanel
        id={`${filename}`}
        filename={filename}
        items={instructionsWithNewLines}
        hoveredItemIdxs={hoveredItemIdxs}
        selectedItemIdxs={selectedItemIdxs}
        currentIdx={currentASMInstructionIdx}
        onSpanMouseEnter={(idx) => setDirectlyHoveredInstructionIdx(idx)}
        onSpanMouseLeave={() => setDirectlyHoveredInstructionIdx(undefined)}
        onSpanClick={(idx) => {
          setDirectlySelectedToken(undefined);
          setDirectlySelectedVMCommand(undefined);
          setDirectlySelectedInstructionIdx(idx);
        }}
        breakpoints={breakpoints}
        setBreakpoints={(newBreakpoints) => {
          setBreakpoints(
            _.pickBy(newBreakpoints, (_on, asmIndex) => {
              return (
                assembly_result.sourcemap.asm_to_machine_code[
                  parseInt(asmIndex, 10)
                ] !== undefined
              );
            })
          );
        }}
      />
      <code className="footer">
        <span className="footer-item" style={{ color: "#8be9fd" }}>
          total: {instructions.length}
        </span>
        <span className="footer-item" style={{ color: "#ff79c6" }}>
          selected: {selectedItemIdxs?.idxs.size ?? 0}
        </span>
      </code>
    </div>
  );
}
