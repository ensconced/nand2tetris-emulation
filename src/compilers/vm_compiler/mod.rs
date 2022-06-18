mod codegen;
mod parser;
mod tokenizer;

use std::{ffi::OsStr, fs, io, path::Path};

use super::utils::source_modules::get_source_modules;
pub use codegen::CodeGenerator;
use parser::{parse_lines, Command};

pub struct ParsedModule<'a> {
    filename: &'a OsStr,
    commands: Box<dyn Iterator<Item = Command> + 'a>,
}

impl<'a> ParsedModule<'a> {
    pub fn new(filename: &'a OsStr, source: &'a str) -> Self {
        Self {
            filename,
            commands: Box::new(parse_lines(source)),
        }
    }
}

pub fn compile(src_path: &Path, dest_path: &Path) -> Result<(), io::Error> {
    let source_modules = get_source_modules(src_path)?;
    let vm_modules: Vec<_> = source_modules
        .iter()
        .map(|source_module| ParsedModule::new(&source_module.filename, &source_module.source))
        .collect();
    let code_generator = CodeGenerator::new();
    fs::write(dest_path, code_generator.generate_asm(vm_modules))
}

#[cfg(test)]
mod tests {
    use crate::compilers::utils::testing::*;

    #[test]
    fn test_initialization() {
        let mut computer = computer_from_vm_code("");
        computer.tick_until(&|computer| stack_pointer(computer) == INITIAL_STACK_POINTER_ADDRESS);
    }

    #[test]
    fn test_push_constant() {
        let mut computer = computer_from_vm_code(
            "
        function Sys.init 0
        push constant 123
        ",
        );
        computer.tick_until(&|computer| {
            stack_pointer(computer) == INITIAL_STACK_POINTER_ADDRESS + 1
                && nth_stack_value(computer, 0) == 123
        });
    }

    #[test]
    fn test_pop_push_static() {
        let mut computer = computer_from_vm_code(
            "
            function Sys.init 0
            push constant 1
            push constant 2
            push constant 3
            pop static 0
            pop static 100
            pop static 200
            push static 0
            push static 100
            push static 200
        ",
        );
        computer.tick_until(&|computer| {
            stack_pointer(computer) == INITIAL_STACK_POINTER_ADDRESS + 3
                && nth_stack_value(computer, 0) == 3
        });
        computer.tick_until(&|computer| {
            stack_pointer(computer) == INITIAL_STACK_POINTER_ADDRESS
                && static_variable(computer, 0) == 3
                && static_variable(computer, 1) == 2
                && static_variable(computer, 2) == 1
        });
        computer.tick_until(&|computer| {
            stack_pointer(computer) == INITIAL_STACK_POINTER_ADDRESS + 3
                && nth_stack_value(computer, 0) == 1
                && nth_stack_value(computer, 1) == 2
                && nth_stack_value(computer, 2) == 3
        });
    }

    #[test]
    fn test_pop_push_this() {
        let mut computer = computer_from_vm_code(
            "
            function Sys.init 0
            push constant 1234
            push constant 2051
            pop pointer 0
            pop this 2
            ",
        );
        computer.tick_until(&|computer| {
            stack_pointer(computer) == INITIAL_STACK_POINTER_ADDRESS + 2
                && nth_stack_value(computer, 0) == 2051
        });
        computer.tick_until(&|computer| {
            stack_pointer(computer) == INITIAL_STACK_POINTER_ADDRESS + 1
                && pointer(computer, 0) == 2051
        });
        computer.tick_until(&|computer| {
            stack_pointer(computer) == INITIAL_STACK_POINTER_ADDRESS && this(computer, 2) == 1234
        });
    }

