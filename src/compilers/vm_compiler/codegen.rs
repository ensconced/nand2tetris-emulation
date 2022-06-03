use std::{ffi::OsStr, iter};

use super::{
    parser::{
        ArithmeticCommandVariant::{self, *},
        BinaryArithmeticCommandVariant::*,
        Command::{self, *},
        FlowCommandVariant,
        FunctionCommandVariant::{self, *},
        MemoryCommandVariant::{self, *},
        MemorySegmentVariant::{self, *},
        OffsetSegmentVariant,
        PointerSegmentVariant::{self, *},
        UnaryArithmeticCommandVariant::*,
    },
    VMModule,
};

fn string_lines(source: &str) -> impl Iterator<Item = String> {
    let vec: Vec<String> = source
        .lines()
        .map(|line| line.trim_start().to_string())
        .collect();
    vec.into_iter()
}

fn prelude() -> impl Iterator<Item = String> {
    string_lines(
        "
      // This will be the very first instruction in the computer's ROM.
      // We don't want to go into an infinite loop quite yet, so skip over it!
      @$skip_infinite_loop
      0;JMP

      // This will be the return address of the main Sys.init function, so when
      // that function exits, the computer just goes into an infinite loop
      ($infinite_loop)
      @$infinite_loop
      0;JMP

      ($skip_infinite_loop)

      // For each stack frame, ARG points to the base of the frame. This is the
      // first stack frame, so here ARG points to the base of the entire stack.
      @256
      D=A
      @ARG
      M=D

      // Initialize the stack pointer. Even though there is no real caller
      // function for Sys.init, we leave the customary space for the saved LCL,
      // ARG, THIS and THAT of the caller. This in addition to the return
      // address means the stack pointer will start 5 addresses above the base
      // of the stack.
      @261
      D=A
      @SP
      M=D

      // LCL starts off pointing to the same address as the stack pointer.
      @261
      D=A
      @LCL
      M=D

      // Load the return address. Sys.init takes no arguments, so this is
      // located right at the base of the stack.
      @$infinite_loop
      D=A
      @256
      M=D

      // Call Sys.init
      @$entry_Sys.init
      0;JMP
    ",
    )
}

fn offset_address(segment: OffsetSegmentVariant, index: u16) -> u16 {
    let (segment_base_address, segment_top_address): (u16, u16) = match segment {
        OffsetSegmentVariant::Pointer => (3, 4),
        OffsetSegmentVariant::Temp => (5, 12),
    };
    let segment_max_index = segment_top_address - segment_base_address;
    if index > segment_max_index {
        panic!(
            "segment index {} is too high - max is {}",
            index, segment_max_index
        )
    }
    segment_base_address + index
}

fn push_from_d_register() -> impl Iterator<Item = String> {
    string_lines(
        "
    // Push from d register
    @SP
    A=M
    M=D
    @SP
    M=M+1
    ",
    )
}

fn pop_into_d_register(pointer: &str) -> impl Iterator<Item = String> {
    // pointer is usually going to be SP but occasionally we want to use a
    // different pointer to perform a pop-like operation
    string_lines(&format!(
        "
    // Pop into d register
    @{}
    MA=M-1
    D=M
    ",
        pointer
    ))
}

fn push_from_offset_memory_segment(segment: OffsetSegmentVariant, index: u16) -> Vec<String> {
    string_lines(&format!(
        "
        @{}
        D=M
        ",
        offset_address(segment, index)
    ))
    .chain(push_from_d_register())
    .collect()
}

fn pop_into_offset_memory_segment(segment: OffsetSegmentVariant, index: u16) -> Vec<String> {
    pop_into_d_register("SP")
        .chain(string_lines(&format!(
            "
        @{}
        M=D
        ",
            offset_address(segment, index)
        )))
        .collect()
}

fn push_from_pointer_memory_segment(segment: PointerSegmentVariant, index: u16) -> Vec<String> {
    let pointer_address = match segment {
        Argument => "ARG",
        Local => "LCL",
        This => "THIS",
        That => "THAT",
    };
    string_lines(&format!(
        "
        @{}
        D=A
        @{}
        A=M+D
        D=M
        ",
        index, pointer_address
    ))
    .chain(push_from_d_register())
    .collect()
}

