use std::{collections::HashMap, ops::Range, rc::Rc};

use super::jack_node_types::{
    Class, ClassVarDeclaration, ClassVarDeclarationKind, Expression, Parameter, Statement,
    SubroutineBody, SubroutineCall, SubroutineDeclaration, VarDeclaration,
};

pub enum JackNode {
    ClassNode(Rc<Class>),
    ClassVarDeclarationKindNode(Rc<ClassVarDeclarationKind>),
    ClassVarDeclarationNode(Rc<ClassVarDeclaration>),
    ExpressionNode(Rc<Expression>),
    ParameterNode(Rc<Parameter>),
    SubroutineCallNode(Rc<SubroutineCall>),
    StatementNode(Rc<Statement>),
    SubroutineBodyNode(Rc<SubroutineBody>),
    SubroutineDeclarationNode(Rc<SubroutineDeclaration>),
    VarDeclarationNode(Rc<VarDeclaration>),
}

pub struct SourceMap {
    pub jack_node_idx_to_token_idx: HashMap<usize, Range<usize>>,
    pub token_idx_to_jack_node_idxs: HashMap<usize, Vec<usize>>,
    // jack_node_idx_to_vm_command_idx: HashMap<usize, usize>,
    // vm_command_idx_to_jack_node_idx: HashMap<usize, usize>,
}

impl SourceMap {
    pub fn new() -> Self {
        SourceMap {
            jack_node_idx_to_token_idx: HashMap::new(),
            token_idx_to_jack_node_idxs: HashMap::new(),
            // jack_node_idx_to_vm_command_idx: HashMap::new(),
            // token_idx_to_jack_node_idx: HashMap::new(),
            // vm_command_idx_to_jack_node_idx: HashMap::new(),
        }
    }
}
