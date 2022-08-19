use serde::Serialize;
use std::{collections::HashMap, ops::Range};

#[derive(Serialize)]
pub struct JackNodeInfo {
    pub token_range: Range<usize>,
    pub node_type: JackNodeType,
}

#[derive(Serialize)]
pub enum ExpressionType {
    Parenthesized,
    PrimitiveTerm,
    Binary,
    Unary,
    Variable,
    SubroutineCall,
    ArrayAccess,
}

#[derive(Serialize)]
pub enum StatementType {
    Let,
    If,
    While,
    Do,
    Return,
}

#[derive(Serialize)]
pub enum JackNodeType {
    ClassNode,
    ClassVarDeclarationKindNode,
    ClassVarDeclarationNode,
    ExpressionNode(ExpressionType),
    ParameterNode,
    SubroutineCallNode,
    StatementNode(StatementType),
    SubroutineBodyNode,
    SubroutineDeclarationNode,
    VarDeclarationNode,
}

#[derive(Serialize)]
pub struct JackParserSourceMap {
    pub token_idx_to_jack_node_idxs: HashMap<usize, Vec<usize>>,
    pub jack_node_infos: Vec<JackNodeInfo>,
}

impl JackParserSourceMap {
    pub fn new() -> Self {
        Self {
            token_idx_to_jack_node_idxs: HashMap::new(),
            jack_node_infos: Vec::new(),
        }
    }

    pub fn record_node(&mut self, token_range: Range<usize>, node_type: JackNodeType) -> usize {
        let node_idx = self.jack_node_infos.len();
        self.jack_node_infos.push(JackNodeInfo {
            token_range: token_range.clone(),
            node_type,
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
