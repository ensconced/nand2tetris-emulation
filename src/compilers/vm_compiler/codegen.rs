use std::thread::current;

use super::parser::{
    ArithmeticCommandVariant,
    Command::{self, *},
    FlowCommandVariant,
    FunctionCommandVariant::{self, *},
    MemoryCommandVariant::{self, *},
    MemorySegmentVariant::*,
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
}

impl CodeGenerator {
    fn new() -> Self {
        Self {
            current_function: None,
        }
    }

    fn compile_function_command(&self, command: FunctionCommandVariant) -> String {
        // TODO - if a function definition, need to set self.current_function
        todo!()
    }

    fn compile_arithmetic_command(&self, command: ArithmeticCommandVariant) -> String {
        todo!()
    }

    fn compile_flow_command(&self, command: FlowCommandVariant) -> String {
        todo!()
    }

    fn compile_memory_command(&self, command: MemoryCommandVariant) -> String {
        todo!()
    }

    fn compile_vm_command(&self, command: Command) -> String {
        match command {
            Function(variant) => self.compile_function_command(variant),
            Arithmetic(variant) => self.compile_arithmetic_command(variant),
            Flow(variant) => self.compile_flow_command(variant),
            Memory(variant) => self.compile_memory_command(variant),
        }
    }

    fn compile_vm_commands(
        self,
        vm_commands: impl Iterator<Item = Command>,
    ) -> impl Iterator<Item = String> {
        vm_commands.map(move |command| self.compile_vm_command(command))
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
