import _ from "lodash";
import "./styles/reset.css";
import data from "../debug-output.json";
import { JackCompilerResult } from "../../bindings/JackCompilerResult";
import { NodeInfo } from "../../bindings/NodeInfo";
import React, { useMemo, useState } from "react";
import { createRoot } from "react-dom/client";
import CodePanel from "./code-panel";

const compilerResults = data as unknown as JackCompilerResult[];

function getElementById(id: string): HTMLElement {
  const element = document.getElementById(id);
  if (element === null) {
    throw new Error(`failed to find element with id ${id}`);
  }
  return element;
}

interface Props {
  compilerResult: JackCompilerResult;
  hidden: boolean;
}

function JackModule({ compilerResult, hidden }: Props) {
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

  function immediateVMCommandIdxs(
    filename: string,
    jackNodeIdx: number
  ): number[] {
    return jack_node_idx_to_vm_command_idx[jackNodeIdx] ?? [];
  }

  function allVMCommandIdxs(filename: string, jackNodeIdx: number): number[] {
    return immediateVMCommandIdxs(filename, jackNodeIdx).concat(
      getJackNodeByIndex(filename, jackNodeIdx).child_node_idxs.flatMap(
        (childNodeIdx) => allVMCommandIdxs(filename, childNodeIdx)
      )
    );
  }

  function getJackNodeByIndex(filename: string, index: number): NodeInfo {
    const node = jack_nodes[index];
    if (node === undefined) {
      throw new Error(`failed to get jack node at index ${index}`);
    }
    return node;
  }

  function findInnermostJackNode(
    filename: string,
    tokenIdx: number
  ): NodeInfo | undefined {
    const tokenJackNodesIdxs = token_idx_to_jack_node_idxs[tokenIdx];
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

  const {
    filename,
    sourcemap: {
      codegen_sourcemap: {
        vm_command_idx_to_jack_node_idx,
        jack_node_idx_to_vm_command_idx,
      },
      parser_sourcemap: { jack_nodes, token_idx_to_jack_node_idxs },
    },
    tokens,
    commands,
  } = compilerResult;

  const hoveredJackNode = useMemo<NodeInfo | undefined>(() => {
    if (hoveredTokenIdx !== undefined) {
      return findInnermostJackNode(filename, hoveredTokenIdx);
    } else if (hoveredVMCommandIdx !== undefined) {
      const jackNodeIdx = vm_command_idx_to_jack_node_idx[hoveredVMCommandIdx];
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
      : vm_command_idx_to_jack_node_idx[mouseSelectedVMCommandIdx];
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

function App() {
  const [currentFileIdx, setCurrentFileIdx] = useState(0);

  return (
    <>
      <fieldset style={{ flex: "0 0 auto" }}>
        {compilerResults.map(({ filename }, idx) => (
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
      {compilerResults.map((compilerResult, idx) => (
        <JackModule
          compilerResult={compilerResult}
          hidden={idx !== currentFileIdx}
        />
      ))}
    </>
  );
}

const root = createRoot(getElementById("root"));
root.render(<App />);
