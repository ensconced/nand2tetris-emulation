use super::parser::{
    BinaryOperator, Class, ClassVarDeclaration, ClassVarDeclarationKind, Expression, Parameter,
    PrimitiveTermVariant, Statement, SubroutineCall, SubroutineDeclaration, SubroutineKind, Type,
    UnaryOperator, VarDeclaration,
};
use std::collections::HashMap;

#[derive(Clone, PartialEq)]
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

struct CodeGenerator {
    class_name: Option<String>,
    class_fields: HashMap<String, Symbol>,
    class_statics: HashMap<String, Symbol>,
    subroutine_while_count: usize,
    subroutine_if_count: usize,
    subroutine_parameters: HashMap<String, Symbol>,
    subroutine_vars: HashMap<String, Symbol>,
    subroutine_kind: Option<SubroutineKind>,
}

impl CodeGenerator {
    pub fn new() -> Self {
        CodeGenerator {
            class_name: None,
            class_fields: HashMap::new(),
            class_statics: HashMap::new(),
            subroutine_while_count: 0,
            subroutine_if_count: 0,
            subroutine_parameters: HashMap::new(),
            subroutine_vars: HashMap::new(),
            subroutine_kind: None,
        }
    }

    fn get_class_name(&self) -> &str {
        self.class_name
            .as_ref()
            .unwrap_or_else(|| panic!("missing class_name"))
    }

    fn clear_subroutine(&mut self) {
        self.subroutine_while_count = 0;
        self.subroutine_if_count = 0;
        self.subroutine_parameters.clear();
        self.subroutine_vars.clear();
        self.subroutine_kind = None;
    }

    fn compile_subroutine_var_declarations(
        &mut self,
        var_declarations: Vec<VarDeclaration>,
    ) -> usize {
        let mut count = 0;
        for var_declaration in var_declarations {
            for var_name in var_declaration.var_names {
                count += 1;
                self.subroutine_vars.insert(
                    var_name,
                    Symbol {
                        offset: self.subroutine_vars.len(),
                        symbol_type: var_declaration.type_name.clone(),
                        kind: SymbolKind::Local,
                    },
                );
            }
        }
        count
    }
    fn compile_do_statement(&mut self, subroutine_call: SubroutineCall) -> String {
        format!(
            "
          {}
          pop constant 0
        ",
            self.compile_subroutine_call_expression(subroutine_call)
        )
    }

    fn compile_let_statement(
        &mut self,
        var_name: String,
        array_index: Option<Expression>,
        value: Expression,
    ) -> String {
        let compiled_value = self.compile_expression(value);
        let compiled_var = self.compile_variable(&var_name);

        if let Some(idx) = array_index {
            let compiled_idx = self.compile_expression(idx);
            format!(
                "
                push {compiled_var}
                {compiled_idx}
                add
                pop pointer 1
                {compiled_value}
                pop that 0
            "
            )
        } else {
            format!(
                "
                {compiled_value}
                pop {compiled_var}
            "
            )
        }
    }

    fn compile_if_statement(
        &mut self,
        condition: Expression,
        if_statements: Vec<Statement>,
        else_statements: Option<Vec<Statement>>,
    ) -> String {
        let if_count = self.subroutine_if_count;
        self.subroutine_if_count += 1;

        let compiled_condition = self.compile_expression(condition);
        let compiled_if_statements = self.compile_statements(if_statements);
        let compiled_else_statements = if let Some(statements) = else_statements {
            self.compile_statements(statements)
        } else {
            "".to_string()
        };

        format!(
            "
            {compiled_condition}
            if-goto if_statements_{if_count}

            {compiled_else_statements}
            goto end_if_{if_count}

            label if_statements_{if_count}
            {compiled_if_statements}

            label end_if_{if_count}

        "
        )
    }

    fn compile_return_statement(&mut self, return_value: Option<Expression>) -> String {
        let push_return_value = if let Some(expression) = return_value {
            self.compile_expression(expression)
        } else {
            "push constant 0".to_string()
        };

        format!(
            "
          {push_return_value}
          return
        "
        )
    }

    fn compile_array_access_expression(&mut self, var_name: String, index: Expression) -> String {
        let array_var = self.compile_variable(&var_name);
        let push_index = self.compile_expression(index);
        format!(
            "
        push {array_var}
        {push_index}
        add
        pop pointer 1
        push that 0
        "
        )
    }

