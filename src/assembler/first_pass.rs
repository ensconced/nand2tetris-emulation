use std::collections::HashMap;

use super::parser::Command;

pub struct FirstPassResult {
    pub resolved_symbols: HashMap<String, usize>,
    pub commands_without_labels: Vec<Command>,
}

pub fn first_pass(mut commands: impl Iterator<Item = Command>) -> FirstPassResult {
    let mut resolved_symbols = HashMap::new();
    let mut commands_without_labels = Vec::new();
    let mut index = 0;
    while let Some(command) = commands.next() {
        if let Command::LCommand { identifier } = command {
            resolved_symbols.insert(identifier, index);
        } else {
            index = index + 1;
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
    use crate::assembler::parser::{parse_lines, AValue};

    #[test]
    fn test_first_pass() {
        let commands = parse_lines(
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
        } = first_pass(commands.into_iter());
        let expected_resolved_symbols = HashMap::from([
            ("foo".to_string(), 0 as usize),
            ("bar".to_string(), 2 as usize),
            ("baz".to_string(), 4 as usize),
        ]);
        assert_eq!(resolved_symbols, expected_resolved_symbols);

        let expected_commands_without_labels = vec![
            Command::CCommand {
                expr: "A+1".to_string(),
                dest: Some("A".to_string()),
                jump: None,
            },
            Command::CCommand {
                expr: "M|A".to_string(),
                dest: Some("M".to_string()),
                jump: None,
            },
            Command::ACommand(AValue::Symbolic("foo".to_string())),
            Command::ACommand(AValue::Symbolic("bar".to_string())),
            Command::ACommand(AValue::Numeric("1234".to_string())),
        ];
        assert_eq!(commands_without_labels, expected_commands_without_labels);
    }
}
