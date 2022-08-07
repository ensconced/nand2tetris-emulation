use super::{
    jack_node_types::{PrimitiveTermVariant::*, *},
    sourcemap::SourceMap,
    tokenizer::{
        token_defs, KeywordTokenVariant,
        OperatorVariant::{self, *},
        TokenKind,
    },
};
use crate::compilers::utils::{
    parser_utils::PeekableTokens,
    tokenizer::{Token, Tokenizer},
};

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
    let tokens: Vec<_> = Tokenizer::new(token_defs())
        .tokenize(source)
        .into_iter()
        .filter(|token| {
            !matches!(
                token,
                Token {
                    kind: TokenKind::Whitespace
                        | TokenKind::MultiLineComment
                        | TokenKind::SingleLineComment,
                    ..
                }
            )
        })
        .collect();
    let mut parser = Parser {
        token_iter: tokens.into_iter().peekable(),
        sourcemap: SourceMap::new(),
    };
    parser.take_class()
}

struct Parser {
    // tokens: Vec<Token<TokenKind>>,
    token_iter: PeekableTokens<TokenKind>,
    sourcemap: SourceMap,
}

impl Parser {
    fn maybe_take_primitive_expression(&mut self) -> Option<Expression> {
        use TokenKind::*;
        let peeked_token = self.token_iter.peek().cloned();
        peeked_token.and_then(|token| match token.kind {
            IntegerLiteral(string) => {
                self.token_iter.next();
                Some(Expression::PrimitiveTerm(IntegerConstant(string)))
            }
            StringLiteral(string) => {
                self.token_iter.next();
                Some(Expression::PrimitiveTerm(StringConstant(string)))
            }
            Keyword(KeywordTokenVariant::True) => {
                self.token_iter.next();
                Some(Expression::PrimitiveTerm(PrimitiveTermVariant::True))
            }
            Keyword(KeywordTokenVariant::False) => {
                self.token_iter.next();
                Some(Expression::PrimitiveTerm(PrimitiveTermVariant::False))
            }
            Keyword(KeywordTokenVariant::Null) => {
                self.token_iter.next();
                Some(Expression::PrimitiveTerm(PrimitiveTermVariant::Null))
            }
            Keyword(KeywordTokenVariant::This) => {
                self.token_iter.next();
                Some(Expression::PrimitiveTerm(PrimitiveTermVariant::This))
            }
            _ => None,
        })
    }

    fn take_array_access(&mut self, var_name: String) -> Expression {
        use TokenKind::*;
        self.take_token(LSquareBracket);
        let index = self.take_expression();
        self.take_token(RSquareBracket);
        Expression::ArrayAccess {
            var_name,
            index: Box::new(index),
        }
    }

    fn maybe_take_parenthesized_expression(&mut self) -> Option<Expression> {
        use TokenKind::*;
        if let Some(Token { kind: LParen, .. }) = self.token_iter.peek() {
            self.token_iter.next();
            let expr = self.take_expression();
            self.take_token(RParen);
            Some(expr)
        } else {
            None
        }
    }

    fn maybe_take_term_starting_with_identifier(&mut self) -> Option<Expression> {
        use TokenKind::*;
        let p = self.token_iter.peek();
        if let Some(Token {
            kind: Identifier(string),
            ..
        }) = p
        {
            let string = string.to_string();
            let identifier = self.take_identifier();
            match self.token_iter.peek() {
                Some(Token {
                    kind: LSquareBracket,
                    ..
                }) => Some(self.take_array_access(identifier)),
                Some(Token {
                    kind: Dot | LParen, ..
                }) => Some(Expression::SubroutineCall(
                    self.take_subroutine_call(identifier),
                )),
                _ => Some(Expression::Variable(string)),
            }
        } else {
            None
        }
    }

