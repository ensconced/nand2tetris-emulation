use serde::Serialize;
use std::{collections::HashMap, ops::Range};

#[derive(Serialize)]
pub struct JackNode {
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
    pub jack_node_idx_to_token_idx: HashMap<usize, Range<usize>>,
    pub token_idx_to_jack_node_idxs: HashMap<usize, Vec<usize>>,
}

impl JackParserSourceMap {
    pub fn new() -> Self {
        Self {
            jack_node_idx_to_token_idx: HashMap::new(),
            token_idx_to_jack_node_idxs: HashMap::new(),
        }
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
