import "./styles/reset.css";
import data from "../debug-output.json";
import { DebugOutput } from "../../bindings/DebugOutput";
import _ from "lodash";
import { NodeInfo } from "../../bindings/NodeInfo";
import React, { useState } from "react";
import { createRoot } from "react-dom/client";
import CodePanel from "./code-panel";
import Footer from "./footer";

const debugOutput = data as DebugOutput;

function getElementById(id: string): HTMLElement {
  const element = document.getElementById(id);
  if (element === null) {
    throw new Error(`failed to find element with id ${id}`);
  }
  return element;
}

function immediateVMCommandIdxs(jackNodeIdx: number): number[] {
  return (
    debugOutput.sourcemap.jack_node_idx_to_vm_command_idx[jackNodeIdx] ?? []
  );
}

function allVMCommandIdxs(jackNodeIdx: number): number[] {
  return immediateVMCommandIdxs(jackNodeIdx).concat(
    getJackNodeByIndex(jackNodeIdx).child_node_idxs.flatMap(allVMCommandIdxs)
  );
}

function getJackNodeByIndex(index: number): NodeInfo {
  const node = debugOutput.sourcemap.jack_nodes[index];
  if (node === undefined) {
    throw new Error(`failed to get jack node at index ${index}`);
  }
  return node;
}

function findInnermostJackNode(tokenIdx: number): NodeInfo | undefined {
  const tokenJackNodesIdxs =
    debugOutput.sourcemap.token_idx_to_jack_node_idxs[tokenIdx];
  if (!tokenJackNodesIdxs) return undefined;
  const tokenJackNodes = tokenJackNodesIdxs.map(getJackNodeByIndex);
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

  let hoveredJackNode: NodeInfo | undefined;
  if (hoveredTokenIdx !== undefined) {
    hoveredJackNode = findInnermostJackNode(hoveredTokenIdx);
  } else if (hoveredVMCommandIdx !== undefined) {
    const jackNodeIdx =
      debugOutput.sourcemap.vm_command_idx_to_jack_node_idx[
        hoveredVMCommandIdx
      ];
    if (jackNodeIdx !== undefined) {
      hoveredJackNode = getJackNodeByIndex(jackNodeIdx);
    }
  }

  const hoveredTokens = jackNodeTokens(hoveredJackNode);
  const hoveredVMCommands = hoveredJackNode
    ? new Set(allVMCommandIdxs(hoveredJackNode.index))
    : new Set<number>();

  let autoSelectedJackNodeIdx;
  if (mouseSelectedVMCommandIdx !== undefined) {
    autoSelectedJackNodeIdx =
      debugOutput.sourcemap.vm_command_idx_to_jack_node_idx[
        mouseSelectedVMCommandIdx
      ];
  }

  let autoSelectedJackNode;
  if (autoSelectedJackNodeIdx !== undefined) {
    autoSelectedJackNode = getJackNodeByIndex(autoSelectedJackNodeIdx);
  }

  let autoSelectedTokens = new Set<number>();
  if (autoSelectedJackNode !== undefined) {
    autoSelectedTokens = jackNodeTokens(autoSelectedJackNode);
  }

  let autoSelectedVMCommands = new Set<number>();
  if (mouseSelectedJackNode !== undefined) {
    autoSelectedVMCommands = new Set(
      allVMCommandIdxs(mouseSelectedJackNode.index)
    );
  }

  const vmCommands = [...hoveredVMCommands].sort((a, b) => a - b);
  const firstVMCommand = vmCommands[0];
  const lastVMCommand = vmCommands[vmCommands.length - 1];
  const vmCommandRange =
    firstVMCommand !== undefined && lastVMCommand !== undefined
      ? `${firstVMCommand} - ${lastVMCommand}`
      : "";

  return (
    <>
      <div id="main">
        <CodePanel
          items={debugOutput.tokens.map((token) => token.source)}
          hoveredItemIdxs={hoveredTokens}
          mouseSelectedItemIdxs={jackNodeTokens(mouseSelectedJackNode)}
          autoSelectedItemIdxs={autoSelectedTokens}
          onSpanMouseEnter={setHoveredTokenIdx}
          onSpanMouseLeave={clearHoverState}
          onSpanClick={(idx) => {
            setMouseSelectedVMCommandIdx(undefined);
            setMouseSelectedJackNode(findInnermostJackNode(idx));
          }}
          footerItems={[
            `node idx: ${hoveredJackNode?.index ?? ""}`,
            `token range: ${
              hoveredJackNode
                ? `${hoveredJackNode.token_range.start} - ${hoveredJackNode.token_range.end}`
                : ""
            }`,
          ]}
        />
        <CodePanel
          items={debugOutput.vm_commands.map((command) => `${command}\n`)}
          hoveredItemIdxs={hoveredVMCommands}
          mouseSelectedItemIdxs={
            autoSelectedJackNodeIdx === undefined
              ? new Set()
              : new Set(allVMCommandIdxs(autoSelectedJackNodeIdx))
          }
          autoSelectedItemIdxs={autoSelectedVMCommands}
          onSpanMouseEnter={setHoveredVMCommandIdx}
          onSpanMouseLeave={clearHoverState}
          onSpanClick={(idx) => {
            setMouseSelectedJackNode(undefined);
            setMouseSelectedVMCommandIdx(idx);
          }}
          footerItems={[vmCommandRange]}
        />
      </div>
    </>
  );
}

const root = createRoot(getElementById("root"));
root.render(<App />);
