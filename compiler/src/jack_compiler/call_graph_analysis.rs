use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use super::{
    codegen::full_subroutine_name,
    jack_node_types::{ASTNode, SubroutineDeclaration},
    parser::JackParserResult,
};

#[derive(Default)]
pub struct SubroutineInfo {
    callers: HashSet<String>,
    calls: HashSet<String>,
}

fn find_calls(subroutine_declaration: &SubroutineDeclaration) -> Vec<String> {
    vec![]
}

pub fn analyse_call_graph(parsed_jack_program: &HashMap<PathBuf, JackParserResult>) -> HashMap<String, SubroutineInfo> {
    let mut call_graph: HashMap<String, SubroutineInfo> = HashMap::new();

    for (_filename, JackParserResult { class, .. }) in parsed_jack_program {
        for ASTNode { node, .. } in &class.subroutine_declarations {
            let callee_name = full_subroutine_name(&class.name, &node.name);
            for called_subroutine in find_calls(node) {
                let caller_info = call_graph.entry(callee_name.clone()).or_default();
                caller_info.calls.insert(called_subroutine.clone());

                let callee_info = call_graph.entry(called_subroutine).or_default();
                callee_info.callers.insert(callee_name.clone());
            }
        }
    }
    call_graph
}
