use crate::compilers::{
    assembler::assemble,
    vm_compiler::{codegen::CodeGenerator, VMModule},
};
use crate::{emulator::computer::Computer, emulator::config, emulator::generate_rom};
use std::path::Path;

pub fn program_computer(vm_code: &str) -> Computer {
    let vm_modules = vec![VMModule::new(
        Path::new("testpath").file_name().unwrap(),
        vm_code,
    )];
    let code_generator = CodeGenerator::new();
    let asm = code_generator.generate_asm(vm_modules);
    let machine_code = assemble(asm, config::ROM_DEPTH);
    Computer::new(generate_rom::from_string(machine_code))
}

pub fn stack_pointer(computer: &Computer) -> i16 {
    computer.ram.lock().unwrap()[0]
}

pub fn this(computer: &Computer, offset: usize) -> i16 {
    let pointer_to_this = pointer(computer, 0);
    let ram = computer.ram.lock().unwrap();
    ram[pointer_to_this as usize + offset]
}

pub fn pointer(computer: &Computer, offset: usize) -> i16 {
    let ram = computer.ram.lock().unwrap();
    ram[3 + offset]
}

pub fn static_variable(computer: &Computer, offset: usize) -> i16 {
    let ram = computer.ram.lock().unwrap();
    ram[16 + offset]
}

pub fn nth_stack_value(computer: &Computer, n: usize) -> i16 {
    let ram = computer.ram.lock().unwrap();
    ram[ram[0] as usize - (1 + n)]
}

pub const INITIAL_STACK_POINTER_ADDRESS: i16 = 261;