fn pop_into_pointer_memory_segment(segment: PointerSegmentVariant, index: u16) -> Vec<String> {
    let pointer_address = match segment {
        Argument => "ARG",
        Local => "LCL",
        This => "THIS",
        That => "THAT",
    };
    pop_into_d_register("SP")
        .chain(string_lines(&format!(
            "
        // stash value from D into R13
        @R13
        M=D

        // put value of pointer in D
        @{}
        D=M

        // add index
        @{}
            D=D+A

            // stash memory address in R14
            @R14
            M=D

            // get value back into D
            @R13
            D=M

            // load value into memory
            @R14
            A=M
            M=D
            ",
            pointer_address, index,
        )))
        .collect()
}

fn push_from_constant(index: u16) -> Vec<String> {
    let max_constant = 32767;
    if index > max_constant {
        panic!("constant {} is bigger than max of {}", index, max_constant);
    }

    string_lines(&format!(
        "
            @{}
            D=A
            ",
        index
    ))
    .chain(push_from_d_register())
    .collect()
}

pub struct CodeGenerator {
    after_set_to_false_count: u32,
    return_address_count: u32,
    current_function: Option<String>,
}

impl CodeGenerator {
    pub fn new() -> Self {
        Self {
            after_set_to_false_count: 0,
            return_address_count: 0,
            current_function: None,
        }
    }

    fn pop_into_static_memory_segment(&self, index: u16, filename: &OsStr) -> Vec<String> {
        pop_into_d_register("SP")
            .chain(string_lines(&format!(
                "
            @{:?}.{}
            M=D
            ",
                filename, index
            )))
            .collect()
    }

    fn push_from_static(&self, index: u16, filename: &OsStr) -> Vec<String> {
        string_lines(&format!(
            "
            @{:?}.{}
            D=M
            ",
            filename, index
        ))
        .chain(push_from_d_register())
        .collect()
    }

    fn push(&self, segment: MemorySegmentVariant, index: u16, filename: &OsStr) -> Vec<String> {
        match segment {
            OffsetSegment(offset_segment) => push_from_offset_memory_segment(offset_segment, index),
            PointerSegment(pointer_segment) => {
                push_from_pointer_memory_segment(pointer_segment, index)
            }
            Static => self.push_from_static(index, filename),
            Constant => push_from_constant(index),
        }
    }

    fn pop(&self, segment: MemorySegmentVariant, index: u16, filename: &OsStr) -> Vec<String> {
        match segment {
            OffsetSegment(offset_segment) => pop_into_offset_memory_segment(offset_segment, index),
            PointerSegment(pointer_segment) => {
                pop_into_pointer_memory_segment(pointer_segment, index)
            }
            Static => self.pop_into_static_memory_segment(index, filename),
            Constant => {
                // popping into a constant doesn't make much sense - I guess it just
                // means decrement the SP but don't do anything with the popped
                // value
                string_lines(
                    "
                    @SP
                    M=M-1
                    ",
                )
                .collect()
            }
        }
    }

    fn compile_memory_command(
        &self,
        command: MemoryCommandVariant,
        filename: &OsStr,
    ) -> Vec<String> {
        match command {
            Push(segment, index) => self.push(segment, index, filename),
            Pop(segment, index) => self.pop(segment, index, filename),
        }
    }

    fn compile_arithmetic_command(&mut self, command: ArithmeticCommandVariant) -> Vec<String> {
        match command {
            Binary(variant) => match variant {
                Add => self.binary_operation("+"),
                And => self.binary_operation("&"),
                Or => self.binary_operation("|"),
                Sub => self.binary_operation("-"),
                Eq => self.comparative_operation("EQ"),
                Gt => self.comparative_operation("GT"),
                Lt => self.comparative_operation("LT"),
            },
            Unary(variant) => match variant {
                Neg => self.unary_operation("-"),
                Not => self.unary_operation("!"),
            },
        }
    }

    fn binary_operation(&self, operation: &str) -> Vec<String> {
        string_lines(&format!(
            "
            // decrement stack pointer, so it's pointing to y
            @SP
            M=M-1
            // load y into D
            A=M
            D=M
            // point A to x
            A=A-1
            M=M{}D
            ",
            operation
        ))
        .collect()
    }

    fn unary_operation(&self, operation: &str) -> Vec<String> {
        string_lines(&format!(
            "
            @SP
            A=M-1
            M={}M
            ",
            operation
        ))
        .collect()
    }

