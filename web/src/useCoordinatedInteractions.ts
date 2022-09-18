import { useMemo } from "react";
import { FileIdx, NodeInfoId } from ".";
import { CompilerResult } from "../../bindings/CompilerResult";

import data from "../debug-output.json";
import { FileIdxs } from "./code-panel";
import {
  allVMCommandIdxs,
  findInnermostJackNode,
  getJackNodeByIndex,
  jackNodeTokens,
} from "./sourcemapUtils";

const compilerResult = data as CompilerResult;
const {
  jack_compiler_result: { sourcemaps: jackCompilerSourcemaps },
  vm_compiler_result: { sourcemap: vmCompilerSourcemap },
} = compilerResult;

export default function useCoordinatedInteractions(
  directlyInteractedVMCmd: FileIdx | undefined,
  directlyInteractedToken: FileIdx | undefined,
  directlyInteractedInstructionIdx: number | undefined
) {
  function vmCommandToJackNode(vmCmd: FileIdx): NodeInfoId | undefined {
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

  const singleInteractedVMCmd = useMemo(() => {
    if (directlyInteractedInstructionIdx === undefined) return undefined;
    return vmCompilerSourcemap.asm_instruction_idx_to_vm_cmd[
      directlyInteractedInstructionIdx
    ];
  }, [directlyInteractedInstructionIdx]);

  const interactedJackNode = useMemo<NodeInfoId | undefined>(() => {
    if (directlyInteractedToken !== undefined) {
      const node = findInnermostJackNode(directlyInteractedToken);
      if (node !== undefined) {
        return node;
      }
    } else if (directlyInteractedVMCmd !== undefined) {
      return vmCommandToJackNode(directlyInteractedVMCmd);
    }
  }, [directlyInteractedToken, directlyInteractedVMCmd]);

  const contextualHoveredJackNode = useMemo(() => {
    if (singleInteractedVMCmd === undefined) return undefined;
    return vmCommandToJackNode({
      filename: singleInteractedVMCmd.filename,
      idx: singleInteractedVMCmd.vm_command_idx,
    });
  }, [singleInteractedVMCmd]);

  const interactedTokens = useMemo(
    () => jackNodeTokens(interactedJackNode),
    [interactedJackNode]
  );

  const interactedVMCommands = useMemo<FileIdxs | undefined>(() => {
    if (interactedJackNode) {
      return {
        filename: interactedJackNode.filename,
        idxs: new Set(
          allVMCommandIdxs({
            filename: interactedJackNode.filename,
            idx: interactedJackNode.node.index,
          })
        ),
      };
    }

    if (singleInteractedVMCmd) {
      return {
        filename: singleInteractedVMCmd.filename,
        idxs: new Set([singleInteractedVMCmd.vm_command_idx]),
      };
    }
  }, [interactedJackNode, singleInteractedVMCmd]);

  const interactedInstructionIdxs = useMemo<Set<number>>(() => {
    const result = new Set<number>();
    if (interactedVMCommands === undefined) {
      return directlyInteractedInstructionIdx === undefined
        ? new Set()
        : new Set([directlyInteractedInstructionIdx]);
    }
    const vmCommandIdxToASMInstructionIdxs =
      vmCompilerSourcemap.vm_filename_and_idx_to_asm_instruction_idx[
        interactedVMCommands.filename
      ];

    if (vmCommandIdxToASMInstructionIdxs === undefined) {
      throw new Error(
        `failed to find instruction idx lookup for ${interactedVMCommands.filename}`
      );
    }

    interactedVMCommands.idxs.forEach((vmCmdIdx) => {
      const asmInstructions = vmCommandIdxToASMInstructionIdxs[vmCmdIdx];
      if (asmInstructions === undefined) {
        throw new Error(
          `failed to find asm instructions for vm command idx ${vmCmdIdx} in file ${interactedVMCommands.filename}`
        );
      }
      asmInstructions.forEach((instruction) => result.add(instruction));
    });

    return result;
  }, [directlyInteractedInstructionIdx, interactedVMCommands]);

  return {
    interactedTokens,
    interactedVMCommands,
    interactedInstructionIdxs,
    interactedJackNode,
  };
}
