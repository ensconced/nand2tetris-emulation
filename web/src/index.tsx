import "./styles/reset.css";
import { NodeInfo } from "../../bindings/NodeInfo";
import React, { useState } from "react";
import { createRoot } from "react-dom/client";
import Footer from "./Footer";
import ASMPanel from "./ASMPanel";
import { filenames, tokensByFilename, vmCommands } from "./sourcemapUtils";
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
    "hover",
    directlyHoveredVMCommand,
    directlyHoveredToken,
    directlyHoveredInstructionIdx
  );

  const [directlySelectedToken, setDirectlySelectedToken] = useState<FileIdx>();
  const [directlySelectedVMCommand, setMouseSelectedVMCommandIdx] =
    useState<FileIdx>();
  const [directlySelectedInstructionIdx, setDirectlySelectedInstructionIdx] =
    useState<number>();

  const {
    interactedTokens: selectedTokens,
    interactedVMCommands: selectedVMCommands,
    interactedInstructionIdxs: selectedInstructionIdxs,
    interactedJackNode: selectedJackNode,
  } = useCoordinatedInteractions(
    "selection",
    directlySelectedVMCommand,
    directlySelectedToken,
    directlySelectedInstructionIdx
  );

  function clearHoverState() {
    setDirectlyHoveredToken(undefined);
    setDirectlyHoveredVMCommand(undefined);
  }

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
            <React.Fragment key={filename}>
              <input
                id={`file-${idx}`}
                type="radio"
                name="file-tab"
                checked={openFileIdx === idx}
                onChange={() => setOpenFileIdx(idx)}
              />
              <label htmlFor={`file-${idx}`}>{filename}</label>
            </React.Fragment>
          ))}
        </fieldset>
        {filenames.map((filename, idx) => {
          return (
            <JackModule
              key={filename}
              tokens={tokensByFilename[filename]!}
              commands={vmCommands[filename]!}
              filename={filename}
              hidden={idx !== openFileIdx}
              hoveredTokens={hoveredTokens}
              selectedTokenIdxs={selectedTokens}
              hoveredVMCommands={hoveredVMCommands}
              selectedVMCommands={selectedVMCommands}
              setHoveredTokenIdx={setDirectlyHoveredToken}
              setHoveredVMCommandIdx={setDirectlyHoveredVMCommand}
              setMouseSelectedTokenIdx={setDirectlySelectedToken}
              setMouseSelectedVMCommandIdx={setMouseSelectedVMCommandIdx}
              clearHoverState={clearHoverState}
            />
          );
        })}
        <Footer
          hoveredJackNode={hoveredJackNode}
          selectedJackNode={selectedJackNode}
        />
      </div>
      <ASMPanel
        directlyHoveredInstructionIdx={directlyHoveredInstructionIdx}
        setDirectlyHoveredInstructionIdx={setDirectlyHoveredInstructionIdx}
        setDirectlySelectedInstructionIdx={setDirectlySelectedInstructionIdx}
        setDirectlySelectedVMCommand={setMouseSelectedVMCommandIdx}
        setDirectlySelectedToken={setDirectlySelectedToken}
        hoveredInstructionIdxs={hoveredInstructionIdxs?.idxs ?? new Set()}
        selectedInstructionIdxs={selectedInstructionIdxs}
      />
    </div>
  );
}

const root = createRoot(getElementById("root"));
root.render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);
