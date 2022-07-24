use std::collections::HashMap;
use std::ops::Deref;

use std::path::Path;

use crate::compilers::jack_compiler::codegen::generate_vm_code;
use crate::compilers::jack_compiler::parser::parse;
use crate::compilers::vm_compiler::parser::{Command, CommandWithOrigin};
use crate::compilers::vm_compiler::ParsedModule;
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

pub fn computer_from_vm_instructions(vm_command_sources: Vec<Vec<CommandWithOrigin>>) -> Computer {
    let parsed_vm_modules: Vec<_> = vm_command_sources
        .into_iter()
        .enumerate()
        .map(|(idx, commands)| ParsedModule {
            filename: format!("some_filename_{idx}").into(),
            commands: Box::new(commands.into_iter().map(|command| command.command)),
        })
        .collect();

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

    let jack_classes: Vec<_> = std_lib_source
        .iter()
        .map(|source| source.deref())
        .chain(jack_code.into_iter())
        .map(parse)
        .collect();

    computer_from_vm_instructions(jack_classes.iter().map(generate_vm_code).collect())
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
    let heap = &ram[2048..18432];
    (0..heap.len()).any(|heap_idx| heap.iter().skip(heap_idx).take(values.len()).eq(values))
}

fn count_nonoverlapping_sequences<T: PartialEq>(haystack: &[T], needle: &[T]) -> usize {
    let mut count = 0;
    let mut match_len = 0;
    for val in haystack {
        if *val == needle[match_len] {
            if match_len == needle.len() - 1 {
                count += 1;
                match_len = 0;
            } else {
                match_len += 1;
            }
        } else {
            match_len = 0;
        }
    }
    count
}

pub fn count_nonoverlapping_sequences_in_heap(computer: &Computer, needle: &[i16]) -> usize {
    let ram = computer.ram.lock().unwrap();
    let heap = &ram[2048..18432];
    count_nonoverlapping_sequences(heap, needle)
}

pub fn heap_avail_list(computer: &Computer) -> HashMap<usize, Vec<i16>> {
    let mut result = HashMap::new();

    let ram = computer.ram.lock().unwrap();
    let avail_list = &ram[2050..2050 + 13];
    for (idx, &list_head) in avail_list.iter().enumerate() {
        let mut current = list_head;
        let mut free_blocks = Vec::new();
        while current != 0 {
            free_blocks.push(current);
            current = ram[current as usize + 2];
        }
        result.insert(2_usize.pow(idx as u32 + 2), free_blocks);
    }
    result
}

pub fn program_completed(computer: &Computer) -> bool {
    computer.cpu.pc == 2
}

pub fn frame_stack_depth(computer: &Computer) -> usize {
    let mut result = 0;
    let ram = computer.ram.lock().unwrap();
    let mut lcl_ptr = ram[1];
    while lcl_ptr >= 256 {
        lcl_ptr = ram[lcl_ptr as usize - 4];
        result += 1;
    }
    result
}

pub fn step_in(computer: &mut Computer) {
    let start_frame_depth = frame_stack_depth(computer);
    tick_until(computer, &|comp| {
        let current_frame_depth = frame_stack_depth(comp);
        if current_frame_depth < start_frame_depth {
            panic!("returned from function without calling anything");
        }
        current_frame_depth == start_frame_depth + 1
    })
}

pub fn step_out(computer: &mut Computer) {
    let start_frame_depth = frame_stack_depth(computer);
    tick_until(computer, &|comp| {
        frame_stack_depth(comp) == start_frame_depth - 1
    })
}

pub fn step_over(computer: &mut Computer) {
    step_in(computer);
    step_out(computer);
}

pub fn top_frame_local(computer: &Computer, local_idx: usize) -> i16 {
    let ram = computer.ram.lock().unwrap();
    ram[ram[1] as usize + local_idx]
}

pub fn top_frame_arg(computer: &Computer, arg_idx: usize) -> i16 {
    let ram = computer.ram.lock().unwrap();
    ram[ram[2] as usize + arg_idx]
}

pub fn static_var(computer: &Computer, idx: usize) -> i16 {
    let ram = computer.ram.lock().unwrap();
    ram[16 + idx]
}

pub fn tick_until(computer: &mut Computer, predicate: &dyn Fn(&Computer) -> bool) {
    let max_ticks: usize = 10_000_000_000;
    for _ in 0..=max_ticks {
        if predicate(computer) {
            return;
        }
        computer.tick();
    }
    panic!("predicate was not true within {} ticks", max_ticks);
}

pub fn tick_within_stack_frame_until(
    computer: &mut Computer,
    predicate: &dyn Fn(&Computer) -> bool,
) {
    let initial_depth = frame_stack_depth(computer);
    let max_ticks: usize = 10_000_000_000;
    for _ in 0..=max_ticks {
        if predicate(computer) {
            return;
        }
        if frame_stack_depth(computer) < initial_depth {
            panic!("predicate was never fulfilled within the stack frame");
        }
        computer.tick();
    }
    panic!("predicate was not true within {} ticks", max_ticks);
}

pub fn string_from_pointer(computer: &Computer, pointer: i16) -> String {
    let ram = computer.ram.lock().unwrap();
    let str_length = ram[pointer as usize + 1] as usize;
    let buffer_base = ram[pointer as usize] as usize;
    let str_buffer = &ram[buffer_base..buffer_base + str_length];
    let u16_buffer: Vec<_> = str_buffer.iter().map(|&x| x as u16).collect();
    String::from_utf16(&u16_buffer).unwrap()
}

#[cfg(test)]
mod tests {
    use super::count_nonoverlapping_sequences;

    #[test]
    fn test_count_nonoverlapping_sequences() {
        let haystack = [1, 2, 3, 4, 1, 2, 3];
        let needle = [1, 2, 3];
        let result = count_nonoverlapping_sequences(&haystack, &needle);
        assert_eq!(result, 2);

        let haystack = [1, 2, 4, 3];
        let needle = [1, 2, 3];
        let result = count_nonoverlapping_sequences(&haystack, &needle);
        assert_eq!(result, 0);

        let haystack = [1, 2, 4, 3, 1, 2, 1, 2, 1, 2, 3, 1, 2];
        let needle = [1, 2, 3];
        let result = count_nonoverlapping_sequences(&haystack, &needle);
        assert_eq!(result, 1);
    }
}
