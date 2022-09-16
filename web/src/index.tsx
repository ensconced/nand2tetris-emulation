import _ from "lodash";
import "./styles/reset.css";
import data from "../debug-output.json";
import { NodeInfo } from "../../bindings/NodeInfo";
import { CompilerResult } from "../../bindings/CompilerResult";
import React, { useMemo, useState } from "react";
import { createRoot } from "react-dom/client";
import CodePanel from "./code-panel";

const compilerResult = data as CompilerResult;
const {
  jack_compiler_result: {
    sourcemaps: jackCompilerSourcemaps,
    tokens: tokensByFilename,
    vm_compiler_inputs: vmCompilerInputs,
  },
  vm_compiler_result: { sourcemap: vmCompilerSourcemap, instructions },
} = compilerResult;

const filenames = Object.keys(tokensByFilename);

function getElementById(id: string): HTMLElement {
  const element = document.getElementById(id);
  if (element === null) {
    throw new Error(`failed to find element with id ${id}`);
  }
  return element;
}

function jackNodeTokens(
  jackNode: NodeInfo | undefined,
  filename: string
): Set<number> {
  if (jackNode === undefined) return new Set();
  const tokenSet: Set<number> = new Set();
  for (let i = jackNode.token_range.start; i < jackNode.token_range.end; i++) {
    const token = tokensByFilename?.[filename]?.[i];
    if (token === undefined) {
      throw new Error("failed to get token");
    }
    tokenSet.add(token.idx);
  }
  return tokenSet;
}

function getJackNodeByIndex(index: FileIdx): NodeInfo {
  const node =
    jackCompilerSourcemaps[index.filename]?.parser_sourcemap.jack_nodes[
      index.idx
    ];
  if (node === undefined) {
    throw new Error(`failed to get jack node at index ${index}`);
  }
  return node;
}

function findInnermostJackNode(tokenIdx: FileIdx): NodeInfo | undefined {
  const tokenJackNodesIdxs =
    jackCompilerSourcemaps[tokenIdx.filename]?.parser_sourcemap
      .token_idx_to_jack_node_idxs[tokenIdx.idx];
  if (!tokenJackNodesIdxs) return undefined;
  const tokenJackNodes = tokenJackNodesIdxs.map((jackNodeIdx) =>
    getJackNodeByIndex({ filename: tokenIdx.filename, idx: jackNodeIdx })
  );
  const smallestJackNode = _.minBy(
    tokenJackNodes,
    (jackNode) => jackNode.token_range.end - jackNode.token_range.start
  );
  if (smallestJackNode === undefined) {
    throw new Error("failed to find jack node for token");
  }
  return smallestJackNode;
}

function immediateVMCommandIdxs(jackNodeIdx: FileIdx): number[] {
  return (
    jackCompilerSourcemaps[jackNodeIdx.filename]?.codegen_sourcemap
      .jack_node_idx_to_vm_command_idx[jackNodeIdx.idx] ?? []
  );
}

function allVMCommandIdxs(jackNodeIdx: FileIdx): number[] {
  return immediateVMCommandIdxs(jackNodeIdx).concat(
    getJackNodeByIndex(jackNodeIdx).child_node_idxs.flatMap((childNodeIdx) =>
      allVMCommandIdxs({ filename: jackNodeIdx.filename, idx: childNodeIdx })
    )
  );
}

interface Props {
  filename: string;
  hidden: boolean;
}

function JackModule({ tokens, hidden }: Props) {
  const tokensWithNewlines = tokens.map((token) => token.source);
  const vmCommandStrings = commands.map((command) => `${command}\n`);

  return (
    <>
      <div style={{ minHeight: 0, display: hidden ? "none" : "flex" }}>
        <CodePanel
          items={tokensWithNewlines}
          hoveredItemIdxs={hoveredTokens}
          mouseSelectedItemIdxs={mouseSelectedTokenIdxs}
          autoSelectedItemIdxs={autoSelectedTokens}
          onSpanMouseEnter={setHoveredTokenIdx}
          onSpanMouseLeave={clearHoverState}
          onSpanClick={(idx) => {
            setMouseSelectedVMCommandIdx(undefined);
            setMouseSelectedJackNode(findInnermostJackNode(idx));
          }}
        />
        <CodePanel
          items={vmCommandStrings}
          hoveredItemIdxs={hoveredVMCommands}
          mouseSelectedItemIdxs={mouseSelectedVMCommandIdxs}
          autoSelectedItemIdxs={autoSelectedVMCommands}
          onSpanMouseEnter={setHoveredVMCommandIdx}
          onSpanMouseLeave={clearHoverState}
          onSpanClick={(idx) => {
            setMouseSelectedJackNode(undefined);
            setMouseSelectedVMCommandIdx(idx);
          }}
        />
      </div>
    </>
  );
}

