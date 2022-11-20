use itertools::Itertools;
use serde::Serialize;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};
use ts_rs::TS;

use self::{
    codegen::{generate_vm_code, CompiledSubroutine},
    parser::{parse, JackParserResult},
    sourcemap::JackCompilerSourceMap,
    tokenizer::{token_defs, TokenKind},
};
use super::utils::{
    source_modules::{get_source_modules, SourceModule},
    tokenizer::{Token, Tokenizer},
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
    #[ts(type = "Record<string, Array<CompiledSubroutine>>")]
    pub subroutines: HashMap<PathBuf, Vec<CompiledSubroutine>>,
}

fn get_all_source_modules(user_code: HashMap<PathBuf, SourceModule>) -> HashMap<PathBuf, SourceModule> {
    let std_lib_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../std_lib");
    let mut source_modules = get_source_modules(&std_lib_dir).expect("failed to get stdlib modules");

    // User modules override any std_lib modules with the same filename.
    source_modules.extend(user_code);
    source_modules
}

fn tokenize_jack_program(source_modules: HashMap<PathBuf, SourceModule>) -> HashMap<PathBuf, Vec<Token<TokenKind>>> {
    source_modules
        .into_iter()
        .map(|(filename, source_module)| (filename, Tokenizer::new(token_defs()).tokenize(&source_module.source)))
        .collect()
}

fn parse_jack_program(jack_program_tokens: &HashMap<PathBuf, Vec<Token<TokenKind>>>) -> HashMap<PathBuf, JackParserResult> {
    jack_program_tokens
        .iter()
        .map(|(filename, tokens)| (filename.clone(), parse(tokens)))
        .collect()
}

pub fn compile_jack(user_code: HashMap<PathBuf, SourceModule>) -> JackCompilerResult {
    let mut result = JackCompilerResult::default();
    let jack_program_tokens = tokenize_jack_program(get_all_source_modules(user_code));
    let parsed_jack_program = parse_jack_program(&jack_program_tokens);
    result.tokens = jack_program_tokens;
    for (filename, parse_result) in parsed_jack_program {
        let codegen_result = generate_vm_code(parse_result.class);
        result.sourcemaps.insert(
            filename.clone(),
            JackCompilerSourceMap {
                parser_sourcemap: parse_result.sourcemap,
                codegen_sourcemap: codegen_result.sourcemap,
            },
        );

        result.subroutines.insert(filename, codegen_result.subroutines);
    }

    result
}

#[cfg(test)]
mod tests {
    use std::{ffi::OsString, fs, path::Path};

    use crate::utils::{source_modules::mock_from_sources, testing::test_utils::*};
    use emulator_core::computer::tick_until;
    use itertools::Itertools;

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
    fn snapshot_tests() {
        let snapshots_dir_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("snapshot_tests");
        let snapshots_dir = fs::read_dir(snapshots_dir_path).unwrap_or_else(|_| panic!("failed to read snapshots dir"));
        for dir_entry_result in snapshots_dir {
            let dir_entry = dir_entry_result.unwrap();
            if !dir_entry.metadata().unwrap().is_dir() {
                continue;
            }
            let snapshot_path = dir_entry.path();
            let snapshot_dir = fs::read_dir(&snapshot_path).unwrap_or_else(|_| panic!("failed to read snapshot path {}", snapshot_path.display()));
            let snapshot_files_by_extension = snapshot_dir
                .map(|maybe_dir_entry| maybe_dir_entry.unwrap_or_else(|_| panic!("failed to read file")))
                .into_group_map_by(|dir_entry| dir_entry.path().extension().unwrap_or_default().to_owned());

            let empty_vec = vec![];
            let jack_files = snapshot_files_by_extension
                .get(&OsString::from("jack"))
                .unwrap_or_else(|| panic!("no jack files"));
            let image_files = snapshot_files_by_extension.get(&OsString::from("pbm")).unwrap_or(&empty_vec);

            let jack_sources: Vec<_> = jack_files
                .iter()
                .map(|file| {
                    let filename = file.file_name();
                    let file_contents = fs::read_to_string(file.path()).unwrap_or_else(|_| panic!("failed to read jack file"));
                    (filename, file_contents)
                })
                .collect();

            let jack_source_refs: Vec<_> = jack_sources
                .iter()
                .map(|(filename, file_contents)| {
                    (
                        filename.to_str().unwrap_or_else(|| panic!("failed to read filename as string")),
                        file_contents.as_str(),
                    )
                })
                .collect();

            let mut computer = computer_from_jack_code(mock_from_sources(jack_source_refs));
            tick_until(&mut computer, &program_completed);
            let screen_bytes = computer.screen_snapshot();
            if let Some(image_file) = image_files.get(0) {
                let expected_bytes = fs::read(image_file.path()).unwrap_or_else(|_| panic!("failed to read pbm snapshot"));
                assert_eq!(screen_bytes, expected_bytes);
            } else {
                // image file doesn't exist - write one
                fs::write(Path::join(&snapshot_path, "screen.pbm"), screen_bytes).unwrap_or_else(|_| panic!("failed to write screen snapshot"));
            }
        }
    }
}
