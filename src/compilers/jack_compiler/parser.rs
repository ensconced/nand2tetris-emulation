use super::tokenizer::{
    KeywordTokenVariant::*,
    TokenKind::{self, *},
};
use crate::compilers::utils::{parser_utils::PeekableTokens, tokenizer::Token};

struct Class {
    name: String,
    var_declarations: Vec<ClassVarDeclaration>,
    subroutine_declarations: Vec<SubroutineDeclaration>,
}

enum ClassVarDeclarationQualifier {
    Static,
    Field,
}

enum Type {
    Int,
    Char,
    Boolean,
    ClassName(String),
}

struct ClassVarDeclaration {
    type_name: Type,
    qualifier: ClassVarDeclarationQualifier,
    var_names: Vec<String>,
}

enum SubroutineKind {
    Constructor,
    Function,
    Method,
}

struct Expression;
struct Parameter {
    type_name: Type,
    var_name: String,
}

enum SubroutineCall {
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

enum Statement {
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
struct VarDeclaration {
    type_name: Type,
    var_names: Vec<String>,
}
struct SubroutineBody {
    var_declarations: Vec<VarDeclaration>,
    statements: Vec<Statement>,
}
struct SubroutineDeclaration {
    subroutine_kind: SubroutineKind,
    return_type: Option<Type>,
    parameters: Vec<Parameter>,
    name: String,
    body: SubroutineBody,
}

fn take_class_keyword(tokens: &mut PeekableTokens<TokenKind>, line_number: usize) {
    todo!()
}
fn take_l_curly(tokens: &mut PeekableTokens<TokenKind>, line_number: usize) {
    todo!()
}
fn take_l_paren(tokens: &mut PeekableTokens<TokenKind>, line_number: usize) {
    todo!()
}
fn take_r_square_paren(tokens: &mut PeekableTokens<TokenKind>, line_number: usize) {
    todo!()
}
fn take_r_paren(tokens: &mut PeekableTokens<TokenKind>, line_number: usize) {
    todo!()
}
fn take_r_curly(tokens: &mut PeekableTokens<TokenKind>, line_number: usize) {
    todo!()
}
fn take_equals(tokens: &mut PeekableTokens<TokenKind>, line_number: usize) {
    todo!()
}
fn take_identifier(tokens: &mut PeekableTokens<TokenKind>, line_number: usize) -> String {
    todo!()
}

fn maybe_take_expression(
    tokens: &mut PeekableTokens<TokenKind>,
    line_number: usize,
) -> Option<Expression> {
    todo!()
}

fn take_expression(tokens: &mut PeekableTokens<TokenKind>, line_number: usize) -> Expression {
    maybe_take_expression(tokens, line_number).expect("expected expression")
}

fn take_expression_list(
    tokens: &mut PeekableTokens<TokenKind>,
    line_number: usize,
) -> Vec<Expression> {
    let mut result = Vec::new();
    if let Some(expression) = maybe_take_expression(tokens, line_number) {
        result.push(expression);
        while let Some(Token { kind: Comma, .. }) = tokens.next() {
            result.push(take_expression(tokens, line_number));
        }
    }
    result
}

fn take_subroutine_call(
    tokens: &mut PeekableTokens<TokenKind>,
    line_number: usize,
) -> SubroutineCall {
    let name = take_identifier(tokens, line_number);
    match tokens.peek() {
        Some(Token { kind: LParen, .. }) => {
            // Direct function call
            tokens.next(); // LParen
            let arguments = take_expression_list(tokens, line_number);
            take_r_paren(tokens, line_number);
            SubroutineCall::Direct {
                subroutine_name: name,
                arguments,
            }
        }
        Some(Token { kind: Dot, .. }) => {
            // Method call
            tokens.next(); // Dot
            let method_name = take_identifier(tokens, line_number);
            take_l_paren(tokens, line_number);
            let arguments = take_expression_list(tokens, line_number);
            take_r_paren(tokens, line_number);
            SubroutineCall::Method {
                this_name: name,
                method_name,
                arguments,
            }
        }
        _ => panic!("expected subroutine call. line: {}", line_number),
    }
}

fn take_subroutine_return_type(
    tokens: &mut PeekableTokens<TokenKind>,
    line_number: usize,
) -> Option<Type> {
    if let Some(Token {
        kind: Keyword(Void),
        ..
    }) = tokens.next()
    {
        None
    } else {
        Some(take_type(tokens, line_number))
    }
}

fn take_parameters(tokens: &mut PeekableTokens<TokenKind>, line_number: usize) -> Vec<Parameter> {
    let mut result = Vec::new();
    if let Some(type_name) = maybe_take_type(tokens, line_number) {
        let var_name = take_identifier(tokens, line_number);
        result.push(Parameter {
            type_name,
            var_name,
        });

        while let Some(Token { kind: Comma, .. }) = tokens.peek() {
            tokens.next(); // comma
            let type_name = take_type(tokens, line_number);
            let var_name = take_identifier(tokens, line_number);
            result.push(Parameter {
                type_name,
                var_name,
            });
        }
    }
    result
}

fn maybe_take_array_index(
    tokens: &mut PeekableTokens<TokenKind>,
    line_number: usize,
) -> Option<Expression> {
    if let Some(Token {
        kind: LSquareParen, ..
    }) = tokens.peek()
    {
        tokens.next();
        let expression = take_expression(tokens, line_number);
        take_r_square_paren(tokens, line_number);
        Some(expression)
    } else {
        None
    }
}

fn take_let_statement(tokens: &mut PeekableTokens<TokenKind>, line_number: usize) -> Statement {
    tokens.next(); // "let" keyword
    let var_name = take_identifier(tokens, line_number);
    let array_index = maybe_take_array_index(tokens, line_number);
    take_equals(tokens, line_number);
    let value = take_expression(tokens, line_number);
    take_semicolon(tokens, line_number);
    Statement::Let {
        var_name,
        array_index,
        value,
    }
}

fn take_statement_block(
    tokens: &mut PeekableTokens<TokenKind>,
    line_number: usize,
) -> Vec<Statement> {
    take_l_curly(tokens, line_number);
    let statements = take_statements(tokens, line_number);
    take_r_curly(tokens, line_number);
    statements
}

fn maybe_take_else_block(
    tokens: &mut PeekableTokens<TokenKind>,
    line_number: usize,
) -> Option<Vec<Statement>> {
    if let Some(Token {
        kind: Keyword(Else),
        ..
    }) = tokens.peek()
    {
        tokens.next(); // "else" keyword
        Some(take_statement_block(tokens, line_number))
    } else {
        None
    }
}

fn take_if_statement(tokens: &mut PeekableTokens<TokenKind>, line_number: usize) -> Statement {
    tokens.next(); // "if" keyword
    take_l_paren(tokens, line_number);
    let condition = take_expression(tokens, line_number);
    take_r_paren(tokens, line_number);
    let if_statements = take_statement_block(tokens, line_number);
    let else_statements = maybe_take_else_block(tokens, line_number);
    Statement::If {
        condition,
        if_statements,
        else_statements,
    }
}

fn take_while_statement(tokens: &mut PeekableTokens<TokenKind>, line_number: usize) -> Statement {
    tokens.next(); // "while" keyword
    take_l_paren(tokens, line_number);
    let expression = take_expression(tokens, line_number);
    take_r_paren(tokens, line_number);
    let statements = take_statement_block(tokens, line_number);
    Statement::While {
        condition: expression,
        statements,
    }
}

fn take_do_statement(tokens: &mut PeekableTokens<TokenKind>, line_number: usize) -> Statement {
    tokens.next(); // "do" keyword
    let subroutine_call = take_subroutine_call(tokens, line_number);
    take_semicolon(tokens, line_number);
    Statement::Do(subroutine_call)
}

fn take_return_statement(tokens: &mut PeekableTokens<TokenKind>, line_number: usize) -> Statement {
    tokens.next(); // "return" keyword
    let expression = maybe_take_expression(tokens, line_number);
    take_semicolon(tokens, line_number);
    Statement::Return(expression)
}

fn maybe_take_statement(
    tokens: &mut PeekableTokens<TokenKind>,
    line_number: usize,
) -> Option<Statement> {
    if let Some(Token {
        kind: Keyword(keyword),
        ..
    }) = tokens.peek()
    {
        match keyword {
            Let => Some(take_let_statement(tokens, line_number)),
            If => Some(take_if_statement(tokens, line_number)),
            While => Some(take_while_statement(tokens, line_number)),
            Do => Some(take_do_statement(tokens, line_number)),
            Return => Some(take_return_statement(tokens, line_number)),
            _ => None,
        }
    } else {
        None
    }
}

fn take_statements(tokens: &mut PeekableTokens<TokenKind>, line_number: usize) -> Vec<Statement> {
    let mut result = Vec::new();
    while let Some(statement) = maybe_take_statement(tokens, line_number) {
        result.push(statement);
    }
    result
}

fn take_var_declaration(
    tokens: &mut PeekableTokens<TokenKind>,
    line_number: usize,
) -> VarDeclaration {
    if let Some(Token {
        kind: Keyword(Var), ..
    }) = tokens.next()
    {
        let type_name = take_type(tokens, line_number);
        let var_names = take_var_names(tokens, line_number);
        take_semicolon(tokens, line_number);
        VarDeclaration {
            type_name,
            var_names,
        }
    } else {
        panic!("expected var keyword. line: {}", line_number);
    }
}

fn take_var_declarations(
    tokens: &mut PeekableTokens<TokenKind>,
    line_number: usize,
) -> Vec<VarDeclaration> {
    let mut result = Vec::new();
    while let Some(Token {
        kind: Keyword(Var), ..
    }) = tokens.peek()
    {
        result.push(take_var_declaration(tokens, line_number));
    }
    result
}

fn take_subroutine_body(
    tokens: &mut PeekableTokens<TokenKind>,
    line_number: usize,
) -> SubroutineBody {
    take_l_curly(tokens, line_number);
    let var_declarations = take_var_declarations(tokens, line_number);
    let statements = take_statements(tokens, line_number);
    take_r_curly(tokens, line_number);
    SubroutineBody {
        var_declarations,
        statements,
    }
}

fn take_subroutine_declaration(
    tokens: &mut PeekableTokens<TokenKind>,
    line_number: usize,
) -> SubroutineDeclaration {
    if let Some(Token {
        kind: Keyword(keyword),
        ..
    }) = tokens.next()
    {
        let subroutine_kind = match keyword {
            Constructor => SubroutineKind::Constructor,
            Function => SubroutineKind::Function,
            Method => SubroutineKind::Method,
            _ => panic!("expected subroutine kind. line: {}", line_number),
        };

        let return_type = take_subroutine_return_type(tokens, line_number);
        let name = take_identifier(tokens, line_number);
        take_l_paren(tokens, line_number);
        let parameters = take_parameters(tokens, line_number);
        take_r_paren(tokens, line_number);
        let body = take_subroutine_body(tokens, line_number);

        SubroutineDeclaration {
            subroutine_kind,
            return_type,
            name,
            parameters,
            body,
        }
    } else {
        panic!("expected subroutine kind. line: {}", line_number);
    }
}

fn maybe_take_subroutine_declaration(
    tokens: &mut PeekableTokens<TokenKind>,
    line_number: usize,
) -> Option<SubroutineDeclaration> {
    if let Some(Token {
        kind: Keyword(Constructor | Function | Method),
        ..
    }) = tokens.peek()
    {
        take_subroutine_declaration(tokens, line_number)
    } else {
        None
    }
}

fn take_class_subroutine_declarations(
    tokens: &mut PeekableTokens<TokenKind>,
    line_number: usize,
) -> Vec<SubroutineDeclaration> {
    let mut result = Vec::new();
    while let Some(subroutine_declaration) = maybe_take_subroutine_declaration(tokens, line_number)
    {
        result.push(subroutine_declaration);
    }
    result
}

fn take_class_var_declaration_qualifier(
    tokens: &mut PeekableTokens<TokenKind>,
    line_number: usize,
) -> ClassVarDeclarationQualifier {
    match tokens.next() {
        Some(Token {
            kind: Keyword(keyword),
            ..
        }) => match keyword {
            Static => ClassVarDeclarationQualifier::Static,
            Field => ClassVarDeclarationQualifier::Field,
            _ => panic!(
                "expected var declaration qualifier at line: {}",
                line_number
            ),
        },
        _ => panic!(
            "expected var declaration qualifier at line: {}",
            line_number
        ),
    }
}

fn take_var_name(tokens: &mut PeekableTokens<TokenKind>, line_number: usize) -> String {
    if let Some(Token {
        kind: Identifier(var_name),
        ..
    }) = tokens.next()
    {
        var_name
    } else {
        panic!("expected var name at line: {}", line_number)
    }
}

fn take_var_names(tokens: &mut PeekableTokens<TokenKind>, line_number: usize) -> Vec<String> {
    // There has to be at least one var name.
    let mut result = vec![take_var_name(tokens, line_number)];
    while let Some(Token { kind: Comma, .. }) = tokens.peek() {
        tokens.next(); // comma
        result.push(take_var_name(tokens, line_number));
    }
    result
}

fn take_type(tokens: &mut PeekableTokens<TokenKind>, line_number: usize) -> Type {
    match tokens.next() {
        Some(Token { kind, .. }) => match kind {
            Keyword(Int) => Type::Int,
            Keyword(Char) => Type::Char,
            Keyword(Boolean) => Type::Boolean,
            Identifier(class_name) => Type::ClassName(class_name),
            _ => panic!("expected var type name at line: {}", line_number),
        },
        _ => panic!("expected var type name at line: {}", line_number),
    }
}

fn maybe_take_type(tokens: &mut PeekableTokens<TokenKind>, line_number: usize) -> Option<Type> {
    if let Some(
        Token {
            kind: Keyword(Int | Char | Boolean) | Identifier(_),
            ..
        },
        ..,
    ) = tokens.peek()
    {
        Some(take_type(tokens, line_number))
    } else {
        None
    }
}

fn take_semicolon(tokens: &mut PeekableTokens<TokenKind>, line_number: usize) {
    if let Some(Token {
        kind: Semicolon, ..
    }) = tokens.next()
    {
        // all good
    } else {
        panic!("expected semicolon at line: {}", line_number)
    }
}

fn take_class_var_declaration(
    tokens: &mut PeekableTokens<TokenKind>,
    line_number: usize,
) -> ClassVarDeclaration {
    let qualifier = take_class_var_declaration_qualifier(tokens, line_number);
    let type_name = take_type(tokens, line_number);
    let var_names = take_var_names(tokens, line_number);
    take_semicolon(tokens, line_number);
    ClassVarDeclaration {
        qualifier,
        type_name,
        var_names,
    }
}

fn maybe_take_class_var_declaration(
    tokens: &mut PeekableTokens<TokenKind>,
    line_number: usize,
) -> Option<ClassVarDeclaration> {
    match tokens.peek().expect("unexpected end of input") {
        Token {
            kind: Keyword(Static | Field),
            ..
        } => Some(take_class_var_declaration(tokens, line_number)),
        _ => None,
    }
}

fn take_class_var_declarations(
    tokens: &mut PeekableTokens<TokenKind>,
    line_number: usize,
) -> Vec<ClassVarDeclaration> {
    let mut result = Vec::new();
    while let Some(class_var_declaration) = maybe_take_class_var_declaration(tokens, line_number) {
        result.push(class_var_declaration);
    }
    result
}

// TODO - tokens should already have whitespace and comments filtered out
fn take_class(tokens: &mut PeekableTokens<TokenKind>, line_number: usize) -> Class {
    take_class_keyword(tokens, line_number);
    let name = take_identifier(tokens, line_number);
    take_l_curly(tokens, line_number);
    let var_declarations = take_class_var_declarations(tokens, line_number);
    let subroutine_declarations = take_class_subroutine_declarations(tokens, line_number);
    take_r_curly(tokens, line_number);
    Class {
        name,
        var_declarations,
        subroutine_declarations,
    }
}
