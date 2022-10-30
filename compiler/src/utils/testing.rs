#[cfg(test)]
pub mod test_utils {
    use crate::compile_to_machine_code;
    use crate::config::ROM_DEPTH;
    use crate::{assembler::assemble, utils::source_modules::SourceModule, vm_compiler};
    use emulator_core::computer::tick_until;
    use emulator_core::{computer::Computer, generate_rom};
    use std::collections::HashMap;
    use std::path::PathBuf;

    pub const INITIAL_STACK_POINTER_ADDRESS: u16 = 261;

    pub fn computer_from_vm_code(vm_code_sources: Vec<&str>) -> Computer {
        let source_modules: Vec<_> = vm_code_sources
            .into_iter()
            .enumerate()
            .map(|(idx, vm_code)| SourceModule {
                // All filenames have to be unique due to the way that static variables work!
                // TODO - just use the original filenames where they exist...
                filename: format!("some_filename_{idx}").into(),
                source: vm_code.to_owned(),
            })
            .collect();

        let parsed_vm_modules: HashMap<_, _> = source_modules
            .iter()
            .map(|source_module| (source_module.filename.clone(), vm_compiler::parse(source_module)))
            .collect();
        let asm = vm_compiler::codegen::generate_asm(&HashMap::new(), &parsed_vm_modules).instructions;
        let assembly_result = assemble(&asm, ROM_DEPTH);
        let machine_code_strings: Vec<_> = assembly_result
            .instructions
            .into_iter()
            .map(|instruction| format!("{:016b}", instruction))
            .collect();
        Computer::new(generate_rom::from_string(machine_code_strings.join("\n")))
    }

    pub fn computer_from_jack_code(jack_code: HashMap<PathBuf, SourceModule>) -> Computer {
        let source_modules: Vec<_> = jack_code.into_iter().map(|(_filename, source_module)| source_module).collect();
        Computer::new(compile_to_machine_code(source_modules).try_into().unwrap())
    }

    pub fn stack_pointer(computer: &Computer) -> u16 {
        computer.ram.lock()[0]
    }

    pub fn this(computer: &Computer, offset: usize) -> u16 {
        let pointer_to_this = pointer(computer, 0);
        let ram = computer.ram.lock();
        ram[pointer_to_this as usize + offset]
    }

    pub fn pointer(computer: &Computer, offset: usize) -> u16 {
        let ram = computer.ram.lock();
        ram[3 + offset]
    }

    pub fn static_variable(computer: &Computer, offset: usize) -> u16 {
        let ram = computer.ram.lock();
        ram[16 + offset]
    }

    pub fn nth_stack_value(computer: &Computer, n: usize) -> u16 {
        let ram = computer.ram.lock();
        ram[ram[0] as usize - (1 + n)]
    }

    pub fn peek_stack(computer: &Computer) -> u16 {
        nth_stack_value(computer, 0)
    }

    pub fn heap_includes(computer: &Computer, values: &[u16]) -> bool {
        let ram = computer.ram.lock();
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

    pub fn count_nonoverlapping_sequences_in_heap(computer: &Computer, needle: &[u16]) -> usize {
        let ram = computer.ram.lock();
        let heap = &ram[2048..18432];
        count_nonoverlapping_sequences(heap, needle)
    }

    pub fn heap_avail_list(computer: &Computer) -> HashMap<usize, Vec<u16>> {
        let mut result = HashMap::new();

        let ram = computer.ram.lock();
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
        let ram = computer.ram.lock();
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
        tick_until(computer, &|comp| frame_stack_depth(comp) == start_frame_depth - 1)
    }

    pub fn step_over(computer: &mut Computer) {
        step_in(computer);
        step_out(computer);
    }

    pub fn top_frame_local(computer: &Computer, local_idx: usize) -> u16 {
        let ram = computer.ram.lock();
        ram[ram[1] as usize + local_idx]
    }

    pub fn top_frame_arg(computer: &Computer, arg_idx: usize) -> u16 {
        let ram = computer.ram.lock();
        ram[ram[2] as usize + arg_idx]
    }

    pub fn string_from_pointer(computer: &Computer, pointer: u16) -> String {
        let ram = computer.ram.lock();
        let str_length = ram[pointer as usize + 2] as usize;
        let buffer_base = ram[pointer as usize] as usize;
        String::from_utf16(&ram[buffer_base..buffer_base + str_length]).unwrap()
    }

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
