use std::{iter, ops::Range};

use super::{
    jack_node_types::{PrimitiveTermVariant::*, *},
    sourcemap::JackParserSourceMap,
    tokenizer::{
        KeywordTokenVariant,
        OperatorVariant::{self, *},
        TokenKind,
    },
};
use crate::utils::{parser_utils::PeekableTokens, tokenizer::Token};

// A version of this should doon be present in the std lib https://github.com/rust-lang/rust/issues/87800
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

pub struct JackParserResult {
    pub class: Class,
    pub sourcemap: JackParserSourceMap,
}

pub fn parse(tokens: &[Token<TokenKind>]) -> JackParserResult {
    let tokens_without_whitespace: Vec<_> = tokens
        .iter()
        .filter(|token| {
            !matches!(
                token,
                Token {
                    kind: TokenKind::Whitespace | TokenKind::MultiLineComment | TokenKind::SingleLineComment,
                    ..
                }
            )
        })
        .collect();

    let cloned_tokens_without_whitespace: Vec<_> = tokens_without_whitespace.into_iter().cloned().collect();

    let mut sourcemap = JackParserSourceMap::new();

    let mut parser = Parser {
        token_iter: cloned_tokens_without_whitespace.iter().peekable(),
        sourcemap: &mut sourcemap,
    };

    let class = parser.take_class();

    JackParserResult { class, sourcemap }
}

struct Parser<'a> {
    token_iter: PeekableTokens<'a, TokenKind>,
    sourcemap: &'a mut JackParserSourceMap,
}

impl<'a> Parser<'a> {
    fn make_ast_node<T>(&mut self, node: T, token_range: Range<usize>, child_node_idxs: Vec<usize>) -> ASTNode<T> {
        let node_idx = self.sourcemap.record_jack_node(token_range.clone(), child_node_idxs);
        ASTNode {
            node: Box::new(node),
            node_idx,
            token_range,
        }
    }

