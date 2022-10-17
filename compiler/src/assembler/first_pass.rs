use std::collections::HashMap;

use super::parser::ASMInstruction;

#[derive(Debug)]
pub struct FirstPassResult<'a> {
    pub resolved_symbols: HashMap<&'a str, i16>,
}

pub fn first_pass(commands: &[ASMInstruction]) -> FirstPassResult {
    let mut resolved_symbols = HashMap::new();
    let mut index: i16 = 0;
    for command in commands {
        if let ASMInstruction::L { identifier } = command {
            resolved_symbols.insert(identifier.as_str(), index);
        } else {
            index += 1;
        }
    }
    FirstPassResult { resolved_symbols }
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
        let FirstPassResult { resolved_symbols } = first_pass(&commands);
        assert_eq!(resolved_symbols, HashMap::from([("foo", 0), ("bar", 2), ("baz", 4)]));
    }
}
