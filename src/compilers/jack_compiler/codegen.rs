use super::parser::{
    BinaryOperator, Class, ClassVarDeclaration, ClassVarDeclarationKind, Expression, Parameter,
    PrimitiveTermVariant, Statement, SubroutineBody, SubroutineCall, SubroutineDeclaration,
    SubroutineKind, Type, UnaryOperator, VarDeclaration,
};
use std::collections::HashMap;

enum SymbolKind {
    Local,
    Parameter,
    Field,
    Static,
}

struct Symbol {
    offset: usize,
    symbol_type: Type,
    kind: SymbolKind,
}

pub struct CodeGenerator {
    class_fields: HashMap<String, Symbol>,
    class_statics: HashMap<String, Symbol>,
    subroutine_while_count: usize,
    subroutine_parameters: HashMap<String, Symbol>,
    subroutine_vars: HashMap<String, Symbol>,
    subroutine_kind: Option<SubroutineKind>,
}

impl CodeGenerator {
    pub fn new() -> Self {
        CodeGenerator {
            subroutine_while_count: 0,
            class_fields: HashMap::new(),
            class_statics: HashMap::new(),
            subroutine_parameters: HashMap::new(),
            subroutine_vars: HashMap::new(),
            subroutine_kind: None,
        }
    }

    fn clear_subroutine(&mut self) {
        self.subroutine_parameters.clear();
        self.subroutine_vars.clear();
        self.subroutine_while_count = 0;
    }

    fn compile_constructor(
        &mut self,
        subroutine: SubroutineDeclaration,
        class_name: &str,
    ) -> String {
        todo!()
    }

    fn compile_function(&mut self, subroutine: SubroutineDeclaration, class_name: &str) -> String {
        todo!()
    }

    fn compile_subroutine_var_declarations(&mut self, var_declarations: Vec<VarDeclaration>) {
        for var_declaration in var_declarations {
            for var_name in var_declaration.var_names {
                self.subroutine_vars.insert(
                    var_name,
                    Symbol {
                        offset: self.subroutine_vars.len(),
                        symbol_type: var_declaration.type_name,
                        kind: SymbolKind::Local,
                    },
                );
            }
        }
    }
    fn compile_do_statement(&mut self, subroutine_call: SubroutineCall) -> String {
        todo!()
    }

    fn compile_let_statement(
        &mut self,
        var_name: String,
        array_index: Option<Expression>,
        value: Expression,
    ) -> String {
        // TODO - check types
        todo!()
    }

    fn compile_if_statement(
        &mut self,
        condition: Expression,
        if_statements: Vec<Statement>,
        else_statements: Option<Vec<Statement>>,
    ) -> String {
        todo!()
    }

    fn compile_return_statement(&mut self, return_value: Option<Expression>) -> String {
        // TODO - check return statements against return type
        todo!()
    }

    fn compile_array_access_expression(&mut self, var_name: String, index: Expression) -> String {
        todo!()
    }

    fn compile_binary_expression(
        &mut self,
        operator: BinaryOperator,
        lhs: Expression,
        rhs: Expression,
    ) -> String {
        todo!()
    }

    fn compile_primitive_term_expression(
        &mut self,
        primitive_term: PrimitiveTermVariant,
    ) -> String {
        todo!()
    }

    fn compile_direct_subroutine_call_expression(
        &mut self,
        subroutine_name: String,
        arguments: Vec<Expression>,
    ) -> String {
        todo!()
    }

    fn compile_method_subroutine_call_expression(
        &mut self,
        this_name: String,
        method_name: String,
        arguments: Vec<Expression>,
    ) -> String {
        todo!()
    }

    fn compile_subroutine_call_expression(&mut self, subroutine_call: SubroutineCall) -> String {
        match subroutine_call {
            SubroutineCall::Direct {
                subroutine_name,
                arguments,
            } => self.compile_direct_subroutine_call_expression(subroutine_name, arguments),
            SubroutineCall::Method {
                this_name,
                method_name,
                arguments,
            } => self.compile_method_subroutine_call_expression(this_name, method_name, arguments),
        }
    }

    fn compile_unary_expression(&mut self, operator: UnaryOperator, operand: Expression) -> String {
        let perform_op = match operator {
            UnaryOperator::Minus => "neg",
            UnaryOperator::Not => "not",
        };

        format!(
            "
        {push_value}
        {perform_op}
        ",
            push_value = self.compile_expression(operand),
            perform_op = perform_op
        )
    }

    fn resolve_symbol(&mut self, var_name: String) -> Option<&Symbol> {
        self.subroutine_vars
            .get(&var_name)
            .or_else(|| self.subroutine_parameters.get(&var_name))
            .or_else(|| {
                if let Some(SubroutineKind::Method | SubroutineKind::Constructor) =
                    self.subroutine_kind
                {
                    self.class_fields.get(&var_name)
                } else {
                    None
                }
            })
            .or_else(|| self.class_statics.get(&var_name))
    }

    fn compile_variable_expression(&mut self, var_name: String) -> String {
        let symbol = self
            .resolve_symbol(var_name)
            .unwrap_or_else(|| panic!("failed to resolve variable {}", var_name));

        let symbol_kind = match symbol.kind {
            SymbolKind::Local => "local",
            SymbolKind::Parameter => "argument",
            SymbolKind::Field => "this",
            SymbolKind::Static => "static",
        };

        format!("push {} {}", symbol_kind, symbol.offset)
    }

