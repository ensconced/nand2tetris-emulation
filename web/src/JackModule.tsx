import React from "react";
import { FileIdx, NodeInfoId } from ".";
import { Token } from "../../bindings/Token";
import { TokenKind } from "../../bindings/TokenKind";
import CodePanel, { FileIdxs } from "./code-panel";
import { findInnermostJackNode } from "./sourcemapUtils";

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

export default function JackModule({
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
