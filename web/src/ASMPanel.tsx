import React, { useMemo } from "react";
import { FileIdx } from ".";
import { CompilerResult } from "../../bindings/CompilerResult";
import data from "../debug-output.json";
import CodePanel, { InteractedInstructionIdxs } from "./code-panel";

const compilerResult = data as CompilerResult;
const {
  vm_compiler_result: { instructions },
} = compilerResult;

const instructionsWithNewLines = instructions.map(
  (instruction) => `${instruction}\n`
);

interface Props {
  directlyHoveredInstructionIdx: number | undefined;
  hoveredInstructionIdxs: Set<number>;
  selectedInstructionIdxs: InteractedInstructionIdxs;
  setDirectlyHoveredInstructionIdx: (idx: number | undefined) => void;
  setDirectlySelectedInstructionIdx: (idx: number | undefined) => void;
  setDirectlySelectedToken: (idx: FileIdx | undefined) => void;
  setDirectlySelectedVMCommand: (idx: FileIdx | undefined) => void;
}

export default function ASMPanel({
  selectedInstructionIdxs,
  hoveredInstructionIdxs,
  setDirectlyHoveredInstructionIdx,
  setDirectlySelectedInstructionIdx,
  setDirectlySelectedToken,
  setDirectlySelectedVMCommand,
}: Props) {
  const filename = "asm";
  const hoveredItemIdxs = { filename, idxs: hoveredInstructionIdxs };

  const selectedItemIdxs = useMemo(() => {
    return selectedInstructionIdxs && { ...selectedInstructionIdxs, filename };
  }, [selectedInstructionIdxs]);

  return (
    <div style={{ height: "100vh", overflow: "auto", display: "flex" }}>
      <CodePanel
        id={`${filename}`}
        filename={filename}
        items={instructionsWithNewLines}
        hoveredItemIdxs={hoveredItemIdxs}
        selectedItemIdxs={selectedItemIdxs}
        onSpanMouseEnter={(idx) => setDirectlyHoveredInstructionIdx(idx)}
        onSpanMouseLeave={() => setDirectlyHoveredInstructionIdx(undefined)}
        onSpanClick={(idx) => {
          setDirectlySelectedToken(undefined);
          setDirectlySelectedVMCommand(undefined);
          setDirectlySelectedInstructionIdx(idx);
        }}
      />
    </div>
  );
}
