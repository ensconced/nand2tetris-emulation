mod codegen;
mod parser;
mod tokenizer;

use codegen::CodeGenerator;
use parser::parse_lines;

pub fn compile_to_asm(vm_code: String) -> String {
    let vm_commands = parse_lines(&vm_code);
    let code_generator = CodeGenerator::new();
    code_generator.generate_asm(vm_commands)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::compilers::assembler::assemble;
    use crate::{computer::Computer, config, generate_rom};

    fn program_computer(vm_code: &str) -> Computer {
        let asm = compile_to_asm(vm_code.to_string());
        let machine_code = assemble(asm, config::ROM_DEPTH);
        Computer::new(generate_rom::from_string(machine_code))
    }

    fn stack_pointer(computer: &Computer) -> i16 {
        computer.ram.lock().unwrap()[0]
    }

    fn nth_stack_value(computer: &Computer, n: usize) -> i16 {
        let ram = computer.ram.lock().unwrap();
        ram[ram[0] as usize - (1 + n)]
    }

    #[test]
    fn test_stack_pointer_initialization() {
        let mut computer = program_computer("");
        computer.tick_until(&|computer| stack_pointer(computer) == 256);
    }

    #[test]
    fn test_push_constant() {
        let mut computer = program_computer("push constant 123");
        computer.tick_until(&|computer| {
            stack_pointer(computer) == 257 && nth_stack_value(computer, 0) == 123
        });
    }

    #[test]
    fn test_pop_push_static() {
        let mut computer = program_computer(
            "
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
            stack_pointer(computer) == 256 + 3 && nth_stack_value(&computer, 0) == 3
        });
        computer.tick_until(&|computer| {
            let all_popped = stack_pointer(computer) == 256;
            let ram = computer.ram.lock().unwrap();
            all_popped && ram[16] == 3 && ram[16 + 100] == 2 && ram[16 + 200] == 1
        });
        computer.tick_until(&|computer| {
            stack_pointer(computer) == 256 + 3
                && nth_stack_value(computer, 0) == 1
                && nth_stack_value(computer, 1) == 2
                && nth_stack_value(computer, 2) == 3
        });
    }
}
