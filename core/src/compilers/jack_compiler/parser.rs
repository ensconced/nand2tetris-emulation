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
    let tokens = Tokenizer::new(token_defs()).tokenize(source);
    let filtered = tokens.filter(|token| {
        !matches!(
            token.kind,
            TokenKind::Whitespace | TokenKind::SingleLineComment | TokenKind::MultiLineComment
        )
    });
    let cleaned_tokens: Box<dyn Iterator<Item = Token<TokenKind>>> = Box::new(filtered);
    let mut parser = Parser {
        tokens: cleaned_tokens.peekable(),
        sourcemap: SourceMap::new(),
    };
    parser.take_class()
}

struct Parser {
    tokens: PeekableTokens<TokenKind>,
    sourcemap: SourceMap,
}

impl Parser {
    fn maybe_take_primitive_expression(&mut self) -> Option<Expression> {
        use TokenKind::*;
        let peeked_token = self.tokens.peek().cloned();
        peeked_token.and_then(|token| match token.kind {
            IntegerLiteral(string) => {
                self.tokens.next();
                Some(Expression::PrimitiveTerm(IntegerConstant(string)))
            }
            StringLiteral(string) => {
                self.tokens.next();
                Some(Expression::PrimitiveTerm(StringConstant(string)))
            }
            Keyword(KeywordTokenVariant::True) => {
                self.tokens.next();
                Some(Expression::PrimitiveTerm(PrimitiveTermVariant::True))
            }
            Keyword(KeywordTokenVariant::False) => {
                self.tokens.next();
                Some(Expression::PrimitiveTerm(PrimitiveTermVariant::False))
            }
            Keyword(KeywordTokenVariant::Null) => {
                self.tokens.next();
                Some(Expression::PrimitiveTerm(PrimitiveTermVariant::Null))
            }
            Keyword(KeywordTokenVariant::This) => {
                self.tokens.next();
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
        if let Some(Token { kind: LParen, .. }) = self.tokens.peek() {
            self.tokens.next();
            let expr = self.take_expression();
            self.take_token(RParen);
            Some(expr)
        } else {
            None
        }
    }

    fn maybe_take_term_starting_with_identifier(&mut self) -> Option<Expression> {
        use TokenKind::*;
        let p = self.tokens.peek();
        if let Some(Token {
            kind: Identifier(string),
            ..
        }) = p
        {
            let string = string.to_string();
            let identifier = self.take_identifier();
            match self.tokens.peek() {
                Some(Token {
                    kind: LSquareBracket,
                    ..
                }) => Some(self.take_array_access(identifier.name)),
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
        }) = self.tokens.peek()
        {
            let op = op.clone();
            let rbp = prefix_precedence(op.clone()).expect("invalid prefix operator");
            self.tokens.next();
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
            match self.tokens.peek() {
                Some(Token {
                    kind: Operator(op), ..
                }) => {
                    let op = op.clone();
                    let (lbp, rbp) = infix_precedence(op.clone()).expect("invalid infix operator");
                    if lbp < binding_power {
                        break;
                    }
                    self.tokens.next();
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
        self.tokens
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
        self.tokens
            .next_if(|token| token.kind == token_kind)
            .unwrap_or_else(|| panic!("expected token {:?}", token_kind))
    }

    fn take_identifier(&mut self) -> Identifier {
        if let Some(Token {
            kind: TokenKind::Identifier(string),
            ..
        }) = self.tokens.next()
        {
            Identifier { name: string }
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
            while let Some(Token { kind: Comma, .. }) = self.tokens.peek() {
                self.tokens.next();
                result.push(self.take_expression());
            }
        }
        result
    }

    fn take_subroutine_call(&mut self, name: Identifier) -> SubroutineCall {
        use TokenKind::*;
        match self.tokens.peek() {
            Some(Token { kind: LParen, .. }) => {
                // Direct function call
                self.tokens.next(); // LParen
                let arguments = self.take_expression_list();
                self.take_token(RParen);
                SubroutineCall::Direct {
                    subroutine_name: name,
                    arguments,
                }
            }
            Some(Token { kind: Dot, .. }) => {
                // Method call
                self.tokens.next(); // Dot
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
        }) = self.tokens.peek()
        {
            self.tokens.next();
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
            }) = self.tokens.peek()
            {
                self.tokens.next(); // comma
                result.push(
                    self.maybe_take_parameter()
                        .unwrap_or_else(|| panic!("expected parameter after comma")),
                );
            }
        }
        result
    }

    fn maybe_take_array_index(&mut self) -> Option<Expression> {
        self.tokens
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
        self.tokens.next(); // "let" keyword
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
        self.tokens
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
        self.tokens.next(); // "if" keyword
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
        self.tokens.next(); // "while" keyword
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
        self.tokens.next(); // "do" keyword
        let identifier = self.take_identifier();
        let subroutine_call = self.take_subroutine_call(identifier);
        self.take_token(TokenKind::Semicolon);
        Statement::Do(subroutine_call)
    }

    fn take_return_statement(&mut self) -> Statement {
        self.tokens.next(); // "return" keyword
        let expression = self.maybe_take_expression_with_binding_power(0);
        self.take_token(TokenKind::Semicolon);
        Statement::Return(expression)
    }

    fn maybe_take_statement(&mut self) -> Option<Statement> {
        use KeywordTokenVariant::*;
        if let Some(Token {
            kind: TokenKind::Keyword(keyword),
            ..
        }) = self.tokens.peek()
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
        }) = self.tokens.next()
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
        }) = self.tokens.peek()
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
        }) = self.tokens.next()
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
        }) = self.tokens.peek()
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
        match self.tokens.next() {
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

    fn take_var_name(&mut self) -> Identifier {
        if let Some(Token {
            kind: TokenKind::Identifier(var_name),
            ..
        }) = self.tokens.next()
        {
            Identifier { name: var_name }
        } else {
            panic!("expected var name")
        }
    }

    fn take_var_names(&mut self) -> VarNames {
        // There has to be at least one var name.
        let first_var = self.take_var_name();
        let mut names = vec![first_var];
        while let Some(Token {
            kind: TokenKind::Comma,
            ..
        }) = self.tokens.peek()
        {
            self.tokens.next(); // comma
            let var = self.take_var_name();
            names.push(var);
        }
        VarNames { names }
    }

    fn take_type(&mut self) -> Type {
        use KeywordTokenVariant::*;
        match self.tokens.next() {
            Some(Token { kind, .. }) => {
                let type_variant = match kind {
                    TokenKind::Keyword(Int) => TypeVariant::Int,
                    TokenKind::Keyword(Char) => TypeVariant::Char,
                    TokenKind::Keyword(Boolean) => TypeVariant::Boolean,
                    TokenKind::Identifier(class_name) => {
                        TypeVariant::ClassName(Identifier { name: class_name })
                    }
                    _ => panic!("expected var type name"),
                };
                Type {
                    variant: type_variant,
                }
            }
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
        ) = self.tokens.peek()
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
        match self.tokens.peek().expect("unexpected end of input") {
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
        let tokens = Tokenizer::new(token_defs()).tokenize(source);
        let filtered = tokens.filter(|token| {
            !matches!(
                token.kind,
                TokenKind::Whitespace | TokenKind::SingleLineComment | TokenKind::MultiLineComment
            )
        });
        let cleaned_tokens: Box<dyn Iterator<Item = Token<TokenKind>>> = Box::new(filtered);
        let mut parser = Parser {
            tokens: cleaned_tokens.peekable(),
            sourcemap: SourceMap::new(),
        };
        parser.take_expression()
    }

    #[test]
    fn test_simple_class() {
        assert_eq!(
            parse("class foo {}"),
            Class {
                name: Identifier {
                    name: "foo".to_string(),
                },
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
                name: Identifier {
                    name: "foo".to_string(),
                },
                var_declarations: vec![ClassVarDeclaration {
                    qualifier: ClassVarDeclarationKind::Static,
                    type_name: Type {
                        variant: TypeVariant::Int,
                    },
                    var_names: VarNames {
                        names: vec![Identifier {
                            name: "bar".to_string(),
                        }],
                    },
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
                name: Identifier {
                    name: "foo".to_string(),
                },
                var_declarations: vec![
                    ClassVarDeclaration {
                        qualifier: ClassVarDeclarationKind::Static,
                        type_name: Type {
                            variant: TypeVariant::Int,
                        },
                        var_names: VarNames {
                            names: vec![Identifier {
                                name: "bar".to_string(),
                            }],
                        },
                    },
                    ClassVarDeclaration {
                        qualifier: ClassVarDeclarationKind::Field,
                        type_name: Type {
                            variant: TypeVariant::Char,
                        },
                        var_names: VarNames {
                            names: vec![
                                Identifier {
                                    name: "baz".to_string(),
                                },
                                Identifier {
                                    name: "buz".to_string(),
                                },
                                Identifier {
                                    name: "boz".to_string(),
                                }
                            ],
                        },
                    },
                    ClassVarDeclaration {
                        qualifier: ClassVarDeclarationKind::Field,
                        type_name: Type {
                            variant: TypeVariant::Boolean,
                        },
                        var_names: VarNames {
                            names: vec![
                                Identifier {
                                    name: "a".to_string(),
                                },
                                Identifier {
                                    name: "b".to_string(),
                                },
                                Identifier {
                                    name: "c".to_string(),
                                },
                            ],
                        },
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
                name: Identifier {
                    name: "foo".to_string(),
                },
                var_declarations: vec![],
                subroutine_declarations: vec![
                    SubroutineDeclaration {
                        subroutine_kind: SubroutineKind::Constructor,
                        return_type: Some(Type {
                            variant: TypeVariant::Boolean,
                        }),
                        parameters: vec![
                            Parameter {
                                type_name: Type {
                                    variant: TypeVariant::Int,
                                },
                                var_name: Identifier {
                                    name: "abc".to_string(),
                                },
                            },
                            Parameter {
                                type_name: Type {
                                    variant: TypeVariant::Char,
                                },
                                var_name: Identifier {
                                    name: "def".to_string(),
                                },
                            },
                            Parameter {
                                type_name: Type {
                                    variant: TypeVariant::ClassName(Identifier {
                                        name: "foo".to_string(),
                                    }),
                                },
                                var_name: Identifier {
                                    name: "ghi".to_string(),
                                },
                            }
                        ],
                        name: Identifier {
                            name: "bar".to_string(),
                        },
                        body: SubroutineBody {
                            var_declarations: vec![],
                            statements: vec![],
                        },
                    },
                    SubroutineDeclaration {
                        subroutine_kind: SubroutineKind::Function,
                        return_type: Some(Type {
                            variant: TypeVariant::Char,
                        }),
                        parameters: vec![Parameter {
                            type_name: Type {
                                variant: TypeVariant::Boolean,
                            },
                            var_name: Identifier {
                                name: "_123".to_string(),
                            },
                        },],
                        name: Identifier {
                            name: "baz".to_string(),
                        },
                        body: SubroutineBody {
                            var_declarations: vec![],
                            statements: vec![],
                        },
                    },
                    SubroutineDeclaration {
                        subroutine_kind: SubroutineKind::Method,
                        return_type: None,
                        parameters: vec![],
                        name: Identifier {
                            name: "qux".to_string(),
                        },
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
                name: Identifier {
                    name: "foo".to_string(),
                },
                var_declarations: vec![],
                subroutine_declarations: vec![SubroutineDeclaration {
                    subroutine_kind: SubroutineKind::Constructor,
                    return_type: Some(Type {
                        variant: TypeVariant::Int,
                    }),
                    parameters: vec![],
                    name: Identifier {
                        name: "blah".to_string(),
                    },
                    body: SubroutineBody {
                        var_declarations: vec![VarDeclaration {
                            type_name: Type {
                                variant: TypeVariant::Int,
                            },
                            var_names: VarNames {
                                names: vec![Identifier {
                                    name: "a".to_string(),
                                }],
                            },
                        }],
                        statements: vec![
                            Statement::Let {
                                var_name: Identifier {
                                    name: "a".to_string(),
                                },
                                array_index: None,
                                value: Expression::PrimitiveTerm(IntegerConstant(
                                    "1234".to_string()
                                ))
                            },
                            Statement::Let {
                                var_name: Identifier {
                                    name: "b".to_string(),
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
                                            },
                                            arguments: vec![]
                                        }),
                                        Statement::Do(SubroutineCall::Direct {
                                            subroutine_name: Identifier {
                                                name: "foobar".to_string(),
                                            },
                                            arguments: vec![Expression::PrimitiveTerm(
                                                IntegerConstant("1".to_string())
                                            )]
                                        }),
                                        Statement::Do(SubroutineCall::Direct {
                                            subroutine_name: Identifier {
                                                name: "foobar".to_string(),
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
                                            },
                                            method_name: Identifier {
                                                name: "bar".to_string(),
                                            },
                                            arguments: vec![]
                                        }),
                                        Statement::Do(SubroutineCall::Method {
                                            this_name: Identifier {
                                                name: "foo".to_string(),
                                            },
                                            method_name: Identifier {
                                                name: "bar".to_string(),
                                            },
                                            arguments: vec![Expression::PrimitiveTerm(
                                                IntegerConstant("1".to_string())
                                            )]
                                        }),
                                        Statement::Do(SubroutineCall::Method {
                                            this_name: Identifier {
                                                name: "foo".to_string(),
                                            },
                                            method_name: Identifier {
                                                name: "bar".to_string(),
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
                name: Identifier {
                    name: "foo".to_string(),
                },
                var_declarations: vec![],
                subroutine_declarations: vec![SubroutineDeclaration {
                    subroutine_kind: SubroutineKind::Method,
                    return_type: None,
                    parameters: vec![],
                    name: Identifier {
                        name: "bar".to_string(),
                    },
                    body: SubroutineBody {
                        var_declarations: vec![],
                        statements: vec![Statement::Let {
                            var_name: Identifier {
                                name: "a".to_string(),
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
                        subroutine_name: Identifier {
                            name: "foo".to_string(),
                        },
                        arguments: vec![
                            Expression::PrimitiveTerm(IntegerConstant("1".to_string())),
                            Expression::SubroutineCall(SubroutineCall::Method {
                                this_name: Identifier {
                                    name: "baz".to_string(),
                                },
                                method_name: Identifier {
                                    name: "bar".to_string(),
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
                                },
                                method_name: Identifier {
                                    name: "boz".to_string(),
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
