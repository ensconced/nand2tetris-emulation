use serde::Serialize;
use std::{collections::HashMap, ops::Range};
use ts_rs::TS;

#[derive(Serialize, TS)]
#[ts(export)]
#[ts(export_to = "../bindings/")]
pub struct NodeInfo {
    token_range: Range<usize>,
    child_node_idxs: Vec<usize>,
}

#[derive(Serialize, TS)]
#[ts(export)]
#[ts(export_to = "../bindings/")]
pub struct SourceMap {
    pub token_idx_to_jack_node_idxs: HashMap<usize, Vec<usize>>,
    pub jack_nodes: Vec<NodeInfo>,
    pub jack_node_idx_to_vm_command_idx: HashMap<usize, Vec<usize>>,
    pub vm_command_idx_to_jack_node_idx: HashMap<usize, usize>,
}

impl SourceMap {
    pub fn new() -> Self {
        Self {
            token_idx_to_jack_node_idxs: HashMap::new(),
            jack_nodes: Vec::new(),
            jack_node_idx_to_vm_command_idx: HashMap::new(),
            vm_command_idx_to_jack_node_idx: HashMap::new(),
        }
    }

    pub fn record_jack_node(&mut self, token_range: Range<usize>, child_node_idxs: Vec<usize>) -> usize {
        let node_idx = self.jack_nodes.len();
        self.jack_nodes.push(NodeInfo {
            token_range: token_range.clone(),
            child_node_idxs,
        });
        for token_idx in token_range {
            let token_jack_node_idxs = self.token_idx_to_jack_node_idxs.entry(token_idx).or_default();
            token_jack_node_idxs.push(node_idx);
        }
        node_idx
    }

    pub fn record_vm_command(&mut self, vm_command_idx: usize, jack_node_idx: usize) {
        let jack_node_vm_command_idxs = self.jack_node_idx_to_vm_command_idx.entry(jack_node_idx).or_default();
        jack_node_vm_command_idxs.push(vm_command_idx);

        self.vm_command_idx_to_jack_node_idx.insert(vm_command_idx, jack_node_idx);
    }
}
