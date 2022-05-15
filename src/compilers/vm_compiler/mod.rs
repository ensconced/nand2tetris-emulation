mod codegen;
mod parser;
mod tokenizer;

fn compile_to_asm(vm_code_source: &str) -> String {
    todo!()
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_sys_init() {
//         let program = "
//           function Sys.init 0
//           push constant 1234
//           return
//         ";
//         let asm = compile_to_asm(program);
//     }
// }
