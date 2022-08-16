use serde::Serialize;
use std::{collections::HashMap, ops::Range, rc::Rc};

use super::jack_node_types::{
    Class, ClassVarDeclaration, ClassVarDeclarationKind, Expression, Parameter, Statement,
    SubroutineBody, SubroutineCall, SubroutineDeclaration, VarDeclaration,
};

#[derive(Serialize)]
pub enum JackNode {
    ClassNode(#[serde(skip_serializing)] Rc<Class>),
    ClassVarDeclarationKindNode(#[serde(skip_serializing)] Rc<ClassVarDeclarationKind>),
    ClassVarDeclarationNode(#[serde(skip_serializing)] Rc<ClassVarDeclaration>),
    ExpressionNode(#[serde(skip_serializing)] Rc<Expression>),
    ParameterNode(#[serde(skip_serializing)] Rc<Parameter>),
    SubroutineCallNode(#[serde(skip_serializing)] Rc<SubroutineCall>),
    StatementNode(#[serde(skip_serializing)] Rc<Statement>),
    SubroutineBodyNode(#[serde(skip_serializing)] Rc<SubroutineBody>),
    SubroutineDeclarationNode(#[serde(skip_serializing)] Rc<SubroutineDeclaration>),
    VarDeclarationNode(#[serde(skip_serializing)] Rc<VarDeclaration>),
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
