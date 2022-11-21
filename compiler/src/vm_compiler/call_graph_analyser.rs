use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use crate::jack_compiler::codegen::{CompiledSubroutine, SourcemappedCommand};

use super::parser::{Command, FunctionCommandVariant, MemoryCommandVariant, MemorySegmentVariant, OffsetSegmentVariant, PointerSegmentVariant};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Pointer {
    Lcl,
    Arg,
    This,
    That,
}

#[derive(Clone, Debug, Default)]
pub struct SubroutineInfo {
    pub calls: HashSet<String>,
    pub callers: HashSet<String>,
    pub reachable_subroutines: HashSet<String>,
    pub pointers_used_directly: HashSet<Pointer>,
    pub pointers_used: HashSet<Pointer>,
    pub pointers_to_restore: HashSet<Pointer>,
}

pub struct CallGraphAnalysis {
    pub live_subroutines: HashSet<String>,
    pub subroutine_info_by_name: HashMap<String, SubroutineInfo>,
}

type CallGraph = HashMap<String, SubroutineInfo>;

fn include_in_call_graph(command: &Command, subroutine_name: &str, call_graph: &mut CallGraph) {
    if let Command::Function(FunctionCommandVariant::Call(callee_name, ..)) = command {
        let caller_info = call_graph.entry(subroutine_name.to_owned()).or_default();
        caller_info.calls.insert(callee_name.clone());

        let callee_info = call_graph.entry(callee_name.clone()).or_default();
        callee_info.callers.insert(subroutine_name.to_owned());
    }
}

fn record_directly_used_pointers(command: &Command, subroutine_name: &str, call_graph: &mut CallGraph) {
    let subroutine_info = call_graph.entry(subroutine_name.to_owned()).or_default();
    if let Command::Memory(MemoryCommandVariant::Pop(memory_segment, offset) | MemoryCommandVariant::Push(memory_segment, offset)) = command {
        match memory_segment {
            MemorySegmentVariant::OffsetSegment(OffsetSegmentVariant::Pointer) => {
                let pointer = if *offset == 0 {
                    Pointer::This
                } else if *offset == 1 {
                    Pointer::That
                } else {
                    panic!("expected offset for pointer to be either 0 or 1")
                };
                subroutine_info.pointers_used_directly.insert(pointer);
            }
            MemorySegmentVariant::PointerSegment(segment) => {
                let pointer = match segment {
                    PointerSegmentVariant::Argument => Pointer::Arg,
                    PointerSegmentVariant::Local => Pointer::Lcl,
                    PointerSegmentVariant::This => Pointer::This,
                    PointerSegmentVariant::That => Pointer::That,
                };
                subroutine_info.pointers_used_directly.insert(pointer);
            }
            _ => {}
        }
    }
}

fn depth_first_search(caller_name: String, call_graph: &CallGraph, discovered: &mut HashSet<String>) {
    let default_caller_info = SubroutineInfo::default();
    let caller_info = call_graph.get(&caller_name).unwrap_or(&default_caller_info);
    if !discovered.contains(&caller_name) {
        discovered.insert(caller_name);
        for callee_name in &caller_info.calls {
            depth_first_search(callee_name.clone(), call_graph, discovered)
        }
    }
}

fn analyse_pointer_usage(call_graph: &mut CallGraph, subroutines: &HashMap<PathBuf, Vec<CompiledSubroutine>>) {
    for subroutine in subroutines.values().flatten() {
        let subroutine_info = call_graph
            .get(&subroutine.name)
            .unwrap_or_else(|| panic!("expected to find subroutine info for {}", subroutine.name));

        let reachable_subroutines = subroutines_reachable_from(&subroutine.name, call_graph);

        let pointers_used = pointers_used_directly_by_subroutines(&reachable_subroutines, call_graph);
        let pointers_used_directly_by_callers = pointers_used_directly_by_subroutines(&subroutine_info.callers, call_graph);
        let pointers_to_restore = pointers_used.intersection(&pointers_used_directly_by_callers).cloned().collect();

        let subroutine_info = call_graph
            .get_mut(&subroutine.name)
            .unwrap_or_else(|| panic!("expected to find subroutine info for {}", subroutine.name));
        subroutine_info.reachable_subroutines = reachable_subroutines;
        subroutine_info.pointers_used = pointers_used;
        subroutine_info.pointers_to_restore = pointers_to_restore;
    }
}

fn subroutines_reachable_from(subroutine_name: &str, call_graph: &CallGraph) -> HashSet<String> {
    let mut discovered = HashSet::new();
    depth_first_search(subroutine_name.to_owned(), call_graph, &mut discovered);
    discovered
}

fn get_call_graph(subroutines: &HashMap<PathBuf, Vec<CompiledSubroutine>>) -> CallGraph {
    let mut call_graph = HashMap::new();
    for file_subroutines in subroutines.values() {
        for subroutine in file_subroutines {
            for SourcemappedCommand { command, .. } in &subroutine.commands {
                include_in_call_graph(command, &subroutine.name, &mut call_graph);
                record_directly_used_pointers(command, &subroutine.name, &mut call_graph);
            }
        }
    }
    analyse_pointer_usage(&mut call_graph, subroutines);
    call_graph
}

fn pointers_used_directly_by_subroutines(subroutines: &HashSet<String>, call_graph: &CallGraph) -> HashSet<Pointer> {
    subroutines
        .iter()
        .flat_map(|reachable_subroutine| {
            let reachable_subroutine_info = call_graph
                .get(reachable_subroutine)
                .unwrap_or_else(|| panic!("expected to find subroutine info for {}", reachable_subroutine));
            reachable_subroutine_info.pointers_used_directly.clone()
        })
        .collect()
}

pub fn analyse_call_graph(subroutines: &HashMap<PathBuf, Vec<CompiledSubroutine>>) -> CallGraphAnalysis {
    let call_graph = get_call_graph(subroutines);

    let live_subroutines = &call_graph
        .get("Sys.init")
        .unwrap_or_else(|| panic!("expected to find subgraph for Sys.init"))
        .reachable_subroutines;

    let subroutine_info_by_name = subroutines
        .values()
        .flatten()
        .map(|subroutine| {
            let subroutine_info = call_graph
                .get(&subroutine.name)
                .unwrap_or_else(|| panic!("expected to find subroutine info for {}", subroutine.name))
                .clone();

            (subroutine.name.clone(), subroutine_info)
        })
        .collect();

    CallGraphAnalysis {
        live_subroutines: live_subroutines.clone(),
        subroutine_info_by_name,
    }
}
