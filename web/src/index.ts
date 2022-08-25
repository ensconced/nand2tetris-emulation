import "./styles/reset.css";
import data from "../debug-output.json";
import { DebugOutput } from "../../bindings/DebugOutput";
import { DARKISH, WHITE } from "./colors";
import _ from "lodash";
import { NodeInfo } from "../../bindings/NodeInfo";
import { TokenKind } from "../../bindings/TokenKind";
import { Token } from "../../bindings/Token";

const debugOutput = data as DebugOutput;

const jackCodeElement = getElementById("jack-code");
const vmCommandsElement = getElementById("vm-commands");

const highlightedTokens: Array<Token<TokenKind>> = [];
const highlightedVMCommands: Array<HTMLSpanElement> = [];

function getElementById(id: string): HTMLElement {
  const element = document.getElementById(id);
  if (element === null) {
    throw new Error(`failed to find element with id ${id}`);
  }
  return element;
}

jackCodeElement.addEventListener("mouseover", (evt) => {
  if (
    evt.target instanceof HTMLSpanElement &&
    evt.target.classList.contains("token")
  ) {
    const tokenIdxAttr = evt.target.getAttribute("token-idx");
    if (tokenIdxAttr === null) throw new Error("failed to find token idx");
    const tokenIdx = parseInt(tokenIdxAttr, 10);
    const jackNode = findInnermostJackNode(tokenIdx);
    if (jackNode) {
      highlightNode(jackNode);
      highlightNodeVMCommands(jackNode.index);
    }
  }
});

function immediateVMCommandIdxs(jackNodeIdx: number): number[] {
  return (
    debugOutput.sourcemap.jack_node_idx_to_vm_command_idx[jackNodeIdx] ?? []
  );
}

function allVMCommandIdxs(jackNodeIdx: number): number[] {
  return immediateVMCommandIdxs(jackNodeIdx).concat(
    getJackNodeByIndex(jackNodeIdx).child_node_idxs.flatMap(allVMCommandIdxs)
  );
}

function highlightNodeVMCommands(jackNodeIdx: number) {
  highlightedVMCommands.forEach(unhighlightElement);
  highlightedVMCommands.length = 0;
  allVMCommandIdxs(jackNodeIdx).forEach((vmCommandIdx) => {
    const vmCommandSpan = vmCommandsElement.children[vmCommandIdx];
    if (!(vmCommandSpan instanceof HTMLSpanElement)) {
      throw new Error(
        `failed to find vm command span with index ${vmCommandIdx}`
      );
    }
    highlightElement(vmCommandSpan);
    highlightedVMCommands.push(vmCommandSpan);
  });
}

function highlightElement(element: HTMLElement) {
  element.style.backgroundColor = WHITE;
  element.style.color = DARKISH;
}

function unhighlightElement(element: HTMLElement) {
  element.style.backgroundColor = DARKISH;
  element.style.color = WHITE;
}

function highlightToken(token: Token<TokenKind>) {
  const tokenSpan = jackCodeElement?.children[token.idx];
  if (!(tokenSpan instanceof HTMLSpanElement)) {
    throw new Error("failed to find token to highlight");
  }
  highlightElement(tokenSpan);
}

function unhighlightToken(token: Token<TokenKind>) {
  const tokenSpan = jackCodeElement?.children[token.idx];
  if (!(tokenSpan instanceof HTMLSpanElement)) {
    throw new Error("failed to find token to unhighlight");
  }
  unhighlightElement(tokenSpan);
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

function getJackNodeByIndex(index: number): NodeInfo {
  const node = debugOutput.sourcemap.jack_nodes[index];
  if (node === undefined) {
    throw new Error(`failed to get jack node at index ${index}`);
  }
  return node;
}

function findInnermostJackNode(tokenIdx: number): NodeInfo | undefined {
  const tokenJackNodesIdxs =
    debugOutput.sourcemap.token_idx_to_jack_node_idxs[tokenIdx];
  if (!tokenJackNodesIdxs) return undefined;
  const tokenJackNodes = tokenJackNodesIdxs.map(getJackNodeByIndex);
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
  jackCodeElement.appendChild(span);
});

debugOutput.vm_commands.forEach((command, idx) => {
  const span = document.createElement("span");
  span.innerText = `${command}\n`;
  span.style.color = WHITE;
  span.classList.add("command");
  span.setAttribute("command-idx", idx.toString());
  vmCommandsElement.appendChild(span);
});

console.log(debugOutput);
