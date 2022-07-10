use std::ops::Deref;

use std::path::Path;

use crate::compilers::{
    assembler::assemble, jack_compiler, utils::source_modules::SourceModule, vm_compiler,
};
use crate::{emulator::computer::Computer, emulator::config, emulator::generate_rom};

use super::source_modules::get_source_modules;

pub const INITIAL_STACK_POINTER_ADDRESS: i16 = 261;

pub fn computer_from_vm_code(vm_code_sources: Vec<&str>) -> Computer {
    let source_modules: Vec<_> = vm_code_sources
        .into_iter()
        .enumerate()
        .map(|(idx, vm_code)| SourceModule {
            // All filenames have to be unique due to the way that static variables work!
            // TODO - just use the original filenames where they exist...
            filename: format!("some_filename_{idx}").into(),
            source: vm_code.to_owned(),
            entrypoint_is_dir: false,
        })
        .collect();

    let parsed_vm_modules: Vec<_> = source_modules.iter().map(vm_compiler::parse).collect();

    let asm = vm_compiler::codegen::generate_asm(parsed_vm_modules);
    let machine_code = assemble(asm, config::ROM_DEPTH);
    Computer::new(generate_rom::from_string(machine_code))
}

pub fn computer_from_jack_code(jack_code: Vec<&str>) -> Computer {
    let std_lib_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("std_lib");

    let std_lib_source: Vec<_> = get_source_modules(&std_lib_dir)
        .expect("failed to get stdlib modules")
        .into_iter()
        .map(|stdlib_module| stdlib_module.source)
        .collect();

    let vm_code: Vec<_> = std_lib_source
        .iter()
        .map(|source| source.deref())
        .chain(jack_code.into_iter())
        .map(jack_compiler::compile)
        .collect();

    computer_from_vm_code(vm_code.iter().map(|x| x.deref()).collect())
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

pub fn peek_stack(computer: &Computer) -> i16 {
    nth_stack_value(computer, 0)
}

pub fn statics_includes(computer: &Computer, value: i16) -> bool {
    (0..240).any(|static_idx| static_variable(computer, static_idx) == value)
}

pub fn heap_includes(computer: &Computer, values: &[i16]) -> bool {
    let ram = computer.ram.lock().unwrap();
    let heap = &ram[2048..16384];
    (0..heap.len()).any(|heap_idx| heap.iter().skip(heap_idx).take(values.len()).eq(values))
}

pub fn program_completed(computer: &Computer) -> bool {
    computer.cpu.pc == 2
}
