use std::{
    collections::HashMap,
    iter,
    path::{Path, PathBuf},
};

use serde::Serialize;
use ts_rs::TS;

use crate::{
    assembler::parser::{ASMInstruction, AValue},
    fonts::glyphs_asm,
    jack_compiler::codegen::{CompiledSubroutine, SourcemappedCommand},
};

use super::{
    call_graph_analyser::{analyse_call_graph, CallGraphAnalysis},
    parser::{
        ArithmeticCommandVariant::{self, *},
        BinaryArithmeticCommandVariant::*,
        Command::{self, *},
        FlowCommandVariant,
        FunctionCommandVariant::{self, *},
        MemoryCommandVariant::{self, *},
        MemorySegmentVariant::{self, *},
        OffsetSegmentVariant,
        PointerSegmentVariant::{self, *},
        UnaryArithmeticCommandVariant::*,
    },
    sourcemap::SourceMap,
};

fn holding_pattern() -> Vec<ASMInstruction> {
    vec![
        // This will be the very first instruction in the computer's ROM.
        // We don't want to go into an infinite loop quite yet, so skip over it!
        ASMInstruction::A(AValue::Symbolic("$skip_holding_pattern".to_string())),
        ASMInstruction::C {
            expr: "0".to_string(),
            dest: None,
            jump: Some("JMP".to_string()),
        },
        // This will be the return address of the main Sys.init function, so when
        // that function exits, the computer just goes into an infinite loop
        ASMInstruction::L {
            identifier: "$holding_pattern".to_string(),
        },
        ASMInstruction::A(AValue::Symbolic("$holding_pattern".to_string())),
        ASMInstruction::C {
            expr: "0".to_string(),
            dest: None,
            jump: Some("JMP".to_string()),
        },
        ASMInstruction::L {
            identifier: "$skip_holding_pattern".to_string(),
        },
    ]
}

fn init_call_stack() -> Vec<ASMInstruction> {
    vec![
        // For each stack frame, ARG points to the base of the frame. This is the
        // first stack frame, so here ARG points to the base of the entire stack.
        ASMInstruction::A(AValue::Numeric("256".to_string())),
        ASMInstruction::C {
            expr: "A".to_string(),
            dest: Some("D".to_string()),
            jump: None,
        },
        ASMInstruction::A(AValue::Symbolic("ARG".to_string())),
        ASMInstruction::C {
            expr: "D".to_string(),
            dest: Some("M".to_string()),
            jump: None,
        },
        // Initialize the stack pointer. Even though there is no real caller
        // function for Sys.init, we leave the customary space for the saved LCL,
        // ARG, THIS and THAT of the caller. This in addition to the return
        // address means the stack pointer will start 5 addresses above the base
        // of the stack.
        ASMInstruction::A(AValue::Numeric("261".to_string())),
        ASMInstruction::C {
            expr: "A".to_string(),
            dest: Some("D".to_string()),
            jump: None,
        },
        ASMInstruction::A(AValue::Symbolic("SP".to_string())),
        ASMInstruction::C {
            expr: "D".to_string(),
            dest: Some("M".to_string()),
            jump: None,
        },
        // LCL starts off pointing to the same address as the stack pointer.
        ASMInstruction::A(AValue::Numeric("261".to_string())),
        ASMInstruction::C {
            expr: "A".to_string(),
            dest: Some("D".to_string()),
            jump: None,
        },
        ASMInstruction::A(AValue::Symbolic("LCL".to_string())),
        ASMInstruction::C {
            expr: "D".to_string(),
            dest: Some("M".to_string()),
            jump: None,
        },
        // Load the return address. Sys.init takes no arguments, so this is
        // located right at the base of the stack.
        ASMInstruction::A(AValue::Symbolic("$holding_pattern".to_string())),
        ASMInstruction::C {
            expr: "A".to_string(),
            dest: Some("D".to_string()),
            jump: None,
        },
        ASMInstruction::A(AValue::Numeric("256".to_string())),
        ASMInstruction::C {
            expr: "D".to_string(),
            dest: Some("M".to_string()),
            jump: None,
        },
        // Call Sys.init
        ASMInstruction::A(AValue::Symbolic("$entry_Sys.init".to_string())),
        ASMInstruction::C {
            expr: "0".to_string(),
            dest: None,
            jump: Some("JMP".to_string()),
        },
    ]
}

