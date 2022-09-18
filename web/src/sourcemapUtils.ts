import { FileIdx, NodeInfoId } from ".";
import { CompilerResult } from "../../bindings/CompilerResult";
import { NodeInfo } from "../../bindings/NodeInfo";
import data from "../debug-output.json";
import { FileIdxs } from "./code-panel";
import _ from "lodash";

const compilerResult = data as CompilerResult;
const {
  jack_compiler_result: {
    sourcemaps: jackCompilerSourcemaps,
    tokens: tokensByFilename,
    vm_commands: vmCommands,
  },
  vm_compiler_result: { sourcemap: vmCompilerSourcemap },
} = compilerResult;

export const filenames = Object.keys(tokensByFilename);

export { tokensByFilename, vmCommands };

export function jackNodeTokens(
  node: NodeInfoId | undefined
): FileIdxs | undefined {
  if (node === undefined) return undefined;
  const tokenSet: Set<number> = new Set();
  for (
    let i = node.node.token_range.start;
    i < node.node.token_range.end;
    i++
  ) {
    const token = tokensByFilename[node.filename]?.[i];
    if (token === undefined) {
      throw new Error("failed to get token");
    }
    tokenSet.add(token.idx);
  }
  return { filename: node.filename, idxs: tokenSet };
}

export function getJackNodeByIndex(index: FileIdx): NodeInfo {
  const node =
    jackCompilerSourcemaps[index.filename]?.parser_sourcemap.jack_nodes[
      index.idx
    ];
  if (node === undefined) {
    throw new Error(`failed to get jack node at index ${index.idx}`);
  }
  return node;
}

export function findInnermostJackNode(
  tokenIdx: FileIdx
): NodeInfoId | undefined {
  const tokenJackNodesIdxs =
    jackCompilerSourcemaps[tokenIdx.filename]?.parser_sourcemap
      .token_idx_to_jack_node_idxs[tokenIdx.idx];
  if (!tokenJackNodesIdxs) return undefined;
  const tokenJackNodes = tokenJackNodesIdxs.map((jackNodeIdx) =>
    getJackNodeByIndex({ filename: tokenIdx.filename, idx: jackNodeIdx })
  );
  const smallestJackNode = _.minBy(
    tokenJackNodes,
    (jackNode) => jackNode.token_range.end - jackNode.token_range.start
  );
  if (smallestJackNode === undefined) {
    throw new Error("failed to find jack node for token");
  }
  return { filename: tokenIdx.filename, node: smallestJackNode };
}

function immediateVMCommandIdxs(jackNodeIdx: FileIdx): number[] {
  return (
    jackCompilerSourcemaps[jackNodeIdx.filename]?.codegen_sourcemap
      .jack_node_idx_to_vm_command_idx[jackNodeIdx.idx] ?? []
  );
}

export function vmCommandJackNodeIdx(vmCommand: FileIdx): FileIdx | undefined {
  const idx =
    jackCompilerSourcemaps[vmCommand.filename]?.codegen_sourcemap
      .vm_command_idx_to_jack_node_idx[vmCommand.idx];

  if (idx !== undefined) {
    return { filename: vmCommand.filename, idx };
  }
}

export function vmCommandToJackNode(vmCmd: FileIdx): NodeInfoId | undefined {
  const jackNodeIdx =
    jackCompilerSourcemaps[vmCmd.filename]?.codegen_sourcemap
      .vm_command_idx_to_jack_node_idx[vmCmd.idx];
  if (jackNodeIdx !== undefined) {
    return {
      filename: vmCmd.filename,
      node: getJackNodeByIndex({
        filename: vmCmd.filename,
        idx: jackNodeIdx,
      }),
    };
  }
}

export function allVMCommandIdxs(jackNodeIdx: FileIdx): number[] {
  return immediateVMCommandIdxs(jackNodeIdx).concat(
    getJackNodeByIndex(jackNodeIdx).child_node_idxs.flatMap((childNodeIdx) =>
      allVMCommandIdxs({ filename: jackNodeIdx.filename, idx: childNodeIdx })
    )
  );
}