    #[test]
    fn test_arithmetic() {
        let mut computer = computer_from_vm_code(
            "
            function Sys.init 0
            push constant 6
            push constant 2
            push constant 3
            push constant 5
            push constant 2
            push constant 3
            add
            eq
            pop constant 0
            add
            eq
            pop constant 0
            ",
        );
        computer.tick_until(&|computer| {
            stack_pointer(computer) == INITIAL_STACK_POINTER_ADDRESS + 6
                && nth_stack_value(computer, 0) == 3
        });
        computer.tick_until(&|computer| {
            stack_pointer(computer) == INITIAL_STACK_POINTER_ADDRESS + 5
                && nth_stack_value(computer, 0) == 5
        });
        computer.tick_until(&|computer| {
            stack_pointer(computer) == INITIAL_STACK_POINTER_ADDRESS + 4
                && nth_stack_value(computer, 0) == -1
        });
        computer.tick_until(&|computer| {
            stack_pointer(computer) == INITIAL_STACK_POINTER_ADDRESS + 3
                && nth_stack_value(computer, 0) == 3
        });
        computer.tick_until(&|computer| {
            stack_pointer(computer) == INITIAL_STACK_POINTER_ADDRESS + 2
                && nth_stack_value(computer, 0) == 5
        });
        computer.tick_until(&|computer| {
            stack_pointer(computer) == INITIAL_STACK_POINTER_ADDRESS + 1
                && nth_stack_value(computer, 0) == 0
        });
        computer.tick_until(&|computer| stack_pointer(computer) == INITIAL_STACK_POINTER_ADDRESS);
    }

    #[test]
    fn test_add_function() {
        let mut computer = computer_from_vm_code(
            "
            function somefile.add 0
            push argument 0
            push argument 1
            add
            return

            function Sys.init 0
            push constant 1
            push constant 2
            call somefile.add 2
            push constant 3
            call somefile.add 2
            ",
        );
        // initialize
        computer.tick_until(&|computer| stack_pointer(computer) == INITIAL_STACK_POINTER_ADDRESS);
        // push first arguments to stack
        computer.tick_until(&|computer| nth_stack_value(computer, 0) == 3);
        // 1 + 2 + 3 should make 6
        computer.tick_until(&|computer| nth_stack_value(computer, 0) == 6);
    }

    #[test]
    fn test_sys_init_with_local() {
        let mut computer = computer_from_vm_code(
            "
            function somefile.add 0
            push argument 0
            push argument 1
            add
            return

            function Sys.init 1
            push constant 1
            push constant 2
            call somefile.add 2
            pop local 0
            push constant 3
            push local 0
            call somefile.add 2
            ",
        );
        // initialize
        computer.tick_until(&|computer| stack_pointer(computer) == INITIAL_STACK_POINTER_ADDRESS);
        // push first arguments to stack
        computer.tick_until(&|computer| nth_stack_value(computer, 0) == 3);
        // 1 + 2 + 3 should make 6
        computer.tick_until(&|computer| nth_stack_value(computer, 0) == 6);
    }

    #[test]
    fn test_fibonacci() {
        let mut computer = computer_from_vm_code(
            "
            function somefile.add 0
            push argument 0
            push argument 1
            add
            return

            function somefile.fibonacci 0
            // if n == 0, return 0
            push argument 0
            push constant 0
            eq
            if-goto return_zero

            // if n == 1, return 1
            push argument 0
            push constant 1
            eq
            if-goto return_one

            // else, compute fibonacci(n - 1)
            push argument 0
            push constant 1
            sub // n - 1
            call somefile.fibonacci 1
            push argument 0
            push constant 2
            sub // n - 2
            call somefile.fibonacci 1
            add
            return

            label return_zero
            push constant 0
            return

            label return_one
            push constant 1
            return

            function Sys.init 0
            push constant 10
            call somefile.fibonacci 1
            ",
        );
        // initialize
        computer.tick_until(&|computer| stack_pointer(computer) == INITIAL_STACK_POINTER_ADDRESS);
        // 1 + 2 + 3 should make 6
        computer.tick_until(&|computer| nth_stack_value(computer, 0) == 55);
    }
}
