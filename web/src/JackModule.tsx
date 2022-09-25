import React from "react";
import { FileIdx } from ".";
import { Token } from "../../bindings/Token";
import { TokenKind } from "../../bindings/TokenKind";
import CodePanel, { FileIdxs, InteractedItemIdxs } from "./code-panel";

interface Props {
  filename: string;
  hidden: boolean;
  tokens: Array<Token<TokenKind>>;
  commands: Array<string>;
  hoveredTokens: FileIdxs | undefined;
  selectedTokenIdxs: InteractedItemIdxs;
  setHoveredTokenIdx: (tokenIdx: FileIdx) => void;
  clearHoverState: () => void;
  setMouseSelectedVMCommandIdx: (idx: FileIdx | undefined) => void;
  setMouseSelectedTokenIdx: (idx: FileIdx | undefined) => void;
  setHoveredVMCommandIdx: (idx: FileIdx | undefined) => void;
  hoveredVMCommands: FileIdxs | undefined;
  selectedVMCommands: InteractedItemIdxs;
}

export default function JackModule({
  filename,
  tokens,
  commands,
  hidden,
  hoveredTokens,
  selectedTokenIdxs,
  setHoveredTokenIdx,
  clearHoverState,
  setMouseSelectedVMCommandIdx,
  setMouseSelectedTokenIdx,
  hoveredVMCommands,
  selectedVMCommands,
  setHoveredVMCommandIdx,
}: Props) {
  const tokenContents = tokens.map((token) => token.source);
  const vmCommandsWithNewLines = commands.map((command) => `${command}\n`);

  return (
    <>
      <div style={{ minHeight: 0, display: hidden ? "none" : "flex" }}>
        <CodePanel
          id={`${filename}-tokens`}
          filename={filename}
          items={tokenContents}
          hoveredItemIdxs={hoveredTokens}
          selectedItemIdxs={selectedTokenIdxs}
          onSpanMouseEnter={(idx) => setHoveredTokenIdx({ filename, idx })}
          onSpanMouseLeave={clearHoverState}
          onSpanClick={(idx) => {
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
            setMouseSelectedTokenIdx(undefined);
            setMouseSelectedVMCommandIdx({ filename, idx });
          }}
        />
      </div>
    </>
  );
}
