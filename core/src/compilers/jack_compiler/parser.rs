use std::rc::Rc;

use serde::Serialize;
use std::ops::Range;

use super::{
    jack_node_types::{PrimitiveTermVariant::*, *},
    sourcemap::{JackNode, JackParserSourceMap},
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

// Soon a version of this should actually be present in the std lib.
fn unzip<T, S>(tuple_opt: Option<(T, S)>) -> (Option<T>, Option<S>) {
    tuple_opt.map_or_else(|| (None, None), |(t, s)| (Some(t), Some(s)))
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

#[derive(Serialize)]
pub struct ParserOutput {
    pub class: Rc<Class>,
    sourcemap: JackParserSourceMap,
    jack_nodes: Vec<JackNode>,
    tokens: Vec<Token<TokenKind>>,
}

pub fn parse(source: &str) -> ParserOutput {
    let tokens: Vec<_> = Tokenizer::new(token_defs()).tokenize(source);
    let tokens_without_whitespace: Vec<_> = tokens
        .iter()
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

    let cloned_tokens_without_whitespace: Vec<_> = tokens_without_whitespace
        .into_iter()
        .map(|x| x.clone())
        .collect();

    let mut parser = Parser {
        token_iter: cloned_tokens_without_whitespace.iter().peekable(),
        sourcemap: JackParserSourceMap::new(),
        jack_nodes: Vec::new(),
    };
    let class = parser.take_class();
    ParserOutput {
        class,
        sourcemap: parser.sourcemap,
        jack_nodes: parser.jack_nodes,
        tokens,
    }
}

struct Parser<'a> {
    token_iter: PeekableTokens<'a, TokenKind>,
    sourcemap: JackParserSourceMap,
    jack_nodes: Vec<JackNode>,
}

impl<'a> Parser<'a> {
    fn record_jack_node(&mut self, jack_node: JackNode, token_range: Range<usize>) -> usize {
        let idx = self.jack_nodes.len();
        self.jack_nodes.push(jack_node);
        self.sourcemap
            .jack_node_idx_to_token_idx
            .insert(idx, token_range.clone());

        for token_idx in token_range {
            let token_jack_node_idxs = self
                .sourcemap
                .token_idx_to_jack_node_idxs
                .entry(token_idx)
                .or_default();
            token_jack_node_idxs.push(idx);
        }
        idx
    }

    fn maybe_take_primitive_expression(
        &mut self,
    ) -> Option<(IndexedJackNode<Expression>, Range<usize>)> {
        use TokenKind::*;
        let peeked_token = self.token_iter.peek().cloned();
        let (expression, exp_token_idx) = peeked_token.and_then(|token| {
            let maybe_exp = match &token.kind {
                IntegerLiteral(string) => {
                    self.token_iter.next();
                    Some(Expression::PrimitiveTerm(IntegerConstant(
                        string.to_string(),
                    )))
                }
                StringLiteral(string) => {
                    self.token_iter.next();
                    Some(Expression::PrimitiveTerm(StringConstant(
                        string.to_string(),
                    )))
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
            };
            maybe_exp.map(|exp| (exp, token.idx))
        })?;
        let rc = Rc::new(expression);
        let jack_node = JackNode::ExpressionNode(rc.clone());
        let token_range = exp_token_idx..exp_token_idx + 1;
        let node_idx = self.record_jack_node(jack_node, token_range.clone());
        Some((IndexedJackNode { node: rc, node_idx }, token_range))
    }

    fn take_array_access(
        &mut self,
        var_name: String,
    ) -> (IndexedJackNode<Expression>, Range<usize>) {
        use TokenKind::*;
        let l_bracket = self.take_token(LSquareBracket);
        let (index_expr, _) = self.take_expression();
        let r_bracket = self.take_token(RSquareBracket);
        let rc = Rc::new(Expression::ArrayAccess {
            var_name,
            index: index_expr,
        });
        let jack_node = JackNode::ExpressionNode(rc.clone());
        let token_range = l_bracket.idx..r_bracket.idx + 1;
        let node_idx = self.record_jack_node(jack_node, token_range.clone());
        (IndexedJackNode { node: rc, node_idx }, token_range)
    }

    fn maybe_take_parenthesized_expression(
        &mut self,
    ) -> Option<(IndexedJackNode<Expression>, Range<usize>)> {
        use TokenKind::*;
        if let Some(Token {
            kind: LParen,
            idx: l_paren_idx,
            ..
        }) = self.token_iter.peek()
        {
            let token_range_start = *l_paren_idx;
            self.token_iter.next();
            let (expr, _) = self.take_expression();
            let parenthesized_expr = Rc::new(Expression::Parenthesized(expr));
            let jack_node = JackNode::ExpressionNode(parenthesized_expr.clone());
            let r_paren = self.take_token(RParen);
            let token_range = token_range_start..r_paren.idx + 1;
            let node_idx = self.record_jack_node(jack_node, token_range.clone());
            Some((
                IndexedJackNode {
                    node: parenthesized_expr,
                    node_idx,
                },
                token_range,
            ))
        } else {
            None
        }
    }

    fn maybe_take_expression_starting_with_identifier(
        &mut self,
    ) -> Option<(IndexedJackNode<Expression>, Range<usize>)> {
        use TokenKind::*;
        // TODO - this is not very nice...maybe we should just use two tokens of
        // lookahead instead? (I think itertools would make that easy).
        let peeked_token = self.token_iter.peek();
        if let Some(Token {
            kind: Identifier(string),
            ..
        }) = peeked_token
        {
            let string = string.to_string();
            let (identifier, identifier_token_idx) = self.take_identifier();
            match self.token_iter.peek() {
                Some(Token {
                    kind: LSquareBracket,
                    ..
                }) => Some(self.take_array_access(identifier)),
                Some(Token {
                    kind: Dot | LParen, ..
                }) => {
                    let (subroutine_call, subroutine_call_token_range) =
                        self.take_subroutine_call(identifier, identifier_token_idx);
                    let expr = Expression::SubroutineCall(subroutine_call);
                    let rc = Rc::new(expr);
                    let node_idx = self.record_jack_node(
                        JackNode::ExpressionNode(rc.clone()),
                        subroutine_call_token_range.clone(),
                    );
                    Some((
                        IndexedJackNode { node: rc, node_idx },
                        subroutine_call_token_range,
                    ))
                }
                _ => {
                    let expr = Expression::Variable(string);
                    let rc = Rc::new(expr);
                    let token_range = identifier_token_idx..identifier_token_idx + 1;
                    let node_idx = self.record_jack_node(
                        JackNode::ExpressionNode(rc.clone()),
                        token_range.clone(),
                    );
                    Some((IndexedJackNode { node: rc, node_idx }, token_range))
                }
            }
        } else {
            None
        }
    }

    fn maybe_take_unary_expression(
        &mut self,
    ) -> Option<(IndexedJackNode<Expression>, Range<usize>)> {
        use TokenKind::*;
        if let Some(Token {
            kind: Operator(op),
            idx,
            ..
        }) = self.token_iter.peek()
        {
            let op_token_idx = *idx;
            let op = op.clone();
            self.token_iter.next();
            let right_binding_power =
                prefix_precedence(op.clone()).expect("invalid prefix operator");
            let (operand, operand_token_range) = self
                .maybe_take_expression_with_binding_power(right_binding_power)
                .expect("unary operator has no operand");
            let operator = match op {
                OperatorVariant::Minus => UnaryOperator::Minus,
                OperatorVariant::Tilde => UnaryOperator::Not,
                _ => panic!("invalid unary operator"),
            };
            let rc = Rc::new(Expression::Unary { operator, operand });
            let token_range = op_token_idx..operand_token_range.end;
            let node_idx =
                self.record_jack_node(JackNode::ExpressionNode(rc.clone()), token_range.clone());
            Some((IndexedJackNode { node: rc, node_idx }, token_range))
        } else {
            None
        }
    }

    fn maybe_append_rhs_to_lhs(
        &mut self,
        mut lhs: IndexedJackNode<Expression>,
        mut lhs_token_range: Range<usize>,
        binding_power: u8,
    ) -> Option<(IndexedJackNode<Expression>, Range<usize>)> {
        use TokenKind::*;
        if let Some(Token {
            kind: Operator(op), ..
        }) = self.token_iter.peek()
        {
            let (lbp, rbp) = infix_precedence(op.clone()).expect("invalid infix operator");
            if lbp < binding_power {
                // There is no rhs to append - the next term will instead associate towards the right.
                return None;
            }
            self.token_iter.next();
            let (rhs, rhs_exp_token_range) = self
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

            let new_lhs = Rc::new(Expression::Binary { operator, lhs, rhs });
            lhs_token_range = lhs_token_range.start..rhs_exp_token_range.end;
            let new_lhs_jack_node_idx = self.record_jack_node(
                JackNode::ExpressionNode(new_lhs.clone()),
                lhs_token_range.clone(),
            );
            lhs = IndexedJackNode {
                node: new_lhs,
                node_idx: new_lhs_jack_node_idx,
            };
            Some((lhs, lhs_token_range))
        } else {
            None
        }
    }

    fn maybe_take_expression_with_binding_power(
        &mut self,
        binding_power: u8,
    ) -> Option<(IndexedJackNode<Expression>, Range<usize>)> {
        let (mut lhs, mut lhs_token_range) = self
            .maybe_take_unary_expression()
            .or_else(|| self.maybe_take_primitive_expression())
            .or_else(|| self.maybe_take_expression_starting_with_identifier())
            .or_else(|| self.maybe_take_parenthesized_expression())?;

        while let Some((new_lhs, new_lhs_token_range)) =
            self.maybe_append_rhs_to_lhs(lhs, lhs_token_range.clone(), binding_power)
        {
            lhs = new_lhs;
            lhs_token_range = new_lhs_token_range
        }

        Some((lhs, lhs_token_range))
    }

    fn take_class_keyword(&mut self) -> &'a Token<TokenKind> {
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

    fn take_token(&mut self, token_kind: TokenKind) -> &'a Token<TokenKind> {
        self.token_iter
            .next_if(|token| token.kind == token_kind)
            .unwrap_or_else(|| panic!("expected token {:?}", token_kind))
    }

    fn take_identifier(&mut self) -> (String, usize) {
        if let Some(Token {
            kind: TokenKind::Identifier(string),
            idx: identifier_token_idx,
            ..
        }) = self.token_iter.next()
        {
            (string.to_string(), *identifier_token_idx)
        } else {
            panic!("expected identifier")
        }
    }

    fn take_expression(&mut self) -> (IndexedJackNode<Expression>, Range<usize>) {
        self.maybe_take_expression_with_binding_power(0)
            .expect("expected expression")
    }

    fn take_expression_list(&mut self) -> Vec<IndexedJackNode<Expression>> {
        use TokenKind::*;
        let mut result = Vec::new();
        if let Some((expression, _)) = self.maybe_take_expression_with_binding_power(0) {
            result.push(expression);
            while let Some(Token { kind: Comma, .. }) = self.token_iter.peek() {
                self.token_iter.next();
                let (expr, _) = self.take_expression();
                result.push(expr);
            }
        }
        result
    }

    fn take_subroutine_call(
        &mut self,
        name: String,
        identifier_token_idx: usize,
    ) -> ((Rc<SubroutineCall>, usize), Range<usize>) {
        use TokenKind::*;
        match self.token_iter.peek() {
            Some(Token { kind: LParen, .. }) => {
                // Direct function call
                self.token_iter.next(); // LParen
                let arguments = self.take_expression_list();
                let r_paren = self.take_token(RParen);
                let subroutine_call = SubroutineCall::Direct {
                    subroutine_name: name,
                    arguments,
                };
                let rc = Rc::new(subroutine_call);
                let token_range = identifier_token_idx..r_paren.idx + 1;
                let jack_node_idx = self.record_jack_node(
                    JackNode::SubroutineCallNode(rc.clone()),
                    token_range.clone(),
                );
                ((rc, jack_node_idx), token_range)
            }
            Some(Token { kind: Dot, .. }) => {
                // Method call
                self.token_iter.next(); // Dot
                let (method_name, method_name_token_idx) = self.take_identifier();
                self.take_token(LParen);
                let arguments = self.take_expression_list();
                let r_paren = self.take_token(RParen);
                let method = SubroutineCall::Method {
                    this_name: name,
                    method_name,
                    arguments,
                };
                let rc = Rc::new(method);
                let token_range = method_name_token_idx..r_paren.idx + 1;
                let jack_node_idx = self.record_jack_node(
                    JackNode::SubroutineCallNode(rc.clone()),
                    token_range.clone(),
                );
                ((rc, jack_node_idx), token_range)
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

    fn maybe_take_parameter(&mut self) -> Option<(Rc<Parameter>, usize)> {
        self.maybe_take_type().map(|type_name| {
            let (var_name, identifier_token_idx) = self.take_identifier();
            let parameter = Parameter {
                type_name,
                var_name,
            };
            let rc = Rc::new(parameter);
            let token_range = identifier_token_idx..identifier_token_idx + 1;
            let jack_node_idx =
                self.record_jack_node(JackNode::ParameterNode(rc.clone()), token_range);
            (rc, jack_node_idx)
        })
    }

    fn take_parameters(&mut self) -> Vec<(Rc<Parameter>, usize)> {
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

    fn maybe_take_array_index(&mut self) -> Option<IndexedJackNode<Expression>> {
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
                let (expression, _) = self.take_expression();
                self.take_token(TokenKind::RSquareBracket);
                expression
            })
    }

    fn take_let_statement(
        &mut self,
        let_keyword_token_idx: usize,
    ) -> ((Rc<Statement>, usize), Range<usize>) {
        self.token_iter.next(); // "let" keyword
        let (var_name, _) = self.take_identifier();
        let array_index = self.maybe_take_array_index();
        self.take_token(TokenKind::Operator(Equals));
        let (value, _) = self.take_expression();
        let semicolon = self.take_token(TokenKind::Semicolon);
        let statement = Statement::Let {
            var_name,
            array_index,
            value,
        };
        let rc = Rc::new(statement);
        let token_range = let_keyword_token_idx..semicolon.idx + 1;
        let jack_node_idx =
            self.record_jack_node(JackNode::StatementNode(rc.clone()), token_range.clone());
        ((rc, jack_node_idx), token_range)
    }

    fn take_statement_block(&mut self) -> (Vec<(Rc<Statement>, usize)>, Range<usize>) {
        let l_curly = self.take_token(TokenKind::LCurly);
        let statements = self.take_statements();
        let r_curly = self.take_token(TokenKind::RCurly);
        (statements, l_curly.idx..r_curly.idx + 1)
    }

    fn maybe_take_else_block(&mut self) -> Option<(Vec<(Rc<Statement>, usize)>, Range<usize>)> {
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

    fn take_if_statement(
        &mut self,
        if_keyword_token_idx: usize,
    ) -> ((Rc<Statement>, usize), Range<usize>) {
        self.token_iter.next(); // "if" keyword
        self.take_token(TokenKind::LParen);
        let (condition, _) = self.take_expression();
        self.take_token(TokenKind::RParen);
        let (if_statements, if_statements_token_range) = self.take_statement_block();
        let (else_statements, else_block_token_range) = unzip(self.maybe_take_else_block());
        let statement = Statement::If {
            condition,
            if_statements,
            else_statements,
        };
        let rc = Rc::new(statement);

        let last_part_of_token_range = else_block_token_range.unwrap_or(if_statements_token_range);
        let token_range = if_keyword_token_idx..last_part_of_token_range.end;
        let jack_node_idx =
            self.record_jack_node(JackNode::StatementNode(rc.clone()), token_range.clone());
        ((rc, jack_node_idx), token_range)
    }

    fn take_while_statement(
        &mut self,
        while_keyword_token_idx: usize,
    ) -> ((Rc<Statement>, usize), Range<usize>) {
        self.token_iter.next(); // "while" keyword
        self.take_token(TokenKind::LParen);
        let (condition, _) = self.take_expression();
        self.take_token(TokenKind::RParen);
        let (statements, statements_token_range) = self.take_statement_block();
        let statement = Statement::While {
            condition,
            statements,
        };
        let rc = Rc::new(statement);
        let token_range = while_keyword_token_idx..statements_token_range.end;
        let jack_node_idx =
            self.record_jack_node(JackNode::StatementNode(rc.clone()), token_range.clone());
        ((rc, jack_node_idx), token_range)
    }

    fn take_do_statement(
        &mut self,
        do_keyword_token_idx: usize,
    ) -> ((Rc<Statement>, usize), Range<usize>) {
        self.token_iter.next(); // "do" keyword
        let (identifier, identifier_token_idx) = self.take_identifier();
        let (subroutine_call, _) = self.take_subroutine_call(identifier, identifier_token_idx);
        let semicolon = self.take_token(TokenKind::Semicolon);
        let statement = Statement::Do(subroutine_call.0, subroutine_call.1);
        let rc = Rc::new(statement);
        let token_range = do_keyword_token_idx..semicolon.idx + 1;
        let jack_node_idx =
            self.record_jack_node(JackNode::StatementNode(rc.clone()), token_range.clone());
        ((rc, jack_node_idx), token_range)
    }

    fn take_return_statement(
        &mut self,
        return_keyword_token_idx: usize,
    ) -> ((Rc<Statement>, usize), Range<usize>) {
        self.token_iter.next(); // "return" keyword
        let expression_result = self.maybe_take_expression_with_binding_power(0);
        let expression = expression_result.map(|(expr, _)| expr);
        let semicolon = self.take_token(TokenKind::Semicolon);
        let statement = Statement::Return(expression);
        let rc = Rc::new(statement);
        let token_range = return_keyword_token_idx..semicolon.idx + 1;
        let jack_node_idx =
            self.record_jack_node(JackNode::StatementNode(rc.clone()), token_range.clone());
        ((rc, jack_node_idx), token_range)
    }

    fn maybe_take_statement(&mut self) -> Option<((Rc<Statement>, usize), Range<usize>)> {
        use KeywordTokenVariant::*;
        if let Some(Token {
            kind: TokenKind::Keyword(keyword),
            idx,
            ..
        }) = self.token_iter.peek()
        {
            let statement_keyword_idx = *idx;
            match keyword {
                Let => Some(self.take_let_statement(statement_keyword_idx)),
                If => Some(self.take_if_statement(statement_keyword_idx)),
                While => Some(self.take_while_statement(statement_keyword_idx)),
                Do => Some(self.take_do_statement(statement_keyword_idx)),
                Return => Some(self.take_return_statement(statement_keyword_idx)),
                _ => None,
            }
        } else {
            None
        }
    }

    fn take_statements(&mut self) -> Vec<(Rc<Statement>, usize)> {
        let mut result = Vec::new();
        while let Some((statement, _)) = self.maybe_take_statement() {
            result.push(statement);
        }
        result
    }

    fn take_var_declaration(&mut self) -> (Rc<VarDeclaration>, usize) {
        if let Some(Token {
            kind: TokenKind::Keyword(KeywordTokenVariant::Var),
            idx: var_keyword_token_idx,
            ..
        }) = self.token_iter.next()
        {
            let type_name = self.take_type();
            let var_names = self.take_var_names();
            let semicolon = self.take_token(TokenKind::Semicolon);
            let var_declaration = VarDeclaration {
                type_name,
                var_names,
            };
            let rc = Rc::new(var_declaration);
            let token_range = *var_keyword_token_idx..semicolon.idx + 1;
            let jack_node_idx =
                self.record_jack_node(JackNode::VarDeclarationNode(rc.clone()), token_range);
            (rc, jack_node_idx)
        } else {
            panic!("expected var keyword");
        }
    }

    fn take_var_declarations(&mut self) -> Vec<(Rc<VarDeclaration>, usize)> {
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

    fn take_subroutine_body(&mut self) -> ((Rc<SubroutineBody>, usize), Range<usize>) {
        let l_curly = self.take_token(TokenKind::LCurly);
        let var_declarations = self.take_var_declarations();
        let statements = self.take_statements();
        let r_curly = self.take_token(TokenKind::RCurly);
        let subroutine_body = SubroutineBody {
            var_declarations,
            statements,
        };
        let rc = Rc::new(subroutine_body);
        let token_range = l_curly.idx..r_curly.idx + 1;
        let jack_node_idx = self.record_jack_node(
            JackNode::SubroutineBodyNode(rc.clone()),
            token_range.clone(),
        );
        ((rc, jack_node_idx), token_range)
    }

    fn take_subroutine_declaration(
        &mut self,
    ) -> (IndexedJackNode<SubroutineDeclaration>, Range<usize>) {
        use KeywordTokenVariant::*;
        if let Some(Token {
            kind: TokenKind::Keyword(keyword),
            idx: subroutine_kind_token_idx,
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
            let (name, _) = self.take_identifier();
            self.take_token(TokenKind::LParen);
            let parameters = self.take_parameters();
            self.take_token(TokenKind::RParen);
            let (body, body_token_range) = self.take_subroutine_body();
            let subroutine_declaration = SubroutineDeclaration {
                subroutine_kind,
                return_type,
                name,
                parameters,
                body,
            };
            let rc = Rc::new(subroutine_declaration);
            let token_range = *subroutine_kind_token_idx..body_token_range.end;

            let node_idx = self.record_jack_node(
                JackNode::SubroutineDeclarationNode(rc.clone()),
                token_range.clone(),
            );
            (IndexedJackNode { node: rc, node_idx }, token_range)
        } else {
            panic!("expected subroutine kind");
        }
    }

    fn maybe_take_subroutine_declaration(
        &mut self,
    ) -> Option<(IndexedJackNode<SubroutineDeclaration>, Range<usize>)> {
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

    fn take_class_subroutine_declarations(
        &mut self,
    ) -> Vec<IndexedJackNode<SubroutineDeclaration>> {
        let mut result = Vec::new();
        while let Some((subroutine_declaration, _)) = self.maybe_take_subroutine_declaration() {
            result.push(subroutine_declaration);
        }
        result
    }

    fn take_class_var_declaration_qualifier(
        &mut self,
    ) -> (IndexedJackNode<ClassVarDeclarationKind>, Range<usize>) {
        use KeywordTokenVariant::*;
        match self.token_iter.next() {
            Some(Token {
                kind: TokenKind::Keyword(keyword),
                idx: token_idx,
                ..
            }) => {
                let qualifier = match keyword {
                    Static => ClassVarDeclarationKind::Static,
                    Field => ClassVarDeclarationKind::Field,
                    _ => panic!("expected var declaration qualifier",),
                };
                let rc = Rc::new(qualifier);
                let token_range = *token_idx..token_idx + 1;
                let node_idx = self.record_jack_node(
                    JackNode::ClassVarDeclarationKindNode(rc.clone()),
                    token_range.clone(),
                );
                (IndexedJackNode { node: rc, node_idx }, token_range)
            }
            _ => panic!("expected var declaration qualifier",),
        }
    }

    fn take_var_name(&mut self) -> String {
        if let Some(Token {
            kind: TokenKind::Identifier(var_name),
            ..
        }) = self.token_iter.next()
        {
            var_name.to_string()
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
                TokenKind::Identifier(class_name) => Type::ClassName(class_name.to_string()),
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

    fn take_class_var_declaration(
        &mut self,
    ) -> (IndexedJackNode<ClassVarDeclaration>, Range<usize>) {
        let (qualifier, qualifier_token_range) = self.take_class_var_declaration_qualifier();
        let type_name = self.take_type();
        let var_names = self.take_var_names();
        let semicolon = self.take_token(TokenKind::Semicolon);
        let class_var_declaration = ClassVarDeclaration {
            qualifier,
            type_name,
            var_names,
        };
        let rc = Rc::new(class_var_declaration);
        let token_range = qualifier_token_range.start..semicolon.idx + 1;
        let node_idx = self.record_jack_node(
            JackNode::ClassVarDeclarationNode(rc.clone()),
            token_range.clone(),
        );
        (IndexedJackNode { node_idx, node: rc }, token_range)
    }

    fn maybe_take_class_var_declaration(&mut self) -> Option<IndexedJackNode<ClassVarDeclaration>> {
        use KeywordTokenVariant::*;
        match self.token_iter.peek().expect("unexpected end of input") {
            Token {
                kind: TokenKind::Keyword(Static | Field),
                ..
            } => {
                let (class_var_declaration, _) = self.take_class_var_declaration();
                Some(class_var_declaration)
            }
            _ => None,
        }
    }

    fn take_class_var_declarations(&mut self) -> Vec<IndexedJackNode<ClassVarDeclaration>> {
        let mut result = Vec::new();
        while let Some(class_var_declaration) = self.maybe_take_class_var_declaration() {
            result.push(class_var_declaration);
        }
        result
    }

    fn take_class(&mut self) -> Rc<Class> {
        let class_keyword = self.take_class_keyword();
        let (name, _) = self.take_identifier();
        self.take_token(TokenKind::LCurly);
        let var_declarations = self.take_class_var_declarations();
        let subroutine_declarations = self.take_class_subroutine_declarations();
        let r_curly = self.take_token(TokenKind::RCurly);
        let class = Class {
            name,
            var_declarations,
            subroutine_declarations,
        };
        let rc = Rc::new(class);
        let token_range = class_keyword.idx..r_curly.idx + 1;
        self.record_jack_node(JackNode::ClassNode(rc.clone()), token_range);
        rc
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_expression(source: &str) -> IndexedJackNode<Expression> {
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
            token_iter: tokens.iter().peekable(),
            sourcemap: JackParserSourceMap::new(),
            jack_nodes: Vec::new(),
        };
        parser.take_expression().0
    }

    #[test]
    fn test_simple_class() {
        assert_eq!(
            *parse("class foo {}").class,
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
            *parse(
                "
            class foo {
              static int bar;
            }"
            )
            .class,
            Class {
                name: "foo".to_string(),
                var_declarations: vec![IndexedJackNode {
                    node: Rc::new(ClassVarDeclaration {
                        qualifier: IndexedJackNode {
                            node: Rc::new(ClassVarDeclarationKind::Static),
                            node_idx: 0
                        },
                        type_name: Type::Int,
                        var_names: vec!["bar".to_string()],
                    }),
                    node_idx: 1
                }],
                subroutine_declarations: vec![],
            }
        );
    }

    #[test]
    fn test_class_with_multiple_var_declarations() {
        assert_eq!(
            *parse(
                "
            class foo {
              static int bar;
              field char baz, buz, boz;
              field boolean a, b, c;
            }"
            )
            .class,
            Class {
                name: "foo".to_string(),
                var_declarations: vec![
                    IndexedJackNode {
                        node: Rc::new(ClassVarDeclaration {
                            qualifier: IndexedJackNode {
                                node: Rc::new(ClassVarDeclarationKind::Static),
                                node_idx: 0
                            },
                            type_name: Type::Int,
                            var_names: vec!["bar".to_string()],
                        }),
                        node_idx: 1
                    },
                    IndexedJackNode {
                        node: Rc::new(ClassVarDeclaration {
                            qualifier: IndexedJackNode {
                                node: Rc::new(ClassVarDeclarationKind::Field),
                                node_idx: 2
                            },
                            type_name: Type::Char,
                            var_names: vec![
                                "baz".to_string(),
                                "buz".to_string(),
                                "boz".to_string()
                            ],
                        }),
                        node_idx: 3
                    },
                    IndexedJackNode {
                        node: Rc::new(ClassVarDeclaration {
                            qualifier: IndexedJackNode {
                                node: Rc::new(ClassVarDeclarationKind::Field),
                                node_idx: 4
                            },
                            type_name: Type::Boolean,
                            var_names: vec!["a".to_string(), "b".to_string(), "c".to_string(),],
                        }),
                        node_idx: 5
                    }
                ],
                subroutine_declarations: vec![],
            }
        );
    }

    #[test]
    fn test_class_with_subroutine_declarations() {
        assert_eq!(
            *parse(
                "
            class foo {
                constructor boolean bar(int abc, char def, foo ghi) {
                }
                function char baz(boolean _123) {
                }
                method void qux() {
                }
            }"
            )
            .class,
            Class {
                name: "foo".to_string(),
                var_declarations: vec![],
                subroutine_declarations: vec![
                    IndexedJackNode {
                        node: Rc::new(SubroutineDeclaration {
                            subroutine_kind: SubroutineKind::Constructor,
                            return_type: Some(Type::Boolean),
                            parameters: vec![
                                (
                                    Rc::new(Parameter {
                                        type_name: Type::Int,
                                        var_name: "abc".to_string(),
                                    }),
                                    0
                                ),
                                (
                                    Rc::new(Parameter {
                                        type_name: Type::Char,
                                        var_name: "def".to_string(),
                                    }),
                                    1
                                ),
                                (
                                    Rc::new(Parameter {
                                        type_name: Type::ClassName("foo".to_string()),
                                        var_name: "ghi".to_string(),
                                    }),
                                    2
                                )
                            ],
                            name: "bar".to_string(),
                            body: (
                                Rc::new(SubroutineBody {
                                    var_declarations: vec![],
                                    statements: vec![],
                                }),
                                3
                            ),
                        }),
                        node_idx: 4
                    },
                    IndexedJackNode {
                        node: Rc::new(SubroutineDeclaration {
                            subroutine_kind: SubroutineKind::Function,
                            return_type: Some(Type::Char),
                            parameters: vec![(
                                Rc::new(Parameter {
                                    type_name: Type::Boolean,
                                    var_name: "_123".to_string(),
                                }),
                                5
                            )],
                            name: "baz".to_string(),
                            body: (
                                Rc::new(SubroutineBody {
                                    var_declarations: vec![],
                                    statements: vec![],
                                }),
                                6
                            ),
                        }),
                        node_idx: 7
                    },
                    IndexedJackNode {
                        node: Rc::new(SubroutineDeclaration {
                            subroutine_kind: SubroutineKind::Method,
                            return_type: None,
                            parameters: vec![],
                            name: "qux".to_string(),
                            body: (
                                Rc::new(SubroutineBody {
                                    var_declarations: vec![],
                                    statements: vec![],
                                }),
                                8
                            ),
                        }),
                        node_idx: 9
                    },
                ],
            }
        );
    }

    #[test]
    fn test_all_statement_types() {
        let if_statements = vec![(
            Rc::new(Statement::While {
                condition: (IndexedJackNode {
                    node: Rc::new(Expression::PrimitiveTerm(IntegerConstant("1".to_string()))),
                    node_idx: 7,
                }),
                statements: vec![
                    (
                        Rc::new(Statement::Do(
                            Rc::new(SubroutineCall::Direct {
                                subroutine_name: "foobar".to_string(),
                                arguments: vec![],
                            }),
                            8,
                        )),
                        9,
                    ),
                    (
                        Rc::new(Statement::Do(
                            Rc::new(SubroutineCall::Direct {
                                subroutine_name: "foobar".to_string(),
                                arguments: vec![
                                    (IndexedJackNode {
                                        node: Rc::new(Expression::PrimitiveTerm(IntegerConstant(
                                            "1".to_string(),
                                        ))),
                                        node_idx: 10,
                                    }),
                                ],
                            }),
                            11,
                        )),
                        12,
                    ),
                    (
                        Rc::new(Statement::Do(
                            Rc::new(SubroutineCall::Direct {
                                subroutine_name: "foobar".to_string(),
                                arguments: vec![
                                    IndexedJackNode {
                                        node: Rc::new(Expression::PrimitiveTerm(IntegerConstant(
                                            "1".to_string(),
                                        ))),
                                        node_idx: 13,
                                    },
                                    IndexedJackNode {
                                        node: Rc::new(Expression::PrimitiveTerm(IntegerConstant(
                                            "2".to_string(),
                                        ))),
                                        node_idx: 14,
                                    },
                                    (IndexedJackNode {
                                        node: Rc::new(Expression::PrimitiveTerm(IntegerConstant(
                                            "3".to_string(),
                                        ))),
                                        node_idx: 15,
                                    }),
                                ],
                            }),
                            16,
                        )),
                        17,
                    ),
                    (
                        Rc::new(Statement::Do(
                            Rc::new(SubroutineCall::Method {
                                this_name: "foo".to_string(),
                                method_name: "bar".to_string(),
                                arguments: vec![],
                            }),
                            18,
                        )),
                        19,
                    ),
                    (
                        Rc::new(Statement::Do(
                            Rc::new(SubroutineCall::Method {
                                this_name: "foo".to_string(),
                                method_name: "bar".to_string(),
                                arguments: vec![IndexedJackNode {
                                    node: Rc::new(Expression::PrimitiveTerm(IntegerConstant(
                                        "1".to_string(),
                                    ))),
                                    node_idx: 20,
                                }],
                            }),
                            21,
                        )),
                        22,
                    ),
                    (
                        Rc::new(Statement::Do(
                            Rc::new(SubroutineCall::Method {
                                this_name: "foo".to_string(),
                                method_name: "bar".to_string(),
                                arguments: vec![
                                    IndexedJackNode {
                                        node: Rc::new(Expression::PrimitiveTerm(IntegerConstant(
                                            "1".to_string(),
                                        ))),
                                        node_idx: 23,
                                    },
                                    IndexedJackNode {
                                        node: Rc::new(Expression::PrimitiveTerm(IntegerConstant(
                                            "2".to_string(),
                                        ))),
                                        node_idx: 24,
                                    },
                                    IndexedJackNode {
                                        node: Rc::new(Expression::PrimitiveTerm(IntegerConstant(
                                            "3".to_string(),
                                        ))),
                                        node_idx: 25,
                                    },
                                ],
                            }),
                            26,
                        )),
                        27,
                    ),
                ],
            }),
            28,
        )];
        assert_eq!(
            *parse(
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
            )
            .class,
            Class {
                name: "foo".to_string(),
                var_declarations: vec![],
                subroutine_declarations: vec![IndexedJackNode {
                    node: Rc::new(SubroutineDeclaration {
                        subroutine_kind: SubroutineKind::Constructor,
                        return_type: Some(Type::Int),
                        parameters: vec![],
                        name: "blah".to_string(),
                        body: (
                            Rc::new(SubroutineBody {
                                var_declarations: vec![(
                                    Rc::new(VarDeclaration {
                                        type_name: Type::Int,
                                        var_names: vec!["a".to_string()],
                                    }),
                                    0,
                                )],
                                statements: vec![
                                    (
                                        Rc::new(Statement::Let {
                                            var_name: "a".to_string(),
                                            array_index: None,
                                            value: IndexedJackNode {
                                                node: Rc::new(Expression::PrimitiveTerm(
                                                    IntegerConstant("1234".to_string(),)
                                                )),
                                                node_idx: 1
                                            },
                                        }),
                                        2,
                                    ),
                                    (
                                        Rc::new(Statement::Let {
                                            var_name: "b".to_string(),
                                            array_index: Some(IndexedJackNode {
                                                node: Rc::new(Expression::PrimitiveTerm(
                                                    IntegerConstant("22".to_string())
                                                )),
                                                node_idx: 3
                                            }),
                                            value: IndexedJackNode {
                                                node: Rc::new(Expression::PrimitiveTerm(
                                                    IntegerConstant("123".to_string(),)
                                                )),
                                                node_idx: 4,
                                            }
                                        }),
                                        5,
                                    ),
                                    (
                                        Rc::new(Statement::If {
                                            condition: IndexedJackNode {
                                                node: Rc::new(Expression::PrimitiveTerm(
                                                    IntegerConstant("1".to_string())
                                                )),
                                                node_idx: 6
                                            },
                                            if_statements,
                                            else_statements: Some(vec![(
                                                Rc::new(Statement::Return(Some(IndexedJackNode {
                                                    node: Rc::new(Expression::PrimitiveTerm(
                                                        IntegerConstant("123".to_string(),)
                                                    )),
                                                    node_idx: 29
                                                },))),
                                                30,
                                            )]),
                                        }),
                                        31,
                                    ),
                                ],
                            }),
                            32
                        ),
                    }),
                    node_idx: 33
                }],
            }
        );
    }

    #[test]
    fn test_simple_expression() {
        assert_eq!(
            parse_expression("1"),
            IndexedJackNode {
                node: Rc::new(Expression::PrimitiveTerm(IntegerConstant("1".to_string()))),
                node_idx: 0
            }
        )
    }

    #[test]
    fn test_simple_binary_expression() {
        assert_eq!(
            parse_expression("1 + 2"),
            IndexedJackNode {
                node: Rc::new(Expression::Binary {
                    operator: BinaryOperator::Plus,
                    lhs: IndexedJackNode {
                        node: Rc::new(Expression::PrimitiveTerm(IntegerConstant("1".to_string()))),
                        node_idx: 0
                    },
                    rhs: IndexedJackNode {
                        node: Rc::new(Expression::PrimitiveTerm(IntegerConstant("2".to_string()))),
                        node_idx: 1
                    }
                }),
                node_idx: 2
            }
        )
    }

    #[test]
    fn test_simple_binary_expression_within_class() {
        assert_eq!(
            *parse(
                "
            class foo {
                method void bar () {
                    let a = 1 + 2 + 3;
                }
            }
            "
            )
            .class,
            Class {
                name: "foo".to_string(),
                var_declarations: vec![],
                subroutine_declarations: vec![IndexedJackNode {
                    node: Rc::new(SubroutineDeclaration {
                        subroutine_kind: SubroutineKind::Method,
                        return_type: None,
                        parameters: vec![],
                        name: "bar".to_string(),
                        body: (
                            Rc::new(SubroutineBody {
                                var_declarations: vec![],
                                statements: vec![(
                                    Rc::new(Statement::Let {
                                        var_name: "a".to_string(),
                                        array_index: None,
                                        value: IndexedJackNode {
                                            node: Rc::new(Expression::Binary {
                                                operator: BinaryOperator::Plus,
                                                lhs: IndexedJackNode {
                                                    node: Rc::new(Expression::Binary {
                                                        operator: BinaryOperator::Plus,
                                                        lhs: IndexedJackNode {
                                                            node: Rc::new(
                                                                Expression::PrimitiveTerm(
                                                                    IntegerConstant(
                                                                        "1".to_string()
                                                                    )
                                                                )
                                                            ),
                                                            node_idx: 0
                                                        },
                                                        rhs: IndexedJackNode {
                                                            node: Rc::new(
                                                                Expression::PrimitiveTerm(
                                                                    IntegerConstant(
                                                                        "2".to_string()
                                                                    )
                                                                )
                                                            ),
                                                            node_idx: 1
                                                        },
                                                    }),
                                                    node_idx: 2
                                                },
                                                rhs: IndexedJackNode {
                                                    node: Rc::new(Expression::PrimitiveTerm(
                                                        IntegerConstant("3".to_string())
                                                    )),
                                                    node_idx: 3
                                                },
                                            }),
                                            node_idx: 4
                                        }
                                    }),
                                    5
                                )],
                            }),
                            6
                        ),
                    }),
                    node_idx: 7
                }],
            }
        )
    }

    #[test]
    fn test_simple_left_associating_expression() {
        assert_eq!(
            parse_expression("1 + 2 + 3 + 4"),
            IndexedJackNode {
                node: Rc::new(Expression::Binary {
                    operator: BinaryOperator::Plus,
                    lhs: IndexedJackNode {
                        node: Rc::new(Expression::Binary {
                            operator: BinaryOperator::Plus,
                            lhs: IndexedJackNode {
                                node: Rc::new(Expression::Binary {
                                    operator: BinaryOperator::Plus,
                                    lhs: IndexedJackNode {
                                        node: Rc::new(Expression::PrimitiveTerm(IntegerConstant(
                                            "1".to_string()
                                        ))),
                                        node_idx: 0
                                    },
                                    rhs: IndexedJackNode {
                                        node: Rc::new(Expression::PrimitiveTerm(IntegerConstant(
                                            "2".to_string()
                                        ))),
                                        node_idx: 1
                                    },
                                }),
                                node_idx: 2
                            },
                            rhs: IndexedJackNode {
                                node: Rc::new(Expression::PrimitiveTerm(IntegerConstant(
                                    "3".to_string()
                                ))),
                                node_idx: 3
                            },
                        }),
                        node_idx: 4
                    },
                    rhs: IndexedJackNode {
                        node: Rc::new(Expression::PrimitiveTerm(IntegerConstant("4".to_string()))),
                        node_idx: 5
                    },
                }),
                node_idx: 6
            }
        )
    }

    #[test]
    fn test_binary_precedence() {
        assert_eq!(
            parse_expression("1 + 2 * 3 + 4"),
            IndexedJackNode {
                node: Rc::new(Expression::Binary {
                    operator: BinaryOperator::Plus,
                    lhs: IndexedJackNode {
                        node: Rc::new(Expression::Binary {
                            operator: BinaryOperator::Plus,
                            lhs: IndexedJackNode {
                                node: Rc::new(Expression::PrimitiveTerm(IntegerConstant(
                                    "1".to_string()
                                ))),
                                node_idx: 0
                            },
                            rhs: IndexedJackNode {
                                node: Rc::new(Expression::Binary {
                                    operator: BinaryOperator::Multiply,
                                    lhs: IndexedJackNode {
                                        node: Rc::new(Expression::PrimitiveTerm(IntegerConstant(
                                            "2".to_string()
                                        ))),
                                        node_idx: 1
                                    },
                                    rhs: IndexedJackNode {
                                        node: Rc::new(Expression::PrimitiveTerm(IntegerConstant(
                                            "3".to_string()
                                        ))),
                                        node_idx: 2
                                    },
                                }),
                                node_idx: 3
                            }
                        }),
                        node_idx: 4
                    },
                    rhs: IndexedJackNode {
                        node: Rc::new(Expression::PrimitiveTerm(IntegerConstant("4".to_string()))),
                        node_idx: 5
                    },
                }),
                node_idx: 6
            }
        )
    }

    #[test]
    fn test_simple_unary_expression() {
        assert_eq!(
            parse_expression("~1"),
            IndexedJackNode {
                node: Rc::new(Expression::Unary {
                    operator: UnaryOperator::Not,
                    operand: IndexedJackNode {
                        node: Rc::new(Expression::PrimitiveTerm(IntegerConstant("1".to_string()))),
                        node_idx: 0
                    }
                }),
                node_idx: 1
            },
        )
    }

    #[test]
    fn test_simple_combined_unary_and_binary_expression() {
        assert_eq!(
            parse_expression("~1 + ~2"),
            IndexedJackNode {
                node: Rc::new(Expression::Binary {
                    operator: BinaryOperator::Plus,
                    lhs: IndexedJackNode {
                        node: Rc::new(Expression::Unary {
                            operator: UnaryOperator::Not,
                            operand: IndexedJackNode {
                                node: Rc::new(Expression::PrimitiveTerm(IntegerConstant(
                                    "1".to_string()
                                ))),
                                node_idx: 0
                            },
                        }),
                        node_idx: 1
                    },
                    rhs: IndexedJackNode {
                        node: Rc::new(Expression::Unary {
                            operator: UnaryOperator::Not,
                            operand: IndexedJackNode {
                                node: Rc::new(Expression::PrimitiveTerm(IntegerConstant(
                                    "2".to_string()
                                ))),
                                node_idx: 2
                            }
                        }),
                        node_idx: 3
                    },
                }),
                node_idx: 4
            }
        )
    }

    #[test]
    fn test_expression_with_subroutine_calls() {
        assert_eq!(
            parse_expression("1 + foo(1, baz.bar(1, 2), 3) + 2"),
            IndexedJackNode {
                node: Rc::new(Expression::Binary {
                    operator: BinaryOperator::Plus,
                    lhs: IndexedJackNode {
                        node: Rc::new(Expression::Binary {
                            operator: BinaryOperator::Plus,
                            lhs: IndexedJackNode {
                                node: Rc::new(Expression::PrimitiveTerm(IntegerConstant(
                                    "1".to_string()
                                ))),
                                node_idx: 0
                            },
                            rhs: IndexedJackNode {
                                node: Rc::new(Expression::SubroutineCall((
                                    Rc::new(SubroutineCall::Direct {
                                        subroutine_name: "foo".to_string(),
                                        arguments: vec![
                                            IndexedJackNode {
                                                node: Rc::new(Expression::PrimitiveTerm(
                                                    IntegerConstant("1".to_string())
                                                )),
                                                node_idx: 1
                                            },
                                            IndexedJackNode {
                                                node: Rc::new(Expression::SubroutineCall((
                                                    Rc::new(SubroutineCall::Method {
                                                        this_name: "baz".to_string(),
                                                        method_name: "bar".to_string(),
                                                        arguments: vec![
                                                            IndexedJackNode {
                                                                node: Rc::new(
                                                                    Expression::PrimitiveTerm(
                                                                        IntegerConstant(
                                                                            "1".to_string()
                                                                        )
                                                                    )
                                                                ),
                                                                node_idx: 2
                                                            },
                                                            IndexedJackNode {
                                                                node: Rc::new(
                                                                    Expression::PrimitiveTerm(
                                                                        IntegerConstant(
                                                                            "2".to_string()
                                                                        )
                                                                    )
                                                                ),
                                                                node_idx: 3
                                                            },
                                                        ]
                                                    }),
                                                    4
                                                ))),
                                                node_idx: 5
                                            },
                                            IndexedJackNode {
                                                node: Rc::new(Expression::PrimitiveTerm(
                                                    IntegerConstant("3".to_string())
                                                )),
                                                node_idx: 6
                                            },
                                        ]
                                    }),
                                    7
                                ))),
                                node_idx: 8
                            },
                        }),
                        node_idx: 9
                    },
                    rhs: IndexedJackNode {
                        node: Rc::new(Expression::PrimitiveTerm(IntegerConstant("2".to_string()))),
                        node_idx: 10
                    },
                }),
                node_idx: 11
            }
        )
    }

    #[test]
    fn test_expression_with_subroutine_call_and_array_access() {
        assert_eq!(
            parse_expression("1 + foo(1, bar[1 + 2], 3) + 2"),
            IndexedJackNode {
                node: Rc::new(Expression::Binary {
                    operator: BinaryOperator::Plus,
                    lhs: IndexedJackNode {
                        node: Rc::new(Expression::Binary {
                            operator: BinaryOperator::Plus,
                            lhs: IndexedJackNode {
                                node: Rc::new(Expression::PrimitiveTerm(IntegerConstant(
                                    "1".to_string()
                                ))),
                                node_idx: 0
                            },
                            rhs: IndexedJackNode {
                                node: Rc::new(Expression::SubroutineCall((
                                    Rc::new(SubroutineCall::Direct {
                                        subroutine_name: "foo".to_string(),
                                        arguments: vec![
                                            IndexedJackNode {
                                                node: Rc::new(Expression::PrimitiveTerm(
                                                    IntegerConstant("1".to_string())
                                                )),
                                                node_idx: 1
                                            },
                                            IndexedJackNode {
                                                node: Rc::new(Expression::ArrayAccess {
                                                    var_name: "bar".to_string(),
                                                    index: IndexedJackNode {
                                                        node: Rc::new(Expression::Binary {
                                                            operator: BinaryOperator::Plus,
                                                            lhs: IndexedJackNode {
                                                                node: Rc::new(
                                                                    Expression::PrimitiveTerm(
                                                                        IntegerConstant(
                                                                            "1".to_string()
                                                                        )
                                                                    )
                                                                ),
                                                                node_idx: 2
                                                            },
                                                            rhs: IndexedJackNode {
                                                                node: Rc::new(
                                                                    Expression::PrimitiveTerm(
                                                                        IntegerConstant(
                                                                            "2".to_string()
                                                                        )
                                                                    )
                                                                ),
                                                                node_idx: 3
                                                            },
                                                        }),
                                                        node_idx: 4
                                                    },
                                                }),
                                                node_idx: 5
                                            },
                                            IndexedJackNode {
                                                node: Rc::new(Expression::PrimitiveTerm(
                                                    IntegerConstant("3".to_string())
                                                )),
                                                node_idx: 6
                                            },
                                        ]
                                    }),
                                    7
                                ))),
                                node_idx: 8
                            },
                        }),
                        node_idx: 9
                    },
                    rhs: IndexedJackNode {
                        node: Rc::new(Expression::PrimitiveTerm(IntegerConstant("2".to_string()))),
                        node_idx: 10
                    },
                }),
                node_idx: 11
            }
        )
    }

    #[test]
    fn test_expression_with_variables_subroutine_calls_and_array_access() {
        assert_eq!(
            parse_expression("foo + bar[baz + buz.boz(qux, wox[123]) / bing]"),
            (IndexedJackNode {
                node: Rc::new(Expression::Binary {
                    operator: BinaryOperator::Plus,
                    lhs: IndexedJackNode {
                        node: Rc::new(Expression::Variable("foo".to_string())),
                        node_idx: 0
                    },
                    rhs: IndexedJackNode {
                        node: Rc::new(Expression::ArrayAccess {
                            var_name: "bar".to_string(),
                            index: IndexedJackNode {
                                node: Rc::new(Expression::Binary {
                                    operator: BinaryOperator::Plus,
                                    lhs: IndexedJackNode {
                                        node: Rc::new(Expression::Variable("baz".to_string())),
                                        node_idx: 1
                                    },
                                    rhs: IndexedJackNode {
                                        node: Rc::new(Expression::Binary {
                                            operator: BinaryOperator::Divide,
                                            lhs: IndexedJackNode {
                                                node: Rc::new(Expression::SubroutineCall((
                                                    Rc::new(SubroutineCall::Method {
                                                        this_name: "buz".to_string(),
                                                        method_name: "boz".to_string(),
                                                        arguments: vec![
                                                                IndexedJackNode{node: Rc::new(Expression::Variable(
                                                                    "qux".to_string()
                                                                )),
                                                                node_idx: 2},
                                              IndexedJackNode{node:    Rc::new(Expression::ArrayAccess {
                                                                    var_name: "wox".to_string(),
                                                                    index: (Rc::new(
                                                                        Expression::PrimitiveTerm(
                                                                            IntegerConstant(
                                                                                "123".to_string()
                                                                            )
                                                                        )
                                                                    ), 3)
                                                                }),
                                                                node_idx: 4},
                                                        ]
                                                    }),
                                                    5
                                                ))),
                                                node_idx: 6
                                            },
                                            rhs: (
                                                Rc::new(Expression::Variable("bing".to_string())),
                                                7
                                            )
                                        }),
                                        node_idx: 8
                                    },
                                }),
                                node_idx: 9
                            }
                        }),
                        node_idx: 10
                    },
                }),
                node_idx: 11
            })
        )
    }

    #[test]
    fn test_primitive_terms() {
        assert_eq!(
            parse_expression("1 + \"hello\" + true + false + null + this"),
            (
                Rc::new(Expression::Binary {
                    operator: BinaryOperator::Plus,
                    lhs: (
                        Rc::new(Expression::Binary {
                            operator: BinaryOperator::Plus,
                            lhs: (
                                Rc::new(Expression::Binary {
                                    operator: BinaryOperator::Plus,
                                    lhs: (
                                        Rc::new(Expression::Binary {
                                            operator: BinaryOperator::Plus,
                                            lhs: (
                                                Rc::new(Expression::Binary {
                                                    operator: BinaryOperator::Plus,
                                                    lhs: (
                                                        Rc::new(Expression::PrimitiveTerm(
                                                            IntegerConstant("1".to_string())
                                                        )),
                                                        0
                                                    ),
                                                    rhs: (
                                                        Rc::new(Expression::PrimitiveTerm(
                                                            StringConstant("hello".to_string())
                                                        )),
                                                        1
                                                    ),
                                                }),
                                                2
                                            ),
                                            rhs: (
                                                Rc::new(Expression::PrimitiveTerm(
                                                    PrimitiveTermVariant::True
                                                )),
                                                3
                                            ),
                                        }),
                                        4
                                    ),
                                    rhs: (
                                        Rc::new(Expression::PrimitiveTerm(
                                            PrimitiveTermVariant::False
                                        )),
                                        5
                                    ),
                                }),
                                6
                            ),
                            rhs: (
                                Rc::new(Expression::PrimitiveTerm(PrimitiveTermVariant::Null)),
                                7
                            ),
                        }),
                        8
                    ),
                    rhs: (
                        Rc::new(Expression::PrimitiveTerm(PrimitiveTermVariant::This)),
                        9
                    )
                }),
                10
            )
        )
    }

    #[test]
    fn test_parenthesized_expression() {
        assert_eq!(
            parse_expression("(1 + 2) * 3"),
            (
                Rc::new(Expression::Binary {
                    operator: BinaryOperator::Multiply,
                    lhs: (
                        Rc::new(Expression::Parenthesized((
                            Rc::new(Expression::Binary {
                                operator: BinaryOperator::Plus,
                                lhs: (
                                    Rc::new(Expression::PrimitiveTerm(IntegerConstant(
                                        "1".to_string()
                                    ))),
                                    0
                                ),
                                rhs: (
                                    Rc::new(Expression::PrimitiveTerm(IntegerConstant(
                                        "2".to_string()
                                    ))),
                                    1
                                ),
                            }),
                            2
                        ))),
                        3
                    ),
                    rhs: (
                        Rc::new(Expression::PrimitiveTerm(IntegerConstant("3".to_string()))),
                        4
                    ),
                }),
                5
            )
        )
    }
}
