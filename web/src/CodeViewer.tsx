import "./styles/reset.css";
import React, { useEffect, useRef, useState } from "react";
import Footer from "./Footer";
import ASMPanel from "./ASMPanel";
import { filenames, tokensByFilename, vmCommands } from "./sourcemapUtils";
import useCoordinatedInteractions from "./useCoordinatedInteractions";
import JackModule from "./JackModule";
import { FileIdx } from "./types";
import TabControls from "./TabControls";

interface Props {
  programCounter: number;
  onTick: () => void;
  onPlay: () => void;
}

export default function CodeViewer({ programCounter, onTick, onPlay }: Props) {
  const jackModuleContainer = useRef<HTMLDivElement>(null);
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
    interactedFilename: selectedFilename,
  } = useCoordinatedInteractions(
    "selection",
    directlySelectedVMCommand,
    directlySelectedToken,
    directlySelectedInstructionIdx
  );

  useEffect(() => {
    const foundIndex = filenames.findIndex(
      (filename) => filename === selectedFilename
    );
    const index = foundIndex === -1 ? 0 : foundIndex;
    setOpenFileIdx(index);
  }, [selectedFilename]);

  useEffect(() => {
    const container = jackModuleContainer.current;
    if (container instanceof HTMLElement) {
      container.scrollTop = openFileIdx * container.clientHeight;
    }
  }, [openFileIdx]);

  function clearHoverState() {
    setDirectlyHoveredToken(undefined);
    setDirectlyHoveredVMCommand(undefined);
  }

  return (
    <>
      <div className="panel-container">
        <TabControls
          groupName="file-tab"
          items={filenames}
          onChange={(idx) => setOpenFileIdx(idx)}
          checkedIdx={openFileIdx}
        />
        <div
          ref={jackModuleContainer}
          style={{
            flex: "1",
            minHeight: 0,
            overflow: "hidden",
          }}
        >
          {filenames.map((filename) => {
            return (
              <JackModule
                key={filename}
                tokens={tokensByFilename[filename]!}
                commands={vmCommands[filename]!}
                filename={filename}
                hoveredTokens={hoveredTokens}
                selectedTokenIdxs={selectedTokens}
                hoveredVMCommands={hoveredVMCommands}
                selectedVMCommands={selectedVMCommands}
                setHoveredTokenIdx={setDirectlyHoveredToken}
                setHoveredVMCommandIdx={setDirectlyHoveredVMCommand}
                setMouseSelectedTokenIdx={setDirectlySelectedToken}
                setMouseSelectedVMCommandIdx={setMouseSelectedVMCommandIdx}
                setDirectlySelectedInstructionIdxs={
                  setDirectlySelectedInstructionIdx
                }
                clearHoverState={clearHoverState}
              />
            );
          })}
        </div>
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
        programCounter={programCounter}
        onTick={onTick}
        onPlay={onPlay}
      />
    </>
  );
}
