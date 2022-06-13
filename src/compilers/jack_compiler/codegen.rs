use super::parser::{Class, ClassVarDeclaration, SubroutineDeclaration, SubroutineKind};
use std::collections::HashMap;

pub struct CodeGenerator {
    class: Class,
    class_symbols: HashMap<String, u16>,
    subroutine_symbols: HashMap<String, u16>,
}

fn get_class_symbols(class: &Class) -> HashMap<String, u16> {
    todo!()
}

impl CodeGenerator {
    pub fn new(class: Class) -> Self {
        let class_symbols = get_class_symbols(&class);
        CodeGenerator {
            class,
            class_symbols,
            subroutine_symbols: HashMap::new(),
        }
    }

    fn compile_constructor(&mut self, subroutine: &SubroutineDeclaration) {
        todo!()
    }

    fn compile_function(&mut self, subroutine: &SubroutineDeclaration) {
        todo!()
    }

    fn compile_method(&mut self, subroutine: &SubroutineDeclaration) {
        todo!()
    }

    pub fn vm_code(mut self) -> String {
        for subroutine in &self.class.subroutine_declarations {
            match &subroutine.subroutine_kind {
                SubroutineKind::Constructor => self.compile_constructor(subroutine),
                SubroutineKind::Function => self.compile_function(subroutine),
                SubroutineKind::Method => self.compile_method(subroutine),
            }
        }
        todo!()
    }
}
