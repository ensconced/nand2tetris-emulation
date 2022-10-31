use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use super::parser::{Command, FunctionCommandVariant};

#[derive(Debug, Default)]
pub struct SubroutineInfo {
    calls: HashSet<String>,
    callers: HashSet<String>,
}

type CallGraph = HashMap<String, SubroutineInfo>;

fn analyse_subroutine(subroutine: &[Command], call_graph: &mut CallGraph) {
    let caller_name = if let Some(Command::Function(FunctionCommandVariant::Define(name, ..))) = subroutine.get(0) {
        name
    } else {
        panic!("expected first command in subroutine to be function definition");
    };

    for command in subroutine {
        if let Command::Function(FunctionCommandVariant::Call(callee_name, ..)) = command {
            let caller_info = call_graph.entry(caller_name.clone()).or_default();
            caller_info.calls.insert(callee_name.clone());

            let callee_info = call_graph.entry(callee_name.clone()).or_default();
            callee_info.callers.insert(caller_name.clone());
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

pub fn find_live_subroutines(subroutines: &HashMap<PathBuf, Vec<Vec<Command>>>) -> HashSet<String> {
    let mut call_graph = HashMap::new();
    for file_subroutines in subroutines.values() {
        for subroutine in file_subroutines {
            analyse_subroutine(subroutine, &mut call_graph);
        }
    }
    depth_first_search("Sys.init".to_owned(), &call_graph, &HashSet::new())
}
