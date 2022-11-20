#[cfg(test)]
pub mod test_utils {
    use crate::compile_to_machine_code;
    use crate::utils::source_modules::SourceModule;
    use emulator_core::computer::Computer;
    use std::collections::HashMap;
    use std::path::PathBuf;

    pub const INITIAL_STACK_POINTER_ADDRESS: u16 = 261;

    pub fn computer_from_jack_code(jack_code: HashMap<PathBuf, SourceModule>) -> Computer {
        Computer::new(compile_to_machine_code(jack_code).try_into().unwrap())
    }

    pub fn stack_pointer(computer: &Computer) -> u16 {
        computer.ram.lock()[0]
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