    fn compile_binary_expression(
        &mut self,
        operator: BinaryOperator,
        lhs: Expression,
        rhs: Expression,
    ) -> String {
        let push_lhs = self.compile_expression(lhs);
        let push_rhs = self.compile_expression(rhs);
        let perform_op = match operator {
            BinaryOperator::And => "and",
            BinaryOperator::Divide => "call Math.divide 2",
            BinaryOperator::Equals => "eq",
            BinaryOperator::GreaterThan => "gt",
            BinaryOperator::LessThan => "lt",
            BinaryOperator::Minus => "sub",
            BinaryOperator::Multiply => "call Math.multiply 2",
            BinaryOperator::Or => "or",
            BinaryOperator::Plus => "add",
        };

        format!(
            "
          {push_lhs}
          {push_rhs}
          {perform_op}
        "
        )
    }

    fn compile_primitive_term_expression(
        &mut self,
        primitive_term: PrimitiveTermVariant,
    ) -> String {
        match primitive_term {
            PrimitiveTermVariant::False | PrimitiveTermVariant::Null => "push 0".to_string(),
            PrimitiveTermVariant::True => "push 0\nnot".to_string(),
            PrimitiveTermVariant::IntegerConstant(int_string) => {
                format!(
                    "push constant {}",
                    int_string
                        .parse::<i16>()
                        .unwrap_or_else(|_| panic!("{} is not valid 16 bit int", int_string))
                )
            }
            PrimitiveTermVariant::This => {
                if let Some(SubroutineKind::Method | SubroutineKind::Constructor) =
                    self.subroutine_kind
                {
                    "push pointer 0".to_string()
                } else {
                    panic!("cannot use \"this\" outside of method or constructor")
                }
            }
            PrimitiveTermVariant::StringConstant(string) => {
                // Strings in Jack are represented in memory in utf16.
                let code_units: Vec<_> = string.encode_utf16().collect();

                let append_chars: Vec<_> = code_units
                    .iter()
                    .map(|code_unit| {
                        if code_unit & 32768 == 32768 {
                            // code_unit exceeds max size for A-instruction, so use
                            // this little trick.
                            format!(
                                "
                            push that 0
                            push {}
                            not
                            call String.append 2
                            pop constant 0
                            ",
                                !code_unit
                            )
                        } else {
                            format!(
                                "
                            push that 0
                            push {}
                            call String.append 2
                            pop constant 0
                                ",
                                code_unit
                            )
                        }
                    })
                    .collect();

                let length = code_units.len();
                let joined_append_chars = append_chars.join("\n");

                format!(
                    "
                  push {length}
                  call String.new 1
                  pop pointer 1
                  {joined_append_chars}
                "
                )
            }
        }
    }

    fn compile_push_arguments(&mut self, arguments: Vec<Expression>) -> String {
        let compiled_arguments: Vec<_> = arguments
            .into_iter()
            .map(|argument| self.compile_expression(argument))
            .collect();
        compiled_arguments.join("\n")
    }

    fn compile_method_subroutine_call_expression(
        &mut self,
        this_name: String,
        method_name: String,
        arguments: Vec<Expression>,
    ) -> String {
        let arg_count = arguments.len();
        let push_arguments = self.compile_push_arguments(arguments);

        if let Some(symbol) = self.maybe_resolve_symbol(&this_name) {
            println!(
                "resolved symbol: this_name: {}, method_name: {}",
                &this_name, &method_name
            );

            // Treat it as a method.
            match symbol.symbol_type.clone() {
                Type::ClassName(this_class) => {
                    let arg_count_with_this = arg_count + 1;
                    let this = self.compile_variable(&this_name);
                    format!(
                        "
                        push {this}
                        {push_arguments}
                        call {this_class}.{method_name} {arg_count_with_this}
                        "
                    )
                }
                other_type => panic!("cannot call method on {:?}", other_type),
            }
        } else {
            println!(
                "failed to resolve symbol: this_name: {}, method_name: {}",
                &this_name, &method_name
            );
            // Treat it as constructor or function. Could be on this class or on
            // a different class. These are not resolved by the jack compiler -
            // resolution happens later, in the vm compiler.
            format!(
                "
              {push_arguments}
              call {this_name}.{method_name} {arg_count}
            "
            )
        }
    }

    fn compile_direct_subroutine_call_expression(
        &mut self,
        subroutine_name: String,
        arguments: Vec<Expression>,
    ) -> String {
        let arg_count = arguments.len();
        let class_name = self.get_class_name().to_owned();
        let push_arguments = self.compile_push_arguments(arguments);
        format!(
            "
        {push_arguments}
        call {class_name}.{subroutine_name} {arg_count}
        ",
        )
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
        let push_value = self.compile_expression(operand);

        format!(
            "
        {push_value}
        {perform_op}
        "
        )
    }

    fn maybe_resolve_symbol(&mut self, var_name: &str) -> Option<&Symbol> {
        self.subroutine_vars
            .get(var_name)
            .or_else(|| self.subroutine_parameters.get(var_name))
            .or_else(|| {
                if let Some(SubroutineKind::Method | SubroutineKind::Constructor) =
                    self.subroutine_kind
                {
                    self.class_fields.get(var_name)
                } else {
                    None
                }
            })
            .or_else(|| self.class_statics.get(var_name))
    }

