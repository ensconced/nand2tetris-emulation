use std::{collections::HashMap, ops::Range, rc::Rc};

use super::jack_node_types::{
    Class, Expression, Statement, SubroutineBody, SubroutineCall, SubroutineDeclaration,
};

pub enum JackNode {
    ClassNode(Rc<Class>),
    ExpressionNode(Rc<Expression>),
    SubroutineCallNode(Rc<SubroutineCall>),
    StatementNode(Rc<Statement>),
    SubroutineBodyNode(Rc<SubroutineBody>),
    SubroutineDeclarationNode(Rc<SubroutineDeclaration>),
}

pub struct SourceMap {
    pub jack_node_idx_to_token_idx: HashMap<usize, Range<usize>>,
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
