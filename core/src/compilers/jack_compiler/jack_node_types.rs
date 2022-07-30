use serde::Serialize;
use ts_rs::TS;

#[derive(Serialize, TS, Debug, PartialEq)]
#[ts(export)]
#[ts(export_to = "../bindings/")]
pub struct Class {
    pub name: String,
    pub var_declarations: Vec<ClassVarDeclaration>,
    pub subroutine_declarations: Vec<SubroutineDeclaration>,
}

#[derive(Serialize, TS, Debug, PartialEq)]
#[ts(export)]
#[ts(export_to = "../bindings/")]
pub enum ClassVarDeclarationKind {
    Static,
    Field,
}

#[derive(Serialize, TS, Clone, Debug, PartialEq)]
#[ts(export)]
#[ts(export_to = "../bindings/")]
pub enum Type {
    Int,
    Char,
    Boolean,
    ClassName(String),
}

#[derive(Serialize, TS, Debug, PartialEq)]
#[ts(export)]
#[ts(export_to = "../bindings/")]
pub struct ClassVarDeclaration {
    pub type_name: Type,
    pub qualifier: ClassVarDeclarationKind,
    pub var_names: Vec<String>,
}

#[derive(Serialize, TS, Copy, Clone, Debug, PartialEq)]
#[ts(export)]
#[ts(export_to = "../bindings/")]
pub enum SubroutineKind {
    Constructor,
    Function,
    Method,
}

#[derive(Serialize, TS, Debug, PartialEq)]
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

#[derive(Serialize, TS, Debug, PartialEq)]
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

#[derive(Serialize, TS, Debug, PartialEq)]
#[ts(export)]
#[ts(export_to = "../bindings/")]
pub enum UnaryOperator {
    Minus,
    Not,
}

#[derive(Serialize, TS, Debug, PartialEq)]
#[ts(export)]
#[ts(export_to = "../bindings/")]
pub enum Expression {
    PrimitiveTerm(PrimitiveTermVariant),
    Binary {
        operator: BinaryOperator,
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    Unary {
        operator: UnaryOperator,
        operand: Box<Expression>,
    },
    Variable(String),
    SubroutineCall(SubroutineCall),
    ArrayAccess {
        var_name: String,
        index: Box<Expression>,
    },
}

#[derive(Serialize, TS, Debug, PartialEq)]
#[ts(export)]
#[ts(export_to = "../bindings/")]
pub struct Parameter {
    pub type_name: Type,
    pub var_name: String,
}

#[derive(Serialize, TS, Debug, PartialEq)]
#[ts(export)]
#[ts(export_to = "../bindings/")]
pub enum SubroutineCall {
    Direct {
        subroutine_name: String,
        arguments: Vec<Expression>,
    },
    Method {
        this_name: String,
        method_name: String,
        arguments: Vec<Expression>,
    },
}

#[derive(Serialize, TS, Debug, PartialEq)]
#[ts(export)]
#[ts(export_to = "../bindings/")]
pub enum Statement {
    Let {
        var_name: String,
        array_index: Option<Expression>,
        value: Expression,
    },
    If {
        condition: Expression,
        if_statements: Vec<Statement>,
        else_statements: Option<Vec<Statement>>,
    },
    While {
        condition: Expression,
        statements: Vec<Statement>,
    },
    Do(SubroutineCall),
    Return(Option<Expression>),
}
#[derive(Serialize, TS, Debug, PartialEq)]
#[ts(export)]
#[ts(export_to = "../bindings/")]
pub struct VarDeclaration {
    pub type_name: Type,
    pub var_names: Vec<String>,
}

#[derive(Serialize, TS, Debug, PartialEq)]
#[ts(export)]
#[ts(export_to = "../bindings/")]
pub struct SubroutineBody {
    pub var_declarations: Vec<VarDeclaration>,
    pub statements: Vec<Statement>,
}
#[derive(Serialize, TS, Debug, PartialEq)]
#[ts(export)]
#[ts(export_to = "../bindings/")]
pub struct SubroutineDeclaration {
    pub subroutine_kind: SubroutineKind,
    pub return_type: Option<Type>,
    pub parameters: Vec<Parameter>,
    pub name: String,
    pub body: SubroutineBody,
}