fn offset_address(segment: &OffsetSegmentVariant, index: u16) -> u16 {
    let (segment_base_address, segment_top_address): (u16, u16) = match segment {
        OffsetSegmentVariant::Pointer => (3, 4),
        OffsetSegmentVariant::Temp => (5, 6),
    };
    let segment_max_index = segment_top_address - segment_base_address;
    if index > segment_max_index {
        panic!("segment index {} is too high - max is {}", index, segment_max_index)
    }
    segment_base_address + index
}

fn push_from_d_register() -> Vec<ASMInstruction> {
    vec![
        ASMInstruction::A(AValue::Symbolic("SP".to_string())),
        ASMInstruction::C {
            expr: "M+1".to_string(),
            dest: Some("M".to_string()),
            jump: None,
        },
        ASMInstruction::C {
            expr: "M-1".to_string(),
            dest: Some("A".to_string()),
            jump: None,
        },
        ASMInstruction::C {
            expr: "D".to_string(),
            dest: Some("M".to_string()),
            jump: None,
        },
    ]
}

fn pop_into_d_register(pointer: &str) -> Vec<ASMInstruction> {
    // pointer is usually going to be SP but occasionally we want to use a
    // different pointer to perform a pop-like operation
    vec![
        ASMInstruction::A(AValue::Symbolic(pointer.to_string())),
        ASMInstruction::C {
            expr: "M-1".to_string(),
            dest: Some("MA".to_string()),
            jump: None,
        },
        ASMInstruction::C {
            expr: "M".to_string(),
            dest: Some("D".to_string()),
            jump: None,
        },
    ]
}

fn push_from_offset_memory_segment(segment: &OffsetSegmentVariant, index: u16) -> Vec<ASMInstruction> {
    vec![
        vec![
            ASMInstruction::A(AValue::Numeric(offset_address(segment, index).to_string())),
            ASMInstruction::C {
                expr: "M".to_string(),
                dest: Some("D".to_string()),
                jump: None,
            },
        ],
        push_from_d_register(),
    ]
    .into_iter()
    .flatten()
    .collect()
}

fn pop_into_offset_memory_segment(segment: &OffsetSegmentVariant, index: u16) -> Vec<ASMInstruction> {
    vec![
        pop_into_d_register("SP"),
        vec![
            ASMInstruction::A(AValue::Numeric(offset_address(segment, index).to_string())),
            ASMInstruction::C {
                expr: "D".to_string(),
                dest: Some("M".to_string()),
                jump: None,
            },
        ],
    ]
    .into_iter()
    .flatten()
    .collect()
}