    fn comparative_operation(&mut self, operation: &str) -> Vec<String> {
        let jump_label = format!("$after_set_to_false_{}", self.after_set_to_false_count);
        self.after_set_to_false_count += 1;

        string_lines(&format!(
            "
            // decrement stack pointer, so it's pointing to y
            @SP
            M=M-1
            // set A to point to x
            A=M-1
            // use R13 as another pointer to x
            D=A
            @R13
            M=D
            // load y into D
            @SP
            A=M
            D=M
            // load x - y into D
            A=A-1
            D=M-D
            // initially set result to true (i.e. 0xffff i.e. -1)
            M=-1
            // then flip to false unless condition holds
            @{jump_label}
            D;J{operation}
            @R13
            A=M
            M=0
            ({jump_label})
            ",
            jump_label = jump_label,
            operation = operation,
        ))
        .collect()
    }

    fn compile_function_call(&mut self, function_name: String, arg_count: u16) -> Vec<String> {
        fn load_return_address_into_d(return_address_label: &str) -> impl Iterator<Item = String> {
            string_lines(&format!(
                "
                // Load return address into D
                @{}
                D=A
                ",
                return_address_label
            ))
        }

        fn save_caller_pointers() -> impl Iterator<Item = String> {
            vec!["LCL", "ARG", "THIS", "THAT"]
                .into_iter()
                .flat_map(|pointer| {
                    iter::once(format!("@{}", pointer))
                        .chain(iter::once("D=M".to_string()))
                        .chain(push_from_d_register())
                })
        }

        fn set_arg_pointer(arg_count: u16) -> impl Iterator<Item = String> {
            // At this point, all the arguments have been pushed to the stack,
            // plus the return address, plus the four saved caller pointers.
            // So to find the correct position for ARG, we can count 5 +
            // arg_count steps back from the stack pointer.
            let steps_back = 5 + arg_count;

            string_lines(&format!(
                "
                // Set arg pointer
                @SP
                D=M
                @{}
                D=D-A
                @ARG
                M=D
                ",
                steps_back
            ))
        }

        fn set_lcl_pointer() -> impl Iterator<Item = String> {
            string_lines(&format!(
                "
                // Set lcl pointer
                @SP
                D=M
                @LCL
                M=D
                ",
            ))
        }

        fn jump(function_name: &str, return_address_label: &str) -> impl Iterator<Item = String> {
            string_lines(&format!(
                "
                // Jump to the callee
                @$entry_{}
                0;JMP

                // Label for return to caller
                ({})
                ",
                function_name, return_address_label
            ))
        }

        let return_address_label = format!("$return_point_{}", self.return_address_count);
        self.return_address_count += 1;

        load_return_address_into_d(&return_address_label)
            .chain(push_from_d_register())
            .chain(save_caller_pointers())
            .chain(set_arg_pointer(arg_count))
            .chain(set_lcl_pointer())
            .chain(jump(&function_name, &return_address_label))
            .collect()
    }

    fn compile_function_definition(
        &mut self,
        function_name: String,
        local_var_count: u16,
    ) -> Vec<String> {
        fn initialize_locals(local_var_count: usize) -> impl Iterator<Item = String> {
            iter::repeat_with(|| iter::once("D=0".to_string()).chain(push_from_d_register()))
                .take(local_var_count)
                .flatten()
        }
        let result = iter::once(format!("($entry_{})", &function_name))
            .chain(initialize_locals(local_var_count as usize))
            .collect();

        self.current_function = Some(function_name);

        result
    }

