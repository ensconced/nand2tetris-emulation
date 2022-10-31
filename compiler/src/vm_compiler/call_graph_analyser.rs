use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use crate::jack_compiler::codegen::{CompiledSubroutine, SourcemappedCommand};

use super::parser::{Command, FunctionCommandVariant};

#[derive(Debug, Default)]
pub struct SubroutineInfo {
    calls: HashSet<String>,
    callers: HashSet<String>,
}

type CallGraph = HashMap<String, SubroutineInfo>;

fn analyse_subroutine(subroutine: &CompiledSubroutine, call_graph: &mut CallGraph) {
    for SourcemappedCommand { command, .. } in &subroutine.commands {
        if let Command::Function(FunctionCommandVariant::Call(callee_name, ..)) = command {
            let caller_info = call_graph.entry(subroutine.name.clone()).or_default();
            caller_info.calls.insert(callee_name.clone());

            let callee_info = call_graph.entry(callee_name.clone()).or_default();
            callee_info.callers.insert(subroutine.name.clone());
        }
    }
}

fn depth_first_search(caller_name: String, call_graph: &CallGraph, discovered: &HashSet<String>) -> HashSet<String> {
    let default_caller_info = SubroutineInfo::default();
    let caller_info = call_graph.get(&caller_name).unwrap_or(&default_caller_info);
    let mut discovered = discovered.clone();
    if !discovered.contains(&caller_name) {
        discovered.insert(caller_name);
        for callee_name in &caller_info.calls {
            discovered = depth_first_search(callee_name.clone(), call_graph, &discovered)
        }
    }
    discovered
}

pub fn find_live_subroutines(subroutines: &HashMap<PathBuf, Vec<CompiledSubroutine>>) -> HashSet<String> {
    let mut call_graph = HashMap::new();
    for file_subroutines in subroutines.values() {
        for subroutine in file_subroutines {
            analyse_subroutine(subroutine, &mut call_graph);
        }
    }
    depth_first_search("Sys.init".to_owned(), &call_graph, &HashSet::new())
}
