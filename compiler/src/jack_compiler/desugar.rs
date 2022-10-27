use super::jack_node_types::{Class, Expression, Statement, SubroutineCall};

fn desugar_expression(expression: &mut Expression) {
    todo!()
}

fn desugar_statement(statement: &mut Statement) {
    match statement {
        Statement::Let { array_index, value, .. } => {
            if let Some(index_expression) = array_index {
                desugar_expression(&mut index_expression.node);
            }
            desugar_expression(&mut value.node);
        }
        Statement::Do(subroutine_call) => match &mut *subroutine_call.node {
            SubroutineCall::Direct { arguments, .. } => {
                for argument in arguments {
                    desugar_expression(&mut *argument.node);
                }
            }
            SubroutineCall::Method { arguments, .. } => {
                for argument in arguments {
                    desugar_expression(&mut *argument.node);
                }
            }
        },
        Statement::If {
            condition,
            if_statements,
            else_statements,
        } => {
            desugar_expression(&mut *condition.node);
            for statement in if_statements {
                desugar_statement(&mut *statement.node);
            }
            if let Some(else_statements) = else_statements {
                for statement in else_statements {
                    desugar_statement(&mut *statement.node);
                }
            }
        }
        Statement::Return(expression) => {
            if let Some(expression) = expression {
                desugar_expression(&mut *expression.node);
            }
        }
        Statement::While { condition, statements } => {
            desugar_expression(&mut *condition.node);
            for statement in statements {
                desugar_statement(&mut *statement.node);
            }
        }
    }
}

fn desugar_class(class: &mut Class) {
    for subroutine in &mut class.subroutine_declarations {
        for statement in &mut subroutine.node.body.node.statements {
            desugar_statement(&mut *statement.node);
        }
    }
}
