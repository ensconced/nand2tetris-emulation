import _ from "lodash";
import "./styles/reset.css";
import data from "../debug-output.json";
import { NodeInfo } from "../../bindings/NodeInfo";
import { CompilerResult } from "../../bindings/CompilerResult";
import React, { useMemo, useState } from "react";
import { createRoot } from "react-dom/client";
import CodePanel, { FileIdxs } from "./code-panel";
import { TokenKind } from "../../bindings/TokenKind";
import { Token } from "../../bindings/Token";
import Footer from "./Footer";
import ASMPanel from "./ASMPanel";

export interface NodeInfoId {
  filename: string;
  node: NodeInfo;
}

const compilerResult = data as CompilerResult;
const {
  jack_compiler_result: {
    sourcemaps: jackCompilerSourcemaps,
    tokens: tokensByFilename,
    vm_commands: vmCommands,
  },
  vm_compiler_result: { sourcemap: vmCompilerSourcemap },
} = compilerResult;

const filenames = Object.keys(tokensByFilename);

function getElementById(id: string): HTMLElement {
  const element = document.getElementById(id);
  if (element === null) {
    throw new Error(`failed to find element with id ${id}`);
  }
  return element;
}

function jackNodeTokens(node: NodeInfoId | undefined): FileIdxs | undefined {
  if (node === undefined) return undefined;
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
  return { filename: node.filename, idxs: tokenSet };
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
  hoveredTokens: FileIdxs | undefined;
  mouseSelectedTokenIdxs: FileIdxs | undefined;
  autoSelectedTokens: FileIdxs | undefined;
  setHoveredTokenIdx: (tokenIdx: FileIdx) => void;
  clearHoverState: () => void;
  setMouseSelectedVMCommandIdx: (idx: FileIdx | undefined) => void;
  setHoveredVMCommandIdx: (idx: FileIdx | undefined) => void;
  setMouseSelectedJackNode: (node: NodeInfoId | undefined) => void;
  hoveredVMCommands: FileIdxs | undefined;
  mouseSelectedVMCommandIdxs: FileIdxs | undefined;
  autoSelectedVMCommands: FileIdxs | undefined;
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
  const tokenContents = tokens.map((token) => token.source);
  const vmCommandsWithNewLines = commands.map((command) => `${command}\n`);

  return (
    <>
      <div style={{ minHeight: 0, display: hidden ? "none" : "flex" }}>
        <CodePanel
          filename={filename}
          items={tokenContents}
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
          filename={filename}
          items={vmCommandsWithNewLines}
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

export interface FileIdx {
  filename: string;
  idx: number;
}

function App() {
  const [openFileIdx, setOpenFileIdx] = useState(0);

  const [directlyHoveredVMCommand, setDirectlyHoveredVMCommand] =
    useState<FileIdx>();
  const [directlyHoveredToken, setDirectlyHoveredToken] = useState<FileIdx>();
  const [directlyHoveredInstructionIdx, setDirectlyHoveredInstructionIdx] =
    useState<number>();

  const [mouseSelectedJackNode, setMouseSelectedJackNode] =
    useState<NodeInfoId>();
  const [mouseSelectedVMCommandIdx, setMouseSelectedVMCommandIdx] =
    useState<FileIdx>();

  function clearHoverState() {
    setDirectlyHoveredToken(undefined);
    setDirectlyHoveredVMCommand(undefined);
  }

  const singleHoveredVMCmd = useMemo(() => {
    if (directlyHoveredInstructionIdx === undefined) return undefined;
    return vmCompilerSourcemap.asm_instruction_idx_to_vm_cmd[
      directlyHoveredInstructionIdx
    ];
  }, [directlyHoveredInstructionIdx]);

  function vmCommandToJackNode(vmCmd: FileIdx): NodeInfoId | undefined {
    const jackNodeIdx =
      jackCompilerSourcemaps[vmCmd.filename]?.codegen_sourcemap
        .vm_command_idx_to_jack_node_idx[vmCmd.idx];
    if (jackNodeIdx !== undefined) {
      return {
        filename: vmCmd.filename,
        node: getJackNodeByIndex({
          filename: vmCmd.filename,
          idx: jackNodeIdx,
        }),
      };
    }
  }

  const hoveredJackNode = useMemo<NodeInfoId | undefined>(() => {
    if (directlyHoveredToken !== undefined) {
      const node = findInnermostJackNode(directlyHoveredToken);
      if (node !== undefined) {
        return node;
      }
    } else if (directlyHoveredVMCommand !== undefined) {
      return vmCommandToJackNode(directlyHoveredVMCommand);
    }
  }, [directlyHoveredToken, directlyHoveredVMCommand]);

  const contextualHoveredJackNode = useMemo(() => {
    if (singleHoveredVMCmd === undefined) return undefined;
    return vmCommandToJackNode({
      filename: singleHoveredVMCmd.filename,
      idx: singleHoveredVMCmd.vm_command_idx,
    });
  }, [singleHoveredVMCmd]);

  const hoveredTokens = useMemo(
    () => jackNodeTokens(hoveredJackNode),
    [hoveredJackNode]
  );

  const hoveredVMCommands = useMemo<FileIdxs | undefined>(() => {
    if (hoveredJackNode) {
      return {
        filename: hoveredJackNode.filename,
        idxs: new Set(
          allVMCommandIdxs({
            filename: hoveredJackNode.filename,
            idx: hoveredJackNode.node.index,
          })
        ),
      };
    }

    if (singleHoveredVMCmd) {
      return {
        filename: singleHoveredVMCmd.filename,
        idxs: new Set([singleHoveredVMCmd.vm_command_idx]),
      };
    }
  }, [hoveredJackNode, singleHoveredVMCmd]);

  const hoveredInstructionIdxs = useMemo<Set<number>>(() => {
    const result = new Set<number>();
    if (hoveredVMCommands === undefined) {
      return directlyHoveredInstructionIdx === undefined
        ? new Set()
        : new Set([directlyHoveredInstructionIdx]);
    }
    const vmCommandIdxToASMInstructionIdxs =
      vmCompilerSourcemap.vm_filename_and_idx_to_asm_instruction_idx[
        hoveredVMCommands.filename
      ];

    if (vmCommandIdxToASMInstructionIdxs === undefined) {
      throw new Error(
        `failed to find instruction idx lookup for ${hoveredVMCommands.filename}`
      );
    }

    hoveredVMCommands.idxs.forEach((vmCmdIdx) => {
      const asmInstructions = vmCommandIdxToASMInstructionIdxs[vmCmdIdx];
      if (asmInstructions === undefined) {
        throw new Error(
          `failed to find asm instructions for vm command idx ${vmCmdIdx} in file ${hoveredVMCommands.filename}`
        );
      }
      asmInstructions.forEach((instruction) => result.add(instruction));
    });

    return result;
  }, [directlyHoveredInstructionIdx, hoveredVMCommands]);

  const autoSelectedVMCommands = useMemo<FileIdxs | undefined>(() => {
    return (
      mouseSelectedJackNode && {
        filename: mouseSelectedJackNode.filename,
        idxs: new Set(
          allVMCommandIdxs({
            filename: mouseSelectedJackNode.filename,
            idx: mouseSelectedJackNode.node.index,
          })
        ),
      }
    );
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

  const autoSelectedTokens = useMemo<FileIdxs | undefined>(() => {
    return autoSelectedJackNode
      ? jackNodeTokens(autoSelectedJackNode)
      : undefined;
  }, [autoSelectedJackNode]);

  const mouseSelectedTokenIdxs = useMemo<FileIdxs | undefined>(
    () => jackNodeTokens(mouseSelectedJackNode),
    [mouseSelectedJackNode]
  );

  const mouseSelectedVMCommandIdxs = useMemo<FileIdxs | undefined>(() => {
    return autoSelectedJackNodeIdx === undefined
      ? undefined
      : {
          filename: autoSelectedJackNodeIdx.filename,
          idxs: new Set(allVMCommandIdxs(autoSelectedJackNodeIdx)),
        };
  }, [autoSelectedJackNodeIdx]);

  return (
    <div style={{ display: "flex" }}>
      <div
        style={{
          flex: 1,
          height: "100vh",
          display: "flex",
          flexDirection: "column",
        }}
      >
        <fieldset style={{ flex: "0 0 auto" }}>
          {filenames.map((filename, idx) => (
            <>
              <input
                id={`file-${idx}`}
                type="radio"
                name="file-tab"
                checked={openFileIdx === idx}
                onChange={() => setOpenFileIdx(idx)}
              />
              <label htmlFor={`file-${idx}`}>{filename}</label>
            </>
          ))}
        </fieldset>
        {filenames.map((filename, idx) => (
          <JackModule
            tokens={tokensByFilename[filename]!}
            commands={vmCommands[filename]!}
            filename={filename}
            hidden={idx !== openFileIdx}
            hoveredTokens={hoveredTokens}
            mouseSelectedTokenIdxs={mouseSelectedTokenIdxs}
            hoveredVMCommands={hoveredVMCommands}
            mouseSelectedVMCommandIdxs={mouseSelectedVMCommandIdxs}
            autoSelectedTokens={autoSelectedTokens}
            autoSelectedVMCommands={autoSelectedVMCommands}
            setHoveredTokenIdx={setDirectlyHoveredToken}
            setHoveredVMCommandIdx={setDirectlyHoveredVMCommand}
            setMouseSelectedJackNode={setMouseSelectedJackNode}
            setMouseSelectedVMCommandIdx={setMouseSelectedVMCommandIdx}
            clearHoverState={clearHoverState}
          />
        ))}
        <Footer
          hoveredJackNode={hoveredJackNode}
          selectedJackNode={autoSelectedJackNode || mouseSelectedJackNode}
        />
      </div>
      <ASMPanel
        directlyHoveredInstructionIdx={directlyHoveredInstructionIdx}
        setDirectlyHoveredInstructionIdx={setDirectlyHoveredInstructionIdx}
        hoveredInstructionIdxs={hoveredInstructionIdxs}
      />
    </div>
  );
}

const root = createRoot(getElementById("root"));
root.render(<App />);
