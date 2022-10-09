import { useMemo } from "react";
import { FileIdx, NodeInfoId } from ".";
import { CompilerResult } from "../bindings/CompilerResult";

import data from "../debug-output.json";
import { InteractedInstructionIdxs, InteractedItemIdxs } from "./code-panel";
import {
  allVMCommandIdxs,
  findInnermostJackNode,
  jackNodeTokens,
  vmCommandToJackNode,
} from "./sourcemapUtils";

const compilerResult = data as CompilerResult;
const {
  vm_compiler_result: { sourcemap: vmCompilerSourcemap },
} = compilerResult;

interface InteractedItems {
  interactedTokens: InteractedItemIdxs;
  interactedVMCommands: InteractedItemIdxs;
  interactedInstructionIdxs: InteractedInstructionIdxs;
  interactedJackNode: NodeInfoId | undefined;
  interactedFilename: string | undefined;
}

export default function useCoordinatedInteractions(
  id: string,
  directlyInteractedVMCmd: FileIdx | undefined,
  directlyInteractedToken: FileIdx | undefined,
  directlyInteractedInstructionIdx: number | undefined
): InteractedItems {
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

  const contextualInteractedJackNode = useMemo(() => {
    if (singleInteractedVMCmd === undefined) return undefined;
    return vmCommandToJackNode({
      filename: singleInteractedVMCmd.filename,
      idx: singleInteractedVMCmd.vm_command_idx,
    });
  }, [singleInteractedVMCmd]);

  const interactedTokens = useMemo<InteractedItemIdxs>(() => {
    const tokens = jackNodeTokens(interactedJackNode);
    if (tokens === undefined) return undefined;
    return {
      ...tokens,
      auto: directlyInteractedToken === undefined,
    };
  }, [interactedJackNode, directlyInteractedToken]);

  const interactedVMCommands = useMemo<InteractedItemIdxs>(() => {
    if (interactedJackNode) {
      return {
        filename: interactedJackNode.filename,
        idxs: new Set(
          allVMCommandIdxs({
            filename: interactedJackNode.filename,
            idx: interactedJackNode.node.index,
          })
        ),
        auto: directlyInteractedVMCmd === undefined,
      };
    }

    if (singleInteractedVMCmd) {
      return {
        filename: singleInteractedVMCmd.filename,
        idxs: new Set([singleInteractedVMCmd.vm_command_idx]),
        auto: directlyInteractedVMCmd === undefined,
      };
    }
  }, [interactedJackNode, singleInteractedVMCmd, directlyInteractedVMCmd]);

  const interactedInstructionIdxs = useMemo(() => {
    if (interactedVMCommands === undefined) {
      return undefined;
      // return directlyInteractedInstructionIdx === undefined
      //   ? undefined
      //   : { idxs: new Set([directlyInteractedInstructionIdx]), auto: false };
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

    const result = new Set<number>();
    interactedVMCommands.idxs.forEach((vmCmdIdx) => {
      const asmInstructions = vmCommandIdxToASMInstructionIdxs[vmCmdIdx];
      if (asmInstructions === undefined) {
        throw new Error(
          `failed to find asm instructions for vm command idx ${vmCmdIdx} in file ${interactedVMCommands.filename}`
        );
      }
      asmInstructions.forEach((instruction) => result.add(instruction));
    });

    return {
      idxs: result,
      auto: directlyInteractedInstructionIdx === undefined,
    };
  }, [directlyInteractedInstructionIdx, interactedVMCommands]);

  const interactedFilename = useMemo(() => {
    return interactedVMCommands?.filename;
  }, [interactedVMCommands]);

  return {
    interactedTokens,
    interactedVMCommands,
    interactedInstructionIdxs,
    interactedJackNode,
    interactedFilename,
  };
}
