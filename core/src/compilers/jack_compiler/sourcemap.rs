use std::collections::HashMap;

pub struct JackNode {
    id: usize,
}

pub struct SourceMap {
    jack_node_idx_to_token_idx: HashMap<usize, usize>,
    // jack_node_idx_to_vm_command_idx: HashMap<usize, usize>,
    // token_idx_to_jack_node_idx: HashMap<usize, usize>,
    // vm_command_idx_to_jack_node_idx: HashMap<usize, usize>,
}

impl SourceMap {
    pub fn new() -> Self {
        SourceMap {
            jack_node_idx_to_token_idx: HashMap::new(),
            // jack_node_idx_to_vm_command_idx: HashMap::new(),
            // token_idx_to_jack_node_idx: HashMap::new(),
            // vm_command_idx_to_jack_node_idx: HashMap::new(),
        }
    }
}
