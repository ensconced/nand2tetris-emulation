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

interface Props {
  filename: string;
  hidden: boolean;
}

function JackModule({ filename, hidden }: Props) {
  const [hoveredVMCommandIdx, setHoveredVMCommandIdx] = useState<number>();
  const [hoveredTokenIdx, setHoveredTokenIdx] = useState<number>();
  const [mouseSelectedJackNode, setMouseSelectedJackNode] =
    useState<NodeInfo>();
  const [mouseSelectedVMCommandIdx, setMouseSelectedVMCommandIdx] =
    useState<number>();

  const jackCompilerSourcemap = jackCompilerSourcemaps[filename];
  if (jackCompilerSourcemap === undefined) {
    throw new Error(
      `failed to find jack compiler sourcemap for file ${filename}`
    );
  }

  const tokens = tokensByFilename[filename]!;
  if (tokens === undefined) {
    throw new Error(`failed to find tokens for file ${filename}`);
  }

  const commands = vmCompilerInputs.find(
    (x) => x.filename === filename
  )?.commands;
  if (commands === undefined) {
    throw new Error(`failed to find commands for file ${filename}`);
  }

  const {
    codegen_sourcemap: {
      jack_node_idx_to_vm_command_idx,
      vm_command_idx_to_jack_node_idx,
    },
    parser_sourcemap: { jack_nodes, token_idx_to_jack_node_idxs },
  } = jackCompilerSourcemap;

  function clearHoverState() {
    setHoveredTokenIdx(undefined);
    setHoveredVMCommandIdx(undefined);
  }

  function immediateVMCommandIdxs(jackNodeIdx: number): number[] {
    return jack_node_idx_to_vm_command_idx[jackNodeIdx] ?? [];
  }

  function allVMCommandIdxs(jackNodeIdx: number): number[] {
    return immediateVMCommandIdxs(jackNodeIdx).concat(
      getJackNodeByIndex(jackNodeIdx).child_node_idxs.flatMap((childNodeIdx) =>
        allVMCommandIdxs(childNodeIdx)
      )
    );
  }

  function getJackNodeByIndex(index: number): NodeInfo {
    const node = jack_nodes[index];
    if (node === undefined) {
      throw new Error(`failed to get jack node at index ${index}`);
    }
    return node;
  }

  function findInnermostJackNode(tokenIdx: number): NodeInfo | undefined {
    const tokenJackNodesIdxs = token_idx_to_jack_node_idxs[tokenIdx];
    if (!tokenJackNodesIdxs) return undefined;
    const tokenJackNodes = tokenJackNodesIdxs.map((jackNodeIdx) =>
      getJackNodeByIndex(jackNodeIdx)
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

  function jackNodeTokens(jackNode: NodeInfo | undefined): Set<number> {
    if (jackNode === undefined) return new Set();
    const tokenSet: Set<number> = new Set();
    for (
      let i = jackNode.token_range.start;
      i < jackNode.token_range.end;
      i++
    ) {
      const token = tokens[i];
      if (token === undefined) {
        throw new Error("failed to get token");
      }
      tokenSet.add(token.idx);
    }
    return tokenSet;
  }

  const hoveredJackNode = useMemo<NodeInfo | undefined>(() => {
    if (hoveredTokenIdx !== undefined) {
      return findInnermostJackNode(hoveredTokenIdx);
    } else if (hoveredVMCommandIdx !== undefined) {
      const jackNodeIdx = vm_command_idx_to_jack_node_idx[hoveredVMCommandIdx];
      if (jackNodeIdx !== undefined) {
        return getJackNodeByIndex(jackNodeIdx);
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

function App() {
  const [currentFileIdx, setCurrentFileIdx] = useState(0);

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
