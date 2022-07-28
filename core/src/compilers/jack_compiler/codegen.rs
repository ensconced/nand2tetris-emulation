use crate::compilers::vm_compiler::parser::{
    ArithmeticCommandVariant, BinaryArithmeticCommandVariant, Command, FlowCommandVariant,
    FunctionCommandVariant, MemoryCommandVariant, MemorySegmentVariant, OffsetSegmentVariant,
    PointerSegmentVariant, UnaryArithmeticCommandVariant,
};

use super::parser::{
    BinaryOperator, Class, ClassVarDeclaration, ClassVarDeclarationKindVariant, Expression,
    JackNode, Parameter, PrimitiveTermVariant, Statement, SubroutineCall, SubroutineDeclaration,
    SubroutineKind, Type, TypeVariant, UnaryOperator, VarDeclaration,
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
    symbol_type: TypeVariant,
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

fn implicit_return(return_type: &Option<Type>) -> Vec<Command> {
    if return_type.is_none() {
        vec![
            Command::Memory(MemoryCommandVariant::Push(
                MemorySegmentVariant::Constant,
                0,
            )),
            Command::Function(FunctionCommandVariant::ReturnFrom),
        ]
    } else {
        vec![Command::Function(FunctionCommandVariant::ReturnFrom)]
    }
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
        var_declarations: &Vec<VarDeclaration>,
    ) -> usize {
        let mut count = 0;
        for var_declaration in var_declarations {
            for var_name in var_declaration.var_names.names.iter() {
                count += 1;
                self.subroutine_vars.insert(
                    var_name.name.clone(),
                    Symbol {
                        offset: self.subroutine_vars.len(),
                        symbol_type: var_declaration.type_name.variant.clone(),
                        kind: SymbolKind::Local,
                    },
                );
            }
        }
        count
    }
    fn compile_do_statement(
        &mut self,
        subroutine_call: &SubroutineCall,
        origin_node: Rc<JackNode>,
    ) -> Vec<Command> {
        let pop_return_val =
            Command::Memory(MemoryCommandVariant::Pop(MemorySegmentVariant::Constant, 0));
        self.compile_subroutine_call_expression(subroutine_call)
            .into_iter()
            .chain(vec![pop_return_val].into_iter())
            .collect()
    }

    fn compile_let_statement(
        &mut self,
        var_name: &str,
        array_index: &Option<Expression>,
        value: &Expression,
        origin_node: Rc<JackNode>,
    ) -> Vec<Command> {
        let compiled_value = self.compile_expression(value);
        let (var_mem_segment, var_seg_idx) = self.compile_variable(&var_name);

        if let Some(idx) = array_index {
            let compiled_idx = self.compile_expression(idx);

            compiled_value
                .into_iter()
                .chain(vec![Command::Memory(MemoryCommandVariant::Push(
                    var_mem_segment,
                    var_seg_idx as u16,
                ))])
                .chain(compiled_idx.into_iter())
                .chain(
                    vec![
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
                    ]
                    .into_iter(),
                )
                .collect()
        } else {
            compiled_value
                .into_iter()
                .chain(
                    vec![Command::Memory(MemoryCommandVariant::Pop(
                        var_mem_segment,
                        var_seg_idx as u16,
                    ))]
                    .into_iter(),
                )
                .collect()
        }
    }

    fn compile_if_statement(
        &mut self,
        condition: &Expression,
        if_statements: &Vec<Statement>,
        else_statements: &Option<Vec<Statement>>,
        origin_node: Rc<JackNode>,
    ) -> Vec<Command> {
        let if_count = self.subroutine_if_count;
        self.subroutine_if_count += 1;

        let compiled_condition = self.compile_expression(condition);
        let compiled_if_statements = self.compile_statements(if_statements);
        let compiled_else_statements = if let Some(statements) = else_statements {
            self.compile_statements(statements)
        } else {
            vec![]
        };

        compiled_condition
            .into_iter()
            .chain(
                vec![Command::Flow(FlowCommandVariant::IfGoTo(format!(
                    "if_statements_{}",
                    if_count
                )))]
                .into_iter(),
            )
            .chain(compiled_else_statements.into_iter())
            .chain(
                vec![Command::Flow(FlowCommandVariant::GoTo(format!(
                    "end_if_{}",
                    if_count
                )))]
                .into_iter(),
            )
            .chain(
                vec![Command::Flow(FlowCommandVariant::Label(format!(
                    "if_statements_{}",
                    if_count
                )))]
                .into_iter(),
            )
            .chain(compiled_if_statements.into_iter())
            .chain(
                vec![Command::Flow(FlowCommandVariant::Label(format!(
                    "end_if_{}",
                    if_count
                )))]
                .into_iter(),
            )
            .collect()
    }

    fn compile_return_statement(
        &mut self,
        return_value: &Option<Expression>,
        origin_node: Rc<JackNode>,
    ) -> Vec<Command> {
        let push_return_value = if let Some(expression) = return_value {
            self.compile_expression(expression)
        } else {
            vec![Command::Memory(MemoryCommandVariant::Push(
                MemorySegmentVariant::Constant,
                0,
            ))]
        };

        push_return_value
            .into_iter()
            .chain(vec![Command::Function(FunctionCommandVariant::ReturnFrom)].into_iter())
            .collect()
    }

    fn compile_array_access_expression(
        &mut self,
        var_name: &str,
        index: &Expression,
        origin_node: Rc<JackNode>,
    ) -> Vec<Command> {
        let (arr_mem_seg, arr_seg_idx) = self.compile_variable(&var_name);
        let push_index = self.compile_expression(index);

        vec![Command::Memory(MemoryCommandVariant::Push(
            arr_mem_seg,
            arr_seg_idx as u16,
        ))]
        .into_iter()
        .chain(push_index.into_iter())
        .chain(
            vec![Command::Arithmetic(ArithmeticCommandVariant::Binary(
                BinaryArithmeticCommandVariant::Add,
            ))]
            .into_iter(),
        )
        .chain(
            vec![
                Command::Memory(MemoryCommandVariant::Pop(
                    MemorySegmentVariant::OffsetSegment(OffsetSegmentVariant::Pointer),
                    1,
                )),
                Command::Memory(MemoryCommandVariant::Push(
                    MemorySegmentVariant::PointerSegment(PointerSegmentVariant::That),
                    0,
                )),
            ]
            .into_iter(),
        )
        .collect()
    }

    fn compile_binary_expression(
        &mut self,
        operator: &BinaryOperator,
        lhs: &Expression,
        rhs: &Expression,
    ) -> Vec<Command> {
        let push_lhs = self.compile_expression(lhs);
        let push_rhs = self.compile_expression(rhs);

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

        let origin_node = Rc::new(JackNode {});

        push_lhs
            .into_iter()
            .chain(push_rhs.into_iter())
            .chain(perform_op.into_iter())
            .collect()
    }

    fn compile_primitive_term_expression(
        &mut self,
        primitive_term: &PrimitiveTermVariant,
    ) -> Vec<Command> {
        match primitive_term {
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

                let append_chars: Vec<_> = code_units
                    .iter()
                    .flat_map(|&code_unit| {
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
                    })
                    .collect();

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
                .chain(append_chars.into_iter())
                .chain(vec![Command::Memory(MemoryCommandVariant::Push(
                    MemorySegmentVariant::OffsetSegment(OffsetSegmentVariant::Temp),
                    0,
                ))])
                .collect()
            }
        }
    }

    fn compile_push_arguments(&mut self, arguments: &Vec<Expression>) -> Vec<Command> {
        arguments
            .into_iter()
            .map(|argument| self.compile_expression(argument))
            .flatten()
            .collect()
    }

    fn compile_method_subroutine_call_expression(
        &mut self,
        this_name: &str,
        method_name: &str,
        arguments: &Vec<Expression>,
        origin_node: Rc<JackNode>,
    ) -> Vec<Command> {
        let arg_count = arguments.len();
        let push_arguments = self.compile_push_arguments(arguments);

        if let Some(symbol) = self.maybe_resolve_symbol(&this_name) {
            // Treat it as a method.
            match symbol.symbol_type.clone() {
                TypeVariant::ClassName(this_class) => {
                    let arg_count_with_this = arg_count + 1;
                    let (this_memory_segment, this_idx) = self.compile_variable(&this_name);
                    vec![Command::Memory(MemoryCommandVariant::Push(
                        this_memory_segment,
                        this_idx as u16,
                    ))]
                    .into_iter()
                    .chain(push_arguments.into_iter())
                    .chain(vec![Command::Function(FunctionCommandVariant::Call(
                        format!("{}.{}", this_class.name, method_name),
                        arg_count_with_this as u16,
                    ))])
                    .collect()
                }
                other_type => panic!("cannot call method on {:?}", other_type),
            }
        } else {
            // Treat it as constructor or function. Could be on this class or on
            // a different class. These are not resolved by the jack compiler -
            // resolution happens later, in the vm compiler.
            push_arguments
                .into_iter()
                .chain(vec![Command::Function(FunctionCommandVariant::Call(
                    format!("{}.{}", this_name, method_name),
                    arg_count as u16,
                ))])
                .collect()
        }
    }

    fn compile_direct_subroutine_call_expression(
        &mut self,
        subroutine_name: &str,
        arguments: &Vec<Expression>,
        origin_node: Rc<JackNode>,
    ) -> Vec<Command> {
        let arg_count = arguments.len();
        let class_name = self.get_class_name().to_owned();
        let push_arguments = self.compile_push_arguments(arguments);
        push_arguments
            .into_iter()
            .chain(vec![Command::Function(FunctionCommandVariant::Call(
                format!("{}.{}", class_name, subroutine_name),
                arg_count as u16,
            ))])
            .collect()
    }

    fn compile_subroutine_call_expression(
        &mut self,
        subroutine_call: &SubroutineCall,
    ) -> Vec<Command> {
        let origin_node = Rc::new(JackNode {});

        match subroutine_call {
            SubroutineCall::Direct {
                subroutine_name,
                arguments,
            } => self.compile_direct_subroutine_call_expression(
                &subroutine_name.name,
                arguments,
                origin_node,
            ),
            SubroutineCall::Method {
                this_name,
                method_name,
                arguments,
            } => self.compile_method_subroutine_call_expression(
                &this_name.name,
                &method_name.name,
                arguments,
                origin_node,
            ),
        }
    }

    fn compile_unary_expression(
        &mut self,
        operator: &UnaryOperator,
        operand: &Expression,
    ) -> Vec<Command> {
        let perform_op = match operator {
            UnaryOperator::Minus => Command::Arithmetic(ArithmeticCommandVariant::Unary(
                UnaryArithmeticCommandVariant::Neg,
            )),
            UnaryOperator::Not => Command::Arithmetic(ArithmeticCommandVariant::Unary(
                UnaryArithmeticCommandVariant::Not,
            )),
        };
        let push_value = self.compile_expression(operand);
        push_value
            .into_iter()
            .chain(vec![perform_op].into_iter())
            .collect()
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

    fn compile_expression(&mut self, expression: &Expression) -> Vec<Command> {
        let origin_node = Rc::new(JackNode {});

        match expression {
            Expression::ArrayAccess { var_name, index } => {
                self.compile_array_access_expression(var_name, index, origin_node)
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
                let (memory_segment, idx) = self.compile_variable(&var_name);
                vec![Command::Memory(MemoryCommandVariant::Push(
                    memory_segment,
                    idx as u16,
                ))]
            }
        }
    }

    fn compile_statements(&mut self, statements: &Vec<Statement>) -> Vec<Command> {
        statements
            .into_iter()
            .flat_map(|statement| self.compile_statement(statement))
            .collect()
    }

    fn compile_while_statement(
        &mut self,
        condition: &Expression,
        statements: &Vec<Statement>,
        origin_node: Rc<JackNode>,
    ) -> Vec<Command> {
        let while_idx = self.subroutine_while_count;
        self.subroutine_while_count += 1;
        let condition = self.compile_expression(condition);
        let body = self.compile_statements(statements);

        vec![Command::Flow(FlowCommandVariant::Label(format!(
            "start_while_{}",
            while_idx
        )))]
        .into_iter()
        .chain(condition.into_iter())
        .chain(
            vec![
                Command::Arithmetic(ArithmeticCommandVariant::Unary(
                    UnaryArithmeticCommandVariant::Not,
                )),
                Command::Flow(FlowCommandVariant::IfGoTo(format!(
                    "end_while_{}",
                    while_idx
                ))),
            ]
            .into_iter(),
        )
        .chain(body.into_iter())
        .chain(
            vec![
                Command::Flow(FlowCommandVariant::GoTo(format!(
                    "start_while_{}",
                    while_idx
                ))),
                Command::Flow(FlowCommandVariant::Label(format!(
                    "end_while_{}",
                    while_idx
                ))),
            ]
            .into_iter(),
        )
        .collect()
    }

    fn compile_statement(&mut self, statement: &Statement) -> Vec<Command> {
        let origin_node = Rc::new(JackNode {});

        match statement {
            Statement::Do(subroutine_call) => {
                self.compile_do_statement(subroutine_call, origin_node)
            }
            Statement::Let {
                var_name,
                array_index,
                value,
            } => self.compile_let_statement(&var_name.name, array_index, value, origin_node),
            Statement::If {
                condition,
                if_statements,
                else_statements,
            } => self.compile_if_statement(condition, if_statements, else_statements, origin_node),
            Statement::Return(expression) => self.compile_return_statement(expression, origin_node),
            Statement::While {
                condition,
                statements,
            } => self.compile_while_statement(condition, statements, origin_node),
        }
    }

    fn compile_subroutine_parameters(&mut self, parameters: &Vec<Parameter>) {
        for parameter in parameters {
            let offset = if self.subroutine_kind == Some(SubroutineKind::Method) {
                self.subroutine_parameters.len() + 1
            } else {
                self.subroutine_parameters.len()
            };

            self.subroutine_parameters.insert(
                parameter.var_name.name.clone(),
                Symbol {
                    offset,
                    symbol_type: parameter.type_name.variant.clone(),
                    kind: SymbolKind::Parameter,
                },
            );
        }
    }

    fn compile_subroutine(
        &mut self,
        subroutine: &SubroutineDeclaration,
        instance_size: usize,
    ) -> Vec<Command> {
        self.clear_subroutine();
        self.subroutine_kind = Some(subroutine.subroutine_kind);

        self.compile_subroutine_parameters(&subroutine.parameters);

        let locals_count =
            self.compile_subroutine_var_declarations(&subroutine.body.var_declarations);

        let compiled_statements = self.compile_statements(&subroutine.body.statements);

        let class_name = self.get_class_name();

        let commands = match subroutine.subroutine_kind {
            SubroutineKind::Function => vec![Command::Function(FunctionCommandVariant::Define(
                format!("{}.{}", class_name, subroutine.name.name),
                locals_count as u16,
            ))],
            SubroutineKind::Method => vec![
                Command::Function(FunctionCommandVariant::Define(
                    format!("{}.{}", class_name, subroutine.name.name),
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
                    format!("{}.{}", class_name, subroutine.name.name),
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

        let implicit_return_commands = implicit_return(&subroutine.return_type);

        let origin_node = Rc::new(JackNode {});

        commands
            .into_iter()
            .chain(compiled_statements.into_iter())
            .chain(implicit_return_commands.into_iter())
            .collect()
    }

    fn compile_subroutines(
        &mut self,
        subroutine_declarations: &Vec<SubroutineDeclaration>,
        instance_size: usize,
    ) -> Vec<Command> {
        subroutine_declarations
            .into_iter()
            .flat_map(|subroutine| self.compile_subroutine(subroutine, instance_size))
            .collect()
    }

    fn compile_var_declarations(&mut self, var_declarations: &Vec<ClassVarDeclaration>) -> usize {
        let mut instance_size = 0;
        for var_declaration in var_declarations {
            let (hashmap, symbol_kind) = match var_declaration.qualifier.variant {
                ClassVarDeclarationKindVariant::Static => {
                    (&mut self.class_statics, SymbolKind::Static)
                }
                ClassVarDeclarationKindVariant::Field => {
                    (&mut self.class_fields, SymbolKind::Field)
                }
            };
            for var_name in var_declaration.var_names.names.iter() {
                if symbol_kind == SymbolKind::Field {
                    instance_size += 1;
                }

                hashmap.insert(
                    var_name.name.clone(),
                    Symbol {
                        offset: hashmap.len(),
                        symbol_type: var_declaration.type_name.variant.clone(),
                        kind: symbol_kind.clone(),
                    },
                );
            }
        }

        instance_size
    }
}

pub fn generate_vm_code(class: &Class) -> Vec<Command> {
    let mut code_generator = CodeGenerator::new();
    code_generator.class_name = Some(class.name.name.clone());
    let class_instance_size = code_generator.compile_var_declarations(&class.var_declarations);
    code_generator.compile_subroutines(&class.subroutine_declarations, class_instance_size)
}
