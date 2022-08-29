import "./styles/reset.css";
import data from "../debug-output.json";
import { DebugOutput } from "../../bindings/DebugOutput";
import _ from "lodash";
import { NodeInfo } from "../../bindings/NodeInfo";
import React, { useState } from "react";
import { createRoot } from "react-dom/client";
import CodePanel from "./code-panel";

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

function App() {
  const [mouseHoveredTokens, setMouseHoveredTokens] = useState<Set<number>>(
    new Set()
  );
  const [mouseHoveredVMCommands, setMouseHoveredVMCommands] = useState<
    Set<number>
  >(new Set());
  const [autoHoveredVMCommands, setAutoHoveredVMCommands] = useState<
    Set<number>
  >(new Set());

  function hoverNode(jackNode: NodeInfo) {
    const newHoveredTokens: Set<number> = new Set();
    for (
      let i = jackNode.token_range.start;
      i < jackNode.token_range.end;
      i++
    ) {
      const token = debugOutput.tokens[i];
      if (token === undefined) {
        throw new Error("failed to get token");
      }
      newHoveredTokens.add(token.idx);
    }
    setMouseHoveredTokens(newHoveredTokens);
  }

  function clearHoverState() {
    setAutoHoveredVMCommands(new Set());
    setMouseHoveredTokens(new Set());
    setMouseHoveredVMCommands(new Set());
  }

  return (
    <>
      <CodePanel
        items={debugOutput.tokens.map((token) => token.source)}
        mouseHoveredItemIdxs={mouseHoveredTokens}
        autoHoveredItemIdxs={new Set()}
        onSpanMouseOver={(idx) => {
          const jackNode = findInnermostJackNode(idx);
          if (jackNode) {
            hoverNode(jackNode);
            setAutoHoveredVMCommands(new Set(allVMCommandIdxs(jackNode.index)));
          }
        }}
        onSpanMouseLeave={clearHoverState}
      />
      <CodePanel
        items={debugOutput.vm_commands.map((command) => `${command}\n`)}
        mouseHoveredItemIdxs={mouseHoveredVMCommands}
        autoHoveredItemIdxs={autoHoveredVMCommands}
        onSpanMouseOver={(idx) => {
          setMouseHoveredVMCommands(new Set([idx]));
        }}
        onSpanMouseLeave={clearHoverState}
      />
    </>
  );
}

const root = createRoot(getElementById("main"));
root.render(<App />);
