use serde::Serialize;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};
use ts_rs::TS;

use self::{
    codegen::generate_vm_code,
    parser::parse,
    sourcemap::JackCompilerSourceMap,
    tokenizer::{token_defs, TokenKind},
};
use super::{
    utils::{
        source_modules::{get_source_modules, SourceModule},
        tokenizer::{Token, Tokenizer},
    },
    vm_compiler::parser::Command,
};

pub mod codegen;
pub mod jack_node_types;
pub mod parser;
pub mod sourcemap;
pub mod tokenizer;

#[derive(Default, Serialize, TS)]
#[ts(export)]
#[ts(export_to = "../web/bindings/")]
pub struct JackCompilerResult {
    pub sourcemaps: HashMap<PathBuf, JackCompilerSourceMap>,
    pub tokens: HashMap<PathBuf, Vec<Token<TokenKind>>>,
    #[ts(type = "Record<string, Array<string>>")]
    pub commands: HashMap<PathBuf, Vec<Command>>,
}

fn compile_jack_mod(jack_source_module: SourceModule, result: &mut JackCompilerResult) {
    let tokens: Vec<_> = Tokenizer::new(token_defs()).tokenize(&jack_source_module.source);
    let parse_result = parse(&tokens);
    let codegen_result = generate_vm_code(parse_result.class);
    let sourcemap = JackCompilerSourceMap {
        parser_sourcemap: parse_result.sourcemap,
        codegen_sourcemap: codegen_result.sourcemap,
    };

    result.sourcemaps.insert(jack_source_module.filename.clone(), sourcemap);
    result.tokens.insert(jack_source_module.filename.clone(), tokens);
    result.commands.insert(jack_source_module.filename, codegen_result.commands);
}

