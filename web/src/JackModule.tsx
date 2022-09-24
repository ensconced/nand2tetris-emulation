import React from "react";
import { FileIdx, NodeInfoId } from ".";
import { Token } from "../../bindings/Token";
import { TokenKind } from "../../bindings/TokenKind";
import CodePanel, { FileIdxs, SelectedItemIdxs } from "./code-panel";
import { findInnermostJackNode } from "./sourcemapUtils";

interface Props {
  filename: string;
  hidden: boolean;
  tokens: Array<Token<TokenKind>>;
  commands: Array<string>;
  hoveredTokens: FileIdxs | undefined;
  selectedTokenIdxs: SelectedItemIdxs;
  setHoveredTokenIdx: (tokenIdx: FileIdx) => void;
  clearHoverState: () => void;
  setMouseSelectedVMCommandIdx: (idx: FileIdx | undefined) => void;
  setHoveredVMCommandIdx: (idx: FileIdx | undefined) => void;
  setMouseSelectedJackNode: (node: NodeInfoId | undefined) => void;
  hoveredVMCommands: FileIdxs | undefined;
  selectedVMCommands: SelectedItemIdxs;
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
  setMouseSelectedJackNode,
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
          filename={filename}
          items={tokenContents}
          hoveredItemIdxs={hoveredTokens}
          selectedItemIdxs={selectedTokenIdxs}
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
          selectedItemIdxs={selectedVMCommands}
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
