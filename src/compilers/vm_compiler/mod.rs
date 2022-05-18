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
        Computer::new(generate_rom::from_string(assemble(
            compile_to_asm(vm_code.to_string()),
            config::ROM_DEPTH,
        )))
    }

    fn stack_pointer(computer: &Computer) -> i16 {
        computer.ram.lock().unwrap()[0]
    }

    fn expect_within_n_ticks(computer: &mut Computer, n: u32, predicate: &dyn Fn(i16) -> bool) {
        for _ in 0..=n {
            if predicate(stack_pointer(computer)) {
                return;
            }
            computer.tick();
        }
        panic!("predicate was not true within {} ticks", n);
    }

    #[test]
    fn test_push() {
        let mut computer = program_computer("push constant 1");
        assert_eq!(stack_pointer(&computer), 0);
        expect_within_n_ticks(&mut computer, 100, &|stack_pointer| stack_pointer == 256);
        expect_within_n_ticks(&mut computer, 100, &|stack_pointer| stack_pointer == 257);
    }
}
