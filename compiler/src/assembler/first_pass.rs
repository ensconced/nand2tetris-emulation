use std::collections::HashMap;

use super::parser::ASMInstruction;

#[derive(Debug)]
pub struct FirstPassResult<'a> {
    pub resolved_symbols: HashMap<&'a str, i16>,
    pub commands_without_labels: Vec<&'a ASMInstruction>,
}

pub fn first_pass<'a>(commands: &'a [ASMInstruction]) -> FirstPassResult<'a> {
    let mut resolved_symbols = HashMap::new();
    let mut commands_without_labels = Vec::new();
    let mut index: i16 = 0;
    for command in commands {
        if let ASMInstruction::L { identifier } = command {
            resolved_symbols.insert(identifier.as_str(), index);
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
    use crate::assembler::parser::{parse, AValue};

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
        } = first_pass(&commands);
        assert_eq!(resolved_symbols, HashMap::from([("foo", 0), ("bar", 2), ("baz", 4)]));

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
        let ref_vec: Vec<_> = expected_commands_without_labels.iter().collect();
        assert_eq!(commands_without_labels, ref_vec);
    }
}
