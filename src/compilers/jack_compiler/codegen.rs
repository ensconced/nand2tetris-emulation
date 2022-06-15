use super::parser::{
    Class, ClassVarDeclaration, ClassVarDeclarationKind, Parameter, SubroutineBody,
    SubroutineDeclaration, SubroutineKind, Type,
};
use std::collections::HashMap;

struct ClassSymbol {
    offset: usize,
    field_type: Type,
}
struct SubroutineSymbol {
    offset: usize,
    field_type: Type,
}

pub struct CodeGenerator {
    class_field_count: usize,
    class_static_count: usize,
    class_fields: HashMap<String, ClassSymbol>,
    class_statics: HashMap<String, ClassSymbol>,
    subroutine_parameters: HashMap<String, SubroutineSymbol>,
}

impl CodeGenerator {
    pub fn new() -> Self {
        CodeGenerator {
            class_field_count: 0,
            class_static_count: 0,
            class_fields: HashMap::new(),
            class_statics: HashMap::new(),
            subroutine_parameters: HashMap::new(),
        }
    }

    fn clear_subroutine(&mut self) {
        self.subroutine_parameters.clear();
    }

    fn compile_constructor(
        &mut self,
        subroutine: SubroutineDeclaration,
        class_name: &str,
    ) -> String {
        todo!()
    }

    fn compile_function(&mut self, subroutine: SubroutineDeclaration, class_name: &str) -> String {
        todo!()
    }

    fn compile_method_body(&mut self, method_body: SubroutineBody) -> String {
        // TODO - when compiling body, should check return statements against return type
        todo!()
    }

    fn compile_method_parameters(&mut self, parameters: Vec<Parameter>) {
        for (idx, parameter) in parameters.into_iter().enumerate() {
            self.subroutine_parameters.insert(
                parameter.var_name,
                SubroutineSymbol {
                    offset: idx + 1,
                    field_type: parameter.type_name,
                },
            );
        }
    }

    fn compile_method(&mut self, method: SubroutineDeclaration, class_name: &str) -> String {
        let param_count = method.parameters.len();
        self.compile_method_parameters(method.parameters);

        let implicit_return = if method.return_type.is_none() {
            "push constant 0"
        } else {
            ""
        };

        format!(
            "
        function {class_name}.{function_name} {params_len}
          push argument 0
          pop pointer 0
          {body}

          {implicit_return}
        ",
            function_name = method.name,
            class_name = class_name,
            params_len = param_count + 1,
            body = self.compile_method_body(method.body),
            implicit_return = implicit_return
        )
    }

    fn compile_subroutines(
        &mut self,
        subroutine_declarations: Vec<SubroutineDeclaration>,
        class_name: String,
    ) -> String {
        let parts: Vec<_> = subroutine_declarations
            .into_iter()
            .map(|subroutine| match subroutine.subroutine_kind {
                SubroutineKind::Constructor => self.compile_constructor(subroutine, &class_name),
                SubroutineKind::Function => self.compile_function(subroutine, &class_name),
                SubroutineKind::Method => self.compile_method(subroutine, &class_name),
            })
            .collect();
        parts.join("\n")
    }

    fn compile_var_declarations(&mut self, var_declarations: Vec<ClassVarDeclaration>) {
        for var_declaration in var_declarations {
            let (hashmap, var_count) = match var_declaration.qualifier {
                ClassVarDeclarationKind::Static => (self.class_statics, &self.class_static_count),
                ClassVarDeclarationKind::Field => (self.class_fields, &self.class_field_count),
            };
            for var_name in var_declaration.var_names {
                hashmap.insert(
                    var_name,
                    ClassSymbol {
                        offset: self.class_field_count,
                        field_type: var_declaration.type_name,
                    },
                );
                *var_count += 1;
            }
        }
    }

    pub fn vm_code(&mut self, class: Class) -> String {
        self.compile_var_declarations(class.var_declarations);
        self.compile_subroutines(class.subroutine_declarations, class.name)
    }
}
