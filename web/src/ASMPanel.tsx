import React, { useMemo } from "react";
import { CompilerResult } from "../bindings/CompilerResult";
import data from "../debug-output.json";
import CodePanel, { InteractedInstructionIdxs } from "./code-panel";
import { FileIdx } from "./types";

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
    <div
      style={{
        height: "100vh",
        overflow: "auto",
        display: "flex",
        flexDirection: "column",
      }}
    >
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
      <code className="footer">
        <span style={{ color: "#8be9fd" }}>total: {instructions.length}</span>
        <span style={{ color: "#ff79c6" }}>
          selected: {selectedItemIdxs?.idxs.size ?? 0}
        </span>
      </code>
    </div>
  );
}
