use serde::Serialize;
use std::{collections::HashMap, ops::Range};

#[derive(Serialize)]
pub struct NodeInfo {
    token_range: Range<usize>,
    child_node_idxs: Vec<usize>,
}

#[derive(Serialize)]
pub struct JackParserSourceMap {
    pub token_idx_to_jack_node_idxs: HashMap<usize, Vec<usize>>,
    pub node_infos: Vec<NodeInfo>,
}

impl JackParserSourceMap {
    pub fn new() -> Self {
        Self {
            token_idx_to_jack_node_idxs: HashMap::new(),
            node_infos: Vec::new(),
        }
    }

    pub fn record_node(&mut self, token_range: Range<usize>, child_node_idxs: Vec<usize>) -> usize {
        let node_idx = self.node_infos.len();
        self.node_infos.push(NodeInfo {
            token_range: token_range.clone(),
            child_node_idxs,
        });
        for token_idx in token_range {
            let token_jack_node_idxs = self.token_idx_to_jack_node_idxs.entry(token_idx).or_default();
            token_jack_node_idxs.push(node_idx);
        }
        node_idx
    }
}

#[derive(Serialize)]
pub struct VMCodegenSourceMap {
    pub jack_node_idx_to_vm_command_idx: HashMap<usize, Range<usize>>,
    vm_command_idx_to_jack_node_idx: HashMap<usize, usize>,
}

impl VMCodegenSourceMap {
    pub fn new() -> Self {
        Self {
            jack_node_idx_to_vm_command_idx: HashMap::new(),
            vm_command_idx_to_jack_node_idx: HashMap::new(),
        }
    }
}
