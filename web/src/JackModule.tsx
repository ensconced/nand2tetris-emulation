import React, { useMemo } from "react";
import { FileIdx } from ".";
import { Token } from "../../bindings/Token";
import { TokenKind } from "../../bindings/TokenKind";
import CodePanel, { FileIdxs, InteractedItemIdxs } from "./code-panel";

interface Props {
  filename: string;
  tokens: Array<Token<TokenKind>>;
  commands: Array<string>;
  hoveredTokens: FileIdxs | undefined;
  selectedTokenIdxs: InteractedItemIdxs;
  setHoveredTokenIdx: (tokenIdx: FileIdx) => void;
  clearHoverState: () => void;
  setMouseSelectedVMCommandIdx: (idx: FileIdx | undefined) => void;
  setDirectlySelectedInstructionIdxs: (idx: number | undefined) => void;
  setMouseSelectedTokenIdx: (idx: FileIdx | undefined) => void;
  setHoveredVMCommandIdx: (idx: FileIdx | undefined) => void;
  hoveredVMCommands: FileIdxs | undefined;
  selectedVMCommands: InteractedItemIdxs;
}

export default function JackModule({
  filename,
  tokens,
  commands,
  hoveredTokens,
  selectedTokenIdxs,
  setHoveredTokenIdx,
  clearHoverState,
  setMouseSelectedVMCommandIdx,
  setMouseSelectedTokenIdx,
  setDirectlySelectedInstructionIdxs,
  hoveredVMCommands,
  selectedVMCommands,
  setHoveredVMCommandIdx,
}: Props) {
  const tokenContents = useMemo(
    () => tokens.map((token) => token.source),
    tokens
  );
  const vmCommandsWithNewLines = useMemo(
    () => commands.map((command) => `${command}\n`),
    [commands]
  );

  return (
    <>
      <div style={{ minHeight: 0, display: "flex", height: "100%" }}>
        <CodePanel
          id={`${filename}-tokens`}
          filename={filename}
          items={tokenContents}
          hoveredItemIdxs={hoveredTokens}
          selectedItemIdxs={selectedTokenIdxs}
          onSpanMouseEnter={(idx) => setHoveredTokenIdx({ filename, idx })}
          onSpanMouseLeave={clearHoverState}
          onSpanClick={(idx) => {
            setDirectlySelectedInstructionIdxs(undefined);
            setMouseSelectedVMCommandIdx(undefined);
            setMouseSelectedTokenIdx({ filename, idx });
          }}
        />
        <CodePanel
          id={`${filename}-vmcommands`}
          filename={filename}
          items={vmCommandsWithNewLines}
          hoveredItemIdxs={hoveredVMCommands}
          selectedItemIdxs={selectedVMCommands}
          onSpanMouseEnter={(idx) => setHoveredVMCommandIdx({ filename, idx })}
          onSpanMouseLeave={clearHoverState}
          onSpanClick={(idx) => {
            setDirectlySelectedInstructionIdxs(undefined);
            setMouseSelectedTokenIdx(undefined);
            setMouseSelectedVMCommandIdx({ filename, idx });
          }}
        />
      </div>
    </>
  );
}
