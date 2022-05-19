use std::iter;

use super::parser::{
    ArithmeticCommandVariant::{self, *},
    BinaryArithmeticCommandVariant::*,
    Command::{self, *},
    FunctionCommandVariant,
    MemoryCommandVariant::{self, *},
    MemorySegmentVariant::{self, *},
    OffsetSegmentVariant,
    PointerSegmentVariant::{self, *},
    UnaryArithmeticCommandVariant::*,
};

fn string_lines(source: &str) -> impl Iterator<Item = String> + '_ {
    source.lines().map(|line| line.to_string())
}

fn initialize_stack_pointer() -> impl Iterator<Item = String> {
    vec!["@256", "D=A", "@SP", "M=D"]
        .into_iter()
        .map(|str| str.to_owned())
}

fn jump_end_spin() -> impl Iterator<Item = String> {
    vec!["@$end_spin", "0;JMP"]
        .into_iter()
        .map(|str| str.to_owned())
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
      @ARG
      M=256

      // Initialize the stack pointer. Even though there is no real caller
      // function for Sys.init, we leave the customary space for the saved LCL,
      // ARG, THIS and THAT of the caller. This in addition to the return
      // address means the stack pointer will start 5 addresses above the base
      // of the stack.
      @SP
      M=261

      // I'm assuming for now that Sys.init has no local variables. Therefore,
      // LCL starts off pointing to the same address as the stack pointer.
      @LCL
      M=261

      // Load the return address. Sys.init takes no arguments, so this is
      // located right at the base of the stack.
      @$infinite_loop
      D=A
      @256
      M=D
    ",
    )
}

fn offset_address(segment: OffsetSegmentVariant, index: u16) -> u16 {
    let (segment_base_address, segment_top_address): (u16, u16) = match segment {
        OffsetSegmentVariant::Pointer => (3, 4),
        OffsetSegmentVariant::Static => (16, 255),
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
    "
    @SP
    A=M
    M=D
    @SP
    M=M+1
    "
    .lines()
    .map(|line| line.to_string())
}

fn pop_into_d_register() -> impl Iterator<Item = String> {
    "
    @SP
    MA=M-1
    D=M
    "
    .lines()
    .map(|line| line.to_string())
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
    pop_into_d_register()
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
    pop_into_d_register()
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
fn push(segment: MemorySegmentVariant, index: u16) -> Vec<String> {
    match segment {
        OffsetSegment(offset_segment) => push_from_offset_memory_segment(offset_segment, index),
        PointerSegment(pointer_segment) => push_from_pointer_memory_segment(pointer_segment, index),
        Constant => push_from_constant(index),
    }
}
fn pop(segment: MemorySegmentVariant, index: u16) -> Vec<String> {
    match segment {
        OffsetSegment(offset_segment) => pop_into_offset_memory_segment(offset_segment, index),
        PointerSegment(pointer_segment) => pop_into_pointer_memory_segment(pointer_segment, index),
        Constant => {
            // popping into a constant doesn't make much sense - I guess it just
            // means decrement the SP but don't do anything with the popped
            // value
            vec!["@SP", "M=M-1"]
                .into_iter()
                .map(|str| str.to_string())
                .collect()
        }
    }
}

fn compile_memory_command(command: MemoryCommandVariant) -> Vec<String> {
    match command {
        Push(segment, index) => push(segment, index),
        Pop(segment, index) => pop(segment, index),
    }
}
pub struct CodeGenerator {
    current_function: Option<String>,
    after_set_to_false_count: u32,
}

impl CodeGenerator {
    pub fn new() -> Self {
        Self {
            current_function: None,
            after_set_to_false_count: 0,
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
        format!(
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
        )
        .lines()
        .map(|line| line.to_string())
        .collect()
    }

    fn unary_operation(&self, operation: &str) -> Vec<String> {
        format!(
            "
            @SP
            A=M-1
            M={}M
            ",
            operation
        )
        .lines()
        .map(|line| line.to_string())
        .collect()
    }

    fn comparative_operation(&mut self, operation: &str) -> Vec<String> {
        let jump_label = format!("$after_set_to_false_{}", self.after_set_to_false_count);
        self.after_set_to_false_count += 1;

        format!(
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
        )
        .lines()
        .map(|line| line.to_string())
        .collect()
    }

    fn compile_function_call(&mut self, function_name: String, arg_count: u16) -> Vec<String> {
        // TODO - check through all this...
        let save_caller_pointers =
            vec!["LCL", "ARG", "THIS", "THAT"]
                .into_iter()
                .flat_map(|pointer| {
                    vec![
                        pointer.to_string(),
                        "D=M".to_string(),
                        push_from_d_register().collect(),
                    ]
                    .into_iter()
                });

        // At this point, all the arguments have been pushed to the stack,
        // plus the return address, plus the four saved caller pointers.
        // So to find the correct position for ARG, we can count 5 +
        // arg_count steps back from the stack pointer.
        let steps_back = 5 + arg_count;

        let set_arg_pointer = format!(
            "
            @SP
            D=M
            @{}
            D=D-A
            @ARG
            M=D
        ",
            steps_back
        );

        let jump = format!(
            "
            @$entry_{function_name}
            0;JMP
            (@$return_address_{function_name}) // TODO - this needs to be different each time the function is called...
            ",
            function_name = function_name
        );

        let load_return_address_into_d = format!(
            "
            @$return_address_{} // TODO - function scoping
            D=A
            ",
            function_name
        );

        string_lines(&load_return_address_into_d)
            .chain(push_from_d_register())
            .chain(save_caller_pointers)
            .chain(string_lines(&set_arg_pointer))
            .chain(string_lines(&jump))
            .collect()
    }

    fn compile_function_definition(
        &mut self,
        function_name: String,
        local_var_count: u16,
    ) -> Vec<String> {
        todo!()
    }

    fn compile_function_return(&mut self) -> Vec<String> {
        todo!()
    }

    fn compile_function_command(
        &mut self,
        function_command: FunctionCommandVariant,
    ) -> Vec<String> {
        match function_command {
            FunctionCommandVariant::Call(function_name, arg_count) => {
                self.compile_function_call(function_name, arg_count)
            }
            FunctionCommandVariant::Define(function_name, local_var_count) => {
                self.compile_function_definition(function_name, local_var_count)
            }
            FunctionCommandVariant::ReturnFrom => self.compile_function_return(),
        }
    }

    fn compile_vm_command(&mut self, command: Command) -> Vec<String> {
        match command {
            Arithmetic(arithmetic_command) => self.compile_arithmetic_command(arithmetic_command),
            Memory(memory_command) => compile_memory_command(memory_command),
            Function(function_command) => self.compile_function_command(function_command),
            Flow(variant) => {
                todo!()
            }
        }
    }

    fn compile_vm_commands(
        mut self,
        vm_commands: impl Iterator<Item = Command>,
    ) -> impl Iterator<Item = String> {
        vm_commands
            .map(move |command| self.compile_vm_command(command).into_iter())
            .flatten()
    }

    pub fn generate_asm(self, vm_commands: impl Iterator<Item = Command>) -> String {
        let vec: Vec<String> = prelude()
            .chain(self.compile_vm_commands(vm_commands))
            .collect();
        vec.join("\n")
    }
}
