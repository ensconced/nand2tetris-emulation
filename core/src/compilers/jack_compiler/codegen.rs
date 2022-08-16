use crate::compilers::vm_compiler::parser::{
    ArithmeticCommandVariant, BinaryArithmeticCommandVariant, Command, FlowCommandVariant,
    FunctionCommandVariant, MemoryCommandVariant, MemorySegmentVariant, OffsetSegmentVariant,
    PointerSegmentVariant, UnaryArithmeticCommandVariant,
};

use super::{
    jack_node_types::{
        BinaryOperator, Class, ClassVarDeclaration, ClassVarDeclarationKind, Expression,
        IndexedJackNode, Parameter, PrimitiveTermVariant, Statement, SubroutineCall,
        SubroutineDeclaration, SubroutineKind, Type, UnaryOperator, VarDeclaration,
    },
    sourcemap::VMCodegenSourceMap,
};
use std::{collections::HashMap, rc::Rc};

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

pub struct CodeGenerator {
    pub class_name: Option<String>,
    class_fields: HashMap<String, Symbol>,
    class_statics: HashMap<String, Symbol>,
    sourcemap: VMCodegenSourceMap,
    subroutine_while_count: usize,
    subroutine_if_count: usize,
    subroutine_parameters: HashMap<String, Symbol>,
    subroutine_vars: HashMap<String, Symbol>,
    subroutine_kind: Option<SubroutineKind>,
    vm_commands: Vec<Command>,
}

