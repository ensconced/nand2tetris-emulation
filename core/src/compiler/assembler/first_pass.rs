use std::collections::HashMap;

use super::parser::ASMInstruction;

#[derive(Debug)]
pub struct FirstPassResult {
    pub resolved_symbols: HashMap<String, i16>,
    pub commands_without_labels: Vec<ASMInstruction>,
}

pub fn first_pass(commands: impl Iterator<Item = ASMInstruction>) -> FirstPassResult {
    let mut resolved_symbols = HashMap::new();
    let mut commands_without_labels = Vec::new();
    let mut index = 0;
    for command in commands {
        if let ASMInstruction::L { identifier } = command {
            resolved_symbols.insert(identifier, index);
        } else {
            index += 1;
            commands_without_labels.push(command);
        }
    }
    FirstPassResult {
        resolved_symbols,
        commands_without_labels,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::assembler::parser::{parse, AValue};

    #[test]
    fn test_first_pass() {
        let commands = parse(
            "
            (foo)
             A=A+1
             M=M|A
             (bar)
             @foo
             @bar
             (baz)
             @1234",
        );
        let FirstPassResult {
            resolved_symbols,
            commands_without_labels,
        } = first_pass(commands);
        let expected_resolved_symbols = HashMap::from([("foo".to_string(), 0), ("bar".to_string(), 2), ("baz".to_string(), 4)]);
        assert_eq!(resolved_symbols, expected_resolved_symbols);

        let expected_commands_without_labels = vec![
            ASMInstruction::C {
                expr: "A+1".to_string(),
                dest: Some("A".to_string()),
                jump: None,
            },
            ASMInstruction::C {
                expr: "M|A".to_string(),
                dest: Some("M".to_string()),
                jump: None,
            },
            ASMInstruction::A(AValue::Symbolic("foo".to_string())),
            ASMInstruction::A(AValue::Symbolic("bar".to_string())),
            ASMInstruction::A(AValue::Numeric("1234".to_string())),
        ];
        assert_eq!(commands_without_labels, expected_commands_without_labels);
    }
}