    fn compile_expression(&mut self, expression: Expression) -> String {
        match expression {
            Expression::ArrayAccess { var_name, index } => {
                self.compile_array_access_expression(var_name, *index)
            }
            Expression::Binary { operator, lhs, rhs } => {
                self.compile_binary_expression(operator, *lhs, *rhs)
            }
            Expression::PrimitiveTerm(primitive_term) => {
                self.compile_primitive_term_expression(primitive_term)
            }
            Expression::SubroutineCall(subroutine_call) => {
                self.compile_subroutine_call_expression(subroutine_call)
            }
            Expression::Unary { operator, operand } => {
                self.compile_unary_expression(operator, *operand)
            }
            Expression::Variable(var_name) => self.compile_variable_expression(var_name),
        }
    }

    fn compile_statements(&mut self, statements: Vec<Statement>) -> String {
        let compiled_statements: Vec<_> = statements
            .into_iter()
            .map(|statement| self.compile_statement(statement))
            .collect();
        compiled_statements.join("\n")
    }

    fn compile_while_statement(
        &mut self,
        condition: Expression,
        statements: Vec<Statement>,
    ) -> String {
        let result = format!(
            "
        label start_while_{while_idx}
          {condition}
          not
          if-goto end_while_{while_idx}
          {body}
          goto start_while_{while_idx}
          label end_while_{while_idx}
        ",
            while_idx = self.subroutine_while_count,
            condition = self.compile_expression(condition),
            body = self.compile_statements(statements)
        );
        self.subroutine_while_count += 1;
        result
    }

    fn compile_statement(&mut self, statement: Statement) -> String {
        match statement {
            Statement::Do(subroutine_call) => self.compile_do_statement(subroutine_call),
            Statement::Let {
                var_name,
                array_index,
                value,
            } => self.compile_let_statement(var_name, array_index, value),
            Statement::If {
                condition,
                if_statements,
                else_statements,
            } => self.compile_if_statement(condition, if_statements, else_statements),
            Statement::Return(expression) => self.compile_return_statement(expression),
            Statement::While {
                condition,
                statements,
            } => self.compile_while_statement(condition, statements),
        }
    }

    fn compile_method_body(
        &mut self,
        method_body: SubroutineBody,
        return_type: Option<Type>,
    ) -> String {
        self.compile_subroutine_var_declarations(method_body.var_declarations);
        let compiled_statements: Vec<_> = method_body
            .statements
            .into_iter()
            .map(|statement| self.compile_statement(statement))
            .collect();
        compiled_statements.join("\n")
    }

    fn compile_method_parameters(&mut self, parameters: Vec<Parameter>) {
        for parameter in parameters {
            self.subroutine_parameters.insert(
                parameter.var_name,
                Symbol {
                    offset: self.subroutine_parameters.len(),
                    symbol_type: parameter.type_name,
                    kind: SymbolKind::Parameter,
                },
            );
        }
    }

    fn compile_method(&mut self, method: SubroutineDeclaration, class_name: &str) -> String {
        let param_count = method.parameters.len();
        self.compile_method_parameters(method.parameters);

        let implicit_return = if method.return_type.is_none() {
            "push constant 0"
        } else {
            ""
        };

        format!(
            "
        function {class_name}.{function_name} {params_len}
          push argument 0
          pop pointer 0
          {body}

          {implicit_return}
        ",
            function_name = method.name,
            class_name = class_name,
            params_len = param_count + 1,
            body = self.compile_method_body(method.body, method.return_type),
            implicit_return = implicit_return
        )
    }

    fn compile_subroutines(
        &mut self,
        subroutine_declarations: Vec<SubroutineDeclaration>,
        class_name: String,
    ) -> String {
        let parts: Vec<_> = subroutine_declarations
            .into_iter()
            .map(|subroutine| {
                self.subroutine_kind = Some(subroutine.subroutine_kind);

                match subroutine.subroutine_kind {
                    SubroutineKind::Constructor => {
                        self.compile_constructor(subroutine, &class_name)
                    }
                    SubroutineKind::Function => self.compile_function(subroutine, &class_name),
                    SubroutineKind::Method => self.compile_method(subroutine, &class_name),
                }
            })
            .collect();
        parts.join("\n")
    }

    fn compile_var_declarations(&mut self, var_declarations: Vec<ClassVarDeclaration>) {
        for var_declaration in var_declarations {
            let (hashmap, symbol_kind) = match var_declaration.qualifier {
                ClassVarDeclarationKind::Static => (self.class_statics, SymbolKind::Static),
                ClassVarDeclarationKind::Field => (self.class_fields, SymbolKind::Field),
            };
            for var_name in var_declaration.var_names {
                hashmap.insert(
                    var_name,
                    Symbol {
                        offset: hashmap.len(),
                        symbol_type: var_declaration.type_name,
                        kind: symbol_kind,
                    },
                );
            }
        }
    }

    pub fn vm_code(&mut self, class: Class) -> String {
        self.compile_var_declarations(class.var_declarations);
        self.compile_subroutines(class.subroutine_declarations, class.name)
    }
}
