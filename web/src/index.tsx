import _ from "lodash";
import "./styles/reset.css";
import data from "../debug-output.json";
import { NodeInfo } from "../../bindings/NodeInfo";
import { CompilerResult } from "../../bindings/CompilerResult";
import React, { useMemo, useState } from "react";
import { createRoot } from "react-dom/client";
import CodePanel from "./code-panel";
import { TokenKind } from "../../bindings/TokenKind";
import { Token } from "../../bindings/Token";

interface NodeInfoId {
  filename: string;
  node: NodeInfo;
}

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

function jackNodeTokens(node: NodeInfoId | undefined): Set<number> {
  if (node === undefined) return new Set();
  const tokenSet: Set<number> = new Set();
  for (
    let i = node.node.token_range.start;
    i < node.node.token_range.end;
    i++
  ) {
    const token = tokensByFilename[node.filename]?.[i];
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
    throw new Error(`failed to get jack node at index ${index.idx}`);
  }
  return node;
}

function findInnermostJackNode(tokenIdx: FileIdx): NodeInfoId | undefined {
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
  return { filename: tokenIdx.filename, node: smallestJackNode };
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
  tokens: Array<Token<TokenKind>>;
  commands: Array<string>;
  hoveredTokens: Set<number>;
  mouseSelectedTokenIdxs: Set<number>;
  autoSelectedTokens: Set<number>;
  setHoveredTokenIdx: (tokenIdx: FileIdx) => void;
  clearHoverState: () => void;
  setMouseSelectedVMCommandIdx: (idx: FileIdx | undefined) => void;
  setHoveredVMCommandIdx: (idx: FileIdx | undefined) => void;
  setMouseSelectedJackNode: (node: NodeInfoId | undefined) => void;
  hoveredVMCommands: Set<number>;
  mouseSelectedVMCommandIdxs: Set<number>;
  autoSelectedVMCommands: Set<number>;
}

function JackModule({
  filename,
  tokens,
  commands,
  hidden,
  hoveredTokens,
  mouseSelectedTokenIdxs,
  autoSelectedTokens,
  setHoveredTokenIdx,
  clearHoverState,
  setMouseSelectedVMCommandIdx,
  setMouseSelectedJackNode,
  hoveredVMCommands,
  mouseSelectedVMCommandIdxs,
  autoSelectedVMCommands,
  setHoveredVMCommandIdx,
}: Props) {
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
          onSpanMouseEnter={(idx) => {
            setHoveredTokenIdx({ filename, idx });
          }}
          onSpanMouseLeave={clearHoverState}
          onSpanClick={(idx) => {
            setMouseSelectedVMCommandIdx(undefined);
            setMouseSelectedJackNode(findInnermostJackNode({ filename, idx }));
          }}
        />
        <CodePanel
          items={vmCommandStrings}
          hoveredItemIdxs={hoveredVMCommands}
          mouseSelectedItemIdxs={mouseSelectedVMCommandIdxs}
          autoSelectedItemIdxs={autoSelectedVMCommands}
          onSpanMouseEnter={(idx) => setHoveredVMCommandIdx({ filename, idx })}
          onSpanMouseLeave={clearHoverState}
          onSpanClick={(idx) => {
            setMouseSelectedJackNode(undefined);
            setMouseSelectedVMCommandIdx({ filename, idx });
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
    useState<NodeInfoId>();
  const [mouseSelectedVMCommandIdx, setMouseSelectedVMCommandIdx] =
    useState<FileIdx>();

  function clearHoverState() {
    setHoveredTokenIdx(undefined);
    setHoveredVMCommandIdx(undefined);
  }

  const hoveredJackNode = useMemo<NodeInfoId | undefined>(() => {
    if (hoveredTokenIdx !== undefined) {
      const node = findInnermostJackNode(hoveredTokenIdx);
      if (node !== undefined) {
        return node;
      }
    } else if (hoveredVMCommandIdx !== undefined) {
      const jackNodeIdx =
        jackCompilerSourcemaps[hoveredVMCommandIdx.filename]?.codegen_sourcemap
          .vm_command_idx_to_jack_node_idx[hoveredVMCommandIdx.idx];
      if (jackNodeIdx !== undefined) {
        return {
          filename: hoveredVMCommandIdx.filename,
          node: getJackNodeByIndex({
            filename: hoveredVMCommandIdx.filename,
            idx: jackNodeIdx,
          }),
        };
      }
    }
  }, [hoveredTokenIdx, hoveredVMCommandIdx]);

  const hoveredTokens = useMemo(
    () => jackNodeTokens(hoveredJackNode),
    [hoveredJackNode]
  );

  const hoveredVMCommands = useMemo<Set<number>>(() => {
    return hoveredJackNode
      ? new Set(
          allVMCommandIdxs({
            filename: hoveredJackNode.filename,
            idx: hoveredJackNode.node.index,
          })
        )
      : new Set();
  }, [hoveredJackNode]);

  const autoSelectedVMCommands = useMemo<Set<number>>(() => {
    return mouseSelectedJackNode
      ? new Set(
          allVMCommandIdxs({
            filename: mouseSelectedJackNode.filename,
            idx: mouseSelectedJackNode.node.index,
          })
        )
      : new Set();
  }, [mouseSelectedJackNode]);

  const autoSelectedJackNodeIdx = useMemo<FileIdx | undefined>(() => {
    if (mouseSelectedVMCommandIdx === undefined) return undefined;

    const idx =
      jackCompilerSourcemaps[mouseSelectedVMCommandIdx.filename]
        ?.codegen_sourcemap.vm_command_idx_to_jack_node_idx[
        mouseSelectedVMCommandIdx.idx
      ];

    if (idx !== undefined) {
      return { filename: mouseSelectedVMCommandIdx.filename, idx };
    }
  }, [mouseSelectedVMCommandIdx]);

  const autoSelectedJackNode = useMemo<NodeInfoId | undefined>(() => {
    return autoSelectedJackNodeIdx === undefined
      ? undefined
      : {
          filename: autoSelectedJackNodeIdx.filename,
          node: getJackNodeByIndex(autoSelectedJackNodeIdx),
        };
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
        <JackModule
          tokens={tokensByFilename[filename]!}
          commands={
            vmCompilerInputs.find((x) => x.filename === filename)!.commands
          }
          filename={filename}
          hidden={idx !== currentFileIdx}
          hoveredTokens={hoveredTokens}
          mouseSelectedTokenIdxs={mouseSelectedTokenIdxs}
          hoveredVMCommands={hoveredVMCommands}
          mouseSelectedVMCommandIdxs={mouseSelectedVMCommandIdxs}
          autoSelectedTokens={autoSelectedTokens}
          autoSelectedVMCommands={autoSelectedVMCommands}
          setHoveredTokenIdx={setHoveredTokenIdx}
          setHoveredVMCommandIdx={setHoveredVMCommandIdx}
          setMouseSelectedJackNode={setMouseSelectedJackNode}
          setMouseSelectedVMCommandIdx={setMouseSelectedVMCommandIdx}
          clearHoverState={clearHoverState}
        />
      ))}
    </>
  );
}

const root = createRoot(getElementById("root"));
root.render(<App />);
