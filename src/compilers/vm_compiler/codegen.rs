use std::intrinsics::offset;

use super::parser::{
    ArithmeticCommandVariant::{self, *},
    BinaryArithmeticCommandVariant::*,
    Command::{self, *},
    MemoryCommandVariant::{self, *},
    MemorySegmentVariant::{self, *},
    OffsetSegmentVariant, PointerSegmentVariant,
    UnaryArithmeticCommandVariant::*,
};

fn initialize_stack_pointer() -> impl Iterator<Item = String> {
    vec!["@SP", "M=256"].into_iter().map(|str| str.to_owned())
}

fn jump_end_spin() -> impl Iterator<Item = String> {
    vec!["@$end_spin", "0;JMP"]
        .into_iter()
        .map(|str| str.to_owned())
}

// fn push_return_address() -> impl Iterator<Item = String> {

//     vec![
//         // Push the return address to the stack
//         "@$begin_spin",
//         "D=A",
//         "@SP",
//         "M=M+1",
//         "A=M",
//         "M=D",
//         // - Save the caller's frame to the stack. In this case, there is no
//         //   caller, so we'll just set LCL, ARG, THIS and THAT all to zero.
//         // - reposition ARG pointer
//         // - reposition LCL pointer
//         // - jump to execute the code of the subroutine
//         "asdf",
//         "asdf",
//         "asdflkajsdf",
//         "asdlfkjsldf",
//         "asdlfkjsdf",
//     ]
//     .into_iter()
//     .map(|str| str.to_owned())

//     vec!["@SP", "M=256"].into_iter().map(|str| str.to_owned())
// }

fn prelude() -> impl Iterator<Item = String> {
    let commands = vec![Memory(Push(Constant, 0))];

    initialize_stack_pointer()
        .chain(jump_end_spin())
        .chain(push_return_address())
}
fn postlude() -> impl Iterator<Item = String> {
    vec![].into_iter()
}

struct CodeGenerator {
    current_function: Option<String>,
    after_set_to_false_count: u32,
}

impl CodeGenerator {
    fn new() -> Self {
        Self {
            current_function: None,
            after_set_to_false_count: 0,
        }
    }

    // fn compile_function_command(
    //     &self,
    //     command: FunctionCommandVariant,
    // ) -> impl Iterator<Item = String> {
    //     // TODO - if a function definition, need to set self.current_function -
    //     // ...then where do we use self.current function? when creating labels?
    //     todo!()
    // }

    fn offset_address(&self, segment: OffsetSegmentVariant, index: u16) -> u16 {
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

    fn push_from_memory_address(&self, address: u16) -> Vec<String> {
        format!(
            "
            @{}
            D=M
            @SP
            A=M
            M=D
            @SP
            M=M+1
            ",
            address
        )
        .lines()
        .map(|line| line.to_string())
        .collect()
    }

    fn compile_pointer_segment(&self, segment: PointerSegmentVariant, index: u16) -> Vec<String> {
        todo!()
    }

    fn compile_constant_segment(&self, index: u16) -> Vec<String> {
        todo!()
    }

    fn push(&self, segment: MemorySegmentVariant, index: u16) -> Vec<String> {
        match segment {
            OffsetSegment(offset_segment) => {
                self.push_from_memory_address(self.offset_address(offset_segment, index))
            }
            PointerSegment(pointer_segment) => self.compile_pointer_segment(pointer_segment, index),
            Constant => self.compile_constant_segment(index),
        }
    }

    fn pop(&self, segment: MemorySegmentVariant, index: u16) -> Vec<String> {
        todo!()
    }

    fn compile_memory_command(&self, command: MemoryCommandVariant) -> Vec<String> {
        match command {
            Push(segment, index) => self.push(segment, index),
            Pop(segment, index) => self.pop(segment, index),
        }
    }

    fn compile_arithmetic_command(&self, command: ArithmeticCommandVariant) -> Vec<String> {
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

    fn compile_vm_command(&self, command: Command) -> Vec<String> {
        match command {
            Arithmetic(variant) => self.compile_arithmetic_command(variant),
            Memory(variant) => self.compile_memory_command(variant),
            Flow(variant) => {
                todo!()
            }
            Function(variant) => {
                todo!()
            }
        }
    }

    fn compile_vm_commands(
        self,
        vm_commands: impl Iterator<Item = Command>,
    ) -> impl Iterator<Item = String> {
        vm_commands
            .map(move |command| self.compile_vm_command(command).into_iter())
            .flatten()
    }

    fn generate_asm(
        self,
        vm_commands: impl Iterator<Item = Command>,
    ) -> impl Iterator<Item = String> {
        prelude()
            .chain(self.compile_vm_commands(vm_commands))
            .chain(postlude())
    }
}
