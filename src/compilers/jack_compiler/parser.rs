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

enum VarTypeName {
    Int,
    Char,
    Boolean,
    ClassName(String),
}

struct ClassVarDeclaration {
    type_name: VarTypeName,
    qualifier: ClassVarDeclarationQualifier,
    var_names: Vec<String>,
}

struct SubroutineDeclaration {}

fn take_class_keyword(tokens: &mut PeekableTokens<TokenKind>, line_number: usize) {
    todo!()
}
fn take_l_curly(tokens: &mut PeekableTokens<TokenKind>, line_number: usize) {
    todo!()
}
fn take_r_curly(tokens: &mut PeekableTokens<TokenKind>, line_number: usize) {
    todo!()
}
fn take_identifier(tokens: &mut PeekableTokens<TokenKind>, line_number: usize) -> String {
    todo!()
}

fn maybe_take_class_subroutine_declaration(
    tokens: &mut PeekableTokens<TokenKind>,
    line_number: usize,
) -> Option<SubroutineDeclaration> {
    todo!()
}

fn take_class_subroutine_declarations(
    tokens: &mut PeekableTokens<TokenKind>,
    line_number: usize,
) -> Vec<SubroutineDeclaration> {
    let mut result = Vec::new();
    while let Some(subroutine_declaration) =
        maybe_take_class_subroutine_declaration(tokens, line_number)
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

fn take_var_type_name(tokens: &mut PeekableTokens<TokenKind>, line_number: usize) -> VarTypeName {
    match tokens.next() {
        Some(Token { kind, .. }) => match kind {
            Keyword(Int) => VarTypeName::Int,
            Keyword(Char) => VarTypeName::Char,
            Keyword(Boolean) => VarTypeName::Boolean,
            Identifier(class_name) => VarTypeName::ClassName(class_name),
            _ => panic!("expected var type name at line: {}", line_number),
        },
        _ => panic!("expected var type name at line: {}", line_number),
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
    let type_name = take_var_type_name(tokens, line_number);
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
    let subroutine_declarations = take_subroutine_declarations(tokens, line_number);
    take_r_curly(tokens, line_number);
    Class {
        name,
        var_declarations,
        subroutine_declarations,
    }
}
