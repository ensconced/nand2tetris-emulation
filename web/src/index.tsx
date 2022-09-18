import "./styles/reset.css";
import { NodeInfo } from "../../bindings/NodeInfo";
import React, { useMemo, useState } from "react";
import { createRoot } from "react-dom/client";
import CodePanel, { FileIdxs } from "./code-panel";
import { TokenKind } from "../../bindings/TokenKind";
import { Token } from "../../bindings/Token";
import Footer from "./Footer";
import ASMPanel from "./ASMPanel";
import {
  allVMCommandIdxs,
  filenames,
  findInnermostJackNode,
  getJackNodeByIndex,
  jackNodeTokens,
  tokensByFilename,
  vmCommandJackNodeIdx,
  vmCommands,
} from "./sourcemapUtils";
import useCoordinatedInteractions from "./useCoordinatedInteractions";

function getElementById(id: string): HTMLElement {
  const element = document.getElementById(id);
  if (element === null) {
    throw new Error(`failed to find element with id ${id}`);
  }
  return element;
}

export interface NodeInfoId {
  filename: string;
  node: NodeInfo;
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

  const {
    interactedTokens: hoveredTokens,
    interactedVMCommands: hoveredVMCommands,
    interactedInstructionIdxs: hoveredInstructionIdxs,
    interactedJackNode: hoveredJackNode,
  } = useCoordinatedInteractions(
    directlyHoveredVMCommand,
    directlyHoveredToken,
    directlyHoveredInstructionIdx
  );

  const [mouseSelectedJackNode, setMouseSelectedJackNode] =
    useState<NodeInfoId>();
  const [mouseSelectedVMCommandIdx, setMouseSelectedVMCommandIdx] =
    useState<FileIdx>();

  function clearHoverState() {
    setDirectlyHoveredToken(undefined);
    setDirectlyHoveredVMCommand(undefined);
  }

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
    return vmCommandJackNodeIdx(mouseSelectedVMCommandIdx);
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