    fn resolve_symbol(&mut self, var_name: &str) -> &Symbol {
        self.maybe_resolve_symbol(var_name)
            .unwrap_or_else(|| panic!("failed to resolve symbol for {}", var_name))
    }

    fn compile_variable(&mut self, var_name: &str) -> String {
        let symbol = self.resolve_symbol(var_name);

        let symbol_kind = match symbol.kind {
            SymbolKind::Local => "local",
            SymbolKind::Parameter => "argument",
            SymbolKind::Field => "this",
            SymbolKind::Static => "static",
        };

        format!("{symbol_kind} {}", symbol.offset)
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
            Expression::Variable(var_name) => {
                format!("push {}", self.compile_variable(&var_name))
            }
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
        let while_idx = self.subroutine_while_count;
        self.subroutine_while_count += 1;
        let condition = self.compile_expression(condition);
        let body = self.compile_statements(statements);

        let result = format!(
            "
        label start_while_{while_idx}
          {condition}
          not
          if-goto end_while_{while_idx}
          {body}
          goto start_while_{while_idx}
          label end_while_{while_idx}
        "
        );
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

    fn compile_subroutine_parameters(&mut self, parameters: Vec<Parameter>) {
        for parameter in parameters {
            let offset = if self.subroutine_kind == Some(SubroutineKind::Method) {
                self.subroutine_parameters.len() + 1
            } else {
                self.subroutine_parameters.len()
            };

            self.subroutine_parameters.insert(
                parameter.var_name,
                Symbol {
                    offset,
                    symbol_type: parameter.type_name,
                    kind: SymbolKind::Parameter,
                },
            );
        }
    }

    fn compile_subroutine(
        &mut self,
        subroutine: SubroutineDeclaration,
        instance_size: usize,
    ) -> String {
        self.clear_subroutine();
        self.subroutine_kind = Some(subroutine.subroutine_kind);

        self.compile_subroutine_parameters(subroutine.parameters);

        let implicit_return = if subroutine.return_type.is_none() {
            "push constant 0\nreturn"
        } else {
            "return"
        };

        let locals_count =
            self.compile_subroutine_var_declarations(subroutine.body.var_declarations);

        let compiled_statements = self.compile_statements(subroutine.body.statements);

        let class_name = self.get_class_name();
        let function_name = subroutine.name;

        match subroutine.subroutine_kind {
            SubroutineKind::Function => format!(
                "
                function {class_name}.{function_name} {locals_count}
                  {compiled_statements}
                  {implicit_return}
                  "
            ),
            SubroutineKind::Method => format!(
                "
                function {class_name}.{function_name} {locals_count}
                  push argument 0
                  pop pointer 0
                  {compiled_statements}
                  {implicit_return}
            "
            ),
            SubroutineKind::Constructor => {
                format!(
                    "
                function {class_name}.{function_name} {locals_count}
                  push constant {instance_size}
                  call Memory.alloc 1
                  pop pointer 0
                  {compiled_statements}
                  {implicit_return}
            "
                )
            }
        }
    }

    fn compile_subroutines(
        &mut self,
        subroutine_declarations: Vec<SubroutineDeclaration>,
        instance_size: usize,
    ) -> String {
        let parts: Vec<_> = subroutine_declarations
            .into_iter()
            .map(|subroutine| self.compile_subroutine(subroutine, instance_size))
            .collect();
        parts.join("\n")
    }

    fn compile_var_declarations(&mut self, var_declarations: Vec<ClassVarDeclaration>) -> usize {
        let mut instance_size = 0;
        for var_declaration in var_declarations {
            let (hashmap, symbol_kind) = match var_declaration.qualifier {
                ClassVarDeclarationKind::Static => (&mut self.class_statics, SymbolKind::Static),
                ClassVarDeclarationKind::Field => (&mut self.class_fields, SymbolKind::Field),
            };
            for var_name in var_declaration.var_names {
                if symbol_kind == SymbolKind::Field {
                    instance_size += 1;
                }

                hashmap.insert(
                    var_name,
                    Symbol {
                        offset: hashmap.len(),
                        symbol_type: var_declaration.type_name.clone(),
                        kind: symbol_kind.clone(),
                    },
                );
            }
        }

        instance_size
    }
}

pub fn generate_vm_code(class: Class) -> String {
    let mut code_generator = CodeGenerator::new();
    code_generator.class_name = Some(class.name.clone());
    let class_instance_size = code_generator.compile_var_declarations(class.var_declarations);
    code_generator.compile_subroutines(class.subroutine_declarations, class_instance_size)
}
