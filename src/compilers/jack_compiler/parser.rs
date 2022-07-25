use std::ops::Range;

use super::tokenizer::{
    token_defs,
    KeywordTokenVariant::{self, *},
    OperatorVariant::{self, *},
    TokenKind::{self, *},
};
use crate::compilers::utils::{
    parser_utils::PeekableTokens,
    tokenizer::{Token, Tokenizer},
};

pub enum JackNode<'a> {
    ExpressionNode(&'a Expression),
    SubroutineCall(&'a SubroutineCall),
    SubroutineDeclaration(&'a SubroutineDeclaration),
    BinaryOperatorNode(&'a BinaryOperator),
    UnaryOperatorNode(&'a UnaryOperator),
    PrimitiveTermNode(&'a PrimitiveTermVariant),
    StatementNode(&'a Statement),
}

#[derive(Debug, PartialEq)]
pub struct Class {
    pub name: Identifier,
    pub var_declarations: Vec<ClassVarDeclaration>,
    pub subroutine_declarations: Vec<SubroutineDeclaration>,
    source_byte_range: Range<usize>,
}

#[derive(Debug, PartialEq)]
pub enum ClassVarDeclarationKindVariant {
    Static,
    Field,
}

#[derive(Debug, PartialEq)]
pub struct ClassVarDeclarationKind {
    pub variant: ClassVarDeclarationKindVariant,
    source_byte_range: Range<usize>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TypeVariant {
    Int,
    Char,
    Boolean,
    ClassName(Identifier),
}

#[derive(Debug, PartialEq)]
pub struct Type {
    pub variant: TypeVariant,
    source_byte_range: Range<usize>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Identifier {
    pub name: String,
    source_byte_range: Range<usize>,
}

#[derive(Debug, PartialEq)]
pub struct ClassVarDeclaration {
    pub type_name: Type,
    pub qualifier: ClassVarDeclarationKind,
    pub var_names: VarNames,
    source_byte_range: Range<usize>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SubroutineKind {
    Constructor,
    Function,
    Method,
}

#[derive(Debug, PartialEq)]
pub enum PrimitiveTermVariant {
    IntegerConstant(String),
    StringConstant(String),
    True,
    False,
    Null,
    This,
}
use PrimitiveTermVariant::*;

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub enum UnaryOperator {
    Minus,
    Not,
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub struct Parameter {
    pub type_name: Type,
    pub var_name: Identifier,
    source_byte_range: Range<usize>,
}

#[derive(Debug, PartialEq)]
pub enum SubroutineCall {
    Direct {
        subroutine_name: Identifier,
        arguments: Vec<Expression>,
    },
    Method {
        this_name: Identifier,
        method_name: Identifier,
        arguments: Vec<Expression>,
    },
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    Let {
        var_name: Identifier,
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
#[derive(Debug, PartialEq)]
pub struct VarNames {
    pub names: Vec<Identifier>,
    source_byte_range: Range<usize>,
}

#[derive(Debug, PartialEq)]
pub struct VarDeclaration {
    pub type_name: Type,
    pub var_names: VarNames,
    source_byte_range: Range<usize>,
}

#[derive(Debug, PartialEq)]
pub struct SubroutineBody {
    pub var_declarations: Vec<VarDeclaration>,
    pub statements: Vec<Statement>,
    source_byte_range: Range<usize>,
}
#[derive(Debug, PartialEq)]
pub struct SubroutineDeclaration {
    pub subroutine_kind: SubroutineKind,
    pub return_type: Option<Type>,
    pub parameters: Vec<Parameter>,
    pub name: Identifier,
    pub body: SubroutineBody,
    source_byte_range: Range<usize>,
}

fn maybe_take_primitive_expression(tokens: &mut PeekableTokens<TokenKind>) -> Option<Expression> {
    let peeked_token = tokens.peek().cloned();
    peeked_token.and_then(|token| match token.kind {
        IntegerLiteral(string) => {
            tokens.next();
            Some(Expression::PrimitiveTerm(IntegerConstant(string)))
        }
        StringLiteral(string) => {
            tokens.next();
            Some(Expression::PrimitiveTerm(StringConstant(string)))
        }
        Keyword(KeywordTokenVariant::True) => {
            tokens.next();
            Some(Expression::PrimitiveTerm(PrimitiveTermVariant::True))
        }
        Keyword(KeywordTokenVariant::False) => {
            tokens.next();
            Some(Expression::PrimitiveTerm(PrimitiveTermVariant::False))
        }
        Keyword(KeywordTokenVariant::Null) => {
            tokens.next();
            Some(Expression::PrimitiveTerm(PrimitiveTermVariant::Null))
        }
        Keyword(KeywordTokenVariant::This) => {
            tokens.next();
            Some(Expression::PrimitiveTerm(PrimitiveTermVariant::This))
        }
        _ => None,
    })
}

fn take_array_access(tokens: &mut PeekableTokens<TokenKind>, var_name: String) -> Expression {
    take_token(tokens, LSquareBracket);
    let index = take_expression(tokens);
    take_token(tokens, RSquareBracket);
    Expression::ArrayAccess {
        var_name,
        index: Box::new(index),
    }
}

fn maybe_take_parenthesized_expression(
    tokens: &mut PeekableTokens<TokenKind>,
) -> Option<Expression> {
    if let Some(Token { kind: LParen, .. }) = tokens.peek() {
        tokens.next();
        let expr = take_expression(tokens);
        take_token(tokens, RParen);
        Some(expr)
    } else {
        None
    }
}

fn maybe_take_term_starting_with_identifier(
    tokens: &mut PeekableTokens<TokenKind>,
) -> Option<Expression> {
    let p = tokens.peek();
    if let Some(Token {
        kind: Identifier(string),
        ..
    }) = p
    {
        let string = string.to_string();
        let identifier = take_identifier(tokens);
        match tokens.peek() {
            Some(Token {
                kind: LSquareBracket,
                ..
            }) => Some(take_array_access(tokens, identifier.name)),
            Some(Token {
                kind: Dot | LParen, ..
            }) => Some(Expression::SubroutineCall(take_subroutine_call(
                tokens, identifier,
            ))),
            _ => Some(Expression::Variable(string)),
        }
    } else {
        None
    }
}

fn maybe_take_expression_with_binding_power(
    tokens: &mut PeekableTokens<TokenKind>,
    binding_power: u8,
) -> Option<Expression> {
    let mut lhs = if let Some(Token {
        kind: Operator(op), ..
    }) = tokens.peek()
    {
        let op = op.clone();
        let rbp = prefix_precedence(op.clone()).expect("invalid prefix operator");
        tokens.next();
        let operand = maybe_take_expression_with_binding_power(tokens, rbp)
            .expect("unary operator has no operand");
        let operator = match op {
            OperatorVariant::Minus => UnaryOperator::Minus,
            OperatorVariant::Tilde => UnaryOperator::Not,
            _ => panic!("invalid unary operator"),
        };
        Expression::Unary {
            operator,
            operand: Box::new(operand),
        }
    } else {
        maybe_take_primitive_expression(tokens)
            .or_else(|| maybe_take_term_starting_with_identifier(tokens))
            .or_else(|| maybe_take_parenthesized_expression(tokens))?
    };

    loop {
        match tokens.peek() {
            Some(Token {
                kind: Operator(op), ..
            }) => {
                let op = op.clone();
                let (lbp, rbp) = infix_precedence(op.clone()).expect("invalid infix operator");
                if lbp < binding_power {
                    break;
                }
                tokens.next();
                let rhs = maybe_take_expression_with_binding_power(tokens, rbp)
                    .expect("expected rhs for binary operator");
                let operator = match op {
                    Plus => BinaryOperator::Plus,
                    Minus => BinaryOperator::Minus,
                    Star => BinaryOperator::Multiply,
                    Slash => BinaryOperator::Divide,
                    Ampersand => BinaryOperator::And,
                    Pipe => BinaryOperator::Or,
                    LessThan => BinaryOperator::LessThan,
                    LessThanOrEquals => BinaryOperator::LessThanOrEquals,
                    GreaterThan => BinaryOperator::GreaterThan,
                    GreaterThanOrEquals => BinaryOperator::GreaterThanOrEquals,
                    Equals => BinaryOperator::Equals,
                    _ => panic!("invalid binary operator"),
                };

                lhs = Expression::Binary {
                    operator,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                };
            }
            None => return Some(lhs),
            _ => break,
        }
    }

    Some(lhs)
}

fn take_class_keyword(tokens: &mut PeekableTokens<TokenKind>) -> Token<TokenKind> {
    tokens
        .next_if(|token| {
            matches!(
                token,
                Token {
                    kind: Keyword(Class),
                    ..
                }
            )
        })
        .expect("expected keyword \"class\".")
}

fn take_token(tokens: &mut PeekableTokens<TokenKind>, token_kind: TokenKind) -> Token<TokenKind> {
    tokens
        .next_if(|token| token.kind == token_kind)
        .unwrap_or_else(|| panic!("expected token {:?}", token_kind))
}

fn take_identifier(tokens: &mut PeekableTokens<TokenKind>) -> Identifier {
    if let Some(Token {
        kind: Identifier(string),
        range,
        ..
    }) = tokens.next()
    {
        Identifier {
            name: string,
            source_byte_range: range,
        }
    } else {
        panic!("expected identifier")
    }
}

fn take_expression(tokens: &mut PeekableTokens<TokenKind>) -> Expression {
    maybe_take_expression_with_binding_power(tokens, 0).expect("expected expression")
}

fn take_expression_list(tokens: &mut PeekableTokens<TokenKind>) -> Vec<Expression> {
    let mut result = Vec::new();
    if let Some(expression) = maybe_take_expression_with_binding_power(tokens, 0) {
        result.push(expression);
        while let Some(Token { kind: Comma, .. }) = tokens.peek() {
            tokens.next();
            result.push(take_expression(tokens));
        }
    }
    result
}

fn take_subroutine_call(
    tokens: &mut PeekableTokens<TokenKind>,
    name: Identifier,
) -> SubroutineCall {
    match tokens.peek() {
        Some(Token { kind: LParen, .. }) => {
            // Direct function call
            tokens.next(); // LParen
            let arguments = take_expression_list(tokens);
            take_token(tokens, RParen);
            SubroutineCall::Direct {
                subroutine_name: name,
                arguments,
            }
        }
        Some(Token { kind: Dot, .. }) => {
            // Method call
            tokens.next(); // Dot
            let method_name = take_identifier(tokens);
            take_token(tokens, LParen);
            let arguments = take_expression_list(tokens);
            take_token(tokens, RParen);
            SubroutineCall::Method {
                this_name: name,
                method_name,
                arguments,
            }
        }
        _ => panic!("expected subroutine call"),
    }
}

fn take_subroutine_return_type(tokens: &mut PeekableTokens<TokenKind>) -> Option<Type> {
    if let Some(Token {
        kind: Keyword(Void),
        ..
    }) = tokens.peek()
    {
        tokens.next();
        None
    } else {
        Some(take_type(tokens))
    }
}

fn maybe_take_parameter(tokens: &mut PeekableTokens<TokenKind>) -> Option<Parameter> {
    maybe_take_type(tokens).map(|type_name| {
        let var_name = take_identifier(tokens);
        let source_byte_range = type_name.source_byte_range.start..var_name.source_byte_range.end;
        Parameter {
            type_name,
            var_name,
            source_byte_range,
        }
    })
}

fn take_parameters(tokens: &mut PeekableTokens<TokenKind>) -> Vec<Parameter> {
    let mut result = Vec::new();
    if let Some(parameter) = maybe_take_parameter(tokens) {
        result.push(parameter);

        while let Some(Token { kind: Comma, .. }) = tokens.peek() {
            tokens.next(); // comma
            result.push(
                maybe_take_parameter(tokens)
                    .unwrap_or_else(|| panic!("expected parameter after comma")),
            );
        }
    }
    result
}

fn maybe_take_array_index(tokens: &mut PeekableTokens<TokenKind>) -> Option<Expression> {
    tokens
        .next_if(|token| {
            matches!(
                token,
                Token {
                    kind: LSquareBracket,
                    ..
                }
            )
        })
        .map(|_| {
            let expression = take_expression(tokens);
            take_token(tokens, RSquareBracket);
            expression
        })
}

fn take_let_statement(tokens: &mut PeekableTokens<TokenKind>) -> Statement {
    tokens.next(); // "let" keyword
    let var_name = take_identifier(tokens);
    let array_index = maybe_take_array_index(tokens);
    take_token(tokens, Operator(Equals));
    let value = take_expression(tokens);
    take_token(tokens, Semicolon);
    Statement::Let {
        var_name,
        array_index,
        value,
    }
}

fn take_statement_block(tokens: &mut PeekableTokens<TokenKind>) -> Vec<Statement> {
    take_token(tokens, LCurly);
    let statements = take_statements(tokens);
    take_token(tokens, RCurly);
    statements
}

fn maybe_take_else_block(tokens: &mut PeekableTokens<TokenKind>) -> Option<Vec<Statement>> {
    tokens
        .next_if(|token| {
            matches!(
                token,
                Token {
                    kind: Keyword(Else),
                    ..
                }
            )
        })
        .map(|_| take_statement_block(tokens))
}

fn take_if_statement(tokens: &mut PeekableTokens<TokenKind>) -> Statement {
    tokens.next(); // "if" keyword
    take_token(tokens, LParen);
    let condition = take_expression(tokens);
    take_token(tokens, RParen);
    let if_statements = take_statement_block(tokens);
    let else_statements = maybe_take_else_block(tokens);
    Statement::If {
        condition,
        if_statements,
        else_statements,
    }
}

fn take_while_statement(tokens: &mut PeekableTokens<TokenKind>) -> Statement {
    tokens.next(); // "while" keyword
    take_token(tokens, LParen);
    let expression = take_expression(tokens);
    take_token(tokens, RParen);
    let statements = take_statement_block(tokens);
    Statement::While {
        condition: expression,
        statements,
    }
}

fn take_do_statement(tokens: &mut PeekableTokens<TokenKind>) -> Statement {
    tokens.next(); // "do" keyword
    let identifier = take_identifier(tokens);
    let subroutine_call = take_subroutine_call(tokens, identifier);
    take_token(tokens, Semicolon);
    Statement::Do(subroutine_call)
}

fn take_return_statement(tokens: &mut PeekableTokens<TokenKind>) -> Statement {
    tokens.next(); // "return" keyword
    let expression = maybe_take_expression_with_binding_power(tokens, 0);
    take_token(tokens, Semicolon);
    Statement::Return(expression)
}

fn maybe_take_statement(tokens: &mut PeekableTokens<TokenKind>) -> Option<Statement> {
    if let Some(Token {
        kind: Keyword(keyword),
        ..
    }) = tokens.peek()
    {
        match keyword {
            Let => Some(take_let_statement(tokens)),
            If => Some(take_if_statement(tokens)),
            While => Some(take_while_statement(tokens)),
            Do => Some(take_do_statement(tokens)),
            Return => Some(take_return_statement(tokens)),
            _ => None,
        }
    } else {
        None
    }
}

fn take_statements(tokens: &mut PeekableTokens<TokenKind>) -> Vec<Statement> {
    let mut result = Vec::new();
    while let Some(statement) = maybe_take_statement(tokens) {
        result.push(statement);
    }
    result
}

fn take_var_declaration(tokens: &mut PeekableTokens<TokenKind>) -> VarDeclaration {
    if let Some(Token {
        kind: Keyword(Var),
        range: keyword_range,
    }) = tokens.next()
    {
        let type_name = take_type(tokens);
        let var_names = take_var_names(tokens);
        take_token(tokens, Semicolon);
        let source_byte_range = keyword_range.start..var_names.source_byte_range.end;
        VarDeclaration {
            type_name,
            var_names,
            source_byte_range,
        }
    } else {
        panic!("expected var keyword");
    }
}

fn take_var_declarations(tokens: &mut PeekableTokens<TokenKind>) -> Vec<VarDeclaration> {
    let mut result = Vec::new();
    while let Some(Token {
        kind: Keyword(Var), ..
    }) = tokens.peek()
    {
        result.push(take_var_declaration(tokens));
    }
    result
}

fn take_subroutine_body(tokens: &mut PeekableTokens<TokenKind>) -> SubroutineBody {
    let start_curly = take_token(tokens, LCurly);
    let var_declarations = take_var_declarations(tokens);
    let statements = take_statements(tokens);
    let end_curly = take_token(tokens, RCurly);
    SubroutineBody {
        var_declarations,
        statements,
        source_byte_range: start_curly.range.start..end_curly.range.end,
    }
}

fn take_subroutine_declaration(tokens: &mut PeekableTokens<TokenKind>) -> SubroutineDeclaration {
    if let Some(Token {
        kind: Keyword(keyword),
        range: subroutine_kind_keyword_range,
        ..
    }) = tokens.next()
    {
        let subroutine_kind = match keyword {
            Constructor => SubroutineKind::Constructor,
            Function => SubroutineKind::Function,
            Method => SubroutineKind::Method,
            _ => panic!("expected subroutine kind"),
        };

        let return_type = take_subroutine_return_type(tokens);
        let name = take_identifier(tokens);
        take_token(tokens, LParen);
        let parameters = take_parameters(tokens);
        take_token(tokens, RParen);
        let body = take_subroutine_body(tokens);
        let source_byte_range = subroutine_kind_keyword_range.start..body.source_byte_range.end;
        SubroutineDeclaration {
            subroutine_kind,
            return_type,
            name,
            parameters,
            body,
            source_byte_range,
        }
    } else {
        panic!("expected subroutine kind");
    }
}

fn maybe_take_subroutine_declaration(
    tokens: &mut PeekableTokens<TokenKind>,
) -> Option<SubroutineDeclaration> {
    if let Some(Token {
        kind: Keyword(Constructor | Function | Method),
        ..
    }) = tokens.peek()
    {
        Some(take_subroutine_declaration(tokens))
    } else {
        None
    }
}

fn take_class_subroutine_declarations(
    tokens: &mut PeekableTokens<TokenKind>,
) -> Vec<SubroutineDeclaration> {
    let mut result = Vec::new();
    while let Some(subroutine_declaration) = maybe_take_subroutine_declaration(tokens) {
        result.push(subroutine_declaration);
    }
    result
}

fn take_class_var_declaration_qualifier(
    tokens: &mut PeekableTokens<TokenKind>,
) -> ClassVarDeclarationKind {
    match tokens.next() {
        Some(Token {
            kind: Keyword(keyword),
            range,
            ..
        }) => {
            let variant = match keyword {
                Static => ClassVarDeclarationKindVariant::Static,
                Field => ClassVarDeclarationKindVariant::Field,
                _ => panic!("expected var declaration qualifier",),
            };
            ClassVarDeclarationKind {
                variant,
                source_byte_range: range,
            }
        }
        _ => panic!("expected var declaration qualifier",),
    }
}

fn take_var_name(tokens: &mut PeekableTokens<TokenKind>) -> Identifier {
    if let Some(Token {
        kind: Identifier(var_name),
        range,
        ..
    }) = tokens.next()
    {
        Identifier {
            name: var_name,
            source_byte_range: range,
        }
    } else {
        panic!("expected var name")
    }
}

fn take_var_names(tokens: &mut PeekableTokens<TokenKind>) -> VarNames {
    // There has to be at least one var name.
    let first_var = take_var_name(tokens);
    let mut source_byte_range = first_var.source_byte_range.clone();
    let mut names = vec![first_var];
    while let Some(Token { kind: Comma, .. }) = tokens.peek() {
        tokens.next(); // comma
        let var = take_var_name(tokens);
        source_byte_range.end = var.source_byte_range.end;
        names.push(var);
    }
    VarNames {
        names,
        source_byte_range,
    }
}

fn take_type(tokens: &mut PeekableTokens<TokenKind>) -> Type {
    match tokens.next() {
        Some(Token { kind, range, .. }) => {
            let type_variant = match kind {
                Keyword(Int) => TypeVariant::Int,
                Keyword(Char) => TypeVariant::Char,
                Keyword(Boolean) => TypeVariant::Boolean,
                Identifier(class_name) => TypeVariant::ClassName(Identifier {
                    name: class_name,
                    source_byte_range: range.clone(),
                }),
                _ => panic!("expected var type name"),
            };
            Type {
                variant: type_variant,
                source_byte_range: range,
            }
        }
        _ => panic!("expected var type name"),
    }
}

fn maybe_take_type(tokens: &mut PeekableTokens<TokenKind>) -> Option<Type> {
    if let Some(
        Token {
            kind: Keyword(Int | Char | Boolean) | Identifier(_),
            ..
        },
        ..,
    ) = tokens.peek()
    {
        Some(take_type(tokens))
    } else {
        None
    }
}

fn take_class_var_declaration(tokens: &mut PeekableTokens<TokenKind>) -> ClassVarDeclaration {
    let qualifier = take_class_var_declaration_qualifier(tokens);
    let type_name = take_type(tokens);
    let var_names = take_var_names(tokens);
    let semicolon = take_token(tokens, Semicolon);
    let source_byte_range = qualifier.source_byte_range.start..semicolon.range.end;
    ClassVarDeclaration {
        qualifier,
        type_name,
        var_names,
        source_byte_range,
    }
}

fn maybe_take_class_var_declaration(
    tokens: &mut PeekableTokens<TokenKind>,
) -> Option<ClassVarDeclaration> {
    match tokens.peek().expect("unexpected end of input") {
        Token {
            kind: Keyword(Static | Field),
            ..
        } => Some(take_class_var_declaration(tokens)),
        _ => None,
    }
}

fn take_class_var_declarations(tokens: &mut PeekableTokens<TokenKind>) -> Vec<ClassVarDeclaration> {
    let mut result = Vec::new();
    while let Some(class_var_declaration) = maybe_take_class_var_declaration(tokens) {
        result.push(class_var_declaration);
    }
    result
}

fn take_class(tokens: &mut PeekableTokens<TokenKind>) -> Class {
    let class_keyword = take_class_keyword(tokens);
    let name = take_identifier(tokens);
    take_token(tokens, LCurly);
    let var_declarations = take_class_var_declarations(tokens);
    let subroutine_declarations = take_class_subroutine_declarations(tokens);
    let end_curly = take_token(tokens, RCurly);
    Class {
        name,
        var_declarations,
        subroutine_declarations,
        source_byte_range: class_keyword.range.start..end_curly.range.end,
    }
}

fn prefix_precedence(operator: OperatorVariant) -> Option<u8> {
    match operator {
        Tilde => Some(20),
        Minus => Some(19),
        _ => None,
    }
}

fn infix_precedence(operator: OperatorVariant) -> Option<(u8, u8)> {
    match operator {
        Star => Some((21, 22)),
        Slash => Some((19, 20)),
        Plus => Some((17, 18)),
        Minus => Some((15, 16)),
        LessThan => Some((13, 14)),
        LessThanOrEquals => Some((11, 12)),
        GreaterThan => Some((9, 10)),
        GreaterThanOrEquals => Some((7, 8)),
        Ampersand => Some((5, 6)),
        Pipe => Some((3, 4)),
        Equals => Some((1, 2)),
        _ => None,
    }
}

pub fn parse(source: &str) -> Class {
    let tokens = Tokenizer::new(token_defs()).tokenize(source);
    let filtered = tokens.filter(|token| {
        !matches!(
            token.kind,
            TokenKind::Whitespace | TokenKind::SingleLineComment | TokenKind::MultiLineComment
        )
    });
    let cleaned_tokens: Box<dyn Iterator<Item = Token<TokenKind>>> = Box::new(filtered);
    let mut cleaned_peekable_tokens = cleaned_tokens.peekable();
    take_class(&mut cleaned_peekable_tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_expression(source: &str) -> Expression {
        let tokens = Tokenizer::new(token_defs()).tokenize(source);
        let filtered = tokens.filter(|token| {
            !matches!(
                token.kind,
                TokenKind::Whitespace | TokenKind::SingleLineComment | TokenKind::MultiLineComment
            )
        });
        let cleaned_tokens: Box<dyn Iterator<Item = Token<TokenKind>>> = Box::new(filtered);
        let mut cleaned_peekable_tokens = cleaned_tokens.peekable();
        take_expression(&mut cleaned_peekable_tokens)
    }

    #[test]
    fn test_simple_class() {
        assert_eq!(
            parse("class foo {}"),
            Class {
                name: Identifier {
                    name: "foo".to_string(),
                    source_byte_range: 6..9
                },
                var_declarations: vec![],
                subroutine_declarations: vec![],
                source_byte_range: 0..12
            }
        );
    }

    #[test]
    fn test_class_with_var_declaration() {
        assert_eq!(
            parse(
                "
            class foo {
              static int bar;
            }"
            ),
            Class {
                name: Identifier {
                    name: "foo".to_string(),
                    source_byte_range: 19..22
                },
                var_declarations: vec![ClassVarDeclaration {
                    qualifier: ClassVarDeclarationKind {
                        variant: ClassVarDeclarationKindVariant::Static,
                        source_byte_range: 39..45
                    },
                    type_name: Type {
                        variant: TypeVariant::Int,
                        source_byte_range: 46..49
                    },
                    var_names: VarNames {
                        names: vec![Identifier {
                            name: "bar".to_string(),
                            source_byte_range: 50..53
                        }],
                        source_byte_range: 50..53
                    },
                    source_byte_range: 39..54
                }],
                subroutine_declarations: vec![],
                source_byte_range: 13..68
            }
        );
    }

    #[test]
    fn test_class_with_multiple_var_declarations() {
        assert_eq!(
            parse(
                "
            class foo {
              static int bar;
              field char baz, buz, boz;
              field boolean a, b, c;
            }"
            ),
            Class {
                name: Identifier {
                    name: "foo".to_string(),
                    source_byte_range: 0..1
                },
                var_declarations: vec![
                    ClassVarDeclaration {
                        qualifier: ClassVarDeclarationKind {
                            variant: ClassVarDeclarationKindVariant::Static,
                            source_byte_range: 0..1,
                        },
                        type_name: Type {
                            variant: TypeVariant::Int,
                            source_byte_range: 0..1
                        },
                        var_names: VarNames {
                            names: vec![Identifier {
                                name: "bar".to_string(),
                                source_byte_range: 0..1,
                            }],
                            source_byte_range: 0..1
                        },
                        source_byte_range: 0..1,
                    },
                    ClassVarDeclaration {
                        qualifier: ClassVarDeclarationKind {
                            variant: ClassVarDeclarationKindVariant::Field,
                            source_byte_range: 0..1
                        },
                        type_name: Type {
                            variant: TypeVariant::Char,
                            source_byte_range: 0..1
                        },
                        var_names: VarNames {
                            names: vec![
                                Identifier {
                                    name: "baz".to_string(),
                                    source_byte_range: 0..1,
                                },
                                Identifier {
                                    name: "buz".to_string(),
                                    source_byte_range: 0..1,
                                },
                                Identifier {
                                    name: "boz".to_string(),
                                    source_byte_range: 0..1,
                                }
                            ],
                            source_byte_range: 0..1
                        },
                        source_byte_range: 0..1,
                    },
                    ClassVarDeclaration {
                        qualifier: ClassVarDeclarationKind {
                            variant: ClassVarDeclarationKindVariant::Field,
                            source_byte_range: 0..1,
                        },
                        type_name: Type {
                            variant: TypeVariant::Boolean,
                            source_byte_range: 0..1,
                        },
                        var_names: VarNames {
                            names: vec![
                                Identifier {
                                    name: "a".to_string(),
                                    source_byte_range: 0..1
                                },
                                Identifier {
                                    name: "b".to_string(),
                                    source_byte_range: 0..1
                                },
                                Identifier {
                                    name: "c".to_string(),
                                    source_byte_range: 0..1
                                },
                            ],
                            source_byte_range: 0..1
                        },
                        source_byte_range: 0..1
                    }
                ],
                subroutine_declarations: vec![],
                source_byte_range: 0..1,
            }
        );
    }

    #[test]
    fn test_class_with_subroutine_declarations() {
        assert_eq!(
            parse(
                "
            class foo {
                constructor boolean bar(int abc, char def, foo ghi) {
                }
                function char baz(boolean _123) {
                }
                method void qux() {
                }
            }"
            ),
            Class {
                name: Identifier {
                    name: "foo".to_string(),
                    source_byte_range: 19..22
                },
                var_declarations: vec![],
                subroutine_declarations: vec![
                    SubroutineDeclaration {
                        subroutine_kind: SubroutineKind::Constructor,
                        return_type: Some(Type {
                            variant: TypeVariant::Boolean,
                            source_byte_range: 53..60,
                        }),
                        parameters: vec![
                            Parameter {
                                type_name: Type {
                                    variant: TypeVariant::Int,
                                    source_byte_range: 65..68
                                },
                                var_name: Identifier {
                                    name: "abc".to_string(),
                                    source_byte_range: 69..72,
                                },
                                source_byte_range: 65..72,
                            },
                            Parameter {
                                type_name: Type {
                                    variant: TypeVariant::Char,
                                    source_byte_range: 74..78
                                },
                                var_name: Identifier {
                                    name: "def".to_string(),
                                    source_byte_range: 79..82,
                                },
                                source_byte_range: 74..82,
                            },
                            Parameter {
                                type_name: Type {
                                    variant: TypeVariant::ClassName(Identifier {
                                        name: "foo".to_string(),
                                        source_byte_range: 84..87,
                                    }),
                                    source_byte_range: 84..87
                                },
                                var_name: Identifier {
                                    name: "ghi".to_string(),
                                    source_byte_range: 88..91,
                                },
                                source_byte_range: 84..91
                            }
                        ],
                        name: Identifier {
                            name: "bar".to_string(),
                            source_byte_range: 61..64
                        },
                        body: SubroutineBody {
                            var_declarations: vec![],
                            statements: vec![],
                            source_byte_range: 93..112
                        },
                        source_byte_range: 41..112
                    },
                    SubroutineDeclaration {
                        subroutine_kind: SubroutineKind::Function,
                        return_type: Some(Type {
                            variant: TypeVariant::Char,
                            source_byte_range: 138..142
                        }),
                        parameters: vec![Parameter {
                            type_name: Type {
                                variant: TypeVariant::Boolean,
                                source_byte_range: 147..154
                            },
                            var_name: Identifier {
                                name: "_123".to_string(),
                                source_byte_range: 155..159
                            },
                            source_byte_range: 147..159
                        },],
                        name: Identifier {
                            name: "baz".to_string(),
                            source_byte_range: 143..146
                        },
                        body: SubroutineBody {
                            var_declarations: vec![],
                            statements: vec![],
                            source_byte_range: 161..180
                        },
                        source_byte_range: 129..180
                    },
                    SubroutineDeclaration {
                        subroutine_kind: SubroutineKind::Method,
                        return_type: None,
                        parameters: vec![],
                        name: Identifier {
                            name: "qux".to_string(),
                            source_byte_range: 209..212
                        },
                        body: SubroutineBody {
                            var_declarations: vec![],
                            statements: vec![],
                            source_byte_range: 215..234
                        },
                        source_byte_range: 197..234
                    }
                ],
                source_byte_range: 13..248
            }
        );
    }

    #[test]
    fn test_all_statement_types() {
        assert_eq!(
            parse(
                "
            class foo {
                constructor int blah() {
                    var int a;
                    let a = 1234;
                    let b[22] = 123;
                    if (1) {
                        while (1) {
                           do foobar();
                           do foobar(1);
                           do foobar(1, 2, 3);
                           do foo.bar();
                           do foo.bar(1);
                           do foo.bar(1, 2, 3);
                        }
                    } else {
                        return 123;
                    }
                }
            }"
            ),
            Class {
                name: Identifier {
                    name: "foo".to_string(),
                    source_byte_range: 19..22
                },
                var_declarations: vec![],
                subroutine_declarations: vec![SubroutineDeclaration {
                    subroutine_kind: SubroutineKind::Constructor,
                    return_type: Some(Type {
                        variant: TypeVariant::Int,
                        source_byte_range: 53..56
                    }),
                    parameters: vec![],
                    name: Identifier {
                        name: "blah".to_string(),
                        source_byte_range: 57..61
                    },
                    body: SubroutineBody {
                        var_declarations: vec![VarDeclaration {
                            type_name: Type {
                                variant: TypeVariant::Int,
                                source_byte_range: 90..93
                            },
                            var_names: VarNames {
                                names: vec![Identifier {
                                    name: "a".to_string(),
                                    source_byte_range: 94..95
                                }],
                                source_byte_range: 94..95
                            },
                            source_byte_range: 86..95
                        }],
                        statements: vec![
                            Statement::Let {
                                var_name: Identifier {
                                    name: "a".to_string(),
                                    source_byte_range: 121..122
                                },
                                array_index: None,
                                value: Expression::PrimitiveTerm(IntegerConstant(
                                    "1234".to_string()
                                ))
                            },
                            Statement::Let {
                                var_name: Identifier {
                                    name: "b".to_string(),
                                    source_byte_range: 155..156
                                },
                                array_index: Some(Expression::PrimitiveTerm(IntegerConstant(
                                    "22".to_string()
                                ))),
                                value: Expression::PrimitiveTerm(IntegerConstant(
                                    "123".to_string()
                                ))
                            },
                            Statement::If {
                                condition: Expression::PrimitiveTerm(IntegerConstant(
                                    "1".to_string()
                                )),
                                if_statements: vec![Statement::While {
                                    condition: Expression::PrimitiveTerm(IntegerConstant(
                                        "1".to_string()
                                    )),
                                    statements: vec![
                                        Statement::Do(SubroutineCall::Direct {
                                            subroutine_name: Identifier {
                                                name: "foobar".to_string(),
                                                source_byte_range: 263..269
                                            },
                                            arguments: vec![]
                                        }),
                                        Statement::Do(SubroutineCall::Direct {
                                            subroutine_name: Identifier {
                                                name: "foobar".to_string(),
                                                source_byte_range: 303..309
                                            },
                                            arguments: vec![Expression::PrimitiveTerm(
                                                IntegerConstant("1".to_string())
                                            )]
                                        }),
                                        Statement::Do(SubroutineCall::Direct {
                                            subroutine_name: Identifier {
                                                name: "foobar".to_string(),
                                                source_byte_range: 344..350
                                            },
                                            arguments: vec![
                                                Expression::PrimitiveTerm(IntegerConstant(
                                                    "1".to_string()
                                                )),
                                                Expression::PrimitiveTerm(IntegerConstant(
                                                    "2".to_string()
                                                )),
                                                Expression::PrimitiveTerm(IntegerConstant(
                                                    "3".to_string()
                                                ))
                                            ]
                                        }),
                                        Statement::Do(SubroutineCall::Method {
                                            this_name: Identifier {
                                                name: "foo".to_string(),
                                                source_byte_range: 391..394
                                            },
                                            method_name: Identifier {
                                                name: "bar".to_string(),
                                                source_byte_range: 395..398
                                            },
                                            arguments: vec![]
                                        }),
                                        Statement::Do(SubroutineCall::Method {
                                            this_name: Identifier {
                                                name: "foo".to_string(),
                                                source_byte_range: 432..435
                                            },
                                            method_name: Identifier {
                                                name: "bar".to_string(),
                                                source_byte_range: 436..439
                                            },
                                            arguments: vec![Expression::PrimitiveTerm(
                                                IntegerConstant("1".to_string())
                                            )]
                                        }),
                                        Statement::Do(SubroutineCall::Method {
                                            this_name: Identifier {
                                                name: "foo".to_string(),
                                                source_byte_range: 474..477
                                            },
                                            method_name: Identifier {
                                                name: "bar".to_string(),
                                                source_byte_range: 478..481
                                            },
                                            arguments: vec![
                                                Expression::PrimitiveTerm(IntegerConstant(
                                                    "1".to_string()
                                                )),
                                                Expression::PrimitiveTerm(IntegerConstant(
                                                    "2".to_string()
                                                )),
                                                Expression::PrimitiveTerm(IntegerConstant(
                                                    "3".to_string()
                                                ))
                                            ]
                                        })
                                    ]
                                }],
                                else_statements: Some(vec![Statement::Return(Some(
                                    Expression::PrimitiveTerm(IntegerConstant("123".to_string()))
                                ))])
                            }
                        ],
                        source_byte_range: 64..622
                    },
                    source_byte_range: 41..622
                }],
                source_byte_range: 13..636
            }
        );
    }

    #[test]
    fn test_simple_expression() {
        assert_eq!(
            parse_expression("1"),
            Expression::PrimitiveTerm(IntegerConstant("1".to_string()))
        )
    }

    #[test]
    fn test_simple_binary_expression() {
        assert_eq!(
            parse_expression("1 + 2"),
            Expression::Binary {
                operator: BinaryOperator::Plus,
                lhs: Box::new(Expression::PrimitiveTerm(IntegerConstant("1".to_string()))),
                rhs: Box::new(Expression::PrimitiveTerm(IntegerConstant("2".to_string()))),
            }
        )
    }

    #[test]
    fn test_simple_binary_expression_within_class() {
        assert_eq!(
            parse(
                "
            class foo {
                method void bar () {
                    let a = 1 + 2 + 3;
                }
            }
            "
            ),
            Class {
                name: Identifier {
                    name: "foo".to_string(),
                    source_byte_range: 19..22
                },
                var_declarations: vec![],
                subroutine_declarations: vec![SubroutineDeclaration {
                    subroutine_kind: SubroutineKind::Method,
                    return_type: None,
                    parameters: vec![],
                    name: Identifier {
                        name: "bar".to_string(),
                        source_byte_range: 53..56
                    },
                    body: SubroutineBody {
                        var_declarations: vec![],
                        statements: vec![Statement::Let {
                            var_name: Identifier {
                                name: "a".to_string(),
                                source_byte_range: 86..87
                            },
                            array_index: None,
                            value: Expression::Binary {
                                operator: BinaryOperator::Plus,
                                lhs: Box::new(Expression::Binary {
                                    operator: BinaryOperator::Plus,
                                    lhs: Box::new(Expression::PrimitiveTerm(IntegerConstant(
                                        "1".to_string()
                                    ))),
                                    rhs: Box::new(Expression::PrimitiveTerm(IntegerConstant(
                                        "2".to_string()
                                    ))),
                                }),
                                rhs: Box::new(Expression::PrimitiveTerm(IntegerConstant(
                                    "3".to_string()
                                ))),
                            }
                        }],
                        source_byte_range: 60..118
                    },
                    source_byte_range: 41..118
                }],
                source_byte_range: 13..132
            }
        )
    }

    #[test]
    fn test_simple_left_associating_expression() {
        assert_eq!(
            parse_expression("1 + 2 + 3 + 4"),
            Expression::Binary {
                operator: BinaryOperator::Plus,
                lhs: Box::new(Expression::Binary {
                    operator: BinaryOperator::Plus,
                    lhs: Box::new(Expression::Binary {
                        operator: BinaryOperator::Plus,
                        lhs: Box::new(Expression::PrimitiveTerm(IntegerConstant("1".to_string()))),
                        rhs: Box::new(Expression::PrimitiveTerm(IntegerConstant("2".to_string()))),
                    }),
                    rhs: Box::new(Expression::PrimitiveTerm(IntegerConstant("3".to_string()))),
                }),
                rhs: Box::new(Expression::PrimitiveTerm(IntegerConstant("4".to_string()))),
            }
        )
    }

    #[test]
    fn test_binary_precedence() {
        assert_eq!(
            parse_expression("1 + 2 * 3 + 4"),
            Expression::Binary {
                operator: BinaryOperator::Plus,
                lhs: Box::new(Expression::Binary {
                    operator: BinaryOperator::Plus,
                    lhs: Box::new(Expression::PrimitiveTerm(IntegerConstant("1".to_string()))),
                    rhs: Box::new(Expression::Binary {
                        operator: BinaryOperator::Multiply,
                        lhs: Box::new(Expression::PrimitiveTerm(IntegerConstant("2".to_string()))),
                        rhs: Box::new(Expression::PrimitiveTerm(IntegerConstant("3".to_string()))),
                    })
                }),
                rhs: Box::new(Expression::PrimitiveTerm(IntegerConstant("4".to_string()))),
            }
        )
    }

    #[test]
    fn test_simple_unary_expression() {
        assert_eq!(
            parse_expression("~1"),
            Expression::Unary {
                operator: UnaryOperator::Not,
                operand: Box::new(Expression::PrimitiveTerm(IntegerConstant("1".to_string()))),
            }
        )
    }

    #[test]
    fn test_simple_combined_unary_and_binary_expression() {
        assert_eq!(
            parse_expression("~1 + ~2"),
            Expression::Binary {
                operator: BinaryOperator::Plus,
                lhs: Box::new(Expression::Unary {
                    operator: UnaryOperator::Not,
                    operand: Box::new(Expression::PrimitiveTerm(IntegerConstant("1".to_string()))),
                }),
                rhs: Box::new(Expression::Unary {
                    operator: UnaryOperator::Not,
                    operand: Box::new(Expression::PrimitiveTerm(IntegerConstant("2".to_string()))),
                }),
            }
        )
    }

    #[test]
    fn test_expression_with_subroutine_calls() {
        assert_eq!(
            parse_expression("1 + foo(1, baz.bar(1, 2), 3) + 2"),
            Expression::Binary {
                operator: BinaryOperator::Plus,
                lhs: Box::new(Expression::Binary {
                    operator: BinaryOperator::Plus,
                    lhs: Box::new(Expression::PrimitiveTerm(IntegerConstant("1".to_string()))),
                    rhs: Box::new(Expression::SubroutineCall(SubroutineCall::Direct {
                        subroutine_name: Identifier {
                            name: "foo".to_string(),
                            source_byte_range: 4..7
                        },
                        arguments: vec![
                            Expression::PrimitiveTerm(IntegerConstant("1".to_string())),
                            Expression::SubroutineCall(SubroutineCall::Method {
                                this_name: Identifier {
                                    name: "baz".to_string(),
                                    source_byte_range: 11..14
                                },
                                method_name: Identifier {
                                    name: "bar".to_string(),
                                    source_byte_range: 15..18
                                },
                                arguments: vec![
                                    Expression::PrimitiveTerm(IntegerConstant("1".to_string())),
                                    Expression::PrimitiveTerm(IntegerConstant("2".to_string())),
                                ]
                            }),
                            Expression::PrimitiveTerm(IntegerConstant("3".to_string())),
                        ]
                    })),
                }),
                rhs: Box::new(Expression::PrimitiveTerm(IntegerConstant("2".to_string()))),
            }
        )
    }

    #[test]
    fn test_expression_with_subroutine_call_and_array_access() {
        assert_eq!(
            parse_expression("1 + foo(1, bar[1 + 2], 3) + 2"),
            Expression::Binary {
                operator: BinaryOperator::Plus,
                lhs: Box::new(Expression::Binary {
                    operator: BinaryOperator::Plus,
                    lhs: Box::new(Expression::PrimitiveTerm(IntegerConstant("1".to_string()))),
                    rhs: Box::new(Expression::SubroutineCall(SubroutineCall::Direct {
                        subroutine_name: Identifier {
                            name: "foo".to_string(),
                            source_byte_range: 4..7
                        },
                        arguments: vec![
                            Expression::PrimitiveTerm(IntegerConstant("1".to_string())),
                            Expression::ArrayAccess {
                                var_name: "bar".to_string(),
                                index: Box::new(Expression::Binary {
                                    operator: BinaryOperator::Plus,
                                    lhs: Box::new(Expression::PrimitiveTerm(IntegerConstant(
                                        "1".to_string()
                                    ))),
                                    rhs: Box::new(Expression::PrimitiveTerm(IntegerConstant(
                                        "2".to_string()
                                    ))),
                                })
                            },
                            Expression::PrimitiveTerm(IntegerConstant("3".to_string())),
                        ]
                    })),
                }),
                rhs: Box::new(Expression::PrimitiveTerm(IntegerConstant("2".to_string()))),
            }
        )
    }

    #[test]
    fn test_expression_with_variables_subroutine_calls_and_array_access() {
        assert_eq!(
            parse_expression("foo + bar[baz + buz.boz(qux, wox[123]) / bing]"),
            Expression::Binary {
                operator: BinaryOperator::Plus,
                lhs: Box::new(Expression::Variable("foo".to_string())),
                rhs: Box::new(Expression::ArrayAccess {
                    var_name: "bar".to_string(),
                    index: Box::new(Expression::Binary {
                        operator: BinaryOperator::Plus,
                        lhs: Box::new(Expression::Variable("baz".to_string())),
                        rhs: Box::new(Expression::Binary {
                            operator: BinaryOperator::Divide,
                            lhs: Box::new(Expression::SubroutineCall(SubroutineCall::Method {
                                this_name: Identifier {
                                    name: "buz".to_string(),
                                    source_byte_range: 16..19
                                },
                                method_name: Identifier {
                                    name: "boz".to_string(),
                                    source_byte_range: 20..23
                                },
                                arguments: vec![
                                    Expression::Variable("qux".to_string()),
                                    Expression::ArrayAccess {
                                        var_name: "wox".to_string(),
                                        index: Box::new(Expression::PrimitiveTerm(
                                            IntegerConstant("123".to_string())
                                        ))
                                    }
                                ]
                            })),
                            rhs: Box::new(Expression::Variable("bing".to_string()))
                        })
                    })
                })
            }
        )
    }

    #[test]
    fn test_primitive_terms() {
        assert_eq!(
            parse_expression("1 + \"hello\" + true + false + null + this"),
            Expression::Binary {
                operator: BinaryOperator::Plus,
                lhs: Box::new(Expression::Binary {
                    operator: BinaryOperator::Plus,
                    lhs: Box::new(Expression::Binary {
                        operator: BinaryOperator::Plus,
                        lhs: Box::new(Expression::Binary {
                            operator: BinaryOperator::Plus,
                            lhs: Box::new(Expression::Binary {
                                operator: BinaryOperator::Plus,
                                lhs: Box::new(Expression::PrimitiveTerm(IntegerConstant(
                                    "1".to_string()
                                ))),
                                rhs: Box::new(Expression::PrimitiveTerm(StringConstant(
                                    "hello".to_string()
                                ))),
                            }),
                            rhs: Box::new(Expression::PrimitiveTerm(PrimitiveTermVariant::True)),
                        }),
                        rhs: Box::new(Expression::PrimitiveTerm(PrimitiveTermVariant::False)),
                    }),
                    rhs: Box::new(Expression::PrimitiveTerm(PrimitiveTermVariant::Null)),
                }),
                rhs: Box::new(Expression::PrimitiveTerm(PrimitiveTermVariant::This))
            }
        )
    }

    #[test]
    fn test_parenthesized_expression() {
        assert_eq!(
            parse_expression("(1 + 2) * 3"),
            Expression::Binary {
                operator: BinaryOperator::Multiply,
                lhs: Box::new(Expression::Binary {
                    operator: BinaryOperator::Plus,
                    lhs: Box::new(Expression::PrimitiveTerm(IntegerConstant("1".to_string()))),
                    rhs: Box::new(Expression::PrimitiveTerm(IntegerConstant("2".to_string()))),
                }),
                rhs: Box::new(Expression::PrimitiveTerm(IntegerConstant("3".to_string()))),
            }
        )
    }
}
