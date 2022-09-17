import React, { useState } from "react";
import { CompilerResult } from "../../bindings/CompilerResult";
import data from "../debug-output.json";
import CodePanel, { FileIdxs } from "./code-panel";

const compilerResult = data as CompilerResult;
const {
  vm_compiler_result: { instructions, sourcemap },
} = compilerResult;

const instructionsWithNewLines = instructions.map(
  (instruction) => `${instruction}\n`
);

export default function ASMPanel() {
  const [hoveredInstructionIdx, setHoveredInstructionIdx] =
    useState<FileIdxs>();

  return (
    <div style={{ height: "100vh", overflow: "auto", display: "flex" }}>
      <CodePanel
        filename={"asm"}
        items={instructionsWithNewLines}
        hoveredItemIdxs={hoveredInstructionIdx}
        mouseSelectedItemIdxs={undefined}
        autoSelectedItemIdxs={undefined}
        onSpanMouseEnter={(idx) =>
          setHoveredInstructionIdx({ filename: "asm", idxs: new Set([idx]) })
        }
        onSpanMouseLeave={() => setHoveredInstructionIdx(undefined)}
        onSpanClick={(idx) => {
          // setMouseSelectedVMCommandIdx(undefined);
          // setMouseSelectedJackNode(findInnermostJackNode({ filename, idx }));
        }}
      />
    </div>
  );
}
