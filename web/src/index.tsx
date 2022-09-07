import "./styles/reset.css";
import data from "../debug-output.json";
import { DebugOutput } from "../../bindings/DebugOutput";
import _ from "lodash";
import { NodeInfo } from "../../bindings/NodeInfo";
import React, { useMemo, useState } from "react";
import { createRoot } from "react-dom/client";
import CodePanel from "./code-panel";

const debugOutput = data as DebugOutput;
const tokens = debugOutput.tokens.map((token) => token.source);
const vmCommandStrings = debugOutput.vm_commands.map(
  (command) => `${command}\n`
);

function getElementById(id: string): HTMLElement {
  const element = document.getElementById(id);
  if (element === null) {
    throw new Error(`failed to find element with id ${id}`);
  }
  return element;
}

function immediateVMCommandIdxs(
  filename: string,
  jackNodeIdx: number
): number[] {
  return (
    debugOutput.sourcemap.jack_node_idx_to_vm_command_idx[filename]?.[
      jackNodeIdx
    ] ?? []
  );
}

function allVMCommandIdxs(filename: string, jackNodeIdx: number): number[] {
  return immediateVMCommandIdxs(filename, jackNodeIdx).concat(
    getJackNodeByIndex(filename, jackNodeIdx).child_node_idxs.flatMap(
      (childNodeIdx) => allVMCommandIdxs(filename, childNodeIdx)
    )
  );
}

function getJackNodeByIndex(filename: string, index: number): NodeInfo {
  const node = debugOutput.sourcemap.jack_nodes[filename]?.[index];
  if (node === undefined) {
    throw new Error(`failed to get jack node at index ${index}`);
  }
  return node;
}

function findInnermostJackNode(
  filename: string,
  tokenIdx: number
): NodeInfo | undefined {
  const tokenJackNodesIdxs =
    debugOutput.sourcemap.token_idx_to_jack_node_idxs[filename]?.[tokenIdx];
  if (!tokenJackNodesIdxs) return undefined;
  const tokenJackNodes = tokenJackNodesIdxs.map((jackNodeIdx) =>
    getJackNodeByIndex(filename, jackNodeIdx)
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
  const tokens: Set<number> = new Set();
  for (let i = jackNode.token_range.start; i < jackNode.token_range.end; i++) {
    const token = debugOutput.tokens[i];
    if (token === undefined) {
      throw new Error("failed to get token");
    }
    tokens.add(token.idx);
  }
  return tokens;
}

const filename = "test";

function App() {
  const [hoveredVMCommandIdx, setHoveredVMCommandIdx] = useState<number>();
  const [hoveredTokenIdx, setHoveredTokenIdx] = useState<number>();
  const [mouseSelectedJackNode, setMouseSelectedJackNode] =
    useState<NodeInfo>();
  const [mouseSelectedVMCommandIdx, setMouseSelectedVMCommandIdx] =
    useState<number>();

  function clearHoverState() {
    setHoveredTokenIdx(undefined);
    setHoveredVMCommandIdx(undefined);
  }

  const hoveredJackNode = useMemo<NodeInfo | undefined>(() => {
    if (hoveredTokenIdx !== undefined) {
      return findInnermostJackNode(filename, hoveredTokenIdx);
    } else if (hoveredVMCommandIdx !== undefined) {
      const jackNodeIdx =
        debugOutput.sourcemap.vm_command_idx_to_jack_node_idx[filename]?.[
          hoveredVMCommandIdx
        ];
      if (jackNodeIdx !== undefined) {
        return getJackNodeByIndex(filename, jackNodeIdx);
      }
    }
  }, [hoveredTokenIdx, hoveredVMCommandIdx]);

  const hoveredTokens = useMemo(
    () => jackNodeTokens(hoveredJackNode),
    [hoveredJackNode]
  );

  const hoveredVMCommands = useMemo<Set<number>>(() => {
    return hoveredJackNode
      ? new Set(allVMCommandIdxs(filename, hoveredJackNode.index))
      : new Set();
  }, [hoveredJackNode]);

  const autoSelectedVMCommands = useMemo<Set<number>>(() => {
    return mouseSelectedJackNode
      ? new Set(allVMCommandIdxs(filename, mouseSelectedJackNode.index))
      : new Set();
  }, [mouseSelectedJackNode]);

  const autoSelectedJackNodeIdx = useMemo(() => {
    return mouseSelectedVMCommandIdx === undefined
      ? undefined
      : debugOutput.sourcemap.vm_command_idx_to_jack_node_idx[filename]?.[
          mouseSelectedVMCommandIdx
        ];
  }, [mouseSelectedVMCommandIdx]);

  const autoSelectedJackNode = useMemo(() => {
    return autoSelectedJackNodeIdx === undefined
      ? undefined
      : getJackNodeByIndex(filename, autoSelectedJackNodeIdx);
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
      : new Set(allVMCommandIdxs(filename, autoSelectedJackNodeIdx));
  }, [autoSelectedJackNodeIdx]);

  return (
    <>
      <div id="main">
        <CodePanel
          items={tokens}
          hoveredItemIdxs={hoveredTokens}
          mouseSelectedItemIdxs={mouseSelectedTokenIdxs}
          autoSelectedItemIdxs={autoSelectedTokens}
          onSpanMouseEnter={setHoveredTokenIdx}
          onSpanMouseLeave={clearHoverState}
          onSpanClick={(idx) => {
            setMouseSelectedVMCommandIdx(undefined);
            setMouseSelectedJackNode(findInnermostJackNode(filename, idx));
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

const root = createRoot(getElementById("root"));
root.render(<App />);
