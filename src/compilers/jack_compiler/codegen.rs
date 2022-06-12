use super::parser::Class;

pub struct CodeGenerator;

impl CodeGenerator {
    pub fn new(class: Class) -> Self {
        CodeGenerator
    }

    pub fn vm_code(self) -> String {
        todo!()
    }
}
