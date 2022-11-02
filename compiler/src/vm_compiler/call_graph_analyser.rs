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

#[derive(Debug, Default)]
pub struct SubroutineInfo {
    calls: HashSet<String>,
    callers: HashSet<String>,
    reachable_subroutines: HashSet<String>,
    directly_used_pointers: HashSet<Pointer>,
    used_pointers: HashSet<Pointer>,
    pointers_to_restore: HashSet<Pointer>,
}

pub struct CallGraphAnalysis {
    pub live_subroutines: HashSet<String>,
    pub pointers_to_restore: HashMap<String, HashSet<Pointer>>,
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
                subroutine_info.directly_used_pointers.insert(pointer);
            }
            MemorySegmentVariant::PointerSegment(segment) => {
                let pointer = match segment {
                    PointerSegmentVariant::Argument => Pointer::Arg,
                    PointerSegmentVariant::Local => Pointer::Lcl,
                    PointerSegmentVariant::This => Pointer::This,
                    PointerSegmentVariant::That => Pointer::That,
                };
                subroutine_info.directly_used_pointers.insert(pointer);
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

fn subroutines_reachable_from(subroutine_name: &str, call_graph: &CallGraph) -> HashSet<String> {
    let mut discovered = HashSet::new();
    depth_first_search(subroutine_name.to_owned(), call_graph, &mut discovered);
    discovered
}

pub fn analyse_call_graph(subroutines: &HashMap<PathBuf, Vec<CompiledSubroutine>>) -> CallGraphAnalysis {
    let mut call_graph = HashMap::new();
    for file_subroutines in subroutines.values() {
        for subroutine in file_subroutines {
            for SourcemappedCommand { command, .. } in &subroutine.commands {
                include_in_call_graph(command, &subroutine.name, &mut call_graph);
                record_directly_used_pointers(command, &subroutine.name, &mut call_graph);
            }
        }
    }

    for file_subroutines in subroutines.values() {
        for subroutine in file_subroutines {
            let subroutine_info = call_graph
                .get(&subroutine.name)
                .unwrap_or_else(|| panic!("expected to find subroutine info for {}", subroutine.name));

            let reachable_subroutines = subroutines_reachable_from(&subroutine.name, &call_graph);

            let used_pointers = reachable_subroutines
                .iter()
                .flat_map(|reachable_subroutine| {
                    let reachable_subroutine_info = call_graph
                        .get(reachable_subroutine)
                        .unwrap_or_else(|| panic!("expected to find subroutine info for {}", reachable_subroutine));
                    reachable_subroutine_info.directly_used_pointers.clone()
                })
                .collect();

            let pointers_to_restore = subroutine_info
                .callers
                .iter()
                .flat_map(|caller| {
                    let caller_info = call_graph
                        .get(caller)
                        .unwrap_or_else(|| panic!("expected to find subroutine info for {}", caller));
                    caller_info.directly_used_pointers.intersection(&used_pointers)
                })
                .cloned()
                .collect();

            let subroutine_info = call_graph
                .get_mut(&subroutine.name)
                .unwrap_or_else(|| panic!("expected to find subroutine info for {}", subroutine.name));
            subroutine_info.reachable_subroutines = reachable_subroutines;
            subroutine_info.used_pointers = used_pointers;
            subroutine_info.pointers_to_restore = pointers_to_restore;
        }
    }

    let live_subroutines = &call_graph
        .get("Sys.init")
        .unwrap_or_else(|| panic!("expected to find subgraph for Sys.init"))
        .reachable_subroutines;

    let pointers_to_restore = subroutines
        .values()
        .flatten()
        .map(|subroutine| {
            let subroutine_info = call_graph
                .get(&subroutine.name)
                .unwrap_or_else(|| panic!("expected to find subroutine info for {}", subroutine.name));

            (
                subroutine.name.clone(),
                subroutine_info
                    .pointers_to_restore
                    .union(&subroutine_info.directly_used_pointers)
                    .cloned()
                    .collect(),
            )
        })
        .collect();

    CallGraphAnalysis {
        live_subroutines: live_subroutines.clone(),
        pointers_to_restore,
    }
}
