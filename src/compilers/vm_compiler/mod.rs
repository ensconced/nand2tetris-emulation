mod codegen;
mod parser;
mod tokenizer;

use std::{ffi::OsStr, fs, io, path::Path};

use codegen::CodeGenerator;
use parser::parse_lines;

use self::parser::Command;

struct VMModule<'a> {
    filename: &'a OsStr,
    source: String,
    commands: Box<dyn Iterator<Item = Command> + 'a>,
}

impl<'a> VMModule<'a> {
    fn new(path: &'a Path) -> Self {
        let src = fs::read_to_string(path).expect("failed to read file to string");
        Self {
            filename: path
                .file_name()
                .expect("file name should not terminate in \"..\""),
            source: src,
            commands: Box::new(parse_lines(&src)),
        }
    }
}

pub fn compile(src_path: &Path, dest_path: &Path) -> Result<(), io::Error> {
    let vm_modules = if fs::metadata(src_path)?.is_dir() {
        fs::read_dir(src_path)?
            .flatten()
            .map(|entry| VMModule::new(&entry.path()))
            .collect()
    } else {
        vec![VMModule::new(&src_path)]
    };
    let code_generator = CodeGenerator::new();
    fs::write(dest_path, code_generator.generate_asm(vm_modules))
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::compilers::assembler::assemble;
    use crate::{computer::Computer, config, generate_rom};

    fn program_computer(vm_code: &str) -> Computer {
        let vm_modules = vec![VMModule::from_str(vm_code, Path::new("testpath"))];
        let code_generator = CodeGenerator::new();
        let asm = code_generator.generate_asm(vm_modules);
        let machine_code = assemble(asm, config::ROM_DEPTH);
        Computer::new(generate_rom::from_string(machine_code))
    }

    fn stack_pointer(computer: &Computer) -> i16 {
        computer.ram.lock().unwrap()[0]
    }

    fn this(computer: &Computer, offset: usize) -> i16 {
        let pointer_to_this = pointer(computer, 0);
        let ram = computer.ram.lock().unwrap();
        ram[pointer_to_this as usize + offset]
    }

    fn pointer(computer: &Computer, offset: usize) -> i16 {
        let ram = computer.ram.lock().unwrap();
        ram[3 + offset]
    }

    fn static_variable(computer: &Computer, offset: usize) -> i16 {
        let ram = computer.ram.lock().unwrap();
        ram[16 + offset]
    }

    fn nth_stack_value(computer: &Computer, n: usize) -> i16 {
        let ram = computer.ram.lock().unwrap();
        ram[ram[0] as usize - (1 + n)]
    }

    const INITIAL_STACK_POINTER_ADDRESS: i16 = 261;

    #[test]
    fn test_initialization() {
        let mut computer = program_computer("");
        computer.tick_until(&|computer| stack_pointer(computer) == INITIAL_STACK_POINTER_ADDRESS);
    }

    #[test]
    fn test_push_constant() {
        let mut computer = program_computer(
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
        let mut computer = program_computer(
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
                && nth_stack_value(&computer, 0) == 3
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
        let mut computer = program_computer(
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
                && nth_stack_value(&computer, 0) == 2051
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
        let mut computer = program_computer(
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
                && nth_stack_value(&computer, 0) == 3
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
        let mut computer = program_computer(
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
        computer.tick_until(&|computer| nth_stack_value(&computer, 0) == 3);
        // 1 + 2 + 3 should make 6
        computer.tick_until(&|computer| nth_stack_value(&computer, 0) == 6);
    }

    #[test]
    fn test_sys_init_with_local() {
        let mut computer = program_computer(
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
        computer.tick_until(&|computer| nth_stack_value(&computer, 0) == 3);
        // 1 + 2 + 3 should make 6
        computer.tick_until(&|computer| nth_stack_value(&computer, 0) == 6);
    }

    #[test]
    fn test_fibonacci() {
        let mut computer = program_computer(
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
        computer.tick_until(&|computer| nth_stack_value(&computer, 0) == 55);
    }
}
