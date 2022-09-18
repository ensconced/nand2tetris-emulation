import "./styles/reset.css";
import { NodeInfo } from "../../bindings/NodeInfo";
import React, { useMemo, useState } from "react";
import { createRoot } from "react-dom/client";
import { FileIdxs } from "./code-panel";
import Footer from "./Footer";
import ASMPanel from "./ASMPanel";
import {
  allVMCommandIdxs,
  filenames,
  getJackNodeByIndex,
  jackNodeTokens,
  tokensByFilename,
  vmCommandJackNodeIdx,
  vmCommands,
} from "./sourcemapUtils";
import useCoordinatedInteractions from "./useCoordinatedInteractions";
import JackModule from "./JackModule";

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
