use super::parser::{
    Class, ClassVarDeclaration, ClassVarDeclarationKind, Expression, Parameter, Statement,
    SubroutineBody, SubroutineCall, SubroutineDeclaration, SubroutineKind, Type, VarDeclaration,
};
use std::collections::HashMap;

struct Symbol {
    offset: usize,
    symbol_type: Type,
}

pub struct CodeGenerator {
    class_fields: HashMap<String, Symbol>,
    class_statics: HashMap<String, Symbol>,
    subroutine_parameters: HashMap<String, Symbol>,
    subroutine_vars: HashMap<String, Symbol>,
}

impl CodeGenerator {
    pub fn new() -> Self {
        CodeGenerator {
            class_fields: HashMap::new(),
            class_statics: HashMap::new(),
            subroutine_parameters: HashMap::new(),
            subroutine_vars: HashMap::new(),
        }
    }

    fn clear_subroutine(&mut self) {
        self.subroutine_parameters.clear();
        self.subroutine_vars.clear();
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

    fn compile_while_statement(
        &mut self,
        condition: Expression,
        statements: Vec<Statement>,
    ) -> String {
        todo!()
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
            .map(|subroutine| match subroutine.subroutine_kind {
                SubroutineKind::Constructor => self.compile_constructor(subroutine, &class_name),
                SubroutineKind::Function => self.compile_function(subroutine, &class_name),
                SubroutineKind::Method => self.compile_method(subroutine, &class_name),
            })
            .collect();
        parts.join("\n")
    }

    fn compile_var_declarations(&mut self, var_declarations: Vec<ClassVarDeclaration>) {
        for var_declaration in var_declarations {
            let hashmap = match var_declaration.qualifier {
                ClassVarDeclarationKind::Static => self.class_statics,
                ClassVarDeclarationKind::Field => self.class_fields,
            };
            for var_name in var_declaration.var_names {
                hashmap.insert(
                    var_name,
                    Symbol {
                        offset: hashmap.len(),
                        symbol_type: var_declaration.type_name,
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