impl CodeGenerator {
    pub fn new() -> Self {
        CodeGenerator {
            class_name: None,
            class_fields: HashMap::new(),
            class_statics: HashMap::new(),
            sourcemap: VMCodegenSourceMap::new(),
            subroutine_while_count: 0,
            subroutine_if_count: 0,
            subroutine_parameters: HashMap::new(),
            subroutine_vars: HashMap::new(),
            subroutine_kind: None,
            vm_commands: Vec::new(),
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
        var_declarations: &[(Rc<VarDeclaration>, usize)],
    ) -> usize {
        let mut count = 0;
        for (var_declaration, _) in var_declarations {
            for var_name in var_declaration.var_names.iter() {
                count += 1;
                self.subroutine_vars.insert(
                    var_name.clone(),
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
    fn compile_do_statement(&mut self, subroutine_call: &(Rc<SubroutineCall>, usize)) {
        let pop_return_val =
            Command::Memory(MemoryCommandVariant::Pop(MemorySegmentVariant::Constant, 0));
        self.compile_subroutine_call_expression(subroutine_call);
        self.vm_commands.push(pop_return_val);
    }

    fn compile_let_statement(
        &mut self,
        var_name: &str,
        array_index: &Option<(Rc<Expression>, usize)>,
        value: &(Rc<Expression>, usize),
    ) {
        self.compile_expression(value);
        let (var_mem_segment, var_seg_idx) = self.compile_variable(var_name);

        if let Some(idx) = array_index {
            self.vm_commands
                .push(Command::Memory(MemoryCommandVariant::Push(
                    var_mem_segment,
                    var_seg_idx as u16,
                )));

            self.compile_expression(idx);

            let commands = vec![
                Command::Arithmetic(ArithmeticCommandVariant::Binary(
                    BinaryArithmeticCommandVariant::Add,
                )),
                Command::Memory(MemoryCommandVariant::Pop(
                    MemorySegmentVariant::OffsetSegment(OffsetSegmentVariant::Pointer),
                    1,
                )),
                Command::Memory(MemoryCommandVariant::Pop(
                    MemorySegmentVariant::PointerSegment(PointerSegmentVariant::That),
                    0,
                )),
            ];
            self.vm_commands.extend(commands.into_iter());
        } else {
            self.vm_commands
                .push(Command::Memory(MemoryCommandVariant::Pop(
                    var_mem_segment,
                    var_seg_idx as u16,
                )));
        }
    }

    fn compile_if_statement(
        &mut self,
        condition: &(Rc<Expression>, usize),
        if_statements: &[(Rc<Statement>, usize)],
        else_statements: &Option<Vec<(Rc<Statement>, usize)>>,
    ) {
        let if_count = self.subroutine_if_count;
        self.subroutine_if_count += 1;

        self.compile_expression(condition);

        self.vm_commands
            .push(Command::Flow(FlowCommandVariant::IfGoTo(format!(
                "if_statements_{}",
                if_count
            ))));

        if let Some(statements) = else_statements {
            self.compile_statements(statements)
        }

        let cmds = vec![
            Command::Flow(FlowCommandVariant::GoTo(format!("end_if_{}", if_count))),
            Command::Flow(FlowCommandVariant::Label(format!(
                "if_statements_{}",
                if_count
            ))),
        ];

        self.vm_commands.extend(cmds.into_iter());

        self.compile_statements(if_statements);

        self.vm_commands
            .push(Command::Flow(FlowCommandVariant::Label(format!(
                "end_if_{}",
                if_count
            ))));
    }

    fn compile_return_statement(&mut self, return_value: &Option<(Rc<Expression>, usize)>) {
        if let Some(expression) = return_value {
            self.compile_expression(expression);
        } else {
            self.vm_commands
                .push(Command::Memory(MemoryCommandVariant::Push(
                    MemorySegmentVariant::Constant,
                    0,
                )));
        };

        self.vm_commands
            .push(Command::Function(FunctionCommandVariant::ReturnFrom))
    }

    fn compile_array_access_expression(&mut self, var_name: &str, index: &(Rc<Expression>, usize)) {
        let (arr_mem_seg, arr_seg_idx) = self.compile_variable(var_name);

        self.vm_commands
            .push(Command::Memory(MemoryCommandVariant::Push(
                arr_mem_seg,
                arr_seg_idx as u16,
            )));

        self.compile_expression(index);

        self.vm_commands
            .push(Command::Arithmetic(ArithmeticCommandVariant::Binary(
                BinaryArithmeticCommandVariant::Add,
            )));

        let cmds = vec![
            Command::Memory(MemoryCommandVariant::Pop(
                MemorySegmentVariant::OffsetSegment(OffsetSegmentVariant::Pointer),
                1,
            )),
            Command::Memory(MemoryCommandVariant::Push(
                MemorySegmentVariant::PointerSegment(PointerSegmentVariant::That),
                0,
            )),
        ];

        self.vm_commands.extend(cmds.into_iter());
    }

    fn compile_binary_expression(
        &mut self,
        operator: &BinaryOperator,
        lhs: &(Rc<Expression>, usize),
        rhs: &(Rc<Expression>, usize),
    ) {
        self.compile_expression(lhs);
        self.compile_expression(rhs);

        let perform_op = match operator {
            BinaryOperator::And => vec![Command::Arithmetic(ArithmeticCommandVariant::Binary(
                BinaryArithmeticCommandVariant::And,
            ))],
            BinaryOperator::Divide => {
                vec![Command::Function(FunctionCommandVariant::Call(
                    "Math.divide".to_string(),
                    2,
                ))]
            }
            BinaryOperator::Equals => vec![Command::Arithmetic(ArithmeticCommandVariant::Binary(
                BinaryArithmeticCommandVariant::Eq,
            ))],
            BinaryOperator::GreaterThan => vec![Command::Arithmetic(
                ArithmeticCommandVariant::Binary(BinaryArithmeticCommandVariant::Gt),
            )],
            BinaryOperator::GreaterThanOrEquals => vec![
                Command::Arithmetic(ArithmeticCommandVariant::Binary(
                    BinaryArithmeticCommandVariant::Lt,
                )),
                Command::Arithmetic(ArithmeticCommandVariant::Unary(
                    UnaryArithmeticCommandVariant::Not,
                )),
            ],
            BinaryOperator::LessThan => vec![Command::Arithmetic(
                ArithmeticCommandVariant::Binary(BinaryArithmeticCommandVariant::Lt),
            )],
            BinaryOperator::LessThanOrEquals => vec![
                Command::Arithmetic(ArithmeticCommandVariant::Binary(
                    BinaryArithmeticCommandVariant::Gt,
                )),
                Command::Arithmetic(ArithmeticCommandVariant::Unary(
                    UnaryArithmeticCommandVariant::Not,
                )),
            ],
            BinaryOperator::Minus => vec![Command::Arithmetic(ArithmeticCommandVariant::Binary(
                BinaryArithmeticCommandVariant::Sub,
            ))],
            BinaryOperator::Multiply => vec![Command::Function(FunctionCommandVariant::Call(
                "Math.multiply".to_string(),
                2,
            ))],
            BinaryOperator::Or => vec![Command::Arithmetic(ArithmeticCommandVariant::Binary(
                BinaryArithmeticCommandVariant::Or,
            ))],
            BinaryOperator::Plus => vec![Command::Arithmetic(ArithmeticCommandVariant::Binary(
                BinaryArithmeticCommandVariant::Add,
            ))],
        };

        self.vm_commands.extend(perform_op.into_iter());
    }

    fn compile_primitive_term_expression(&mut self, primitive_term: &PrimitiveTermVariant) {
        let cmds = match primitive_term {
            PrimitiveTermVariant::False | PrimitiveTermVariant::Null => {
                vec![Command::Memory(MemoryCommandVariant::Push(
                    MemorySegmentVariant::Constant,
                    0,
                ))]
            }
            PrimitiveTermVariant::True => vec![
                Command::Memory(MemoryCommandVariant::Push(
                    MemorySegmentVariant::Constant,
                    0,
                )),
                Command::Arithmetic(ArithmeticCommandVariant::Unary(
                    UnaryArithmeticCommandVariant::Not,
                )),
            ],
            PrimitiveTermVariant::IntegerConstant(int_string) => {
                let val = int_string
                    .parse::<i16>()
                    .unwrap_or_else(|_| panic!("{} is not valid 16 bit int", int_string));
                vec![Command::Memory(MemoryCommandVariant::Push(
                    MemorySegmentVariant::Constant,
                    val as u16,
                ))]
            }
            PrimitiveTermVariant::This => {
                if let Some(SubroutineKind::Method | SubroutineKind::Constructor) =
                    self.subroutine_kind
                {
                    vec![Command::Memory(MemoryCommandVariant::Push(
                        MemorySegmentVariant::OffsetSegment(OffsetSegmentVariant::Pointer),
                        0,
                    ))]
                } else {
                    panic!("cannot use \"this\" outside of method or constructor")
                }
            }
            PrimitiveTermVariant::StringConstant(string) => {
                // Strings in Jack are represented in memory in utf16.
                let code_units: Vec<_> = string.encode_utf16().collect();

                let append_chars = code_units.iter().flat_map(|&code_unit| {
                    if code_unit & 32768 == 32768 {
                        // code_unit exceeds max size for A-instruction, so use
                        // this little trick.
                        vec![
                            Command::Memory(MemoryCommandVariant::Push(
                                MemorySegmentVariant::OffsetSegment(OffsetSegmentVariant::Temp),
                                0,
                            )),
                            Command::Memory(MemoryCommandVariant::Push(
                                MemorySegmentVariant::Constant,
                                !code_unit,
                            )),
                            Command::Arithmetic(ArithmeticCommandVariant::Unary(
                                UnaryArithmeticCommandVariant::Not,
                            )),
                            Command::Function(FunctionCommandVariant::Call(
                                "String.appendChar".to_string(),
                                2,
                            )),
                            Command::Memory(MemoryCommandVariant::Pop(
                                MemorySegmentVariant::Constant,
                                0,
                            )),
                        ]
                    } else {
                        vec![
                            Command::Memory(MemoryCommandVariant::Push(
                                MemorySegmentVariant::OffsetSegment(OffsetSegmentVariant::Temp),
                                0,
                            )),
                            Command::Memory(MemoryCommandVariant::Push(
                                MemorySegmentVariant::Constant,
                                code_unit,
                            )),
                            Command::Function(FunctionCommandVariant::Call(
                                "String.appendChar".to_string(),
                                2,
                            )),
                            Command::Memory(MemoryCommandVariant::Pop(
                                MemorySegmentVariant::Constant,
                                0,
                            )),
                        ]
                    }
                });

                let length = code_units.len();

                vec![
                    Command::Memory(MemoryCommandVariant::Push(
                        MemorySegmentVariant::Constant,
                        length as u16,
                    )),
                    Command::Function(FunctionCommandVariant::Call("String.new".to_string(), 1)),
                    Command::Memory(MemoryCommandVariant::Pop(
                        MemorySegmentVariant::OffsetSegment(OffsetSegmentVariant::Temp),
                        0,
                    )),
                ]
                .into_iter()
                .chain(append_chars)
                .chain(vec![Command::Memory(MemoryCommandVariant::Push(
                    MemorySegmentVariant::OffsetSegment(OffsetSegmentVariant::Temp),
                    0,
                ))])
                .collect()
            }
        };
        self.vm_commands.extend(cmds.into_iter());
    }

    fn compile_push_arguments(&mut self, arguments: &[(Rc<Expression>, usize)]) {
        for argument in arguments {
            self.compile_expression(argument);
        }
    }

    fn compile_method_subroutine_call_expression(
        &mut self,
        this_name: &str,
        method_name: &str,
        arguments: &[(Rc<Expression>, usize)],
    ) {
        let arg_count = arguments.len();

        if let Some(symbol) = self.maybe_resolve_symbol(this_name) {
            // Treat it as a method.
            match symbol.symbol_type.clone() {
                Type::ClassName(this_class) => {
                    let arg_count_with_this = arg_count + 1;
                    let (this_memory_segment, this_idx) = self.compile_variable(this_name);
                    self.vm_commands
                        .push(Command::Memory(MemoryCommandVariant::Push(
                            this_memory_segment,
                            this_idx as u16,
                        )));
                    self.compile_push_arguments(arguments);
                    self.vm_commands
                        .push(Command::Function(FunctionCommandVariant::Call(
                            format!("{}.{}", this_class, method_name),
                            arg_count_with_this as u16,
                        )));
                }
                other_type => panic!("cannot call method on {:?}", other_type),
            }
        } else {
            // Treat it as constructor or function. Could be on this class or on
            // a different class. These are not resolved by the jack compiler -
            // resolution happens later, in the vm compiler.
            self.compile_push_arguments(arguments);
            self.vm_commands
                .push(Command::Function(FunctionCommandVariant::Call(
                    format!("{}.{}", this_name, method_name),
                    arg_count as u16,
                )));
        }
    }

    fn compile_direct_subroutine_call_expression(
        &mut self,
        subroutine_name: &str,
        arguments: &Vec<(Rc<Expression>, usize)>,
    ) {
        let arg_count = arguments.len();
        let class_name = self.get_class_name().to_owned();
        self.compile_push_arguments(arguments);
        self.vm_commands
            .push(Command::Function(FunctionCommandVariant::Call(
                format!("{}.{}", class_name, subroutine_name),
                arg_count as u16,
            )));
    }

    fn compile_subroutine_call_expression(
        &mut self,
        subroutine_call: &(Rc<SubroutineCall>, usize),
    ) {
        match &*subroutine_call.0 {
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

    fn compile_unary_expression(
        &mut self,
        operator: &UnaryOperator,
        operand: &(Rc<Expression>, usize),
    ) {
        let perform_op = match operator {
            UnaryOperator::Minus => Command::Arithmetic(ArithmeticCommandVariant::Unary(
                UnaryArithmeticCommandVariant::Neg,
            )),
            UnaryOperator::Not => Command::Arithmetic(ArithmeticCommandVariant::Unary(
                UnaryArithmeticCommandVariant::Not,
            )),
        };
        self.compile_expression(operand);
        self.vm_commands.push(perform_op);
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

    fn compile_variable(&mut self, var_name: &str) -> (MemorySegmentVariant, usize) {
        let symbol = self.resolve_symbol(var_name);

        let symbol_kind = match symbol.kind {
            SymbolKind::Local => MemorySegmentVariant::PointerSegment(PointerSegmentVariant::Local),
            SymbolKind::Parameter => {
                MemorySegmentVariant::PointerSegment(PointerSegmentVariant::Argument)
            }
            SymbolKind::Field => MemorySegmentVariant::PointerSegment(PointerSegmentVariant::This),
            SymbolKind::Static => MemorySegmentVariant::Static,
        };

        (symbol_kind, symbol.offset)
    }

    fn compile_expression(&mut self, expression: &(Rc<Expression>, usize)) {
        match &*expression.0 {
            Expression::Parenthesized(expr) => self.compile_expression(expr),
            Expression::ArrayAccess { var_name, index } => {
                self.compile_array_access_expression(var_name, index)
            }
            Expression::Binary { operator, lhs, rhs } => {
                self.compile_binary_expression(operator, lhs, rhs)
            }
            Expression::PrimitiveTerm(primitive_term) => {
                self.compile_primitive_term_expression(primitive_term)
            }
            Expression::SubroutineCall(subroutine_call) => {
                self.compile_subroutine_call_expression(subroutine_call)
            }
            Expression::Unary { operator, operand } => {
                self.compile_unary_expression(operator, operand)
            }
            Expression::Variable(var_name) => {
                let (memory_segment, idx) = self.compile_variable(var_name);
                self.vm_commands
                    .push(Command::Memory(MemoryCommandVariant::Push(
                        memory_segment,
                        idx as u16,
                    )))
            }
        }
    }

    fn compile_statements(&mut self, statements: &[(Rc<Statement>, usize)]) {
        for statement in statements {
            self.compile_statement(statement);
        }
    }

    fn compile_while_statement(
        &mut self,
        condition: &(Rc<Expression>, usize),
        statements: &[(Rc<Statement>, usize)],
    ) {
        let while_idx = self.subroutine_while_count;
        self.subroutine_while_count += 1;

        self.vm_commands
            .push(Command::Flow(FlowCommandVariant::Label(format!(
                "start_while_{}",
                while_idx
            ))));

        self.compile_expression(condition);

        let cmds = vec![
            Command::Arithmetic(ArithmeticCommandVariant::Unary(
                UnaryArithmeticCommandVariant::Not,
            )),
            Command::Flow(FlowCommandVariant::IfGoTo(format!(
                "end_while_{}",
                while_idx
            ))),
        ];

        self.vm_commands.extend(cmds.into_iter());

        self.compile_statements(statements);

        let cmds = vec![
            Command::Flow(FlowCommandVariant::GoTo(format!(
                "start_while_{}",
                while_idx
            ))),
            Command::Flow(FlowCommandVariant::Label(format!(
                "end_while_{}",
                while_idx
            ))),
        ];
        self.vm_commands.extend(cmds.into_iter());
    }

    fn compile_statement(&mut self, statement: &(Rc<Statement>, usize)) {
        match &*statement.0 {
            Statement::Do(subroutine_call, jack_node_idx) => {
                self.compile_do_statement(&(subroutine_call.clone(), *jack_node_idx))
            }
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

    fn compile_subroutine_parameters(&mut self, parameters: &Vec<(Rc<Parameter>, usize)>) {
        for (parameter, _) in parameters {
            let offset = if self.subroutine_kind == Some(SubroutineKind::Method) {
                self.subroutine_parameters.len() + 1
            } else {
                self.subroutine_parameters.len()
            };

            self.subroutine_parameters.insert(
                parameter.var_name.clone(),
                Symbol {
                    offset,
                    symbol_type: parameter.type_name.clone(),
                    kind: SymbolKind::Parameter,
                },
            );
        }
    }

    fn implicit_return(&mut self, return_type: &Option<Type>) {
        let commands = if return_type.is_none() {
            vec![
                Command::Memory(MemoryCommandVariant::Push(
                    MemorySegmentVariant::Constant,
                    0,
                )),
                Command::Function(FunctionCommandVariant::ReturnFrom),
            ]
        } else {
            vec![Command::Function(FunctionCommandVariant::ReturnFrom)]
        };

        self.vm_commands.extend(commands.into_iter());
    }

    fn compile_subroutine(&mut self, subroutine: &SubroutineDeclaration, instance_size: usize) {
        self.clear_subroutine();
        self.subroutine_kind = Some(subroutine.subroutine_kind);

        self.compile_subroutine_parameters(&subroutine.parameters);

        let locals_count =
            self.compile_subroutine_var_declarations(&subroutine.body.0.var_declarations);

        let class_name = self.get_class_name();

        let commands = match subroutine.subroutine_kind {
            SubroutineKind::Function => vec![Command::Function(FunctionCommandVariant::Define(
                format!("{}.{}", class_name, subroutine.name),
                locals_count as u16,
            ))],
            SubroutineKind::Method => vec![
                Command::Function(FunctionCommandVariant::Define(
                    format!("{}.{}", class_name, subroutine.name),
                    locals_count as u16,
                )),
                Command::Memory(MemoryCommandVariant::Push(
                    MemorySegmentVariant::PointerSegment(PointerSegmentVariant::Argument),
                    0,
                )),
                Command::Memory(MemoryCommandVariant::Pop(
                    MemorySegmentVariant::OffsetSegment(OffsetSegmentVariant::Pointer),
                    0,
                )),
            ],
            SubroutineKind::Constructor => vec![
                Command::Function(FunctionCommandVariant::Define(
                    format!("{}.{}", class_name, subroutine.name),
                    locals_count as u16,
                )),
                Command::Memory(MemoryCommandVariant::Push(
                    MemorySegmentVariant::Constant,
                    instance_size as u16,
                )),
                Command::Function(FunctionCommandVariant::Call("Memory.alloc".to_string(), 1)),
                Command::Memory(MemoryCommandVariant::Pop(
                    MemorySegmentVariant::OffsetSegment(OffsetSegmentVariant::Pointer),
                    0,
                )),
            ],
        };

        self.vm_commands.extend(commands.into_iter());
        self.compile_statements(&subroutine.body.0.statements);
        self.implicit_return(&subroutine.return_type);
    }

    pub fn compile_subroutines(
        &mut self,
        subroutine_declarations: &[(Rc<SubroutineDeclaration>, usize)],
        instance_size: usize,
    ) {
        for (subroutine, _) in subroutine_declarations {
            self.compile_subroutine(subroutine, instance_size);
        }
    }

    pub fn compile_var_declarations(
        &mut self,
        var_declarations: &Vec<IndexedJackNode<ClassVarDeclaration>>,
    ) -> usize {
        let mut instance_size = 0;
        for var_declaration in var_declarations {
            let (hashmap, symbol_kind) = match *var_declaration.node.qualifier.0 {
                ClassVarDeclarationKind::Static => (&mut self.class_statics, SymbolKind::Static),
                ClassVarDeclarationKind::Field => (&mut self.class_fields, SymbolKind::Field),
            };
            for var_name in var_declaration.node.var_names.iter() {
                if symbol_kind == SymbolKind::Field {
                    instance_size += 1;
                }

                hashmap.insert(
                    var_name.clone(),
                    Symbol {
                        offset: hashmap.len(),
                        symbol_type: var_declaration.node.type_name.clone(),
                        kind: symbol_kind.clone(),
                    },
                );
            }
        }

        instance_size
    }
}

pub fn generate_vm_code(class: Rc<Class>) -> Vec<Command> {
    let mut code_generator = CodeGenerator::new();
    code_generator.class_name = Some(class.name.clone());
    let class_instance_size = code_generator.compile_var_declarations(&class.var_declarations);
    code_generator.compile_subroutines(&class.subroutine_declarations, class_instance_size);
    code_generator.vm_commands
}
