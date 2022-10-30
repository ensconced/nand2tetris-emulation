use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use super::{
    codegen::full_subroutine_name,
    jack_node_types::{ASTNode, Expression, Statement, SubroutineCall, SubroutineDeclaration},
    parser::JackParserResult,
};

#[derive(Default, Debug)]
pub struct SubroutineInfo {
    callers: HashSet<String>,
    calls: HashSet<String>,
}

#[derive(Default)]
pub struct CallGraphAnalyser {
    current_class_name: Option<String>,
}

impl CallGraphAnalyser {
    fn find_calls_in_expression(&self, expression: &ASTNode<Expression>) -> Vec<String> {
        match &*expression.node {
            Expression::ArrayAccess { index, .. } => self.find_calls_in_expression(index),
            Expression::Binary { lhs, rhs, .. } => self
                .find_calls_in_expression(lhs)
                .into_iter()
                .chain(self.find_calls_in_expression(rhs))
                .collect(),

            Expression::Parenthesized(inner_expression) => self.find_calls_in_expression(inner_expression),
            Expression::SubroutineCall(subroutine_call) => self.find_calls_in_subroutine_call(subroutine_call),
            Expression::Unary { operand, .. } => self.find_calls_in_expression(operand),
            Expression::PrimitiveTerm(_) | Expression::Variable(_) => vec![],
        }
    }

    fn find_calls_in_expressions(&self, expressions: &[ASTNode<Expression>]) -> Vec<String> {
        expressions
            .iter()
            .flat_map(|expression| self.find_calls_in_expression(expression))
            .collect()
    }

    fn find_calls_in_optional_expression(&self, maybe_expression: &Option<ASTNode<Expression>>) -> Vec<String> {
        maybe_expression
            .as_ref()
            .map(|expression| self.find_calls_in_expression(expression))
            .into_iter()
            .flatten()
            .collect()
    }

    fn find_calls_in_subroutine_call(&self, subroutine_call: &ASTNode<SubroutineCall>) -> Vec<String> {
        let current_class_name = self.current_class_name.clone().unwrap_or_else(|| panic!("current class name is none"));
        match &*subroutine_call.node {
            SubroutineCall::Direct { arguments, subroutine_name } => {
                let callee_name = full_subroutine_name(&current_class_name, subroutine_name);
                vec![callee_name].into_iter().chain(self.find_calls_in_expressions(arguments)).collect()
            }
            SubroutineCall::Method {
                this_name,
                method_name,
                arguments,
            } => {
                let callee_name = full_subroutine_name(this_name, method_name);
                vec![callee_name].into_iter().chain(self.find_calls_in_expressions(arguments)).collect()
            }
        }
    }

    fn find_calls_in_statements(&self, statements: &[ASTNode<Statement>]) -> Vec<String> {
        statements.iter().flat_map(|statement| self.find_calls_in_statement(statement)).collect()
    }

    fn find_calls_in_optional_statements(&self, maybe_statements: &Option<Vec<ASTNode<Statement>>>) -> Vec<String> {
        maybe_statements
            .as_ref()
            .into_iter()
            .flat_map(|statements| self.find_calls_in_statements(statements))
            .collect()
    }

    fn find_calls_in_statement(&self, statement: &ASTNode<Statement>) -> Vec<String> {
        match &*statement.node {
            Statement::Do(subroutine_call) => self.find_calls_in_subroutine_call(subroutine_call),
            Statement::If {
                condition,
                if_statements,
                else_statements,
            } => self
                .find_calls_in_expression(condition)
                .into_iter()
                .chain(self.find_calls_in_statements(if_statements))
                .chain(self.find_calls_in_optional_statements(else_statements))
                .collect(),
            Statement::Let { array_index, value, .. } => self
                .find_calls_in_expression(value)
                .into_iter()
                .chain(self.find_calls_in_optional_expression(array_index))
                .collect(),
            Statement::Return(expression) => self.find_calls_in_optional_expression(expression),
            Statement::While { condition, statements } => self
                .find_calls_in_expression(condition)
                .into_iter()
                .chain(self.find_calls_in_statements(statements))
                .collect(),
        }
    }

    fn find_calls(&self, subroutine_declaration: &SubroutineDeclaration) -> Vec<String> {
        subroutine_declaration
            .body
            .node
            .statements
            .iter()
            .flat_map(|statement| self.find_calls_in_statement(statement))
            .collect()
    }

    fn analyse_call_graph(&mut self, parsed_jack_program: &HashMap<PathBuf, JackParserResult>) -> HashMap<String, SubroutineInfo> {
        let mut call_graph: HashMap<String, SubroutineInfo> = HashMap::new();

        for JackParserResult { class, .. } in parsed_jack_program.values() {
            self.current_class_name = Some(class.name.clone());
            for ASTNode { node, .. } in &class.subroutine_declarations {
                let callee_name = full_subroutine_name(&class.name, &node.name);
                for called_subroutine in self.find_calls(node) {
                    let caller_info = call_graph.entry(callee_name.clone()).or_default();
                    caller_info.calls.insert(called_subroutine.clone());

                    let callee_info = call_graph.entry(called_subroutine).or_default();
                    callee_info.callers.insert(callee_name.clone());
                }
            }
        }
        call_graph
    }
}

pub fn call_graph_analysis(parsed_jack_program: &HashMap<PathBuf, JackParserResult>) -> HashMap<String, SubroutineInfo> {
    let mut analyser = CallGraphAnalyser::default();
    analyser.analyse_call_graph(parsed_jack_program)
}