pub fn compile_jack(user_code: HashMap<PathBuf, SourceModule>) -> JackCompilerResult {
    let std_lib_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../std_lib");
    let mut source_modules = get_source_modules(&std_lib_dir).expect("failed to get stdlib modules");

    let mut result = JackCompilerResult::default();

    source_modules.extend(user_code);
    for (_filename, module) in source_modules {
        compile_jack_mod(module, &mut result);
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::utils::{source_modules::mock_from_sources, testing::test_utils::*};
    use emulator_core::computer::tick_until;
    use itertools::repeat_n;

    #[test]
    fn test_addition() {
        let mut computer = computer_from_jack_code(mock_from_sources(vec![(
            "Sys.jack",
            "
            class Sys {
                function void init () {
                    var int a;
                    let a = 1000 + 1000 + 1000;
                }
            }
        ",
        )]));
        tick_until(&mut computer, &|computer| stack_pointer(computer) == INITIAL_STACK_POINTER_ADDRESS);
        tick_until(&mut computer, &|computer| peek_stack(computer) == 3000);
    }

    #[test]
    fn test_function_calling() {
        let mut computer = computer_from_jack_code(mock_from_sources(vec![(
            "Sys.jack",
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
        )]));
        tick_until(&mut computer, &|computer| stack_pointer(computer) == INITIAL_STACK_POINTER_ADDRESS);
        tick_until(&mut computer, &|computer| peek_stack(computer) == 2000);
        tick_until(&mut computer, &|computer| peek_stack(computer) == 2003);
    }

    #[test]
    fn test_fibonacci() {
        let mut computer = computer_from_jack_code(mock_from_sources(vec![(
            "Sys.jack",
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
        )]));
        tick_until(&mut computer, &|computer| stack_pointer(computer) == INITIAL_STACK_POINTER_ADDRESS);
        tick_until(&mut computer, &|computer| peek_stack(computer) == 28657);
    }

    #[test]
    fn test_sum_even_fibonaccis() {
        let mut computer = computer_from_jack_code(mock_from_sources(vec![(
            "Sys.jack",
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
        )]));
        tick_until(&mut computer, &|computer| stack_pointer(computer) == INITIAL_STACK_POINTER_ADDRESS);
        tick_until(&mut computer, &|computer| peek_stack(computer) == 3382);
    }

    #[test]
    fn test_class_methods() {
        let mut computer = computer_from_jack_code(mock_from_sources(vec![
            (
                "Sys.jack",
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
            ),
            (
                "Rectangle.jack",
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
            ),
        ]));
        tick_until(&mut computer, &|computer| stack_pointer(computer) == INITIAL_STACK_POINTER_ADDRESS);
        tick_until(&mut computer, &|computer| peek_stack(computer) == 18);
    }

    #[test]
    fn test_multiplication() {
        let mut computer = computer_from_jack_code(mock_from_sources(vec![(
            "Sys.jack",
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
        )]));
        tick_until(&mut computer, &|computer| stack_pointer(computer) == INITIAL_STACK_POINTER_ADDRESS);
        tick_until(&mut computer, &|computer| peek_stack(computer) == 333 * 83);
        tick_until(&mut computer, &|computer| peek_stack(computer) == (10 * -2_i16) as u16);
        tick_until(&mut computer, &|computer| peek_stack(computer) == (-123_i16 * -123_i16) as u16);
    }

    #[test]
    fn test_abs() {
        let mut computer = computer_from_jack_code(mock_from_sources(vec![(
            "Sys.jack",
            "
            class Sys {
                function void init () {
                    var int a, b, c, d;
                    let a = Math.abs(1234) + Math.abs(-1234);
                    let b = Math.abs(-999) + Math.abs(999);
                }
            }
        ",
        )]));
        tick_until(&mut computer, &|computer| stack_pointer(computer) == INITIAL_STACK_POINTER_ADDRESS);
        tick_until(&mut computer, &|computer| peek_stack(computer) == 1234 + 1234);
        tick_until(&mut computer, &|computer| peek_stack(computer) == 999 + 999);
    }

    #[test]
    fn test_division() {
        let mut computer = computer_from_jack_code(mock_from_sources(vec![(
            "Sys.jack",
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
        )]));
        tick_until(&mut computer, &|computer| stack_pointer(computer) == INITIAL_STACK_POINTER_ADDRESS);
        tick_until(&mut computer, &|computer| peek_stack(computer) == (4191 / -3_i16) as u16);
        tick_until(&mut computer, &|computer| peek_stack(computer) == 1234 / 123);
        tick_until(&mut computer, &|computer| peek_stack(computer) == (-5198_i16 / 182) as u16);
        tick_until(&mut computer, &|computer| peek_stack(computer) == 9099 / 33);
    }

    #[test]
    fn test_sqrt() {
        let mut computer = computer_from_jack_code(mock_from_sources(vec![(
            "Sys.jack",
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
        )]));
        tick_until(&mut computer, &|computer| stack_pointer(computer) == INITIAL_STACK_POINTER_ADDRESS);
        tick_until(&mut computer, &|computer| peek_stack(computer) == 12);
        tick_until(&mut computer, &|computer| peek_stack(computer) == 10);
        tick_until(&mut computer, &|computer| peek_stack(computer) == 100);
        tick_until(&mut computer, &|computer| peek_stack(computer) == 121);
    }

    #[test]
    fn test_string_alloc() {
        let mut computer = computer_from_jack_code(mock_from_sources(vec![(
            "Sys.jack",
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
        )]));
        tick_until(&mut computer, &|computer| stack_pointer(computer) == INITIAL_STACK_POINTER_ADDRESS);
        let chars: Vec<_> = "hello".encode_utf16().collect();
        for char in chars.iter() {
            tick_until(&mut computer, &|computer| peek_stack(computer) == *char);
        }
        tick_until(&mut computer, &|computer| heap_includes(computer, &chars));
    }

    #[test]
    fn test_alloc_many_small_arrays() {
        let mut computer = computer_from_jack_code(mock_from_sources(vec![(
            "Sys.jack",
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
        )]));
        tick_until(&mut computer, &program_completed);
        for outer_idx in 0..14 {
            let start = outer_idx * 1000;
            let nums: Vec<_> = (start..start + 1000).into_iter().collect();
            assert!(heap_includes(&computer, &nums));
        }
    }

    #[test]
    fn test_memory_init() {
        let mut computer = computer_from_jack_code(mock_from_sources(vec![(
            "Sys.jack",
            "
            class Sys {
                function void init () {
                    do Memory.init();
                }
            }
            ",
        )]));
        tick_until(&mut computer, &program_completed);

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
        let mut computer = computer_from_jack_code(mock_from_sources(vec![(
            "Sys.jack",
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
        )]));
        tick_until(&mut computer, &program_completed);

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
        let mut computer = computer_from_jack_code(mock_from_sources(vec![(
            "Sys.jack",
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
        )]));
        tick_until(&mut computer, &program_completed);

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
        let mut computer = computer_from_jack_code(mock_from_sources(vec![(
            "Sys.jack",
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
        )]));
        tick_until(&mut computer, &program_completed);

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
        let mut computer = computer_from_jack_code(mock_from_sources(vec![(
            "Sys.jack",
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
        )]));
        tick_until(&mut computer, &program_completed);

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
        let mut computer = computer_from_jack_code(mock_from_sources(vec![(
            "Sys.jack",
            "
            class Sys {
                function void init () {
                    var int i, j, ptr;

                    do Memory.init();

                    while (true) {
                        let ptr = Memory.alloc(20);
                        if (ptr) {
                            let j = 0;
                            while (j < 20) {
                                let ptr[j] = 1234;
                                let j = j + 1;
                            }
                        } else {
                            return;
                        }
                        let i = i + 1;
                    }
                }
            }
            ",
        )]));
        // We should be able to allocate 511 arrays. This means the variable i should reach 511.
        tick_until(&mut computer, &|computer| {
            frame_stack_depth(computer) == 1 && top_frame_local(computer, 0) == 511
        });
        tick_until(&mut computer, &program_completed);
    }

    #[test]
    fn test_memory_alloc_dealloc_small_array_stress_test() {
        let mut computer = computer_from_jack_code(mock_from_sources(vec![(
            "Sys.jack",
            "
            class Sys {
                static int ptrs, ptr_count;

                function void alloc_arrays(int fill_value) {
                    var int i, j, ptr;

                    let i = 0;
                    while (i < ptr_count) {
                        let ptr = Memory.alloc(20);
                        let ptrs[i] = ptr;
                        let j = 0;
                        while (j < 20) {
                            let ptr[j] = fill_value;
                            let j = j + 1;
                        }
                        let i = i + 1;
                    }
                }

                function void dealloc_arrays() {
                    var int i;

                    let i = 0;

                    while (i < ptr_count) {
                        do Memory.deAlloc(ptrs[i]);
                        let i = i + 1;
                    }
                }

                function void init () {
                    var int ptr, i, rounds;

                    let ptr_count = 495;
                    do Memory.init();

                    let ptrs = Memory.alloc(ptr_count);

                    let rounds = 10;
                    let i = 0;
                    while (i < rounds) {
                        do alloc_arrays(i);
                        do dealloc_arrays();
                        let i = i + 1;
                    }

                    do Memory.deAlloc(ptrs);
                }
            }
            ",
        )]));

        // init stuff
        tick_until(&mut computer, &|computer| frame_stack_depth(computer) == 1);

        // step over Memory.init call
        step_over(&mut computer);

        // step over Memory.alloc call for ptrs
        step_over(&mut computer);

        for _ in 0..9 {
            // step over alloc_arrays call
            step_over(&mut computer);
            // step over dealloc_arrays call
            step_over(&mut computer);
        }

        // step over final alloc_arrays call
        step_over(&mut computer);

        // check arrays were properly allocated
        let sequence: Vec<_> = repeat_n(9, 20).collect();
        assert_eq!(count_nonoverlapping_sequences_in_heap(&computer, &sequence), 495);

        // The heap should be completely full, apart from one remaining 16-word
        // block which we don't have any use for.
        assert_eq!(
            heap_avail_list(&computer),
            vec![
                (4, vec![]),
                (8, vec![]),
                (16, vec![2064]),
                (32, vec![]),
                (64, vec![]),
                (128, vec![]),
                (256, vec![]),
                (512, vec![]),
                (1024, vec![]),
                (2048, vec![]),
                (4096, vec![]),
                (8192, vec![]),
                (16384, vec![]),
            ]
            .into_iter()
            .collect()
        );

        tick_until(&mut computer, &program_completed);

        // heap should be back to how it was after the initial Memory.init
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
    fn test_string_erase_last_char() {
        let mut computer = computer_from_jack_code(mock_from_sources(vec![(
            "Sys.jack",
            "
            class Sys {
                static String str;

                function void setupString() {
                    let str = \"hello there\";
                }

                function void removeChars(String str_ptr) {
                  do str_ptr.eraseLastChar();
                  do str_ptr.eraseLastChar();
                }

                function void init () {
                    do Memory.init();

                    do setupString();
                    do removeChars(str);
                }
            }
            ",
        )]));

        tick_until(&mut computer, &|computer| frame_stack_depth(computer) == 1);

        step_over(&mut computer); // step over Memory.init();
        step_over(&mut computer); // step over setupString
        step_in(&mut computer); // step into removeChars
        assert_eq!(string_from_pointer(&computer, top_frame_arg(&computer, 0)), "hello there");
        step_over(&mut computer); // step over eraseLastChar()
        assert_eq!(string_from_pointer(&computer, top_frame_arg(&computer, 0)), "hello ther");
        step_over(&mut computer); // step over eraseLastChar()
        assert_eq!(string_from_pointer(&computer, top_frame_arg(&computer, 0)), "hello the");
    }

    #[test]
    fn test_string_int_value() {
        let mut computer = computer_from_jack_code(mock_from_sources(vec![(
            "Sys.jack",
            "
            class Sys {
                static String str;

                function void setupString() {
                    let str = \"1234\";
                }

                function void init () {
                    var int i;

                    do Memory.init();
                    do setupString();
                    let i = str.intValue();
                }
            }
            ",
        )]));

        tick_until(&mut computer, &|computer| frame_stack_depth(computer) == 1);

        step_over(&mut computer); // step over Memory.init();
        step_over(&mut computer); // step over setupString
        step_over(&mut computer); // step over str.intValue();
        assert_eq!(peek_stack(&computer), 1234);
    }
}