    fn compile_function_return(&mut self) -> Vec<String> {
        // This is carefully designed to not require tracking of the number of
        // arguments or locals for the callee.

        // Use R13 as a copy of ARG. We'll use this when placing the return
        // value and restoring the stack pointer. We can't use ARG directly
        // because it's going to be overwritten when restoring the caller state.
        fn copy_arg_to_r13() -> impl Iterator<Item = String> {
            string_lines(
                "
                @ARG
                D=M
                @R13
                M=D
                ",
            )
        }

        // Use R14 as copy of LCL. We'll use this to pop all the caller state.
        // We can't use LCL directly because LCL is one of the pieces of we're
        // restoring, so we would end up overwriting our pointer part way
        // through the process. (If LCL was the last thing to be restored we
        // would be able to get away with this, but since we want to carry on to
        // also pop the return address, it doesn't work.)
        fn copy_lcl_to_r14() -> impl Iterator<Item = String> {
            string_lines(
                "
                @LCL
                D=M
                @R14
                M=D
                ",
            )
        }

        fn restore_caller_state() -> impl Iterator<Item = String> {
            pop_into_d_register("R14")
                .chain(string_lines(
                    "
            @THAT
            M=D
            ",
                ))
                .chain(pop_into_d_register("R14"))
                .chain(string_lines(
                    "
            @THIS
            M=D
            ",
                ))
                .chain(pop_into_d_register("R14"))
                .chain(string_lines(
                    "
            @ARG
            M=D
            ",
                ))
                .chain(pop_into_d_register("R14"))
                .chain(string_lines(
                    "
            @LCL
            M=D
            ",
                ))
        }

        fn stash_return_address_in_r14() -> impl Iterator<Item = String> {
            pop_into_d_register("R14").chain(string_lines(
                "
            @R14
            M=D
            ",
            ))
        }

        fn place_return_value() -> impl Iterator<Item = String> {
            pop_into_d_register("SP").chain(string_lines(
                "
            @R13
            A=M
            M=D
            ",
            ))
        }

        fn restore_stack_pointer() -> impl Iterator<Item = String> {
            string_lines(
                "
            @R13
            D=M
            @SP
            M=D+1
            ",
            )
        }

        fn goto_return_address() -> impl Iterator<Item = String> {
            string_lines(
                "
            @R14
            A=M
            0;JMP
            ",
            )
        }

        copy_arg_to_r13()
            .chain(copy_lcl_to_r14())
            .chain(restore_caller_state())
            .chain(stash_return_address_in_r14())
            .chain(place_return_value())
            .chain(restore_stack_pointer())
            .chain(goto_return_address())
            .collect()
    }

    fn compile_function_command(
        &mut self,
        function_command: FunctionCommandVariant,
    ) -> Vec<String> {
        match function_command {
            Call(function_name, arg_count) => self.compile_function_call(function_name, arg_count),
            Define(function_name, local_var_count) => {
                self.compile_function_definition(function_name, local_var_count)
            }
            ReturnFrom => self.compile_function_return(),
        }
    }

    fn compile_goto(&self, label: String) -> Vec<String> {
        if let Some(current_function) = &self.current_function {
            string_lines(&format!(
                "
            @{}${}
            0;JMP
            ",
                current_function, label
            ))
            .collect()
        } else {
            panic!(
                "not in a function definition while compiling goto label: {}",
                label
            )
        }
    }

    fn compile_label(&self, label: String) -> Vec<String> {
        if let Some(current_function) = &self.current_function {
            vec![format!("({}${})", current_function, label)]
        } else {
            panic!(
                "not in a function definition while compiling label: {}",
                label
            )
        }
    }

    fn compile_ifgoto(&self, label: String) -> Vec<String> {
        if let Some(current_function) = &self.current_function {
            pop_into_d_register("SP")
                .chain(string_lines(&format!(
                    "
                @{}${}
                D;JNE
                ",
                    current_function, label
                )))
                .collect()
        } else {
            panic!(
                "not in a function definition while compiling ifgoto label: {}",
                label
            )
        }
    }

    fn compile_flow_command(&mut self, flow_command: FlowCommandVariant) -> Vec<String> {
        match flow_command {
            FlowCommandVariant::GoTo(label) => self.compile_goto(label),
            FlowCommandVariant::IfGoTo(label) => self.compile_ifgoto(label),
            FlowCommandVariant::Label(label) => self.compile_label(label),
        }
    }

    fn compile_vm_command(&mut self, command: Command, filename: &OsStr) -> Vec<String> {
        match command {
            Arithmetic(arithmetic_command) => self.compile_arithmetic_command(arithmetic_command),
            Memory(memory_command) => self.compile_memory_command(memory_command, filename),
            Function(function_command) => self.compile_function_command(function_command),
            Flow(flow_command) => self.compile_flow_command(flow_command),
        }
    }
    pub fn generate_asm(mut self, vm_modules: Vec<VMModule>) -> String {
        let mut result = Vec::new();
        for vm_module in vm_modules {
            for command in vm_module.commands {
                for asm_instruction in self.compile_vm_command(command, vm_module.filename) {
                    result.push(asm_instruction);
                }
            }
        }
        let v: Vec<_> = prelude().chain(result.into_iter()).collect();
        v.join("\n")
    }
}