    fn maybe_take_expression_with_binding_power(
        &mut self,
        binding_power: u8,
    ) -> Option<Expression> {
        use TokenKind::*;
        let mut lhs = if let Some(Token {
            kind: Operator(op), ..
        }) = self.token_iter.peek()
        {
            let op = op.clone();
            let rbp = prefix_precedence(op.clone()).expect("invalid prefix operator");
            self.token_iter.next();
            let operand = self
                .maybe_take_expression_with_binding_power(rbp)
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
            self.maybe_take_primitive_expression()
                .or_else(|| self.maybe_take_term_starting_with_identifier())
                .or_else(|| self.maybe_take_parenthesized_expression())?
        };

        loop {
            match self.token_iter.peek() {
                Some(Token {
                    kind: Operator(op), ..
                }) => {
                    let op = op.clone();
                    let (lbp, rbp) = infix_precedence(op.clone()).expect("invalid infix operator");
                    if lbp < binding_power {
                        break;
                    }
                    self.token_iter.next();
                    let rhs = self
                        .maybe_take_expression_with_binding_power(rbp)
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

    fn take_class_keyword(&mut self) -> Token<TokenKind> {
        self.token_iter
            .next_if(|token| {
                matches!(
                    token,
                    Token {
                        kind: TokenKind::Keyword(KeywordTokenVariant::Class),
                        ..
                    }
                )
            })
            .expect("expected keyword \"class\".")
    }

    fn take_token(&mut self, token_kind: TokenKind) -> Token<TokenKind> {
        self.token_iter
            .next_if(|token| token.kind == token_kind)
            .unwrap_or_else(|| panic!("expected token {:?}", token_kind))
    }

    fn take_identifier(&mut self) -> String {
        if let Some(Token {
            kind: TokenKind::Identifier(string),
            ..
        }) = self.token_iter.next()
        {
            string
        } else {
            panic!("expected identifier")
        }
    }

    fn take_expression(&mut self) -> Expression {
        self.maybe_take_expression_with_binding_power(0)
            .expect("expected expression")
    }

    fn take_expression_list(&mut self) -> Vec<Expression> {
        use TokenKind::*;
        let mut result = Vec::new();
        if let Some(expression) = self.maybe_take_expression_with_binding_power(0) {
            result.push(expression);
            while let Some(Token { kind: Comma, .. }) = self.token_iter.peek() {
                self.token_iter.next();
                result.push(self.take_expression());
            }
        }
        result
    }

    fn take_subroutine_call(&mut self, name: String) -> SubroutineCall {
        use TokenKind::*;
        match self.token_iter.peek() {
            Some(Token { kind: LParen, .. }) => {
                // Direct function call
                self.token_iter.next(); // LParen
                let arguments = self.take_expression_list();
                self.take_token(RParen);
                SubroutineCall::Direct {
                    subroutine_name: name,
                    arguments,
                }
            }
            Some(Token { kind: Dot, .. }) => {
                // Method call
                self.token_iter.next(); // Dot
                let method_name = self.take_identifier();
                self.take_token(LParen);
                let arguments = self.take_expression_list();
                self.take_token(RParen);
                SubroutineCall::Method {
                    this_name: name,
                    method_name,
                    arguments,
                }
            }
            _ => panic!("expected subroutine call"),
        }
    }

    fn take_subroutine_return_type(&mut self) -> Option<Type> {
        if let Some(Token {
            kind: TokenKind::Keyword(KeywordTokenVariant::Void),
            ..
        }) = self.token_iter.peek()
        {
            self.token_iter.next();
            None
        } else {
            Some(self.take_type())
        }
    }

    fn maybe_take_parameter(&mut self) -> Option<Parameter> {
        self.maybe_take_type().map(|type_name| {
            let var_name = self.take_identifier();
            Parameter {
                type_name,
                var_name,
            }
        })
    }

    fn take_parameters(&mut self) -> Vec<Parameter> {
        let mut result = Vec::new();
        if let Some(parameter) = self.maybe_take_parameter() {
            result.push(parameter);

            while let Some(Token {
                kind: TokenKind::Comma,
                ..
            }) = self.token_iter.peek()
            {
                self.token_iter.next(); // comma
                result.push(
                    self.maybe_take_parameter()
                        .unwrap_or_else(|| panic!("expected parameter after comma")),
                );
            }
        }
        result
    }

    fn maybe_take_array_index(&mut self) -> Option<Expression> {
        self.token_iter
            .next_if(|token| {
                matches!(
                    token,
                    Token {
                        kind: TokenKind::LSquareBracket,
                        ..
                    }
                )
            })
            .map(|_| {
                let expression = self.take_expression();
                self.take_token(TokenKind::RSquareBracket);
                expression
            })
    }

    fn take_let_statement(&mut self) -> Statement {
        self.token_iter.next(); // "let" keyword
        let var_name = self.take_identifier();
        let array_index = self.maybe_take_array_index();
        self.take_token(TokenKind::Operator(Equals));
        let value = self.take_expression();
        self.take_token(TokenKind::Semicolon);
        Statement::Let {
            var_name,
            array_index,
            value,
        }
    }

    fn take_statement_block(&mut self) -> Vec<Statement> {
        self.take_token(TokenKind::LCurly);
        let statements = self.take_statements();
        self.take_token(TokenKind::RCurly);
        statements
    }

    fn maybe_take_else_block(&mut self) -> Option<Vec<Statement>> {
        self.token_iter
            .next_if(|token| {
                matches!(
                    token,
                    Token {
                        kind: TokenKind::Keyword(KeywordTokenVariant::Else),
                        ..
                    }
                )
            })
            .map(|_| self.take_statement_block())
    }

    fn take_if_statement(&mut self) -> Statement {
        self.token_iter.next(); // "if" keyword
        self.take_token(TokenKind::LParen);
        let condition = self.take_expression();
        self.take_token(TokenKind::RParen);
        let if_statements = self.take_statement_block();
        let else_statements = self.maybe_take_else_block();
        Statement::If {
            condition,
            if_statements,
            else_statements,
        }
    }

    fn take_while_statement(&mut self) -> Statement {
        self.token_iter.next(); // "while" keyword
        self.take_token(TokenKind::LParen);
        let expression = self.take_expression();
        self.take_token(TokenKind::RParen);
        let statements = self.take_statement_block();
        Statement::While {
            condition: expression,
            statements,
        }
    }

    fn take_do_statement(&mut self) -> Statement {
        self.token_iter.next(); // "do" keyword
        let identifier = self.take_identifier();
        let subroutine_call = self.take_subroutine_call(identifier);
        self.take_token(TokenKind::Semicolon);
        Statement::Do(subroutine_call)
    }

    fn take_return_statement(&mut self) -> Statement {
        self.token_iter.next(); // "return" keyword
        let expression = self.maybe_take_expression_with_binding_power(0);
        self.take_token(TokenKind::Semicolon);
        Statement::Return(expression)
    }

    fn maybe_take_statement(&mut self) -> Option<Statement> {
        use KeywordTokenVariant::*;
        if let Some(Token {
            kind: TokenKind::Keyword(keyword),
            ..
        }) = self.token_iter.peek()
        {
            match keyword {
                Let => Some(self.take_let_statement()),
                If => Some(self.take_if_statement()),
                While => Some(self.take_while_statement()),
                Do => Some(self.take_do_statement()),
                Return => Some(self.take_return_statement()),
                _ => None,
            }
        } else {
            None
        }
    }

    fn take_statements(&mut self) -> Vec<Statement> {
        let mut result = Vec::new();
        while let Some(statement) = self.maybe_take_statement() {
            result.push(statement);
        }
        result
    }

    fn take_var_declaration(&mut self) -> VarDeclaration {
        if let Some(Token {
            kind: TokenKind::Keyword(KeywordTokenVariant::Var),
            ..
        }) = self.token_iter.next()
        {
            let type_name = self.take_type();
            let var_names = self.take_var_names();
            self.take_token(TokenKind::Semicolon);
            VarDeclaration {
                type_name,
                var_names,
            }
        } else {
            panic!("expected var keyword");
        }
    }

    fn take_var_declarations(&mut self) -> Vec<VarDeclaration> {
        let mut result = Vec::new();
        while let Some(Token {
            kind: TokenKind::Keyword(KeywordTokenVariant::Var),
            ..
        }) = self.token_iter.peek()
        {
            result.push(self.take_var_declaration());
        }
        result
    }

    fn take_subroutine_body(&mut self) -> SubroutineBody {
        self.take_token(TokenKind::LCurly);
        let var_declarations = self.take_var_declarations();
        let statements = self.take_statements();
        self.take_token(TokenKind::RCurly);
        SubroutineBody {
            var_declarations,
            statements,
        }
    }

    fn take_subroutine_declaration(&mut self) -> SubroutineDeclaration {
        use KeywordTokenVariant::*;
        if let Some(Token {
            kind: TokenKind::Keyword(keyword),
            ..
        }) = self.token_iter.next()
        {
            let subroutine_kind = match keyword {
                Constructor => SubroutineKind::Constructor,
                Function => SubroutineKind::Function,
                Method => SubroutineKind::Method,
                _ => panic!("expected subroutine kind"),
            };

            let return_type = self.take_subroutine_return_type();
            let name = self.take_identifier();
            self.take_token(TokenKind::LParen);
            let parameters = self.take_parameters();
            self.take_token(TokenKind::RParen);
            let body = self.take_subroutine_body();
            SubroutineDeclaration {
                subroutine_kind,
                return_type,
                name,
                parameters,
                body,
            }
        } else {
            panic!("expected subroutine kind");
        }
    }

    fn maybe_take_subroutine_declaration(&mut self) -> Option<SubroutineDeclaration> {
        use KeywordTokenVariant::*;
        if let Some(Token {
            kind: TokenKind::Keyword(Constructor | Function | Method),
            ..
        }) = self.token_iter.peek()
        {
            Some(self.take_subroutine_declaration())
        } else {
            None
        }
    }

    fn take_class_subroutine_declarations(&mut self) -> Vec<SubroutineDeclaration> {
        let mut result = Vec::new();
        while let Some(subroutine_declaration) = self.maybe_take_subroutine_declaration() {
            result.push(subroutine_declaration);
        }
        result
    }

    fn take_class_var_declaration_qualifier(&mut self) -> ClassVarDeclarationKind {
        use KeywordTokenVariant::*;
        match self.token_iter.next() {
            Some(Token {
                kind: TokenKind::Keyword(keyword),
                ..
            }) => match keyword {
                Static => ClassVarDeclarationKind::Static,
                Field => ClassVarDeclarationKind::Field,
                _ => panic!("expected var declaration qualifier",),
            },
            _ => panic!("expected var declaration qualifier",),
        }
    }

    fn take_var_name(&mut self) -> String {
        if let Some(Token {
            kind: TokenKind::Identifier(var_name),
            ..
        }) = self.token_iter.next()
        {
            var_name
        } else {
            panic!("expected var name")
        }
    }

    fn take_var_names(&mut self) -> Vec<String> {
        // There has to be at least one var name.
        let first_var = self.take_var_name();
        let mut names = vec![first_var];
        while let Some(Token {
            kind: TokenKind::Comma,
            ..
        }) = self.token_iter.peek()
        {
            self.token_iter.next(); // comma
            let var = self.take_var_name();
            names.push(var);
        }
        names
    }

    fn take_type(&mut self) -> Type {
        use KeywordTokenVariant::*;
        match self.token_iter.next() {
            Some(Token { kind, .. }) => match kind {
                TokenKind::Keyword(Int) => Type::Int,
                TokenKind::Keyword(Char) => Type::Char,
                TokenKind::Keyword(Boolean) => Type::Boolean,
                TokenKind::Identifier(class_name) => Type::ClassName(class_name),
                _ => panic!("expected var type name"),
            },
            _ => panic!("expected var type name"),
        }
    }

    fn maybe_take_type(&mut self) -> Option<Type> {
        use KeywordTokenVariant::*;
        if let Some(
            Token {
                kind: TokenKind::Keyword(Int | Char | Boolean) | TokenKind::Identifier(_),
                ..
            },
            ..,
        ) = self.token_iter.peek()
        {
            Some(self.take_type())
        } else {
            None
        }
    }

    fn take_class_var_declaration(&mut self) -> ClassVarDeclaration {
        let qualifier = self.take_class_var_declaration_qualifier();
        let type_name = self.take_type();
        let var_names = self.take_var_names();
        self.take_token(TokenKind::Semicolon);
        ClassVarDeclaration {
            qualifier,
            type_name,
            var_names,
        }
    }

    fn maybe_take_class_var_declaration(&mut self) -> Option<ClassVarDeclaration> {
        use KeywordTokenVariant::*;
        match self.token_iter.peek().expect("unexpected end of input") {
            Token {
                kind: TokenKind::Keyword(Static | Field),
                ..
            } => Some(self.take_class_var_declaration()),
            _ => None,
        }
    }

    fn take_class_var_declarations(&mut self) -> Vec<ClassVarDeclaration> {
        let mut result = Vec::new();
        while let Some(class_var_declaration) = self.maybe_take_class_var_declaration() {
            result.push(class_var_declaration);
        }
        result
    }

    fn take_class(&mut self) -> Class {
        self.take_class_keyword();
        let name = self.take_identifier();
        self.take_token(TokenKind::LCurly);
        let var_declarations = self.take_class_var_declarations();
        let subroutine_declarations = self.take_class_subroutine_declarations();
        self.take_token(TokenKind::RCurly);
        Class {
            name,
            var_declarations,
            subroutine_declarations,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_expression(source: &str) -> Expression {
        let tokens: Vec<_> = Tokenizer::new(token_defs())
            .tokenize(source)
            .into_iter()
            .filter(|token| {
                !matches!(
                    token,
                    Token {
                        kind: TokenKind::Whitespace
                            | TokenKind::MultiLineComment
                            | TokenKind::SingleLineComment,
                        ..
                    }
                )
            })
            .collect();
        let mut parser = Parser {
            token_iter: tokens.into_iter().peekable(),
            sourcemap: SourceMap::new(),
        };
        parser.take_expression()
    }

    #[test]
    fn test_simple_class() {
        assert_eq!(
            parse("class foo {}"),
            Class {
                name: "foo".to_string(),
                var_declarations: vec![],
                subroutine_declarations: vec![],
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
                name: "foo".to_string(),
                var_declarations: vec![ClassVarDeclaration {
                    qualifier: ClassVarDeclarationKind::Static,
                    type_name: Type::Int,
                    var_names: vec!["bar".to_string()],
                }],
                subroutine_declarations: vec![],
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
                name: "foo".to_string(),
                var_declarations: vec![
                    ClassVarDeclaration {
                        qualifier: ClassVarDeclarationKind::Static,
                        type_name: Type::Int,
                        var_names: vec!["bar".to_string()],
                    },
                    ClassVarDeclaration {
                        qualifier: ClassVarDeclarationKind::Field,
                        type_name: Type::Char,
                        var_names: vec!["baz".to_string(), "buz".to_string(), "boz".to_string()],
                    },
                    ClassVarDeclaration {
                        qualifier: ClassVarDeclarationKind::Field,
                        type_name: Type::Boolean,
                        var_names: vec!["a".to_string(), "b".to_string(), "c".to_string(),],
                    }
                ],
                subroutine_declarations: vec![],
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
                name: "foo".to_string(),
                var_declarations: vec![],
                subroutine_declarations: vec![
                    SubroutineDeclaration {
                        subroutine_kind: SubroutineKind::Constructor,
                        return_type: Some(Type::Boolean),
                        parameters: vec![
                            Parameter {
                                type_name: Type::Int,
                                var_name: "abc".to_string(),
                            },
                            Parameter {
                                type_name: Type::Char,
                                var_name: "def".to_string(),
                            },
                            Parameter {
                                type_name: Type::ClassName("foo".to_string()),
                                var_name: "ghi".to_string(),
                            }
                        ],
                        name: "bar".to_string(),
                        body: SubroutineBody {
                            var_declarations: vec![],
                            statements: vec![],
                        },
                    },
                    SubroutineDeclaration {
                        subroutine_kind: SubroutineKind::Function,
                        return_type: Some(Type::Char),
                        parameters: vec![Parameter {
                            type_name: Type::Boolean,
                            var_name: "_123".to_string(),
                        },],
                        name: "baz".to_string(),
                        body: SubroutineBody {
                            var_declarations: vec![],
                            statements: vec![],
                        },
                    },
                    SubroutineDeclaration {
                        subroutine_kind: SubroutineKind::Method,
                        return_type: None,
                        parameters: vec![],
                        name: "qux".to_string(),
                        body: SubroutineBody {
                            var_declarations: vec![],
                            statements: vec![],
                        },
                    }
                ],
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
                name: "foo".to_string(),
                var_declarations: vec![],
                subroutine_declarations: vec![SubroutineDeclaration {
                    subroutine_kind: SubroutineKind::Constructor,
                    return_type: Some(Type::Int),
                    parameters: vec![],
                    name: "blah".to_string(),
                    body: SubroutineBody {
                        var_declarations: vec![VarDeclaration {
                            type_name: Type::Int,
                            var_names: vec!["a".to_string()],
                        }],
                        statements: vec![
                            Statement::Let {
                                var_name: "a".to_string(),
                                array_index: None,
                                value: Expression::PrimitiveTerm(IntegerConstant(
                                    "1234".to_string()
                                ))
                            },
                            Statement::Let {
                                var_name: "b".to_string(),
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
                                            subroutine_name: "foobar".to_string(),
                                            arguments: vec![]
                                        }),
                                        Statement::Do(SubroutineCall::Direct {
                                            subroutine_name: "foobar".to_string(),
                                            arguments: vec![Expression::PrimitiveTerm(
                                                IntegerConstant("1".to_string())
                                            )]
                                        }),
                                        Statement::Do(SubroutineCall::Direct {
                                            subroutine_name: "foobar".to_string(),
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
                                            this_name: "foo".to_string(),
                                            method_name: "bar".to_string(),
                                            arguments: vec![]
                                        }),
                                        Statement::Do(SubroutineCall::Method {
                                            this_name: "foo".to_string(),
                                            method_name: "bar".to_string(),
                                            arguments: vec![Expression::PrimitiveTerm(
                                                IntegerConstant("1".to_string())
                                            )]
                                        }),
                                        Statement::Do(SubroutineCall::Method {
                                            this_name: "foo".to_string(),
                                            method_name: "bar".to_string(),
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
                    },
                }],
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
                name: "foo".to_string(),
                var_declarations: vec![],
                subroutine_declarations: vec![SubroutineDeclaration {
                    subroutine_kind: SubroutineKind::Method,
                    return_type: None,
                    parameters: vec![],
                    name: "bar".to_string(),
                    body: SubroutineBody {
                        var_declarations: vec![],
                        statements: vec![Statement::Let {
                            var_name: "a".to_string(),
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
                    },
                }],
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
                        subroutine_name: "foo".to_string(),
                        arguments: vec![
                            Expression::PrimitiveTerm(IntegerConstant("1".to_string())),
                            Expression::SubroutineCall(SubroutineCall::Method {
                                this_name: "baz".to_string(),
                                method_name: "bar".to_string(),
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
                        subroutine_name: "foo".to_string(),
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
                                this_name: "buz".to_string(),
                                method_name: "boz".to_string(),
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