fn load_pointer_value_into_a(pointer_address: &str, index: u16) -> Vec<ASMInstruction> {
    if index == 0 {
        vec![
            ASMInstruction::A(AValue::Symbolic(pointer_address.to_string())),
            ASMInstruction::C {
                expr: "M".to_string(),
                dest: Some("A".to_string()),
                jump: None,
            },
        ]
    } else if index == 1 {
        vec![
            ASMInstruction::A(AValue::Symbolic(pointer_address.to_string())),
            ASMInstruction::C {
                expr: "M+1".to_string(),
                dest: Some("A".to_string()),
                jump: None,
            },
        ]
    } else {
        vec![
            load_constant_into_d(index),
            vec![
                ASMInstruction::A(AValue::Symbolic(pointer_address.to_string())),
                ASMInstruction::C {
                    expr: "M+D".to_string(),
                    dest: Some("A".to_string()),
                    jump: None,
                },
            ],
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

fn push_from_pointer_memory_segment(segment: &PointerSegmentVariant, index: u16) -> Vec<ASMInstruction> {
    let pointer_address = match segment {
        Argument => "ARG",
        Local => "LCL",
        This => "THIS",
        That => "THAT",
    };

    vec![
        load_pointer_value_into_a(pointer_address, index),
        vec![ASMInstruction::C {
            expr: "M".to_string(),
            dest: Some("D".to_string()),
            jump: None,
        }],
        push_from_d_register(),
    ]
    .into_iter()
    .flatten()
    .collect()
}

fn pop_into_pointer_memory_segment(segment: &PointerSegmentVariant, index: u16) -> Vec<ASMInstruction> {
    let pointer_address = match segment {
        Argument => "ARG",
        Local => "LCL",
        This => "THIS",
        That => "THAT",
    };

    let instructions = if index == 0 {
        vec![
            ASMInstruction::A(AValue::Symbolic(pointer_address.to_string())),
            ASMInstruction::C {
                expr: "M".to_string(),
                dest: Some("A".to_string()),
                jump: None,
            },
            ASMInstruction::C {
                expr: "D".to_string(),
                dest: Some("M".to_string()),
                jump: None,
            },
        ]
    } else {
        return vec![
            ASMInstruction::A(AValue::Symbolic(pointer_address.to_string())),
            ASMInstruction::C {
                expr: "M".to_string(),
                dest: Some("D".to_string()),
                jump: None,
            },
            ASMInstruction::A(AValue::Numeric(index.to_string())),
            ASMInstruction::C {
                expr: "D+A".to_string(),
                dest: Some("D".to_string()),
                jump: None,
            },
            ASMInstruction::A(AValue::Symbolic("R7".to_string())),
            ASMInstruction::C {
                expr: "D".to_string(),
                dest: Some("M".to_string()),
                jump: None,
            },
            ASMInstruction::A(AValue::Symbolic("SP".to_string())),
            ASMInstruction::C {
                expr: "M-1".to_string(),
                dest: Some("MA".to_string()),
                jump: None,
            },
            ASMInstruction::C {
                expr: "M".to_string(),
                dest: Some("D".to_string()),
                jump: None,
            },
            ASMInstruction::A(AValue::Symbolic("R7".to_string())),
            ASMInstruction::C {
                expr: "M".to_string(),
                dest: Some("A".to_string()),
                jump: None,
            },
            ASMInstruction::C {
                expr: "D".to_string(),
                dest: Some("M".to_string()),
                jump: None,
            },
        ];
    };

    vec![pop_into_d_register("SP"), instructions].into_iter().flatten().collect()
}

fn load_constant_into_d(constant: u16) -> Vec<ASMInstruction> {
    if constant == 0 || constant == 1 {
        vec![ASMInstruction::C {
            expr: constant.to_string(),
            dest: Some("D".to_string()),
            jump: None,
        }]
    } else {
        vec![
            ASMInstruction::A(AValue::Numeric(constant.to_string())),
            ASMInstruction::C {
                expr: "A".to_string(),
                dest: Some("D".to_string()),
                jump: None,
            },
        ]
    }
}

fn push_from_constant(constant: u16) -> Vec<ASMInstruction> {
    let max_constant = 32767;
    if constant > max_constant {
        panic!("constant {} is bigger than max of {}", constant, max_constant);
    }

    if constant == 0 || constant == 1 {
        vec![
            ASMInstruction::A(AValue::Symbolic("SP".to_string())),
            ASMInstruction::C {
                expr: "M+1".to_string(),
                dest: Some("M".to_string()),
                jump: None,
            },
            ASMInstruction::C {
                expr: "M-1".to_string(),
                dest: Some("A".to_string()),
                jump: None,
            },
            ASMInstruction::C {
                expr: constant.to_string(),
                dest: Some("M".to_string()),
                jump: None,
            },
        ]
    } else {
        vec![load_constant_into_d(constant), push_from_d_register()]
            .into_iter()
            .flatten()
            .collect()
    }
}

fn load_avalue_into_register(avalue: AValue, register: &str) -> Vec<ASMInstruction> {
    vec![
        ASMInstruction::A(avalue),
        ASMInstruction::C {
            expr: "A".to_string(),
            dest: Some("D".to_string()),
            jump: None,
        },
        ASMInstruction::A(AValue::Symbolic(register.to_string())),
        ASMInstruction::C {
            expr: "D".to_string(),
            dest: Some("M".to_string()),
            jump: None,
        },
    ]
}

fn initialize_locals(local_var_count: usize) -> Vec<ASMInstruction> {
    if local_var_count > 2 {
        // In this case, we can take fewer instructions by only updating SP once, after pushing all
        // the locals.
        vec![
            ASMInstruction::A(AValue::Symbolic("SP".to_string())),
            ASMInstruction::C {
                expr: "M".to_string(),
                dest: Some("A".to_string()),
                jump: None,
            },
            ASMInstruction::C {
                expr: "0".to_string(),
                dest: Some("M".to_string()),
                jump: None,
            },
        ]
        .into_iter()
        .chain(
            iter::repeat_with(|| {
                vec![
                    ASMInstruction::C {
                        expr: "A+1".to_string(),
                        dest: Some("A".to_string()),
                        jump: None,
                    },
                    ASMInstruction::C {
                        expr: "0".to_string(),
                        dest: Some("M".to_string()),
                        jump: None,
                    },
                ]
                .into_iter()
            })
            .take(local_var_count - 1)
            .flatten(),
        )
        .chain(vec![
            ASMInstruction::A(AValue::Numeric(local_var_count.to_string())),
            ASMInstruction::C {
                expr: "A".to_string(),
                dest: Some("D".to_string()),
                jump: None,
            },
            ASMInstruction::A(AValue::Symbolic("SP".to_string())),
            ASMInstruction::C {
                expr: "M+D".to_string(),
                dest: Some("M".to_string()),
                jump: None,
            },
        ])
        .collect()
    } else {
        iter::repeat_with(|| push_from_constant(0).into_iter())
            .take(local_var_count)
            .flatten()
            .collect()
    }
}

#[derive(Default)]
struct CodeGenerator {
    after_set_to_false_count: u32,
    return_address_count: u32,
    current_function: Option<String>,
}

impl CodeGenerator {
    fn pop_into_static_memory_segment(&self, index: u16, filename: &Path) -> Vec<ASMInstruction> {
        vec![
            pop_into_d_register("SP"),
            vec![
                ASMInstruction::A(AValue::Symbolic(format!("{}.{}", filename.to_str().unwrap(), index))),
                ASMInstruction::C {
                    expr: "D".to_string(),
                    dest: Some("M".to_string()),
                    jump: None,
                },
            ],
        ]
        .into_iter()
        .flatten()
        .collect()
    }

    fn push_from_static(&self, index: u16, filename: &Path) -> Vec<ASMInstruction> {
        vec![
            vec![
                ASMInstruction::A(AValue::Symbolic(format!("{}.{}", filename.to_str().unwrap(), index))),
                ASMInstruction::C {
                    expr: "M".to_string(),
                    dest: Some("D".to_string()),
                    jump: None,
                },
            ],
            push_from_d_register(),
        ]
        .into_iter()
        .flatten()
        .collect()
    }

    fn push(&self, segment: &MemorySegmentVariant, index: u16, filename: &Path) -> Vec<ASMInstruction> {
        match segment {
            OffsetSegment(offset_segment) => push_from_offset_memory_segment(offset_segment, index),
            PointerSegment(pointer_segment) => push_from_pointer_memory_segment(pointer_segment, index),
            Static => self.push_from_static(index, filename),
            Constant => push_from_constant(index),
        }
    }

    fn pop(&self, segment: &MemorySegmentVariant, index: u16, filename: &Path) -> Vec<ASMInstruction> {
        match segment {
            OffsetSegment(offset_segment) => pop_into_offset_memory_segment(offset_segment, index),
            PointerSegment(pointer_segment) => pop_into_pointer_memory_segment(pointer_segment, index),
            Static => self.pop_into_static_memory_segment(index, filename),
            Constant => {
                // popping into a constant doesn't make much sense - I guess it just
                // means decrement the SP but don't do anything with the popped
                // value
                vec![
                    ASMInstruction::A(AValue::Symbolic("SP".to_string())),
                    ASMInstruction::C {
                        expr: "M-1".to_string(),
                        dest: Some("M".to_string()),
                        jump: None,
                    },
                ]
            }
        }
    }

    fn compile_memory_command(&self, command: &MemoryCommandVariant, filename: &Path) -> Vec<ASMInstruction> {
        match command {
            Push(segment, index) => self.push(segment, *index, filename),
            Pop(segment, index) => self.pop(segment, *index, filename),
        }
    }

    fn compile_arithmetic_command(&mut self, command: &ArithmeticCommandVariant) -> Vec<ASMInstruction> {
        match command {
            Binary(variant) => match variant {
                Add => self.binary_operation("+"),
                And => self.binary_operation("&"),
                Or => self.binary_operation("|"),
                Sub => self.binary_operation("-"),
                Eq => self.comparative_operation("EQ"),
                Gt => self.comparative_operation("GT"),
                Lt => self.comparative_operation("LT"),
            },
            Unary(variant) => match variant {
                Neg => self.unary_operation("-"),
                Not => self.unary_operation("!"),
            },
        }
    }

    fn binary_operation(&self, operation: &str) -> Vec<ASMInstruction> {
        vec![
            ASMInstruction::A(AValue::Symbolic("SP".to_string())),
            ASMInstruction::C {
                expr: "M-1".to_string(),
                dest: Some("M".to_string()),
                jump: None,
            },
            ASMInstruction::C {
                expr: "M".to_string(),
                dest: Some("A".to_string()),
                jump: None,
            },
            ASMInstruction::C {
                expr: "M".to_string(),
                dest: Some("D".to_string()),
                jump: None,
            },
            ASMInstruction::C {
                expr: "A-1".to_string(),
                dest: Some("A".to_string()),
                jump: None,
            },
            ASMInstruction::C {
                expr: format!("M{}D", operation),
                dest: Some("M".to_string()),
                jump: None,
            },
        ]
    }

    fn unary_operation(&self, operation: &str) -> Vec<ASMInstruction> {
        vec![
            ASMInstruction::A(AValue::Symbolic("SP".to_string())),
            ASMInstruction::C {
                expr: "M-1".to_string(),
                dest: Some("A".to_string()),
                jump: None,
            },
            ASMInstruction::C {
                expr: format!("{}M", operation),
                dest: Some("M".to_string()),
                jump: None,
            },
        ]
    }

    fn comparative_operation(&mut self, operation: &str) -> Vec<ASMInstruction> {
        let jump_label = format!("$after_set_to_false_{}", self.after_set_to_false_count);
        self.after_set_to_false_count += 1;
        vec![
            // decrement stack pointer, so it's pointing to y
            ASMInstruction::A(AValue::Symbolic("SP".to_string())),
            ASMInstruction::C {
                expr: "M-1".to_string(),
                dest: Some("M".to_string()),
                jump: None,
            },
            // set both A and R7 to point to x
            ASMInstruction::C {
                expr: "M-1".to_string(),
                dest: Some("AD".to_string()),
                jump: None,
            },
            ASMInstruction::A(AValue::Symbolic("R7".to_string())),
            ASMInstruction::C {
                expr: "D".to_string(),
                dest: Some("M".to_string()),
                jump: None,
            },
            // load y into D
            ASMInstruction::A(AValue::Symbolic("SP".to_string())),
            ASMInstruction::C {
                expr: "M".to_string(),
                dest: Some("A".to_string()),
                jump: None,
            },
            ASMInstruction::C {
                expr: "M".to_string(),
                dest: Some("D".to_string()),
                jump: None,
            },
            // load x - y into D
            ASMInstruction::C {
                expr: "A-1".to_string(),
                dest: Some("A".to_string()),
                jump: None,
            },
            ASMInstruction::C {
                expr: "M-D".to_string(),
                dest: Some("D".to_string()),
                jump: None,
            },
            // initially set result to true (i.e. 0xffff i.e. -1)
            ASMInstruction::C {
                expr: "-1".to_string(),
                dest: Some("M".to_string()),
                jump: None,
            },
            // then flip to false unless condition holds
            ASMInstruction::A(AValue::Symbolic(jump_label.clone())),
            ASMInstruction::C {
                expr: "D".to_string(),
                dest: None,
                jump: Some(format!("J{}", operation)),
            },
            ASMInstruction::A(AValue::Symbolic("R7".to_string())),
            ASMInstruction::C {
                expr: "M".to_string(),
                dest: Some("A".to_string()),
                jump: None,
            },
            ASMInstruction::C {
                expr: "0".to_string(),
                dest: Some("M".to_string()),
                jump: None,
            },
            ASMInstruction::L { identifier: jump_label },
        ]
    }

    fn compile_function_call(&mut self, function_name: &str, arg_count: u16) -> Vec<ASMInstruction> {
        fn jump(function_name: &str, return_address_label: &str) -> Vec<ASMInstruction> {
            vec![
                // Jump to the callee
                ASMInstruction::A(AValue::Symbolic(format!("$entry_{}", function_name))),
                ASMInstruction::C {
                    expr: "0".to_string(),
                    dest: None,
                    jump: Some("JMP".to_string()),
                },
                // Label for return to caller
                ASMInstruction::L {
                    identifier: return_address_label.to_string(),
                },
            ]
        }

        let return_address_label = format!("$return_point_{}", self.return_address_count);
        self.return_address_count += 1;

        vec![
            load_avalue_into_register(AValue::Symbolic(return_address_label.to_string()), "R8"),
            load_avalue_into_register(AValue::Numeric((5 + arg_count).to_string()), "R9"),
            vec![
                ASMInstruction::A(AValue::Symbolic("R8".to_string())),
                ASMInstruction::C {
                    expr: "M".to_string(),
                    dest: Some("D".to_string()),
                    jump: None,
                },
            ],
            push_from_d_register(),
            vec!["LCL", "ARG", "THIS", "THAT"]
                .into_iter()
                .flat_map(|pointer| {
                    vec![
                        ASMInstruction::A(AValue::Symbolic(pointer.to_string())),
                        ASMInstruction::C {
                            expr: "M".to_string(),
                            dest: Some("D".to_string()),
                            jump: None,
                        },
                    ]
                    .into_iter()
                    .chain(push_from_d_register())
                })
                .collect(),
            // Set arg pointer - at this point, all the arguments have been pushed to the stack,
            // plus the return address, plus the four saved caller pointers.
            // So to find the correct position for ARG, we can count 5 +
            // arg_count steps back from the stack pointer.
            vec![
                ASMInstruction::A(AValue::Symbolic("SP".to_string())),
                ASMInstruction::C {
                    expr: "M".to_string(),
                    dest: Some("D".to_string()),
                    jump: None,
                },
                ASMInstruction::A(AValue::Symbolic("R9".to_string())),
                ASMInstruction::C {
                    expr: "M".to_string(),
                    dest: Some("A".to_string()),
                    jump: None,
                },
                ASMInstruction::C {
                    expr: "D-A".to_string(),
                    dest: Some("D".to_string()),
                    jump: None,
                },
                ASMInstruction::A(AValue::Symbolic("ARG".to_string())),
                ASMInstruction::C {
                    expr: "D".to_string(),
                    dest: Some("M".to_string()),
                    jump: None,
                },
            ],
            // set lcl pointer
            vec![
                ASMInstruction::A(AValue::Symbolic("SP".to_string())),
                ASMInstruction::C {
                    expr: "M".to_string(),
                    dest: Some("D".to_string()),
                    jump: None,
                },
                ASMInstruction::A(AValue::Symbolic("LCL".to_string())),
                ASMInstruction::C {
                    expr: "D".to_string(),
                    dest: Some("M".to_string()),
                    jump: None,
                },
            ],
            jump(function_name, &return_address_label),
        ]
        .into_iter()
        .flatten()
        .collect()
    }

    fn compile_function_definition(&mut self, function_name: &str, local_var_count: u16) -> Vec<ASMInstruction> {
        let result = iter::once(ASMInstruction::L {
            identifier: format!("$entry_{}", &function_name),
        })
        .chain(initialize_locals(local_var_count as usize))
        .collect();

        self.current_function = Some(function_name.to_string());

        result
    }

    fn compile_function_return(&mut self) -> Vec<ASMInstruction> {
        // This is carefully designed to not require tracking of the number of
        // arguments or locals for the callee.

        // Use R7 as a copy of ARG. We'll use this when placing the return
        // value and restoring the stack pointer. We can't use ARG directly
        // because it's going to be overwritten when restoring the caller state.
        fn copy_arg_to_r7() -> Vec<ASMInstruction> {
            vec![
                ASMInstruction::A(AValue::Symbolic("ARG".to_string())),
                ASMInstruction::C {
                    expr: "M".to_string(),
                    dest: Some("D".to_string()),
                    jump: None,
                },
                ASMInstruction::A(AValue::Symbolic("R7".to_string())),
                ASMInstruction::C {
                    expr: "D".to_string(),
                    dest: Some("M".to_string()),
                    jump: None,
                },
            ]
        }

        // Use R8 as copy of LCL. We'll use this to pop all the caller state.
        // We can't use LCL directly because LCL is one of the pieces of we're
        // restoring, so we would end up overwriting our pointer part way
        // through the process. (If LCL was the last thing to be restored we
        // would be able to get away with this, but since we want to carry on to
        // also pop the return address, it doesn't work.)
        fn copy_lcl_to_r8() -> Vec<ASMInstruction> {
            vec![
                ASMInstruction::A(AValue::Symbolic("LCL".to_string())),
                ASMInstruction::C {
                    expr: "M".to_string(),
                    dest: Some("D".to_string()),
                    jump: None,
                },
                ASMInstruction::A(AValue::Symbolic("R8".to_string())),
                ASMInstruction::C {
                    expr: "D".to_string(),
                    dest: Some("M".to_string()),
                    jump: None,
                },
            ]
        }

        fn restore_caller_state() -> Vec<ASMInstruction> {
            vec![
                pop_into_d_register("R8"),
                vec![
                    ASMInstruction::A(AValue::Symbolic("THAT".to_string())),
                    ASMInstruction::C {
                        expr: "D".to_string(),
                        dest: Some("M".to_string()),
                        jump: None,
                    },
                ],
                pop_into_d_register("R8"),
                vec![
                    ASMInstruction::A(AValue::Symbolic("THIS".to_string())),
                    ASMInstruction::C {
                        expr: "D".to_string(),
                        dest: Some("M".to_string()),
                        jump: None,
                    },
                ],
                pop_into_d_register("R8"),
                vec![
                    ASMInstruction::A(AValue::Symbolic("ARG".to_string())),
                    ASMInstruction::C {
                        expr: "D".to_string(),
                        dest: Some("M".to_string()),
                        jump: None,
                    },
                ],
                pop_into_d_register("R8"),
                vec![
                    ASMInstruction::A(AValue::Symbolic("LCL".to_string())),
                    ASMInstruction::C {
                        expr: "D".to_string(),
                        dest: Some("M".to_string()),
                        jump: None,
                    },
                ],
            ]
            .into_iter()
            .flatten()
            .collect()
        }

        // When we place the return value, we may overwrite the return address. So we have to save
        // the return address first.
        fn stash_return_address_in_r8() -> Vec<ASMInstruction> {
            vec![
                pop_into_d_register("R8"),
                vec![
                    ASMInstruction::A(AValue::Symbolic("R8".to_string())),
                    ASMInstruction::C {
                        expr: "D".to_string(),
                        dest: Some("M".to_string()),
                        jump: None,
                    },
                ],
            ]
            .into_iter()
            .flatten()
            .collect()
        }

        fn place_return_value() -> Vec<ASMInstruction> {
            vec![
                pop_into_d_register("SP"),
                vec![
                    ASMInstruction::A(AValue::Symbolic("R7".to_string())),
                    ASMInstruction::C {
                        expr: "M".to_string(),
                        dest: Some("A".to_string()),
                        jump: None,
                    },
                    ASMInstruction::C {
                        expr: "D".to_string(),
                        dest: Some("M".to_string()),
                        jump: None,
                    },
                ],
            ]
            .into_iter()
            .flatten()
            .collect()
        }

        fn restore_stack_pointer() -> Vec<ASMInstruction> {
            vec![
                ASMInstruction::A(AValue::Symbolic("R7".to_string())),
                ASMInstruction::C {
                    expr: "M".to_string(),
                    dest: Some("D".to_string()),
                    jump: None,
                },
                ASMInstruction::A(AValue::Symbolic("SP".to_string())),
                ASMInstruction::C {
                    expr: "D+1".to_string(),
                    dest: Some("M".to_string()),
                    jump: None,
                },
            ]
        }

        fn goto_return_address() -> Vec<ASMInstruction> {
            vec![
                ASMInstruction::A(AValue::Symbolic("R8".to_string())),
                ASMInstruction::C {
                    expr: "M".to_string(),
                    dest: Some("A".to_string()),
                    jump: None,
                },
                ASMInstruction::C {
                    expr: "0".to_string(),
                    dest: None,
                    jump: Some("JMP".to_string()),
                },
            ]
        }

        vec![
            copy_arg_to_r7(),
            copy_lcl_to_r8(),
            restore_caller_state(),
            stash_return_address_in_r8(),
            place_return_value(),
            restore_stack_pointer(),
            goto_return_address(),
        ]
        .into_iter()
        .flatten()
        .collect()
    }

    fn compile_function_command(&mut self, function_command: &FunctionCommandVariant) -> Vec<ASMInstruction> {
        match function_command {
            Call(function_name, arg_count) => self.compile_function_call(function_name, *arg_count),
            Define(function_name, local_var_count) => self.compile_function_definition(function_name, *local_var_count),
            ReturnFrom => self.compile_function_return(),
        }
    }

    fn compile_goto(&self, label: &str) -> Vec<ASMInstruction> {
        if let Some(current_function) = &self.current_function {
            vec![
                ASMInstruction::A(AValue::Symbolic(format!("{}${}", current_function, label))),
                ASMInstruction::C {
                    expr: "0".to_string(),
                    dest: None,
                    jump: Some("JMP".to_string()),
                },
            ]
        } else {
            panic!("not in a function definition while compiling goto label: {}", label)
        }
    }

    fn compile_label(&self, label: &str) -> Vec<ASMInstruction> {
        if let Some(current_function) = &self.current_function {
            vec![ASMInstruction::L {
                identifier: format!("{}${}", current_function, label),
            }]
        } else {
            panic!("not in a function definition while compiling label: {}", label)
        }
    }

    fn compile_ifgoto(&self, label: &str) -> Vec<ASMInstruction> {
        if let Some(current_function) = &self.current_function {
            vec![
                pop_into_d_register("SP"),
                vec![
                    ASMInstruction::A(AValue::Symbolic(format!("{}${}", current_function, label))),
                    ASMInstruction::C {
                        expr: "D".to_string(),
                        dest: None,
                        jump: Some("JNE".to_string()),
                    },
                ],
            ]
            .into_iter()
            .flatten()
            .collect()
        } else {
            panic!("not in a function definition while compiling ifgoto label: {}", label)
        }
    }

    fn compile_flow_command(&mut self, flow_command: &FlowCommandVariant) -> Vec<ASMInstruction> {
        match flow_command {
            FlowCommandVariant::GoTo(label) => self.compile_goto(label),
            FlowCommandVariant::IfGoTo(label) => self.compile_ifgoto(label),
            FlowCommandVariant::Label(label) => self.compile_label(label),
        }
    }

    fn compile_vm_command(&mut self, command: &Command, filename: &Path) -> Vec<ASMInstruction> {
        match command {
            Arithmetic(arithmetic_command) => self.compile_arithmetic_command(arithmetic_command),
            Memory(memory_command) => self.compile_memory_command(memory_command, filename),
            Function(function_command) => self.compile_function_command(function_command),
            Flow(flow_command) => self.compile_flow_command(flow_command),
        }
    }
}

#[derive(Default, Serialize, TS)]
#[ts(export)]
#[ts(export_to = "../web/bindings/")]
pub struct VMCompilerResult {
    pub sourcemap: SourceMap,
    #[ts(type = "Array<string>")]
    pub instructions: Vec<ASMInstruction>,
}

pub fn generate_asm(subroutines: &HashMap<PathBuf, Vec<CompiledSubroutine>>) -> VMCompilerResult {
    let CallGraphAnalysis {
        live_subroutines,
        pointers_to_restore,
    } = analyse_call_graph(subroutines);

    let mut sourcemap = SourceMap::new();
    let mut code_generator = CodeGenerator::default();
    let mut instructions: Vec<_> = holding_pattern();

    if live_subroutines.contains("Output.getGlyph") {
        instructions.extend(glyphs_asm());
    }

    instructions.extend(init_call_stack());

    for (filename, file_subroutines) in subroutines {
        let mut vm_command_idx = 0;
        for subroutine in file_subroutines.iter() {
            let subroutine_pointers_to_restore = pointers_to_restore
                .get(&subroutine.name)
                .unwrap_or_else(|| panic!("expected to find subroutine info for {}", subroutine.name));

            for SourcemappedCommand { command, .. } in &subroutine.commands {
                if live_subroutines.contains(&subroutine.name) {
                    for asm_instruction in code_generator.compile_vm_command(command, filename) {
                        sourcemap.record_asm_instruction(filename, vm_command_idx, instructions.len());
                        instructions.push(asm_instruction);
                    }
                }
                vm_command_idx += 1;
            }
        }
    }
    VMCompilerResult { sourcemap, instructions }
}
