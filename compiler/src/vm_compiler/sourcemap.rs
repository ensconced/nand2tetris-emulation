use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use serde::Serialize;
use ts_rs::TS;

#[derive(Default, Serialize, TS)]
#[ts(export)]
#[ts(export_to = "../web/bindings/")]
pub struct VMCommandIdentifier {
    filename: PathBuf,
    vm_command_idx: usize,
}

#[derive(Default, Serialize, TS)]
#[ts(export)]
#[ts(export_to = "../web/bindings/")]
pub struct SourceMap {
    pub asm_instruction_idx_to_vm_cmd: HashMap<usize, VMCommandIdentifier>,
    pub vm_filename_and_idx_to_asm_instruction_idx: HashMap<PathBuf, HashMap<usize, Vec<usize>>>,
}

impl SourceMap {
    pub fn new() -> Self {
        Self {
            asm_instruction_idx_to_vm_cmd: HashMap::new(),
            vm_filename_and_idx_to_asm_instruction_idx: HashMap::new(),
        }
    }

    pub fn record_asm_instruction(&mut self, vm_filename: &Path, vm_command_idx: usize, asm_idx: usize) {
        self.asm_instruction_idx_to_vm_cmd.insert(
            asm_idx,
            VMCommandIdentifier {
                filename: vm_filename.to_owned(),
                vm_command_idx,
            },
        );

        let cmd_idx_to_asm_instruction_idx = self.vm_filename_and_idx_to_asm_instruction_idx.entry(vm_filename.into()).or_default();
        let asm_instructions = cmd_idx_to_asm_instruction_idx.entry(vm_command_idx).or_default();
        asm_instructions.push(asm_idx);
    }
}
