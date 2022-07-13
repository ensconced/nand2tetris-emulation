use self::parser::parse;

use super::utils::source_modules::{get_source_modules, SourceModule};
use std::{
    fs,
    path::{Path, PathBuf},
};

mod codegen;
mod parser;
mod tokenizer;

pub fn compile(source: &str) -> String {
    codegen::generate_vm_code(parse(source))
}

fn compile_modules(modules: &[SourceModule]) -> Vec<String> {
    modules
        .iter()
        .map(|module| compile(&module.source))
        .collect()
}

pub fn compile_files(src_path: &Path, dest_path: &Path) {
    let source_modules = get_source_modules(src_path).expect("failed to get source modules");
    for source_module in source_modules {
        let vm_code = compile(&source_module.source);
        let mut module_dest_path = if source_module.entrypoint_is_dir {
            dest_path.join(source_module.filename)
        } else {
            PathBuf::from(dest_path)
        };
        module_dest_path.set_extension("vm");
        fs::write(module_dest_path, vm_code).expect("failed to write compilation output")
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, iter};

    use itertools::repeat_n;

    use crate::compilers::utils::testing::*;

    #[test]
    fn test_addition() {
        let mut computer = computer_from_jack_code(vec![
            "
            class Sys {
                function void init () {
                    var int a;
                    let a = 1000 + 1000 + 1000;
                }
            }
        ",
        ]);
        computer.tick_until(&|computer| stack_pointer(computer) == INITIAL_STACK_POINTER_ADDRESS);
        computer.tick_until(&|computer| peek_stack(computer) == 3000);
    }

    #[test]
    fn test_function_calling() {
        let mut computer = computer_from_jack_code(vec![
            "
            class Sys {
                function void init () {
                    return add(1000, 1000) + 3;
                }

                function int add(int a, int b) {
                    return a + b;
                }
            }
        ",
        ]);
        computer.tick_until(&|computer| stack_pointer(computer) == INITIAL_STACK_POINTER_ADDRESS);
        computer.tick_until(&|computer| peek_stack(computer) == 2000);
        computer.tick_until(&|computer| peek_stack(computer) == 2003);
    }

    #[test]
    fn test_fibonacci() {
        let mut computer = computer_from_jack_code(vec![
            "
            class Sys {
                function void init () {
                    return fibonacci(30);
                }

                function int fibonacci(int n) {
                    if (n = 0) {
                        return 0;
                    }
                    if (n = 1) {
                        return 1;
                    }
                    return fibonacci(n - 1) + fibonacci(n - 2);
                }
            }
        ",
        ]);
        computer.tick_until(&|computer| stack_pointer(computer) == INITIAL_STACK_POINTER_ADDRESS);
        computer.tick_until(&|computer| peek_stack(computer) == 28657);
    }

    #[test]
    fn test_sum_even_fibonaccis() {
        let mut computer = computer_from_jack_code(vec![
            "
            class Sys {
                function void init () {
                    do sum_even_fibonacci_numbers();
                }

                function void sum_even_fibonacci_numbers () {
                    var int sum, i, fib;
                    let sum = 0;
                    let i = 0;

                    while (i < 20) {
                        let fib = fibonacci(i);
                        if (is_even(fib)) {
                            let sum = sum + fib;
                        }
                        let i = i + 1;
                    }

                    return sum;
                }

                function int fibonacci(int n) {
                    if (n = 0) {
                        return 0;
                    }
                    if (n = 1) {
                        return 1;
                    }
                    return fibonacci(n - 1) + fibonacci(n - 2);
                }

                function bool is_even(int n) {
                    return (n & 1) = 0;
                }
            }
        ",
        ]);
        computer.tick_until(&|computer| stack_pointer(computer) == INITIAL_STACK_POINTER_ADDRESS);
        computer.tick_until(&|computer| peek_stack(computer) == 3382);
    }

    #[test]
    fn test_class_methods() {
        let mut computer = computer_from_jack_code(vec![
            "
            class Sys {
                function void init () {
                    var Rectangle rect;

                    do Memory.init();

                    let rect = Rectangle.new(4, 5);
                    do rect.perimeter();
                    return;
                }
            }
        ",
            "
            class Rectangle {
                field int width, height;

                constructor Rectangle new(int w, int h) {
                    let width = w;
                    let height = h;
                    return this;
                }

                method int perimeter() {
                    return width + width + height + height;
                }
            }
        ",
        ]);
        computer.tick_until(&|computer| stack_pointer(computer) == INITIAL_STACK_POINTER_ADDRESS);
        computer.tick_until(&|computer| peek_stack(computer) == 18);
    }

    #[test]
    fn test_multiplication() {
        let mut computer = computer_from_jack_code(vec![
            "
            class Sys {
                function void init () {
                    var int a, b, c, d;
                    let a = 333 * 83;
                    let b = 10 * -2;
                    let c = 3 * -3;
                    let c = -123 * -123;
                }
            }
        ",
        ]);
        computer.tick_until(&|computer| stack_pointer(computer) == INITIAL_STACK_POINTER_ADDRESS);
        computer.tick_until(&|computer| peek_stack(computer) == 333 * 83);
        computer.tick_until(&|computer| peek_stack(computer) == 10 * -2);
        computer.tick_until(&|computer| peek_stack(computer) == -123 * -123);
    }

    #[test]
    fn test_abs() {
        let mut computer = computer_from_jack_code(vec![
            "
            class Sys {
                function void init () {
                    var int a, b, c, d;
                    let a = Math.abs(1234) + Math.abs(-1234);
                    let b = Math.abs(-999) + Math.abs(999);
                }
            }
        ",
        ]);
        computer.tick_until(&|computer| stack_pointer(computer) == INITIAL_STACK_POINTER_ADDRESS);
        computer.tick_until(&|computer| peek_stack(computer) == 1234 + 1234);
        computer.tick_until(&|computer| peek_stack(computer) == 999 + 999);
    }

    #[test]
    fn test_division() {
        let mut computer = computer_from_jack_code(vec![
            "
            class Sys {
                function void init () {
                    var int a;

                    let a = 4191 / -3;
                    let a = 1234 / 123;
                    let a = -5198 / 182;
                    let a = 9099 / 33;
                }
            }
        ",
        ]);
        computer.tick_until(&|computer| stack_pointer(computer) == INITIAL_STACK_POINTER_ADDRESS);
        computer.tick_until(&|computer| peek_stack(computer) == 4191 / -3);
        computer.tick_until(&|computer| peek_stack(computer) == 1234 / 123);
        computer.tick_until(&|computer| peek_stack(computer) == -5198 / 182);
        computer.tick_until(&|computer| peek_stack(computer) == 9099 / 33);
    }

    #[test]
    fn test_sqrt() {
        let mut computer = computer_from_jack_code(vec![
            "
            class Sys {
                function void init () {
                    var int a, b, c, d;
                    let a = Math.sqrt(144);
                    let b = Math.sqrt(100);
                    let c = Math.sqrt(10000);
                    let d = Math.sqrt(14641);
                }
            }
        ",
        ]);
        computer.tick_until(&|computer| stack_pointer(computer) == INITIAL_STACK_POINTER_ADDRESS);
        computer.tick_until(&|computer| peek_stack(computer) == 12);
        computer.tick_until(&|computer| peek_stack(computer) == 10);
        computer.tick_until(&|computer| peek_stack(computer) == 100);
        computer.tick_until(&|computer| peek_stack(computer) == 121);
    }

    #[test]
    fn test_string_alloc() {
        let mut computer = computer_from_jack_code(vec![
            "
            class Sys {
                function void init () {
                    var String a;
                    do Memory.init();

                    let a = \"hello\";
                    do a.dispose();
                }
            }
            ",
        ]);
        computer.tick_until(&|computer| stack_pointer(computer) == INITIAL_STACK_POINTER_ADDRESS);
        let chars: Vec<_> = "hello".encode_utf16().map(|ch| ch as i16).collect();
        for char in chars.iter() {
            computer.tick_until(&|computer| peek_stack(computer) == *char);
        }
        computer.tick_until(&|computer| heap_includes(computer, &chars));
    }

    #[test]
    fn test_alloc_many_small_arrays() {
        let mut computer = computer_from_jack_code(vec![
            "
            class Sys {
                function void init () {
                    var String a;
                    var int inner_idx, outer_idx, val, count, arr, arr_length;
                    do Memory.init();

                    let arr_length = 1000;
                    let count = 14;
                    let outer_idx = 0;
                    let val = 0;


                    while (outer_idx < count) {
                        let arr = Memory.alloc(arr_length);
                        let inner_idx = 0;
                        while (inner_idx < arr_length) {
                            let arr[inner_idx] = val;
                            let inner_idx = inner_idx + 1;
                            let val = val + 1;
                        }
                        let outer_idx = outer_idx + 1;
                    }

                }
            }
            ",
        ]);
        computer.tick_until(&program_completed);
        for outer_idx in 0..14 {
            let start = outer_idx * 1000;
            let nums: Vec<_> = (start..start + 1000).into_iter().collect();
            assert!(heap_includes(&computer, &nums));
        }
    }

    #[test]
    fn test_memory_init() {
        let mut computer = computer_from_jack_code(vec![
            "
            class Sys {
                function void init () {
                    do Memory.init();
                }
            }
            ",
        ]);
        computer.tick_until(&program_completed);

        assert_eq!(
            heap_avail_list(&computer),
            vec![
                (4, vec![]),
                (8, vec![]),
                (16, vec![2064]),
                (32, vec![2080]),
                (64, vec![2112]),
                (128, vec![2176]),
                (256, vec![2304]),
                (512, vec![2560]),
                (1024, vec![3072]),
                (2048, vec![4096]),
                (4096, vec![6144]),
                (8192, vec![10240]),
                (16384, vec![]),
            ]
            .into_iter()
            .collect()
        );
    }

    #[test]
    fn test_memory_alloc_4_word_block() {
        let mut computer = computer_from_jack_code(vec![
            "
            class Sys {
                function void init () {
                    var int ptr;

                    do Memory.init();

                    // This should use a 4-word block.
                    let ptr = Memory.alloc(2);
                }
            }
            ",
        ]);
        computer.tick_until(&program_completed);

        // To generate a 4-word block, we have to split a 16-word block into 2
        // 8-word blocks, then split one of those again.

        assert_eq!(
            heap_avail_list(&computer),
            vec![
                (4, vec![2068]),
                (8, vec![2072]),
                (16, vec![]),
                (32, vec![2080]),
                (64, vec![2112]),
                (128, vec![2176]),
                (256, vec![2304]),
                (512, vec![2560]),
                (1024, vec![3072]),
                (2048, vec![4096]),
                (4096, vec![6144]),
                (8192, vec![10240]),
                (16384, vec![]),
            ]
            .into_iter()
            .collect()
        );
    }

    #[test]
    fn test_memory_alloc_dealloc_without_merge() {
        let mut computer = computer_from_jack_code(vec![
            "
            class Sys {
                function void init () {
                    var int ptr;

                    do Memory.init();

                    // This should use an existing 16-word block.
                    let ptr = Memory.alloc(12);
                    do Memory.deAlloc(ptr);
                }
            }
            ",
        ]);
        computer.tick_until(&program_completed);

        assert_eq!(
            heap_avail_list(&computer),
            vec![
                (4, vec![]),
                (8, vec![]),
                (16, vec![2064]),
                (32, vec![2080]),
                (64, vec![2112]),
                (128, vec![2176]),
                (256, vec![2304]),
                (512, vec![2560]),
                (1024, vec![3072]),
                (2048, vec![4096]),
                (4096, vec![6144]),
                (8192, vec![10240]),
                (16384, vec![]),
            ]
            .into_iter()
            .collect()
        );
    }

    #[test]
    fn test_memory_alloc_dealloc_with_single_merge() {
        let mut computer = computer_from_jack_code(vec![
            "
            class Sys {
                function void init () {
                    var int ptr;

                    do Memory.init();

                    // This should cause a 16-word block to be split into 2 8-word blocks.
                    let ptr = Memory.alloc(5);
                    do Memory.deAlloc(ptr);
                }
            }
            ",
        ]);
        computer.tick_until(&program_completed);

        assert_eq!(
            heap_avail_list(&computer),
            vec![
                (4, vec![]),
                (8, vec![]),
                (16, vec![2064]),
                (32, vec![2080]),
                (64, vec![2112]),
                (128, vec![2176]),
                (256, vec![2304]),
                (512, vec![2560]),
                (1024, vec![3072]),
                (2048, vec![4096]),
                (4096, vec![6144]),
                (8192, vec![10240]),
                (16384, vec![]),
            ]
            .into_iter()
            .collect()
        );
    }

    #[test]
    fn test_memory_alloc_dealloc_with_multiple_merges() {
        let mut computer = computer_from_jack_code(vec![
            "
            class Sys {
                function void init () {
                    var int ptr;

                    do Memory.init();

                    // This should cause a 4-word block to be formed by
                    // splitting a 16-word block into 8-word blocks, then
                    // splitting again.
                    let ptr = Memory.alloc(2);

                    // On deAlloc, the 4-word blocks should merge together, and
                    // then the 8-word blocks should merge back together too.
                    do Memory.deAlloc(ptr);
                }
            }
            ",
        ]);
        computer.tick_until(&program_completed);

        assert_eq!(
            heap_avail_list(&computer),
            vec![
                (4, vec![]),
                (8, vec![]),
                (16, vec![2064]),
                (32, vec![2080]),
                (64, vec![2112]),
                (128, vec![2176]),
                (256, vec![2304]),
                (512, vec![2560]),
                (1024, vec![3072]),
                (2048, vec![4096]),
                (4096, vec![6144]),
                (8192, vec![10240]),
                (16384, vec![]),
            ]
            .into_iter()
            .collect()
        );
    }

    #[test]
    fn test_memory_alloc_small_array_stress_test() {
        let mut computer = computer_from_jack_code(vec![
            "
            class Sys {
                function void init () {
                    var int nested_arr, array_count, array_length, rounds, arr, i, j, k;

                    do Memory.init();

                    let rounds = 1000;
                    let array_count = 150;
                    let array_length = 2;
                    let nested_arr = Memory.alloc(array_count);

                    let i = 0;
                    while (i < rounds) {
                        let j = 0;
                        while (j < array_count) {
                            let arr = Memory.alloc(array_length);
                            let nested_arr[j] = arr;
                            let k = 0;
                            while (k < array_length) {
                                let arr[k] = i;
                                let k = k + 1;
                            }
                            let j = j + 1;
                        }

                        let j = 0;
                        if (i < rounds - 1) {
                            while (j < array_count) {
                                do Memory.deAlloc(nested_arr[j]);
                                let j = j + 1;
                            }
                        }

                        let i = i + 1;
                    }

                }
            }
            ",
        ]);
        computer.tick_until(&program_completed);
        assert_eq!(
            count_nonoverlapping_sequences_in_heap(&computer, &[999, 2]),
            150
        );
    }
}
