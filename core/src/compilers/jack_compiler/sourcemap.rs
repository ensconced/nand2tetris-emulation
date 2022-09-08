use serde::Serialize;
use std::{collections::HashMap, ops::Range, path::Path};
use ts_rs::TS;

#[derive(Serialize, TS)]
#[ts(export)]
#[ts(export_to = "../bindings/")]
pub struct JackParserSourceMap {
    pub token_idx_to_jack_node_idxs: HashMap<String, HashMap<usize, Vec<usize>>>,
    pub jack_nodes: HashMap<String, Vec<NodeInfo>>,
}

fn stringify_filename(filename: &Path) -> String {
    filename.to_str().expect("filename is not valid utf8").to_owned()
}

impl JackParserSourceMap {
    pub fn new() -> Self {
        Self {
            token_idx_to_jack_node_idxs: HashMap::new(),
            jack_nodes: HashMap::new(),
        }
    }

    pub fn record_jack_node(&mut self, filename: &Path, token_range: Range<usize>, child_node_idxs: Vec<usize>) -> usize {
        let file_jack_nodes = self.jack_nodes.entry(stringify_filename(filename)).or_default();
        let node_idx = file_jack_nodes.len();
        file_jack_nodes.push(NodeInfo {
            token_range: token_range.clone(),
            child_node_idxs,
            index: node_idx,
        });

        let token_lookup = self.token_idx_to_jack_node_idxs.entry(stringify_filename(filename)).or_default();
        for token_idx in token_range {
            let token_jack_node_idxs = token_lookup.entry(token_idx).or_default();
            token_jack_node_idxs.push(node_idx);
        }
        node_idx
    }
}

#[derive(Serialize, TS)]
#[ts(export)]
#[ts(export_to = "../bindings/")]
pub struct NodeInfo {
    token_range: Range<usize>,
    child_node_idxs: Vec<usize>,
    index: usize,
}

#[derive(Serialize, TS)]
#[ts(export)]
#[ts(export_to = "../bindings/")]
pub struct JackCodegenSourceMap {
    pub jack_node_idx_to_vm_command_idx: HashMap<String, HashMap<usize, Vec<usize>>>,
    pub vm_command_idx_to_jack_node_idx: HashMap<String, HashMap<usize, usize>>,
}

impl JackCodegenSourceMap {
    pub fn new() -> Self {
        Self {
            jack_node_idx_to_vm_command_idx: HashMap::new(),
            vm_command_idx_to_jack_node_idx: HashMap::new(),
        }
    }

    pub fn record_vm_command(&mut self, filename: &Path, vm_command_idx: usize, jack_node_idx: usize) {
        let file_jack_node_idx_to_vm_command_idx = self.jack_node_idx_to_vm_command_idx.entry(stringify_filename(filename)).or_default();
        let jack_node_vm_command_idxs = file_jack_node_idx_to_vm_command_idx.entry(jack_node_idx).or_default();
        jack_node_vm_command_idxs.push(vm_command_idx);

        let file_vm_command_idx_to_jack_node_idx = self.vm_command_idx_to_jack_node_idx.entry(stringify_filename(filename)).or_default();
        file_vm_command_idx_to_jack_node_idx.insert(vm_command_idx, jack_node_idx);
    }
}

#[derive(Serialize, TS)]
#[ts(export)]
#[ts(export_to = "../bindings/")]
pub struct JackCompilerSourceMap {
    pub parser_sourcemap: JackParserSourceMap,
    pub codegen_sourcemap: JackCodegenSourceMap,
}
