use std::{ops::Range, rc::Rc};

use serde::Serialize;
use ts_rs::TS;

#[derive(Serialize, TS, Debug, PartialEq, Eq)]
#[ts(export)]
#[ts(export_to = "../bindings/")]
pub struct IndexedJackNode<T> {
    pub node: Rc<T>,
    pub node_idx: usize,
    pub token_range: Range<usize>,
}

#[derive(Serialize, TS, Debug, PartialEq, Eq)]
#[ts(export)]
#[ts(export_to = "../bindings/")]
pub struct Class {
    pub name: String,
    pub var_declarations: Vec<IndexedJackNode<ClassVarDeclaration>>,
    pub subroutine_declarations: Vec<IndexedJackNode<SubroutineDeclaration>>,
}

#[derive(Serialize, TS, Debug, PartialEq, Eq)]
#[ts(export)]
#[ts(export_to = "../bindings/")]
pub enum ClassVarDeclarationKind {
    Static,
    Field,
}

#[derive(Serialize, TS, Clone, Debug, PartialEq, Eq)]
#[ts(export)]
#[ts(export_to = "../bindings/")]
pub enum Type {
    Int,
    Char,
    Boolean,
    ClassName(String),
}

#[derive(Serialize, TS, Debug, PartialEq, Eq)]
#[ts(export)]
#[ts(export_to = "../bindings/")]
pub struct ClassVarDeclaration {
    pub type_name: Type,
    pub qualifier: IndexedJackNode<ClassVarDeclarationKind>,
    pub var_names: Vec<String>,
}

#[derive(Serialize, TS, Copy, Clone, Debug, PartialEq, Eq)]
#[ts(export)]
#[ts(export_to = "../bindings/")]
pub enum SubroutineKind {
    Constructor,
    Function,
    Method,
}

#[derive(Serialize, TS, Debug, PartialEq, Eq)]
#[ts(export)]
#[ts(export_to = "../bindings/")]
pub enum PrimitiveTermVariant {
    IntegerConstant(String),
    StringConstant(String),
    True,
    False,
    Null,
    This,
}

#[derive(Serialize, TS, Debug, PartialEq, Eq)]
#[ts(export)]
#[ts(export_to = "../bindings/")]
pub enum BinaryOperator {
    Plus,
    Minus,
    Multiply,
    Divide,
    And,
    Or,
    LessThan,
    LessThanOrEquals,
    GreaterThan,
    GreaterThanOrEquals,
    Equals,
}

#[derive(Serialize, TS, Debug, PartialEq, Eq)]
#[ts(export)]
#[ts(export_to = "../bindings/")]
pub enum UnaryOperator {
    Minus,
    Not,
}

#[derive(Serialize, TS, Debug, PartialEq, Eq)]
#[ts(export)]
#[ts(export_to = "../bindings/")]
pub enum Expression {
    Parenthesized(IndexedJackNode<Expression>),
    PrimitiveTerm(PrimitiveTermVariant),
    Binary {
        operator: BinaryOperator,
        lhs: IndexedJackNode<Expression>,
        rhs: IndexedJackNode<Expression>,
    },
    Unary {
        operator: UnaryOperator,
        operand: IndexedJackNode<Expression>,
    },
    Variable(String),
    SubroutineCall(IndexedJackNode<SubroutineCall>),
    ArrayAccess {
        var_name: String,
        index: IndexedJackNode<Expression>,
    },
}

#[derive(Serialize, TS, Debug, PartialEq, Eq)]
#[ts(export)]
#[ts(export_to = "../bindings/")]
pub struct Parameter {
    pub type_name: Type,
    pub var_name: String,
}

#[derive(Serialize, TS, Debug, PartialEq, Eq)]
#[ts(export)]
#[ts(export_to = "../bindings/")]
pub enum SubroutineCall {
    Direct {
        subroutine_name: String,
        arguments: Vec<IndexedJackNode<Expression>>,
    },
    Method {
        this_name: String,
        method_name: String,
        arguments: Vec<IndexedJackNode<Expression>>,
    },
}

#[derive(Serialize, TS, Debug, PartialEq, Eq)]
#[ts(export)]
#[ts(export_to = "../bindings/")]
pub enum Statement {
    Let {
        var_name: String,
        array_index: Option<IndexedJackNode<Expression>>,
        value: IndexedJackNode<Expression>,
    },
    If {
        condition: IndexedJackNode<Expression>,
        if_statements: Vec<IndexedJackNode<Statement>>,
        else_statements: Option<Vec<IndexedJackNode<Statement>>>,
    },
    While {
        condition: IndexedJackNode<Expression>,
        statements: Vec<IndexedJackNode<Statement>>,
    },
    Do(IndexedJackNode<SubroutineCall>),
    Return(Option<IndexedJackNode<Expression>>),
}
#[derive(Serialize, TS, Debug, PartialEq, Eq)]
#[ts(export)]
#[ts(export_to = "../bindings/")]
pub struct VarDeclaration {
    pub type_name: Type,
    pub var_names: Vec<String>,
}

#[derive(Serialize, TS, Debug, PartialEq, Eq)]
#[ts(export)]
#[ts(export_to = "../bindings/")]
pub struct SubroutineBody {
    pub var_declarations: Vec<IndexedJackNode<VarDeclaration>>,
    pub statements: Vec<IndexedJackNode<Statement>>,
}
#[derive(Serialize, TS, Debug, PartialEq, Eq)]
#[ts(export)]
#[ts(export_to = "../bindings/")]
pub struct SubroutineDeclaration {
    pub subroutine_kind: SubroutineKind,
    pub return_type: Option<Type>,
    pub parameters: Vec<IndexedJackNode<Parameter>>,
    pub name: String,
    pub body: IndexedJackNode<SubroutineBody>,
}
