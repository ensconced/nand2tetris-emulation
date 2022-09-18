import React from "react";
import { CompilerResult } from "../../bindings/CompilerResult";
import data from "../debug-output.json";
import CodePanel from "./code-panel";

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
  setDirectlyHoveredInstructionIdx: (idx: number | undefined) => void;
}

export default function ASMPanel({
  hoveredInstructionIdxs,
  setDirectlyHoveredInstructionIdx,
}: Props) {
  const filename = "asm";
  const hoveredItemIdxs = { filename, idxs: hoveredInstructionIdxs };

  return (
    <div style={{ height: "100vh", overflow: "auto", display: "flex" }}>
      <CodePanel
        filename={filename}
        items={instructionsWithNewLines}
        hoveredItemIdxs={hoveredItemIdxs}
        mouseSelectedItemIdxs={undefined}
        autoSelectedItemIdxs={undefined}
        onSpanMouseEnter={(idx) => setDirectlyHoveredInstructionIdx(idx)}
        onSpanMouseLeave={() => setDirectlyHoveredInstructionIdx(undefined)}
        onSpanClick={(idx) => {
          // setMouseSelectedVMCommandIdx(undefined);
          // setMouseSelectedJackNode(findInnermostJackNode({ filename, idx }));
        }}
      />
    </div>
  );
}
