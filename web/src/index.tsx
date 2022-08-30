import "./styles/reset.css";
import data from "../debug-output.json";
import { DebugOutput } from "../../bindings/DebugOutput";
import _ from "lodash";
import { NodeInfo } from "../../bindings/NodeInfo";
import React, { useCallback, useState } from "react";
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
  const [hoveredTokens, setHoveredTokens] = useState<Set<number>>(new Set());
  const [hoveredVMCommands, setHoveredVMCommands] = useState<Set<number>>(
    new Set()
  );
  const [mouseSelectedTokens, setMouseSelectedTokens] = useState<Set<number>>(
    new Set()
  );
  const [mouseSelectedVMCommands, setMouseSelectedVMCommands] = useState<
    Set<number>
  >(new Set());
  const [autoSelectedTokens, setAutoSelectedTokens] = useState<Set<number>>(
    new Set()
  );
  const [autoSelectedVMCommands, setAutoSelectedVMCommands] = useState<
    Set<number>
  >(new Set());

  function jackNodeTokens(jackNode: NodeInfo): Set<number> {
    const tokens: Set<number> = new Set();
    for (
      let i = jackNode.token_range.start;
      i < jackNode.token_range.end;
      i++
    ) {
      const token = debugOutput.tokens[i];
      if (token === undefined) {
        throw new Error("failed to get token");
      }
      tokens.add(token.idx);
    }
    return tokens;
  }

  function clearHoverState() {
    setHoveredTokens(new Set());
    setHoveredVMCommands(new Set());
  }

  const handleVMCommandMouseOver = useCallback((idx: number) => {
    const jackNodeIdx =
      debugOutput.sourcemap.vm_command_idx_to_jack_node_idx[idx];
    if (jackNodeIdx) {
      const jackNode = getJackNodeByIndex(jackNodeIdx);
      setHoveredTokens(jackNodeTokens(jackNode));
      setHoveredVMCommands(new Set(allVMCommandIdxs(jackNodeIdx)));
    }
  }, []);

  const handleTokenMouseOver = useCallback((idx: number) => {
    const jackNode = findInnermostJackNode(idx);
    if (jackNode) {
      setHoveredTokens(jackNodeTokens(jackNode));
      setHoveredVMCommands(new Set(allVMCommandIdxs(jackNode.index)));
    }
  }, []);

  const handleTokenClick = useCallback((idx: number) => {
    const jackNode = findInnermostJackNode(idx);
    if (jackNode) {
      setMouseSelectedVMCommands(new Set());
      setAutoSelectedTokens(new Set());
      setMouseSelectedTokens(jackNodeTokens(jackNode));
      setAutoSelectedVMCommands(new Set(allVMCommandIdxs(jackNode.index)));
    }
  }, []);

  const handleVMCommandClick = useCallback((idx: number) => {
    const jackNodeIdx =
      debugOutput.sourcemap.vm_command_idx_to_jack_node_idx[idx];
    if (jackNodeIdx) {
      const jackNode = getJackNodeByIndex(jackNodeIdx);
      setMouseSelectedTokens(new Set());
      setAutoSelectedVMCommands(new Set());
      setAutoSelectedTokens(jackNodeTokens(jackNode));
      setMouseSelectedVMCommands(new Set(allVMCommandIdxs(jackNodeIdx)));
    }
  }, []);

  return (
    <>
      <div id="main">
        <CodePanel
          items={debugOutput.tokens.map((token) => token.source)}
          hoveredItemIdxs={hoveredTokens}
          mouseSelectedItemIdxs={mouseSelectedTokens}
          autoSelectedItemIdxs={autoSelectedTokens}
          onSpanMouseOver={handleTokenMouseOver}
          onSpanMouseLeave={clearHoverState}
          onSpanClick={handleTokenClick}
        />
        <CodePanel
          items={debugOutput.vm_commands.map((command) => `${command}\n`)}
          hoveredItemIdxs={hoveredVMCommands}
          mouseSelectedItemIdxs={mouseSelectedVMCommands}
          autoSelectedItemIdxs={autoSelectedVMCommands}
          onSpanMouseOver={handleVMCommandMouseOver}
          onSpanMouseLeave={clearHoverState}
          onSpanClick={handleVMCommandClick}
        />
      </div>
      <code id="footer">hello there</code>
    </>
  );
}

const root = createRoot(getElementById("root"));
root.render(<App />);
