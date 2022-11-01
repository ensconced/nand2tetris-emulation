use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use crate::jack_compiler::codegen::{CompiledSubroutine, SourcemappedCommand};

use super::parser::{Command, FunctionCommandVariant, MemoryCommandVariant, MemorySegmentVariant, OffsetSegmentVariant, PointerSegmentVariant};

#[derive(Debug, PartialEq, Eq, Hash)]
enum Pointer {
    Lcl,
    Arg,
    This,
    That,
}

#[derive(Debug, Default)]
pub struct SubroutineInfo {
    calls: HashSet<String>,
    callers: HashSet<String>,
    directly_used_pointers: HashSet<Pointer>,
    used_pointers: HashSet<Pointer>,
}

pub struct CallGraphAnalysis {
    pub live_subroutines: HashSet<String>,
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

fn analyse_subroutine(subroutine: &CompiledSubroutine, call_graph: &mut CallGraph) {
    for SourcemappedCommand { command, .. } in &subroutine.commands {
        include_in_call_graph(command, &subroutine.name, call_graph);
        record_directly_used_pointers(command, &subroutine.name, call_graph);
    }
}

fn subroutines_reachable_from(caller_name: String, call_graph: &CallGraph, discovered: &HashSet<String>) -> HashSet<String> {
    let default_caller_info = SubroutineInfo::default();
    let caller_info = call_graph.get(&caller_name).unwrap_or(&default_caller_info);
    let mut discovered = discovered.clone();
    if !discovered.contains(&caller_name) {
        discovered.insert(caller_name);
        for callee_name in &caller_info.calls {
            discovered = subroutines_reachable_from(callee_name.clone(), call_graph, &discovered)
        }
    }
    discovered
}

pub fn analyse_call_graph(subroutines: &HashMap<PathBuf, Vec<CompiledSubroutine>>) -> CallGraphAnalysis {
    let mut call_graph = HashMap::new();
    for file_subroutines in subroutines.values() {
        for subroutine in file_subroutines {
            analyse_subroutine(subroutine, &mut call_graph);
        }
    }
    let live_subroutines = subroutines_reachable_from("Sys.init".to_owned(), &call_graph, &HashSet::new());
    CallGraphAnalysis { live_subroutines }
}
