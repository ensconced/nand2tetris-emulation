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
    let result = codegen::generate_vm_code(parse(source));
    println!("{}", &result);
    result
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
                    var int a, b, c, d;
                    let a = 4191 / -3;
                    let b = 1234 / 123;
                    let c = -5198 / 182;
                    let c = 9099 / 33;
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
    fn test_string() {
        let mut computer = computer_from_jack_code(vec![
            "
            class Sys {
                function void init () {
                    var int a;
                    do Memory.init();
                    let a = \"hello\";
                    do Memory.usage();
                }
            }
            ",
        ]);
        computer.tick_until(&|computer| stack_pointer(computer) == INITIAL_STACK_POINTER_ADDRESS);
        let chars: Vec<_> = "hello".encode_utf16().map(|x| x as i16).collect();
        for char in chars.iter() {
            computer.tick_until(&|computer| peek_stack(computer) == *char);
        }
        // expect memory usage to be 2 for string itself, plus 5 for the underlying buffer
        computer.tick_until(&|computer| peek_stack(computer) == 7);
        computer.tick_until(&|computer| heap_includes(computer, &chars));
    }
}
