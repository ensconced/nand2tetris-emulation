import "./styles/reset.css";
import data from "../debug-output.json";
import { DebugOutput } from "../../bindings/DebugOutput";
import { BLACK, DARKISH, WHITE } from "./colors";
import _ from "lodash";
import { NodeInfo } from "../../bindings/NodeInfo";
import { TokenKind } from "../../bindings/TokenKind";
import { Token } from "../../bindings/Token";

const debugOutput = data as DebugOutput;

const codeBlock = document.querySelector("#main>code");
if (codeBlock === null) {
  throw new Error("code block is missing");
}

const highlightedTokens: Array<Token<TokenKind>> = [];

codeBlock.addEventListener("mouseover", (evt) => {
  if (
    evt.target instanceof HTMLSpanElement &&
    evt.target.classList.contains("token")
  ) {
    const tokenIdxAttr = evt.target.getAttribute("token-idx");
    if (tokenIdxAttr === null) throw new Error("failed to find token idx");
    const jackNode = findInnermostJackNode(parseInt(tokenIdxAttr, 10));
    if (jackNode) {
      highlightNode(jackNode);
    }
  }
});

function highlightToken(token: Token<TokenKind>) {
  const tokenSpan = codeBlock?.children[token.idx];
  if (!(tokenSpan instanceof HTMLSpanElement)) {
    throw new Error("failed to find token to highlight");
  }
  tokenSpan.style.backgroundColor = WHITE;
  tokenSpan.style.color = BLACK;
}

function unhighlightToken(token: Token<TokenKind>) {
  const tokenSpan = codeBlock?.children[token.idx];
  if (!(tokenSpan instanceof HTMLSpanElement)) {
    throw new Error("failed to find token to unhighlight");
  }
  tokenSpan.style.backgroundColor = DARKISH;
  tokenSpan.style.color = WHITE;
}

function highlightNode(jackNode: NodeInfo) {
  highlightedTokens.forEach(unhighlightToken);
  highlightedTokens.length = 0;
  for (let i = jackNode.token_range.start; i < jackNode.token_range.end; i++) {
    const token = debugOutput.tokens[i];
    if (token === undefined) {
      throw new Error("failed to get token");
    }
    highlightedTokens.push(token);
    highlightToken(token);
  }
}

function findInnermostJackNode(tokenIdx: number): NodeInfo | undefined {
  const tokenJackNodesIdxs =
    debugOutput.sourcemap.token_idx_to_jack_node_idxs[tokenIdx];
  if (!tokenJackNodesIdxs) return undefined;
  const tokenJackNodes = tokenJackNodesIdxs.map(
    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
    (jackNodeIdx) => debugOutput.sourcemap.jack_nodes[jackNodeIdx]!
  );
  const smallestJackNode = _.minBy(
    tokenJackNodes,
    (jackNode) => jackNode.token_range.end - jackNode.token_range.start
  );
  if (smallestJackNode === undefined) {
    throw new Error("failed to find jack node for token");
  }
  return smallestJackNode;
}

debugOutput.tokens.forEach((token) => {
  const span = document.createElement("span");
  span.innerText = token.source;
  span.style.color = WHITE;
  span.classList.add("token");
  span.setAttribute("token-idx", token.idx.toString());
  codeBlock.appendChild(span);
});

console.log(debugOutput);
