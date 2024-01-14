use std::ops::Range;

use serde::Serialize;
use ts_rs::TS;

#[derive(Serialize, TS, Debug, PartialEq, Eq)]
#[ts(export)]
#[ts(export_to = "../web/bindings/")]
pub struct ASTNode<T> {
    pub node: Box<T>,
    pub node_idx: usize,
    pub token_range: Range<usize>,
}

#[derive(Serialize, TS, Debug, PartialEq, Eq)]
#[ts(export)]
#[ts(export_to = "../web/bindings/")]
pub struct Class {
    pub name: String,
    pub var_declarations: Vec<ASTNode<ClassVarDeclaration>>,
    pub subroutine_declarations: Vec<ASTNode<SubroutineDeclaration>>,
}

#[derive(Serialize, TS, Debug, PartialEq, Eq)]
#[ts(export)]
#[ts(export_to = "../web/bindings/")]
pub enum ClassVarDeclarationKind {
    Static,
    Field,
}

#[derive(Serialize, TS, Clone, Debug, PartialEq, Eq)]
#[ts(export)]
#[ts(export_to = "../web/bindings/")]
pub enum Type {
    Int,
    Char,
    Boolean,
    ClassName(String),
}

#[derive(Serialize, TS, Debug, PartialEq, Eq)]
#[ts(export)]
#[ts(export_to = "../web/bindings/")]
pub struct ClassVarDeclaration {
    pub type_name: Type,
    pub qualifier: ASTNode<ClassVarDeclarationKind>,
    pub var_names: Vec<String>,
}

#[derive(Serialize, TS, Copy, Clone, Debug, PartialEq, Eq)]
#[ts(export)]
#[ts(export_to = "../web/bindings/")]
pub enum SubroutineKind {
    Constructor,
    Function,
    Method,
}

#[derive(Serialize, TS, Debug, PartialEq, Eq)]
#[ts(export)]
#[ts(export_to = "../web/bindings/")]
pub enum NumericConstantVariant {
    IntegerDecimalConstant(String),
    IntegerBinaryConstant(String),
}

#[derive(Serialize, TS, Debug, PartialEq, Eq)]
#[ts(export)]
#[ts(export_to = "../web/bindings/")]
pub enum PrimitiveTermVariant {
    NumericConstant(NumericConstantVariant),
    StringConstant(String),
    True,
    False,
    Null,
    This,
}

#[derive(Serialize, TS, Debug, PartialEq, Eq)]
#[ts(export)]
#[ts(export_to = "../web/bindings/")]
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
#[ts(export_to = "../web/bindings/")]
pub enum UnaryOperator {
    Minus,
    Not,
}

#[derive(Serialize, TS, Debug, PartialEq, Eq)]
#[ts(export)]
#[ts(export_to = "../web/bindings/")]
pub enum Expression {
    Parenthesized(ASTNode<Expression>),
    PrimitiveTerm(PrimitiveTermVariant),
    Binary {
        operator: BinaryOperator,
        lhs: ASTNode<Expression>,
        rhs: ASTNode<Expression>,
    },
    Unary {
        operator: UnaryOperator,
        operand: ASTNode<Expression>,
    },
    Variable(String),
    SubroutineCall(ASTNode<SubroutineCall>),
    ArrayAccess {
        var_name: String,
        index: ASTNode<Expression>,
    },
}

#[derive(Serialize, TS, Debug, PartialEq, Eq)]
#[ts(export)]
#[ts(export_to = "../web/bindings/")]
pub struct Parameter {
    pub type_name: Type,
    pub var_name: String,
}

#[derive(Serialize, TS, Debug, PartialEq, Eq)]
#[ts(export)]
#[ts(export_to = "../web/bindings/")]
pub enum SubroutineCall {
    Direct {
        subroutine_name: String,
        arguments: Vec<ASTNode<Expression>>,
    },
    Method {
        this_name: String,
        method_name: String,
        arguments: Vec<ASTNode<Expression>>,
    },
}

#[derive(Serialize, TS, Debug, PartialEq, Eq)]
#[ts(export)]
#[ts(export_to = "../web/bindings/")]
pub enum Statement {
    Block(Vec<ASTNode<Statement>>),
    Let {
        var_name: String,
        array_index: Option<ASTNode<Expression>>,
        value: ASTNode<Expression>,
    },
    If {
        condition: ASTNode<Expression>,
        if_statement: ASTNode<Statement>,
        else_statement: Option<ASTNode<Statement>>,
    },
    While {
        condition: ASTNode<Expression>,
        statement: ASTNode<Statement>,
    },
    Do(ASTNode<SubroutineCall>),
    Return(Option<ASTNode<Expression>>),
}
#[derive(Serialize, TS, Debug, PartialEq, Eq)]
#[ts(export)]
#[ts(export_to = "../web/bindings/")]
pub struct VarDeclaration {
    pub type_name: Type,
    pub var_names: Vec<String>,
}

#[derive(Serialize, TS, Debug, PartialEq, Eq)]
#[ts(export)]
#[ts(export_to = "../web/bindings/")]
pub struct SubroutineBody {
    pub var_declarations: Vec<ASTNode<VarDeclaration>>,
    pub statements: Vec<ASTNode<Statement>>,
}
#[derive(Serialize, TS, Debug, PartialEq, Eq)]
#[ts(export)]
#[ts(export_to = "../web/bindings/")]
pub struct SubroutineDeclaration {
    pub subroutine_kind: SubroutineKind,
    pub return_type: Option<Type>,
    pub parameters: Vec<ASTNode<Parameter>>,
    pub name: String,
    pub body: ASTNode<SubroutineBody>,
}