interface FileIdx {
  filename: string;
  idx: number;
}

function App() {
  const [currentFileIdx, setCurrentFileIdx] = useState(0);
  const [hoveredVMCommandIdx, setHoveredVMCommandIdx] = useState<FileIdx>();
  const [hoveredTokenIdx, setHoveredTokenIdx] = useState<FileIdx>();
  const [mouseSelectedJackNode, setMouseSelectedJackNode] =
    useState<NodeInfo>();
  const [mouseSelectedVMCommandIdx, setMouseSelectedVMCommandIdx] =
    useState<FileIdx>();

  function clearHoverState() {
    setHoveredTokenIdx(undefined);
    setHoveredVMCommandIdx(undefined);
  }

  const hoveredJackNode = useMemo<
    { filename: string; node: NodeInfo } | undefined
  >(() => {
    if (hoveredTokenIdx !== undefined) {
      return findInnermostJackNode(hoveredTokenIdx);
    } else if (hoveredVMCommandIdx !== undefined) {
      const jackNodeIdx =
        jackCompilerSourcemaps[hoveredVMCommandIdx.filename]?.codegen_sourcemap
          .vm_command_idx_to_jack_node_idx[hoveredVMCommandIdx.idx];
      if (jackNodeIdx !== undefined) {
        return getJackNodeByIndex({
          filename: hoveredVMCommandIdx.filename,
          idx: jackNodeIdx,
        });
      }
    }
  }, [hoveredTokenIdx, hoveredVMCommandIdx]);

  const hoveredTokens = useMemo(
    () => jackNodeTokens(hoveredJackNode),
    [hoveredJackNode]
  );

  const hoveredVMCommands = useMemo<Set<number>>(() => {
    return hoveredJackNode
      ? new Set(allVMCommandIdxs(hoveredJackNode.index))
      : new Set();
  }, [hoveredJackNode]);

  const autoSelectedVMCommands = useMemo<Set<number>>(() => {
    return mouseSelectedJackNode
      ? new Set(allVMCommandIdxs(mouseSelectedJackNode.index))
      : new Set();
  }, [mouseSelectedJackNode]);

  const autoSelectedJackNodeIdx = useMemo(() => {
    return mouseSelectedVMCommandIdx === undefined
      ? undefined
      : vm_command_idx_to_jack_node_idx[mouseSelectedVMCommandIdx];
  }, [mouseSelectedVMCommandIdx]);

  const autoSelectedJackNode = useMemo(() => {
    return autoSelectedJackNodeIdx === undefined
      ? undefined
      : getJackNodeByIndex(autoSelectedJackNodeIdx);
  }, [autoSelectedJackNodeIdx]);

  const autoSelectedTokens = useMemo<Set<number>>(() => {
    return autoSelectedJackNode
      ? jackNodeTokens(autoSelectedJackNode)
      : new Set();
  }, [autoSelectedJackNode]);

  const mouseSelectedTokenIdxs = useMemo(
    () => jackNodeTokens(mouseSelectedJackNode),
    [mouseSelectedJackNode]
  );

  const mouseSelectedVMCommandIdxs = useMemo<Set<number>>(() => {
    return autoSelectedJackNodeIdx === undefined
      ? new Set()
      : new Set(allVMCommandIdxs(autoSelectedJackNodeIdx));
  }, [autoSelectedJackNodeIdx]);

  return (
    <>
      <fieldset style={{ flex: "0 0 auto" }}>
        {filenames.map((filename, idx) => (
          <>
            <input
              id={`file-${idx}`}
              type="radio"
              name="file-tab"
              checked={currentFileIdx === idx}
              onChange={() => setCurrentFileIdx(idx)}
            />
            <label htmlFor={`file-${idx}`}>{filename}</label>
          </>
        ))}
      </fieldset>
      {filenames.map((filename, idx) => (
        <JackModule filename={filename} hidden={idx !== currentFileIdx} />
      ))}
    </>
  );
}

const root = createRoot(getElementById("root"));
root.render(<App />);
