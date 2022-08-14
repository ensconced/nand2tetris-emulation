use std::rc::Rc;

use serde::Serialize;
use ts_rs::TS;

#[derive(Serialize, TS, Debug, PartialEq, Eq)]
#[ts(export)]
#[ts(export_to = "../bindings/")]
pub struct Class {
    pub name: String,
    pub var_declarations: Vec<(Rc<ClassVarDeclaration>, usize)>,
    pub subroutine_declarations: Vec<(Rc<SubroutineDeclaration>, usize)>,
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
    pub qualifier: (Rc<ClassVarDeclarationKind>, usize),
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
    PrimitiveTerm(PrimitiveTermVariant),
    Binary {
        operator: BinaryOperator,
        lhs: (Rc<Expression>, usize),
        rhs: (Rc<Expression>, usize),
    },
    Unary {
        operator: UnaryOperator,
        operand: (Rc<Expression>, usize),
    },
    Variable(String),
    SubroutineCall(Rc<SubroutineCall>),
    ArrayAccess {
        var_name: String,
        index: (Rc<Expression>, usize),
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
        arguments: Vec<(Rc<Expression>, usize)>,
    },
    Method {
        this_name: String,
        method_name: String,
        arguments: Vec<(Rc<Expression>, usize)>,
    },
}

#[derive(Serialize, TS, Debug, PartialEq, Eq)]
#[ts(export)]
#[ts(export_to = "../bindings/")]
pub enum Statement {
    Let {
        var_name: String,
        array_index: Option<(Rc<Expression>, usize)>,
        value: (Rc<Expression>, usize),
    },
    If {
        condition: (Rc<Expression>, usize),
        if_statements: Vec<(Rc<Statement>, usize)>,
        else_statements: Option<Vec<(Rc<Statement>, usize)>>,
    },
    While {
        condition: (Rc<Expression>, usize),
        statements: Vec<(Rc<Statement>, usize)>,
    },
    Do(Rc<SubroutineCall>),
    Return(Option<(Rc<Expression>, usize)>),
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
    pub var_declarations: Vec<(Rc<VarDeclaration>, usize)>,
    pub statements: Vec<(Rc<Statement>, usize)>,
}
#[derive(Serialize, TS, Debug, PartialEq, Eq)]
#[ts(export)]
#[ts(export_to = "../bindings/")]
pub struct SubroutineDeclaration {
    pub subroutine_kind: SubroutineKind,
    pub return_type: Option<Type>,
    pub parameters: Vec<(Rc<Parameter>, usize)>,
    pub name: String,
    pub body: (Rc<SubroutineBody>, usize),
}