    fn maybe_take_primitive_expression(&mut self) -> Option<ASTNode<Expression>> {
        use TokenKind::*;
        let peeked_token = self.token_iter.peek().cloned();
        let (expression, exp_token_idx) = peeked_token.and_then(|token| {
            let maybe_exp = match &token.kind {
                IntegerLiteral(string) => {
                    self.token_iter.next();
                    Some(Expression::PrimitiveTerm(IntegerConstant(string.to_string())))
                }
                StringLiteral(string) => {
                    self.token_iter.next();
                    Some(Expression::PrimitiveTerm(StringConstant(string.to_string())))
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
        let token_range = exp_token_idx..exp_token_idx + 1;
        Some(self.make_ast_node(expression, token_range, vec![]))
    }

    fn take_array_access(&mut self, var_name: String) -> ASTNode<Expression> {
        use TokenKind::*;
        let l_bracket = self.take_token(LSquareBracket);
        let index_expr = self.take_expression();
        let r_bracket = self.take_token(RSquareBracket);
        let token_range = l_bracket.idx..r_bracket.idx + 1;
        let child_node_idxs = vec![index_expr.node_idx];
        self.make_ast_node(Expression::ArrayAccess { var_name, index: index_expr }, token_range, child_node_idxs)
    }

    fn maybe_take_parenthesized_expression(&mut self) -> Option<ASTNode<Expression>> {
        use TokenKind::*;
        if let Some(Token {
            kind: LParen,
            idx: l_paren_idx,
            ..
        }) = self.token_iter.peek()
        {
            let token_range_start = *l_paren_idx;
            self.token_iter.next();
            let expr = self.take_expression();
            // let jack_node = JackNode::ExpressionNode(rc.clone());
            let r_paren = self.take_token(RParen);
            let token_range = token_range_start..r_paren.idx + 1;
            let child_node_idxs = vec![expr.node_idx];
            Some(self.make_ast_node(Expression::Parenthesized(expr), token_range, child_node_idxs))
        } else {
            None
        }
    }

    fn maybe_take_expression_starting_with_identifier(&mut self) -> Option<ASTNode<Expression>> {
        use TokenKind::*;
        // TODO - this is not very nice...maybe we should just use two tokens of
        // lookahead instead? (I think itertools would make that easy).
        let peeked_token = self.token_iter.peek();
        if let Some(Token {
            kind: Identifier(string), ..
        }) = peeked_token
        {
            let string = string.to_string();
            let (identifier, identifier_token_idx) = self.take_identifier();
            match self.token_iter.peek() {
                Some(Token { kind: LSquareBracket, .. }) => Some(self.take_array_access(identifier)),
                Some(Token { kind: Dot | LParen, .. }) => {
                    let subroutine_call = self.take_subroutine_call(identifier, identifier_token_idx);
                    let subroutine_call_token_range = subroutine_call.token_range.clone();
                    let child_node_idxs = vec![subroutine_call.node_idx];
                    Some(self.make_ast_node(Expression::SubroutineCall(subroutine_call), subroutine_call_token_range, child_node_idxs))
                }
                _ => {
                    let token_range = identifier_token_idx..identifier_token_idx + 1;
                    Some(self.make_ast_node(Expression::Variable(string), token_range, vec![]))
                }
            }
        } else {
            None
        }
    }

    fn maybe_take_unary_expression(&mut self) -> Option<ASTNode<Expression>> {
        use TokenKind::*;
        if let Some(Token { kind: Operator(op), idx, .. }) = self.token_iter.peek() {
            let op_token_idx = *idx;
            let op = op.clone();
            self.token_iter.next();
            let right_binding_power = prefix_precedence(op.clone()).expect("invalid prefix operator");
            let operand = self
                .maybe_take_expression_with_binding_power(right_binding_power)
                .expect("unary operator has no operand");
            let operator = match op {
                OperatorVariant::Minus => UnaryOperator::Minus,
                OperatorVariant::Tilde => UnaryOperator::Not,
                _ => panic!("invalid unary operator"),
            };
            let token_range = op_token_idx..operand.token_range.end;
            let child_node_idxs = vec![operand.node_idx];
            Some(self.make_ast_node(Expression::Unary { operator, operand }, token_range, child_node_idxs))
        } else {
            None
        }
    }

    fn maybe_append_rhs_to_lhs(&mut self, lhs_node: ASTNode<Expression>, binding_power: u8) -> (ASTNode<Expression>, bool) {
        use TokenKind::*;
        if let Some(Token { kind: Operator(op), .. }) = self.token_iter.peek() {
            let (lbp, rbp) = infix_precedence(op.clone()).expect("invalid infix operator");
            if lbp < binding_power {
                // There is no rhs to append - the next term will instead associate towards the right.
                return (lhs_node, true);
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

            let new_token_range = lhs_node.token_range.start..rhs.token_range.end;
            let lhs = ASTNode {
                node: lhs_node.node,
                node_idx: lhs_node.node_idx,
                token_range: new_token_range.clone(),
            };
            let child_node_idxs = vec![lhs.node_idx, rhs.node_idx];
            let new_lhs_node = Expression::Binary { operator, lhs, rhs };
            (self.make_ast_node(new_lhs_node, new_token_range, child_node_idxs), false)
        } else {
            (lhs_node, true)
        }
    }

    fn maybe_take_expression_with_binding_power(&mut self, binding_power: u8) -> Option<ASTNode<Expression>> {
        let mut lhs = self
            .maybe_take_unary_expression()
            .or_else(|| self.maybe_take_primitive_expression())
            .or_else(|| self.maybe_take_expression_starting_with_identifier())
            .or_else(|| self.maybe_take_parenthesized_expression())?;

        loop {
            let (new_lhs, done) = self.maybe_append_rhs_to_lhs(lhs, binding_power);
            lhs = new_lhs;
            if done {
                break;
            }
        }

        Some(lhs)
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

    fn take_expression(&mut self) -> ASTNode<Expression> {
        self.maybe_take_expression_with_binding_power(0).expect("expected expression")
    }

    fn take_expression_list(&mut self) -> Vec<ASTNode<Expression>> {
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

    fn take_subroutine_call(&mut self, name: String, identifier_token_idx: usize) -> ASTNode<SubroutineCall> {
        use TokenKind::*;
        match self.token_iter.peek() {
            Some(Token { kind: LParen, .. }) => {
                // Direct function call
                self.token_iter.next(); // LParen
                let arguments = self.take_expression_list();
                let r_paren = self.take_token(RParen);
                let child_node_idxs = arguments.iter().map(|arg| arg.node_idx).collect();
                let subroutine_call = SubroutineCall::Direct {
                    subroutine_name: name,
                    arguments,
                };
                let token_range = identifier_token_idx..r_paren.idx + 1;
                self.make_ast_node(subroutine_call, token_range, child_node_idxs)
            }
            Some(Token { kind: Dot, .. }) => {
                // Method call
                self.token_iter.next(); // Dot
                let (method_name, method_name_token_idx) = self.take_identifier();
                self.take_token(LParen);
                let arguments = self.take_expression_list();
                let r_paren = self.take_token(RParen);
                let child_node_idxs = arguments.iter().map(|arg| arg.node_idx).collect();
                let method = SubroutineCall::Method {
                    this_name: name,
                    method_name,
                    arguments,
                };
                let token_range = method_name_token_idx..r_paren.idx + 1;
                self.make_ast_node(method, token_range, child_node_idxs)
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

    fn maybe_take_parameter(&mut self) -> Option<ASTNode<Parameter>> {
        self.maybe_take_type().map(|type_name| {
            let (var_name, identifier_token_idx) = self.take_identifier();
            let parameter = Parameter { type_name, var_name };
            let token_range = identifier_token_idx..identifier_token_idx + 1;
            self.make_ast_node(parameter, token_range, vec![])
        })
    }

    fn take_parameters(&mut self) -> Vec<ASTNode<Parameter>> {
        let mut result = Vec::new();
        if let Some(parameter) = self.maybe_take_parameter() {
            result.push(parameter);

            while let Some(Token { kind: TokenKind::Comma, .. }) = self.token_iter.peek() {
                self.token_iter.next(); // comma
                result.push(self.maybe_take_parameter().unwrap_or_else(|| panic!("expected parameter after comma")));
            }
        }
        result
    }

    fn maybe_take_array_index(&mut self) -> Option<ASTNode<Expression>> {
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

    fn take_let_statement(&mut self, let_keyword_token_idx: usize) -> ASTNode<Statement> {
        self.token_iter.next(); // "let" keyword
        let (var_name, _) = self.take_identifier();
        let array_index = self.maybe_take_array_index();
        self.take_token(TokenKind::Operator(Equals));
        let value = self.take_expression();
        let semicolon = self.take_token(TokenKind::Semicolon);
        let mut child_node_idxs = vec![value.node_idx];
        if let Some(arr_idx) = &array_index {
            child_node_idxs.push(arr_idx.node_idx);
        }
        let statement = Statement::Let {
            var_name,
            array_index,
            value,
        };
        let token_range = let_keyword_token_idx..semicolon.idx + 1;
        self.make_ast_node(statement, token_range, child_node_idxs)
    }

    fn take_statement_block(&mut self) -> (Vec<ASTNode<Statement>>, Range<usize>) {
        let l_curly = self.take_token(TokenKind::LCurly);
        let statements = self.take_statements();
        let r_curly = self.take_token(TokenKind::RCurly);
        (statements, l_curly.idx..r_curly.idx + 1)
    }

    fn maybe_take_else_block(&mut self) -> Option<(Vec<ASTNode<Statement>>, Range<usize>)> {
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

    fn take_if_statement(&mut self, if_keyword_token_idx: usize) -> ASTNode<Statement> {
        self.token_iter.next(); // "if" keyword
        self.take_token(TokenKind::LParen);
        let condition = self.take_expression();
        self.take_token(TokenKind::RParen);
        let (if_statements, if_statements_token_range) = self.take_statement_block();
        let (else_statements, else_block_token_range) = unzip(self.maybe_take_else_block());
        let mut child_node_idxs = vec![condition.node_idx];
        child_node_idxs.extend(if_statements.iter().map(|stmt| stmt.node_idx));
        if let Some(else_stmts) = &else_statements {
            child_node_idxs.extend(else_stmts.iter().map(|stmt| stmt.node_idx));
        }
        let statement = Statement::If {
            condition,
            if_statements,
            else_statements,
        };
        let last_part_of_token_range = else_block_token_range.unwrap_or(if_statements_token_range);
        let token_range = if_keyword_token_idx..last_part_of_token_range.end;

        self.make_ast_node(statement, token_range, child_node_idxs)
    }

    fn take_while_statement(&mut self, while_keyword_token_idx: usize) -> ASTNode<Statement> {
        self.token_iter.next(); // "while" keyword
        self.take_token(TokenKind::LParen);
        let condition = self.take_expression();
        self.take_token(TokenKind::RParen);
        let (statements, statements_token_range) = self.take_statement_block();
        let mut child_node_idxs = vec![condition.node_idx];
        child_node_idxs.extend(statements.iter().map(|stmt| stmt.node_idx));
        let statement = Statement::While { condition, statements };
        let token_range = while_keyword_token_idx..statements_token_range.end;
        self.make_ast_node(statement, token_range, child_node_idxs)
    }

    fn take_do_statement(&mut self, do_keyword_token_idx: usize) -> ASTNode<Statement> {
        self.token_iter.next(); // "do" keyword
        let (identifier, identifier_token_idx) = self.take_identifier();
        let subroutine_call = self.take_subroutine_call(identifier, identifier_token_idx);
        let child_node_idxs = vec![subroutine_call.node_idx];
        let semicolon = self.take_token(TokenKind::Semicolon);
        let statement = Statement::Do(subroutine_call);
        let token_range = do_keyword_token_idx..semicolon.idx + 1;
        self.make_ast_node(statement, token_range, child_node_idxs)
    }

    fn take_return_statement(&mut self, return_keyword_token_idx: usize) -> ASTNode<Statement> {
        self.token_iter.next(); // "return" keyword
        let expression = self.maybe_take_expression_with_binding_power(0);
        let child_node_idxs = expression.as_ref().map(|expr| expr.node_idx).into_iter().collect();
        let semicolon = self.take_token(TokenKind::Semicolon);
        let statement = Statement::Return(expression);
        let token_range = return_keyword_token_idx..semicolon.idx + 1;
        self.make_ast_node(statement, token_range, child_node_idxs)
    }

    fn maybe_take_statement(&mut self) -> Option<ASTNode<Statement>> {
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

    fn take_statements(&mut self) -> Vec<ASTNode<Statement>> {
        let mut result = Vec::new();
        while let Some(statement) = self.maybe_take_statement() {
            result.push(statement);
        }
        result
    }

    fn take_var_declaration(&mut self) -> ASTNode<VarDeclaration> {
        if let Some(Token {
            kind: TokenKind::Keyword(KeywordTokenVariant::Var),
            idx: var_keyword_token_idx,
            ..
        }) = self.token_iter.next()
        {
            let type_name = self.take_type();
            let var_names = self.take_var_names();
            let semicolon = self.take_token(TokenKind::Semicolon);
            let var_declaration = VarDeclaration { type_name, var_names };
            let token_range = *var_keyword_token_idx..semicolon.idx + 1;
            self.make_ast_node(var_declaration, token_range, vec![])
        } else {
            panic!("expected var keyword");
        }
    }

    fn take_var_declarations(&mut self) -> Vec<ASTNode<VarDeclaration>> {
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

    fn take_subroutine_body(&mut self) -> ASTNode<SubroutineBody> {
        let l_curly = self.take_token(TokenKind::LCurly);
        let var_declarations = self.take_var_declarations();
        let statements = self.take_statements();
        let child_node_idxs = var_declarations
            .iter()
            .map(|var_dec| var_dec.node_idx)
            .chain(statements.iter().map(|stmt| stmt.node_idx))
            .collect();
        let r_curly = self.take_token(TokenKind::RCurly);
        let subroutine_body = SubroutineBody {
            var_declarations,
            statements,
        };
        let token_range = l_curly.idx..r_curly.idx + 1;
        self.make_ast_node(subroutine_body, token_range, child_node_idxs)
    }

    fn take_subroutine_declaration(&mut self) -> ASTNode<SubroutineDeclaration> {
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
            let body = self.take_subroutine_body();
            let child_node_idxs = parameters.iter().map(|param| param.node_idx).chain(iter::once(body.node_idx)).collect();
            let token_range = *subroutine_kind_token_idx..body.token_range.end;
            let subroutine_declaration = SubroutineDeclaration {
                subroutine_kind,
                return_type,
                name,
                parameters,
                body,
            };
            self.make_ast_node(subroutine_declaration, token_range, child_node_idxs)
        } else {
            panic!("expected subroutine kind");
        }
    }

    fn maybe_take_subroutine_declaration(&mut self) -> Option<ASTNode<SubroutineDeclaration>> {
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

    fn take_class_subroutine_declarations(&mut self) -> Vec<ASTNode<SubroutineDeclaration>> {
        let mut result = Vec::new();
        while let Some(subroutine_declaration) = self.maybe_take_subroutine_declaration() {
            result.push(subroutine_declaration);
        }
        result
    }

    fn take_class_var_declaration_qualifier(&mut self) -> ASTNode<ClassVarDeclarationKind> {
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
                let token_range = *token_idx..token_idx + 1;
                self.make_ast_node(qualifier, token_range, vec![])
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
        while let Some(Token { kind: TokenKind::Comma, .. }) = self.token_iter.peek() {
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

    fn take_class_var_declaration(&mut self) -> ASTNode<ClassVarDeclaration> {
        let qualifier = self.take_class_var_declaration_qualifier();
        let type_name = self.take_type();
        let var_names = self.take_var_names();
        let semicolon = self.take_token(TokenKind::Semicolon);
        let token_range = qualifier.token_range.start..semicolon.idx + 1;
        let child_node_idxs = vec![qualifier.node_idx];
        let class_var_declaration = ClassVarDeclaration {
            qualifier,
            type_name,
            var_names,
        };
        self.make_ast_node(class_var_declaration, token_range, child_node_idxs)
    }

    fn maybe_take_class_var_declaration(&mut self) -> Option<ASTNode<ClassVarDeclaration>> {
        use KeywordTokenVariant::*;
        match self.token_iter.peek().expect("unexpected end of input") {
            Token {
                kind: TokenKind::Keyword(Static | Field),
                ..
            } => {
                let class_var_declaration = self.take_class_var_declaration();
                Some(class_var_declaration)
            }
            _ => None,
        }
    }

    fn take_class_var_declarations(&mut self) -> Vec<ASTNode<ClassVarDeclaration>> {
        let mut result = Vec::new();
        while let Some(class_var_declaration) = self.maybe_take_class_var_declaration() {
            result.push(class_var_declaration);
        }
        result
    }

    fn take_class(&mut self) -> Class {
        let class_keyword = self.take_class_keyword();
        let (name, _) = self.take_identifier();
        self.take_token(TokenKind::LCurly);
        let var_declarations = self.take_class_var_declarations();
        let subroutine_declarations = self.take_class_subroutine_declarations();
        let r_curly = self.take_token(TokenKind::RCurly);
        let child_node_idxs = var_declarations
            .iter()
            .map(|var_dec| var_dec.node_idx)
            .chain(subroutine_declarations.iter().map(|subroutine| subroutine.node_idx))
            .collect();
        let class = Class {
            name,
            var_declarations,
            subroutine_declarations,
        };
        let token_range = class_keyword.idx..r_curly.idx + 1;
        let res = self.make_ast_node(class, token_range, child_node_idxs);
        *res.node
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::jack_compiler::tokenizer::token_defs;
    use crate::utils::tokenizer::Tokenizer;

    use pretty_assertions::assert_eq;

    fn parse_expression(source: &str) -> ASTNode<Expression> {
        let tokens: Vec<_> = Tokenizer::new(token_defs())
            .tokenize(source)
            .into_iter()
            .filter(|token| {
                !matches!(
                    token,
                    Token {
                        kind: TokenKind::Whitespace | TokenKind::MultiLineComment | TokenKind::SingleLineComment,
                        ..
                    }
                )
            })
            .collect();
        let mut sourcemap = JackParserSourceMap::new();
        let mut parser = Parser {
            token_iter: tokens.iter().peekable(),
            sourcemap: &mut sourcemap,
        };
        parser.take_expression()
    }

    #[test]
    fn test_simple_class() {
        let source = "class foo {}";
        let tokens: Vec<_> = Tokenizer::new(token_defs()).tokenize(source);
        assert_eq!(
            parse(&tokens).class,
            Class {
                name: "foo".to_string(),
                var_declarations: vec![],
                subroutine_declarations: vec![],
            },
        );
    }

    #[test]
    fn test_class_with_var_declaration() {
        let source = "
            class foo {
              static int bar;
            }";
        let tokens: Vec<_> = Tokenizer::new(token_defs()).tokenize(source);
        assert_eq!(
            parse(&tokens).class,
            Class {
                name: "foo".to_string(),
                var_declarations: vec![ASTNode {
                    node: Box::new(ClassVarDeclaration {
                        qualifier: ASTNode {
                            node: Box::new(ClassVarDeclarationKind::Static),
                            node_idx: 0,
                            token_range: 7..8,
                        },
                        type_name: Type::Int,
                        var_names: vec!["bar".to_string()],
                    }),
                    node_idx: 1,
                    token_range: 7..13,
                }],
                subroutine_declarations: vec![],
            }
        );
    }

    #[test]
    fn test_class_with_multiple_var_declarations() {
        let source = "
            class foo {
              static int bar;
              field char baz, buz, boz;
              field boolean a, b, c;
            }";
        let tokens: Vec<_> = Tokenizer::new(token_defs()).tokenize(source);
        assert_eq!(
            parse(&tokens).class,
            Class {
                name: "foo".to_string(),
                var_declarations: vec![
                    ASTNode {
                        node: Box::new(ClassVarDeclaration {
                            qualifier: ASTNode {
                                node: Box::new(ClassVarDeclarationKind::Static),
                                node_idx: 0,
                                token_range: 7..8,
                            },
                            type_name: Type::Int,
                            var_names: vec!["bar".to_string()],
                        }),
                        node_idx: 1,
                        token_range: 7..13,
                    },
                    ASTNode {
                        node: Box::new(ClassVarDeclaration {
                            qualifier: ASTNode {
                                node: Box::new(ClassVarDeclarationKind::Field),
                                node_idx: 2,
                                token_range: 14..15,
                            },
                            type_name: Type::Char,
                            var_names: vec!["baz".to_string(), "buz".to_string(), "boz".to_string()],
                        }),
                        node_idx: 3,
                        token_range: 14..26,
                    },
                    ASTNode {
                        node: Box::new(ClassVarDeclaration {
                            qualifier: ASTNode {
                                node: Box::new(ClassVarDeclarationKind::Field),
                                node_idx: 4,
                                token_range: 27..28,
                            },
                            type_name: Type::Boolean,
                            var_names: vec!["a".to_string(), "b".to_string(), "c".to_string(),],
                        }),
                        node_idx: 5,
                        token_range: 27..39,
                    }
                ],
                subroutine_declarations: vec![],
            }
        );
    }

    #[test]
    fn test_class_with_subroutine_declarations() {
        let source = "
            class foo {
                constructor boolean bar(int abc, char def, foo ghi) {
                }
                function char baz(boolean _123) {
                }
                method void qux() {
                }
            }";
        let tokens: Vec<_> = Tokenizer::new(token_defs()).tokenize(source);
        assert_eq!(
            parse(&tokens).class,
            Class {
                name: "foo".to_string(),
                var_declarations: vec![],
                subroutine_declarations: vec![
                    ASTNode {
                        node: Box::new(SubroutineDeclaration {
                            subroutine_kind: SubroutineKind::Constructor,
                            return_type: Some(Type::Boolean),
                            parameters: vec![
                                ASTNode {
                                    node: Box::new(Parameter {
                                        type_name: Type::Int,
                                        var_name: "abc".to_string(),
                                    }),
                                    node_idx: 0,
                                    token_range: 15..16,
                                },
                                ASTNode {
                                    node: Box::new(Parameter {
                                        type_name: Type::Char,
                                        var_name: "def".to_string(),
                                    }),
                                    node_idx: 1,
                                    token_range: 20..21,
                                },
                                ASTNode {
                                    node: Box::new(Parameter {
                                        type_name: Type::ClassName("foo".to_string()),
                                        var_name: "ghi".to_string(),
                                    }),
                                    node_idx: 2,
                                    token_range: 25..26,
                                }
                            ],
                            name: "bar".to_string(),
                            body: ASTNode {
                                node: Box::new(SubroutineBody {
                                    var_declarations: vec![],
                                    statements: vec![],
                                }),
                                node_idx: 3,
                                token_range: 28..31,
                            },
                        }),
                        node_idx: 4,
                        token_range: 7..31
                    },
                    ASTNode {
                        node: Box::new(SubroutineDeclaration {
                            subroutine_kind: SubroutineKind::Function,
                            return_type: Some(Type::Char),
                            parameters: vec![ASTNode {
                                node: Box::new(Parameter {
                                    type_name: Type::Boolean,
                                    var_name: "_123".to_string(),
                                }),
                                node_idx: 5,
                                token_range: 40..41,
                            }],
                            name: "baz".to_string(),
                            body: ASTNode {
                                node: Box::new(SubroutineBody {
                                    var_declarations: vec![],
                                    statements: vec![],
                                }),
                                node_idx: 6,
                                token_range: 43..46,
                            },
                        }),
                        node_idx: 7,
                        token_range: 32..46
                    },
                    ASTNode {
                        node: Box::new(SubroutineDeclaration {
                            subroutine_kind: SubroutineKind::Method,
                            return_type: None,
                            parameters: vec![],
                            name: "qux".to_string(),
                            body: ASTNode {
                                node: Box::new(SubroutineBody {
                                    var_declarations: vec![],
                                    statements: vec![],
                                }),
                                node_idx: 8,
                                token_range: 55..58,
                            }
                        }),
                        node_idx: 9,
                        token_range: 47..58,
                    },
                ],
            }
        );
    }

    #[test]
    fn test_all_statement_types() {
        let source = "
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
            }";
        let tokens: Vec<_> = Tokenizer::new(token_defs()).tokenize(source);
        assert_eq!(
            parse(&tokens).class,
            Class {
                name: "foo".to_string(),
                var_declarations: vec![],
                subroutine_declarations: vec![ASTNode {
                    node: Box::new(SubroutineDeclaration {
                        subroutine_kind: SubroutineKind::Constructor,
                        return_type: Some(Type::Int),
                        parameters: vec![],
                        name: "blah".to_string(),
                        body: ASTNode {
                            node: Box::new(SubroutineBody {
                                var_declarations: vec![ASTNode {
                                    node: Box::new(VarDeclaration {
                                        type_name: Type::Int,
                                        var_names: vec!["a".to_string()],
                                    }),
                                    node_idx: 0,
                                    token_range: 17..23,
                                }],
                                statements: vec![
                                    ASTNode {
                                        node: Box::new(Statement::Let {
                                            var_name: "a".to_string(),
                                            array_index: None,
                                            value: ASTNode {
                                                node: Box::new(Expression::PrimitiveTerm(IntegerConstant("1234".to_string(),))),
                                                node_idx: 1,
                                                token_range: 30..31,
                                            },
                                        }),
                                        node_idx: 2,
                                        token_range: 24..32,
                                    },
                                    ASTNode {
                                        node: Box::new(Statement::Let {
                                            var_name: "b".to_string(),
                                            array_index: Some(ASTNode {
                                                node: Box::new(Expression::PrimitiveTerm(IntegerConstant("22".to_string()))),
                                                node_idx: 3,
                                                token_range: 37..38,
                                            }),
                                            value: ASTNode {
                                                node: Box::new(Expression::PrimitiveTerm(IntegerConstant("123".to_string(),))),
                                                node_idx: 4,
                                                token_range: 42..43,
                                            }
                                        }),
                                        node_idx: 5,
                                        token_range: 33..44
                                    },
                                    ASTNode {
                                        node: Box::new(Statement::If {
                                            condition: ASTNode {
                                                node: Box::new(Expression::PrimitiveTerm(IntegerConstant("1".to_string()))),
                                                node_idx: 6,
                                                token_range: 48..49,
                                            },
                                            if_statements: vec![ASTNode {
                                                node: Box::new(Statement::While {
                                                    condition: (ASTNode {
                                                        node: Box::new(Expression::PrimitiveTerm(IntegerConstant("1".to_string()))),
                                                        node_idx: 7,
                                                        token_range: 56..57,
                                                    }),
                                                    statements: vec![
                                                        ASTNode {
                                                            node: Box::new(Statement::Do(ASTNode {
                                                                node: Box::new(SubroutineCall::Direct {
                                                                    subroutine_name: "foobar".to_string(),
                                                                    arguments: vec![],
                                                                }),
                                                                node_idx: 8,
                                                                token_range: 63..66,
                                                            })),
                                                            node_idx: 9,
                                                            token_range: 61..67,
                                                        },
                                                        ASTNode {
                                                            node: Box::new(Statement::Do(ASTNode {
                                                                node: Box::new(SubroutineCall::Direct {
                                                                    subroutine_name: "foobar".to_string(),
                                                                    arguments: vec![
                                                                        (ASTNode {
                                                                            node: Box::new(Expression::PrimitiveTerm(IntegerConstant(
                                                                                "1".to_string()
                                                                            ))),
                                                                            node_idx: 10,
                                                                            token_range: 72..73,
                                                                        }),
                                                                    ],
                                                                }),
                                                                node_idx: 11,
                                                                token_range: 70..74,
                                                            },)),
                                                            node_idx: 12,
                                                            token_range: 68..75,
                                                        },
                                                        ASTNode {
                                                            node: Box::new(Statement::Do(ASTNode {
                                                                node: Box::new(SubroutineCall::Direct {
                                                                    subroutine_name: "foobar".to_string(),
                                                                    arguments: vec![
                                                                        ASTNode {
                                                                            node: Box::new(Expression::PrimitiveTerm(IntegerConstant(
                                                                                "1".to_string()
                                                                            ))),
                                                                            node_idx: 13,
                                                                            token_range: 80..81,
                                                                        },
                                                                        ASTNode {
                                                                            node: Box::new(Expression::PrimitiveTerm(IntegerConstant(
                                                                                "2".to_string()
                                                                            ))),
                                                                            node_idx: 14,
                                                                            token_range: 83..84,
                                                                        },
                                                                        ASTNode {
                                                                            node: Box::new(Expression::PrimitiveTerm(IntegerConstant(
                                                                                "3".to_string()
                                                                            ))),
                                                                            node_idx: 15,
                                                                            token_range: 86..87,
                                                                        },
                                                                    ],
                                                                }),
                                                                node_idx: 16,
                                                                token_range: 78..88,
                                                            })),
                                                            node_idx: 17,
                                                            token_range: 76..89,
                                                        },
                                                        ASTNode {
                                                            node: Box::new(Statement::Do(ASTNode {
                                                                node: Box::new(SubroutineCall::Method {
                                                                    this_name: "foo".to_string(),
                                                                    method_name: "bar".to_string(),
                                                                    arguments: vec![],
                                                                }),
                                                                node_idx: 18,
                                                                token_range: 94..97,
                                                            })),
                                                            node_idx: 19,
                                                            token_range: 90..98,
                                                        },
                                                        ASTNode {
                                                            node: Box::new(Statement::Do(ASTNode {
                                                                node: Box::new(SubroutineCall::Method {
                                                                    this_name: "foo".to_string(),
                                                                    method_name: "bar".to_string(),
                                                                    arguments: vec![ASTNode {
                                                                        node: Box::new(Expression::PrimitiveTerm(IntegerConstant("1".to_string()))),
                                                                        node_idx: 20,
                                                                        token_range: 105..106,
                                                                    }],
                                                                }),
                                                                node_idx: 21,
                                                                token_range: 103..107,
                                                            })),
                                                            node_idx: 22,
                                                            token_range: 99..108,
                                                        },
                                                        ASTNode {
                                                            node: Box::new(Statement::Do(ASTNode {
                                                                node: Box::new(SubroutineCall::Method {
                                                                    this_name: "foo".to_string(),
                                                                    method_name: "bar".to_string(),
                                                                    arguments: vec![
                                                                        ASTNode {
                                                                            node: Box::new(Expression::PrimitiveTerm(IntegerConstant(
                                                                                "1".to_string()
                                                                            ))),
                                                                            node_idx: 23,
                                                                            token_range: 115..116
                                                                        },
                                                                        ASTNode {
                                                                            node: Box::new(Expression::PrimitiveTerm(IntegerConstant(
                                                                                "2".to_string()
                                                                            ))),
                                                                            node_idx: 24,
                                                                            token_range: 118..119
                                                                        },
                                                                        ASTNode {
                                                                            node: Box::new(Expression::PrimitiveTerm(IntegerConstant(
                                                                                "3".to_string()
                                                                            ))),
                                                                            node_idx: 25,
                                                                            token_range: 121..122
                                                                        },
                                                                    ],
                                                                }),
                                                                node_idx: 26,
                                                                token_range: 113..123
                                                            },)),
                                                            node_idx: 27,
                                                            token_range: 109..124
                                                        },
                                                    ],
                                                }),
                                                node_idx: 28,
                                                token_range: 53..126
                                            },],
                                            else_statements: Some(vec![ASTNode {
                                                node: Box::new(Statement::Return(Some(ASTNode {
                                                    node: Box::new(Expression::PrimitiveTerm(IntegerConstant("123".to_string(),))),
                                                    node_idx: 29,
                                                    token_range: 135..136
                                                },))),
                                                node_idx: 30,
                                                token_range: 133..137
                                            },]),
                                        }),
                                        node_idx: 31,
                                        token_range: 45..139
                                    },
                                ],
                            }),
                            node_idx: 32,
                            token_range: 15..141
                        },
                    }),
                    node_idx: 33,
                    token_range: 7..141
                }],
            }
        );
    }

    #[test]
    fn test_simple_expression() {
        assert_eq!(
            parse_expression("1"),
            ASTNode {
                node: Box::new(Expression::PrimitiveTerm(IntegerConstant("1".to_string()))),
                node_idx: 0,
                token_range: 0..1,
            }
        )
    }

    #[test]
    fn test_simple_binary_expression() {
        assert_eq!(
            parse_expression("1 + 2"),
            ASTNode {
                node: Box::new(Expression::Binary {
                    operator: BinaryOperator::Plus,
                    lhs: ASTNode {
                        node: Box::new(Expression::PrimitiveTerm(IntegerConstant("1".to_string()))),
                        node_idx: 0,
                        token_range: 0..5,
                    },
                    rhs: ASTNode {
                        node: Box::new(Expression::PrimitiveTerm(IntegerConstant("2".to_string()))),
                        node_idx: 1,
                        token_range: 4..5,
                    }
                }),
                node_idx: 2,
                token_range: 0..5,
            }
        )
    }

    #[test]
    fn test_simple_binary_expression_within_class() {
        let source = "
            class foo {
                method void bar () {
                    let a = 1 + 2 + 3;
                }
            }
            ";
        let tokens: Vec<_> = Tokenizer::new(token_defs()).tokenize(source);
        assert_eq!(
            parse(&tokens).class,
            Class {
                name: "foo".to_string(),
                var_declarations: vec![],
                subroutine_declarations: vec![ASTNode {
                    node: Box::new(SubroutineDeclaration {
                        subroutine_kind: SubroutineKind::Method,
                        return_type: None,
                        parameters: vec![],
                        name: "bar".to_string(),
                        body: ASTNode {
                            node: Box::new(SubroutineBody {
                                var_declarations: vec![],
                                statements: vec![ASTNode {
                                    node: Box::new(Statement::Let {
                                        var_name: "a".to_string(),
                                        array_index: None,
                                        value: ASTNode {
                                            node: Box::new(Expression::Binary {
                                                operator: BinaryOperator::Plus,
                                                lhs: ASTNode {
                                                    node: Box::new(Expression::Binary {
                                                        operator: BinaryOperator::Plus,
                                                        lhs: ASTNode {
                                                            node: Box::new(Expression::PrimitiveTerm(IntegerConstant("1".to_string()))),
                                                            node_idx: 0,
                                                            token_range: 24..29,
                                                        },
                                                        rhs: ASTNode {
                                                            node: Box::new(Expression::PrimitiveTerm(IntegerConstant("2".to_string()))),
                                                            node_idx: 1,
                                                            token_range: 28..29,
                                                        },
                                                    }),
                                                    node_idx: 2,
                                                    token_range: 24..33,
                                                },
                                                rhs: ASTNode {
                                                    node: Box::new(Expression::PrimitiveTerm(IntegerConstant("3".to_string()))),
                                                    node_idx: 3,
                                                    token_range: 32..33,
                                                },
                                            }),
                                            node_idx: 4,
                                            token_range: 24..33,
                                        }
                                    }),
                                    node_idx: 5,
                                    token_range: 18..34,
                                }],
                            }),
                            node_idx: 6,
                            token_range: 16..36,
                        },
                    }),
                    node_idx: 7,
                    token_range: 7..36,
                }],
            }
        )
    }

    #[test]
    fn test_simple_left_associating_expression() {
        assert_eq!(
            parse_expression("1 + 2 + 3 + 4"),
            ASTNode {
                node: Box::new(Expression::Binary {
                    operator: BinaryOperator::Plus,
                    lhs: ASTNode {
                        node: Box::new(Expression::Binary {
                            operator: BinaryOperator::Plus,
                            lhs: ASTNode {
                                node: Box::new(Expression::Binary {
                                    operator: BinaryOperator::Plus,
                                    lhs: ASTNode {
                                        node: Box::new(Expression::PrimitiveTerm(IntegerConstant("1".to_string()))),
                                        node_idx: 0,
                                        token_range: 0..5,
                                    },
                                    rhs: ASTNode {
                                        node: Box::new(Expression::PrimitiveTerm(IntegerConstant("2".to_string()))),
                                        node_idx: 1,
                                        token_range: 4..5,
                                    },
                                }),
                                node_idx: 2,
                                token_range: 0..9,
                            },
                            rhs: ASTNode {
                                node: Box::new(Expression::PrimitiveTerm(IntegerConstant("3".to_string()))),
                                node_idx: 3,
                                token_range: 8..9,
                            },
                        }),
                        node_idx: 4,
                        token_range: 0..13,
                    },
                    rhs: ASTNode {
                        node: Box::new(Expression::PrimitiveTerm(IntegerConstant("4".to_string()))),
                        node_idx: 5,
                        token_range: 12..13,
                    },
                }),
                node_idx: 6,
                token_range: 0..13,
            }
        )
    }

    #[test]
    fn test_binary_precedence() {
        assert_eq!(
            parse_expression("1 + 2 * 3 + 4"),
            ASTNode {
                node: Box::new(Expression::Binary {
                    operator: BinaryOperator::Plus,
                    lhs: ASTNode {
                        node: Box::new(Expression::Binary {
                            operator: BinaryOperator::Plus,
                            lhs: ASTNode {
                                node: Box::new(Expression::PrimitiveTerm(IntegerConstant("1".to_string()))),
                                node_idx: 0,
                                token_range: 0..9
                            },
                            rhs: ASTNode {
                                node: Box::new(Expression::Binary {
                                    operator: BinaryOperator::Multiply,
                                    lhs: ASTNode {
                                        node: Box::new(Expression::PrimitiveTerm(IntegerConstant("2".to_string()))),
                                        node_idx: 1,
                                        token_range: 4..9
                                    },
                                    rhs: ASTNode {
                                        node: Box::new(Expression::PrimitiveTerm(IntegerConstant("3".to_string()))),
                                        node_idx: 2,
                                        token_range: 8..9
                                    },
                                }),
                                node_idx: 3,
                                token_range: 4..9
                            }
                        }),
                        node_idx: 4,
                        token_range: 0..13
                    },
                    rhs: ASTNode {
                        node: Box::new(Expression::PrimitiveTerm(IntegerConstant("4".to_string()))),
                        node_idx: 5,
                        token_range: 12..13
                    },
                }),
                node_idx: 6,
                token_range: 0..13
            }
        )
    }

    #[test]
    fn test_simple_unary_expression() {
        assert_eq!(
            parse_expression("~1"),
            ASTNode {
                node: Box::new(Expression::Unary {
                    operator: UnaryOperator::Not,
                    operand: ASTNode {
                        node: Box::new(Expression::PrimitiveTerm(IntegerConstant("1".to_string()))),
                        node_idx: 0,
                        token_range: 1..2
                    }
                }),
                node_idx: 1,
                token_range: 0..2
            },
        )
    }

    #[test]
    fn test_simple_combined_unary_and_binary_expression() {
        assert_eq!(
            parse_expression("~1 + ~2"),
            ASTNode {
                node: Box::new(Expression::Binary {
                    operator: BinaryOperator::Plus,
                    lhs: ASTNode {
                        node: Box::new(Expression::Unary {
                            operator: UnaryOperator::Not,
                            operand: ASTNode {
                                node: Box::new(Expression::PrimitiveTerm(IntegerConstant("1".to_string()))),
                                node_idx: 0,
                                token_range: 1..2
                            },
                        }),
                        node_idx: 1,
                        token_range: 0..7
                    },
                    rhs: ASTNode {
                        node: Box::new(Expression::Unary {
                            operator: UnaryOperator::Not,
                            operand: ASTNode {
                                node: Box::new(Expression::PrimitiveTerm(IntegerConstant("2".to_string()))),
                                node_idx: 2,
                                token_range: 6..7
                            }
                        }),
                        node_idx: 3,
                        token_range: 5..7
                    },
                }),
                node_idx: 4,
                token_range: 0..7
            }
        )
    }

    #[test]
    fn test_expression_with_subroutine_calls() {
        assert_eq!(
            parse_expression("1 + foo(1, baz.bar(1, 2), 3) + 2"),
            ASTNode {
                node: Box::new(Expression::Binary {
                    operator: BinaryOperator::Plus,
                    lhs: ASTNode {
                        node: Box::new(Expression::Binary {
                            operator: BinaryOperator::Plus,
                            lhs: ASTNode {
                                node: Box::new(Expression::PrimitiveTerm(IntegerConstant("1".to_string()))),
                                node_idx: 0,
                                token_range: 0..22
                            },
                            rhs: ASTNode {
                                node: Box::new(Expression::SubroutineCall(ASTNode {
                                    node: Box::new(SubroutineCall::Direct {
                                        subroutine_name: "foo".to_string(),
                                        arguments: vec![
                                            ASTNode {
                                                node: Box::new(Expression::PrimitiveTerm(IntegerConstant("1".to_string()))),
                                                node_idx: 1,
                                                token_range: 6..7
                                            },
                                            ASTNode {
                                                node: Box::new(Expression::SubroutineCall(ASTNode {
                                                    node: Box::new(SubroutineCall::Method {
                                                        this_name: "baz".to_string(),
                                                        method_name: "bar".to_string(),
                                                        arguments: vec![
                                                            ASTNode {
                                                                node: Box::new(Expression::PrimitiveTerm(IntegerConstant("1".to_string()))),
                                                                node_idx: 2,
                                                                token_range: 13..14
                                                            },
                                                            ASTNode {
                                                                node: Box::new(Expression::PrimitiveTerm(IntegerConstant("2".to_string()))),
                                                                node_idx: 3,
                                                                token_range: 16..17
                                                            },
                                                        ]
                                                    }),
                                                    node_idx: 4,
                                                    token_range: 11..18
                                                })),
                                                node_idx: 5,
                                                token_range: 11..18
                                            },
                                            ASTNode {
                                                node: Box::new(Expression::PrimitiveTerm(IntegerConstant("3".to_string()))),
                                                node_idx: 6,
                                                token_range: 20..21
                                            },
                                        ]
                                    }),
                                    node_idx: 7,
                                    token_range: 4..22
                                })),
                                node_idx: 8,
                                token_range: 4..22
                            },
                        }),
                        node_idx: 9,
                        token_range: 0..26
                    },
                    rhs: ASTNode {
                        node: Box::new(Expression::PrimitiveTerm(IntegerConstant("2".to_string()))),
                        node_idx: 10,
                        token_range: 25..26
                    },
                }),
                node_idx: 11,
                token_range: 0..26
            }
        )
    }

    #[test]
    fn test_expression_with_subroutine_call_and_array_access() {
        assert_eq!(
            parse_expression("1 + foo(1, bar[1 + 2], 3) + 2"),
            ASTNode {
                node: Box::new(Expression::Binary {
                    operator: BinaryOperator::Plus,
                    lhs: ASTNode {
                        node: Box::new(Expression::Binary {
                            operator: BinaryOperator::Plus,
                            lhs: ASTNode {
                                node: Box::new(Expression::PrimitiveTerm(IntegerConstant("1".to_string()))),
                                node_idx: 0,
                                token_range: 0..21
                            },
                            rhs: ASTNode {
                                node: Box::new(Expression::SubroutineCall(ASTNode {
                                    node: Box::new(SubroutineCall::Direct {
                                        subroutine_name: "foo".to_string(),
                                        arguments: vec![
                                            ASTNode {
                                                node: Box::new(Expression::PrimitiveTerm(IntegerConstant("1".to_string()))),
                                                node_idx: 1,
                                                token_range: 6..7
                                            },
                                            ASTNode {
                                                node: Box::new(Expression::ArrayAccess {
                                                    var_name: "bar".to_string(),
                                                    index: ASTNode {
                                                        node: Box::new(Expression::Binary {
                                                            operator: BinaryOperator::Plus,
                                                            lhs: ASTNode {
                                                                node: Box::new(Expression::PrimitiveTerm(IntegerConstant("1".to_string()))),
                                                                node_idx: 2,
                                                                token_range: 11..16
                                                            },
                                                            rhs: ASTNode {
                                                                node: Box::new(Expression::PrimitiveTerm(IntegerConstant("2".to_string()))),
                                                                node_idx: 3,
                                                                token_range: 15..16
                                                            },
                                                        }),
                                                        node_idx: 4,
                                                        token_range: 11..16
                                                    },
                                                }),
                                                node_idx: 5,
                                                token_range: 10..17
                                            },
                                            ASTNode {
                                                node: Box::new(Expression::PrimitiveTerm(IntegerConstant("3".to_string()))),
                                                node_idx: 6,
                                                token_range: 19..20
                                            },
                                        ]
                                    }),
                                    node_idx: 7,
                                    token_range: 4..21
                                })),
                                node_idx: 8,
                                token_range: 4..21
                            },
                        }),
                        node_idx: 9,
                        token_range: 0..25
                    },
                    rhs: ASTNode {
                        node: Box::new(Expression::PrimitiveTerm(IntegerConstant("2".to_string()))),
                        node_idx: 10,
                        token_range: 24..25
                    },
                }),
                node_idx: 11,
                token_range: 0..25
            }
        )
    }

    #[test]
    fn test_expression_with_variables_subroutine_calls_and_array_access() {
        assert_eq!(
            parse_expression("foo + bar[baz + buz.boz(qux, wox[123]) / bing]"),
            (ASTNode {
                node: Box::new(Expression::Binary {
                    operator: BinaryOperator::Plus,
                    lhs: ASTNode {
                        node: Box::new(Expression::Variable("foo".to_string())),
                        node_idx: 0,
                        token_range: 0..27
                    },
                    rhs: ASTNode {
                        node: Box::new(Expression::ArrayAccess {
                            var_name: "bar".to_string(),
                            index: ASTNode {
                                node: Box::new(Expression::Binary {
                                    operator: BinaryOperator::Plus,
                                    lhs: ASTNode {
                                        node: Box::new(Expression::Variable("baz".to_string())),
                                        node_idx: 1,
                                        token_range: 6..26
                                    },
                                    rhs: ASTNode {
                                        node: Box::new(Expression::Binary {
                                            operator: BinaryOperator::Divide,
                                            lhs: ASTNode {
                                                node: Box::new(Expression::SubroutineCall(ASTNode {
                                                    node: Box::new(SubroutineCall::Method {
                                                        this_name: "buz".to_string(),
                                                        method_name: "boz".to_string(),
                                                        arguments: vec![
                                                            ASTNode {
                                                                node: Box::new(Expression::Variable("qux".to_string())),
                                                                node_idx: 2,
                                                                token_range: 14..15
                                                            },
                                                            ASTNode {
                                                                node: Box::new(Expression::ArrayAccess {
                                                                    var_name: "wox".to_string(),
                                                                    index: ASTNode {
                                                                        node: Box::new(Expression::PrimitiveTerm(IntegerConstant("123".to_string()))),
                                                                        node_idx: 3,
                                                                        token_range: 19..20
                                                                    }
                                                                }),
                                                                node_idx: 4,
                                                                token_range: 18..21
                                                            },
                                                        ]
                                                    }),
                                                    node_idx: 5,
                                                    token_range: 12..22
                                                })),
                                                node_idx: 6,
                                                token_range: 12..26
                                            },
                                            rhs: ASTNode {
                                                node: Box::new(Expression::Variable("bing".to_string())),
                                                node_idx: 7,
                                                token_range: 25..26
                                            }
                                        }),
                                        node_idx: 8,
                                        token_range: 12..26
                                    },
                                }),
                                node_idx: 9,
                                token_range: 6..26
                            }
                        }),
                        node_idx: 10,
                        token_range: 5..27
                    },
                }),
                node_idx: 11,
                token_range: 0..27
            })
        )
    }

    #[test]
    fn test_primitive_terms() {
        assert_eq!(
            parse_expression("1 + \"hello\" + true + false + null + this"),
            ASTNode {
                node: Box::new(Expression::Binary {
                    operator: BinaryOperator::Plus,
                    lhs: ASTNode {
                        node: Box::new(Expression::Binary {
                            operator: BinaryOperator::Plus,
                            lhs: ASTNode {
                                node: Box::new(Expression::Binary {
                                    operator: BinaryOperator::Plus,
                                    lhs: ASTNode {
                                        node: Box::new(Expression::Binary {
                                            operator: BinaryOperator::Plus,
                                            lhs: ASTNode {
                                                node: Box::new(Expression::Binary {
                                                    operator: BinaryOperator::Plus,
                                                    lhs: ASTNode {
                                                        node: Box::new(Expression::PrimitiveTerm(IntegerConstant("1".to_string()))),
                                                        node_idx: 0,
                                                        token_range: 0..5
                                                    },
                                                    rhs: ASTNode {
                                                        node: Box::new(Expression::PrimitiveTerm(StringConstant("hello".to_string()))),
                                                        node_idx: 1,
                                                        token_range: 4..5
                                                    },
                                                }),
                                                node_idx: 2,
                                                token_range: 0..9
                                            },
                                            rhs: ASTNode {
                                                node: Box::new(Expression::PrimitiveTerm(PrimitiveTermVariant::True)),
                                                node_idx: 3,
                                                token_range: 8..9
                                            },
                                        }),
                                        node_idx: 4,
                                        token_range: 0..13
                                    },
                                    rhs: ASTNode {
                                        node: Box::new(Expression::PrimitiveTerm(PrimitiveTermVariant::False)),
                                        node_idx: 5,
                                        token_range: 12..13
                                    },
                                }),
                                node_idx: 6,
                                token_range: 0..17
                            },
                            rhs: ASTNode {
                                node: Box::new(Expression::PrimitiveTerm(PrimitiveTermVariant::Null)),
                                node_idx: 7,
                                token_range: 16..17
                            },
                        }),
                        node_idx: 8,
                        token_range: 0..21
                    },
                    rhs: ASTNode {
                        node: Box::new(Expression::PrimitiveTerm(PrimitiveTermVariant::This)),
                        node_idx: 9,
                        token_range: 20..21
                    }
                }),
                node_idx: 10,
                token_range: 0..21
            }
        )
    }

    #[test]
    fn test_parenthesized_expression() {
        assert_eq!(
            parse_expression("(1 + 2) * 3"),
            ASTNode {
                node: Box::new(Expression::Binary {
                    operator: BinaryOperator::Multiply,
                    lhs: ASTNode {
                        node: Box::new(Expression::Parenthesized(ASTNode {
                            node: Box::new(Expression::Binary {
                                operator: BinaryOperator::Plus,
                                lhs: ASTNode {
                                    node: Box::new(Expression::PrimitiveTerm(IntegerConstant("1".to_string()))),
                                    node_idx: 0,
                                    token_range: 1..6
                                },
                                rhs: ASTNode {
                                    node: Box::new(Expression::PrimitiveTerm(IntegerConstant("2".to_string()))),
                                    node_idx: 1,
                                    token_range: 5..6
                                },
                            }),
                            node_idx: 2,
                            token_range: 1..6
                        })),
                        node_idx: 3,
                        token_range: 0..11
                    },
                    rhs: ASTNode {
                        node: Box::new(Expression::PrimitiveTerm(IntegerConstant("3".to_string()))),
                        node_idx: 4,
                        token_range: 10..11
                    },
                }),
                node_idx: 5,
                token_range: 0..11
            }
        )
    }
}
