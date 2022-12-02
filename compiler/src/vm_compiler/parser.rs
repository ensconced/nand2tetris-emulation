use std::fmt::{Display, Formatter};

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum UnaryArithmeticCommandVariant {
    Neg,
    Not,
}

impl Display for UnaryArithmeticCommandVariant {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self {
            UnaryArithmeticCommandVariant::Neg => write!(f, "neg"),
            UnaryArithmeticCommandVariant::Not => write!(f, "not"),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum BinaryArithmeticCommandVariant {
    Add,
    Sub,
    Eq,
    Gt,
    Lt,
    And,
    Or,
}

impl Display for BinaryArithmeticCommandVariant {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self {
            BinaryArithmeticCommandVariant::Add => write!(f, "add"),
            BinaryArithmeticCommandVariant::Sub => write!(f, "sub"),
            BinaryArithmeticCommandVariant::Eq => write!(f, "eq"),
            BinaryArithmeticCommandVariant::Gt => write!(f, "gt"),
            BinaryArithmeticCommandVariant::Lt => write!(f, "lt"),
            BinaryArithmeticCommandVariant::And => write!(f, "and"),
            BinaryArithmeticCommandVariant::Or => write!(f, "or"),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ArithmeticCommandVariant {
    Unary(UnaryArithmeticCommandVariant),
    Binary(BinaryArithmeticCommandVariant),
}

impl Display for ArithmeticCommandVariant {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self {
            ArithmeticCommandVariant::Unary(unary_cmd) => write!(f, "{}", unary_cmd),
            ArithmeticCommandVariant::Binary(binary_cmd) => write!(f, "{}", binary_cmd),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum MemoryCommandVariant {
    Push(MemorySegmentVariant, u16),
    Pop(MemorySegmentVariant, u16),
}

impl Display for MemoryCommandVariant {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self {
            MemoryCommandVariant::Push(memory_segment, offset) => {
                write!(f, "push {} {}", memory_segment, offset)
            }
            MemoryCommandVariant::Pop(memory_segment, offset) => {
                write!(f, "pop {} {}", memory_segment, offset)
            }
        }
    }
}

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub enum PointerSegmentVariant {
    Argument,
    Local,
    This,
    That,
}

impl Display for PointerSegmentVariant {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self {
            PointerSegmentVariant::Argument => write!(f, "ARG"),
            PointerSegmentVariant::Local => write!(f, "LCL"),
            PointerSegmentVariant::This => write!(f, "THIS"),
            PointerSegmentVariant::That => write!(f, "THAT"),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum OffsetSegmentVariant {
    Pointer,
    Temp,
}

impl Display for OffsetSegmentVariant {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self {
            OffsetSegmentVariant::Pointer => write!(f, "pointer"),
            OffsetSegmentVariant::Temp => write!(f, "temp"),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum MemorySegmentVariant {
    PointerSegment(PointerSegmentVariant),
    OffsetSegment(OffsetSegmentVariant),
    Static,
    Constant,
}

impl Display for MemorySegmentVariant {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self {
            MemorySegmentVariant::PointerSegment(pointer_segment) => {
                write!(f, "{}", pointer_segment)
            }
            MemorySegmentVariant::OffsetSegment(offset_segment) => write!(f, "{}", offset_segment),
            MemorySegmentVariant::Static => write!(f, "static"),
            MemorySegmentVariant::Constant => write!(f, "constant"),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum FlowCommandVariant {
    GoTo(String),
    Label(String),
    IfGoTo(String),
}

impl Display for FlowCommandVariant {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self {
            FlowCommandVariant::GoTo(label) => write!(f, "goto {}", label),
            FlowCommandVariant::Label(label) => write!(f, "label {}", label),
            FlowCommandVariant::IfGoTo(label) => write!(f, "if-goto {}", label),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum FunctionCommandVariant {
    Define(String, u16),
    Call(String, u16),
    ReturnFrom,
}

impl Display for FunctionCommandVariant {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self {
            FunctionCommandVariant::Define(fn_name, locals_count) => {
                write!(f, "function {} {}", fn_name, locals_count)
            }
            FunctionCommandVariant::Call(fn_name, arg_count) => {
                write!(f, "call {} {}", fn_name, arg_count)
            }
            FunctionCommandVariant::ReturnFrom => write!(f, "return"),
        }
    }
}

#[derive(Clone, Serialize, PartialEq, Eq, Debug)]
#[serde(into = "String")]
pub enum Command {
    Function(FunctionCommandVariant),
    Flow(FlowCommandVariant),
    Arithmetic(ArithmeticCommandVariant),
    Memory(MemoryCommandVariant),
}

impl Display for Command {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Command::Function(function_cmd) => write!(f, "{}", function_cmd),
            Command::Flow(flow_cmd) => write!(f, "{}", flow_cmd),
            Command::Arithmetic(arithmetic_cmd) => write!(f, "{}", arithmetic_cmd),
            Command::Memory(memory_cmd) => write!(f, "{}", memory_cmd),
        }
    }
}

impl From<Command> for String {
    fn from(command: Command) -> Self {
        command.to_string()
    }
}

use serde::Serialize;
