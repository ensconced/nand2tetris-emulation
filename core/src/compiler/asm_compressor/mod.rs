use std::collections::HashMap;

use super::assembler::parser::ASMInstruction;

#[derive(Debug, PartialEq)]
struct Subroutine {
    start_idx: usize,
    length: usize,
}

type Subroutines = HashMap<usize, Subroutine>;

fn find_existing_matching_subroutine<'a, 'b>(
    instructions: &'a [ASMInstruction],
    index: usize,
    subroutines: &'b Subroutines,
) -> Option<&'b Subroutine> {
    if let Some(subroutine) = subroutines.get(&index) {
        let subroutine_instructions = instructions.iter().skip(subroutine.start_idx).take(subroutine.length);
        if instructions
            .iter()
            .skip(index)
            .zip(subroutine_instructions)
            .all(|(this_instruction, that_instruction)| this_instruction == that_instruction)
        {
            return Some(subroutine);
        }
    }
    None
}

fn find_all_matching_sequences<'a>(
    instructions: &'a [ASMInstruction],
    index: usize,
    subroutines: &'a Subroutines,
) -> impl Iterator<Item = Subroutine> + 'a {
    if let Some(subroutine) = find_existing_matching_subroutine(instructions, index, subroutines) {

        // use the existing compressed section
    } else {
        // see if we can make a new compressed section
    }

    (0..index).filter_map(move |start_idx| {
        let match_len = instructions
            .iter()
            .skip(start_idx)
            .take(index - start_idx)
            .zip(instructions.iter().enumerate().skip(index))
            .take_while(|(other_instruction, (idx, instruction))| instruction == other_instruction && !folded_sequences.contains_key(idx))
            .count();
        if match_len == 0 {
            None
        } else {
            Some(Subroutine {
                start_idx,
                length: match_len,
            })
        }
    })
}

fn find_best_match(instructions: &[ASMInstruction], index: usize, folded_sequences: &Subroutines) -> Option<Subroutine> {
    find_all_matching_sequences(instructions, index, folded_sequences)
        .into_iter()
        .max_by_key(|seq| seq.length)
}

fn apply_compression_at(instructions: &[ASMInstruction], index: usize, folded_sequences: &mut Subroutines) {
    if let Some(Subroutine { start_idx, length }) = find_best_match(instructions, index, folded_sequences) {}
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::compiler::{
        asm_compressor::{find_best_match, Subroutine},
        assembler::parser::{ASMInstruction, AValue},
    };

    use super::find_all_matching_sequences;

    #[test]
    fn test_find_all_matching_sequences() {
        let instructions = vec![
            ASMInstruction::A(AValue::Numeric("1".to_string())),
            ASMInstruction::A(AValue::Numeric("2".to_string())),
            ASMInstruction::A(AValue::Numeric("3".to_string())),
            ASMInstruction::A(AValue::Numeric("4".to_string())),
            ASMInstruction::A(AValue::Numeric("5".to_string())),
            ASMInstruction::A(AValue::Numeric("7".to_string())),
            ASMInstruction::A(AValue::Numeric("4".to_string())),
            ASMInstruction::A(AValue::Numeric("5".to_string())),
            ASMInstruction::A(AValue::Numeric("1".to_string())),
            ASMInstruction::A(AValue::Numeric("2".to_string())),
            ASMInstruction::A(AValue::Numeric("4".to_string())),
            ASMInstruction::A(AValue::Numeric("5".to_string())),
            ASMInstruction::A(AValue::Numeric("7".to_string())),
        ];
        let folded_sequences = HashMap::new();
        assert_eq!(
            find_all_matching_sequences(&instructions, 10, &folded_sequences).collect::<Vec<_>>(),
            vec![Subroutine { start_idx: 3, length: 3 }, Subroutine { start_idx: 6, length: 2 }]
        );
    }

    fn test_find_best_match() {
        let instructions = vec![
            ASMInstruction::A(AValue::Numeric("1".to_string())),
            ASMInstruction::A(AValue::Numeric("2".to_string())),
            ASMInstruction::A(AValue::Numeric("3".to_string())),
            ASMInstruction::A(AValue::Numeric("4".to_string())),
            ASMInstruction::A(AValue::Numeric("5".to_string())),
            ASMInstruction::A(AValue::Numeric("7".to_string())),
            ASMInstruction::A(AValue::Numeric("4".to_string())),
            ASMInstruction::A(AValue::Numeric("5".to_string())),
            ASMInstruction::A(AValue::Numeric("1".to_string())),
            ASMInstruction::A(AValue::Numeric("2".to_string())),
            ASMInstruction::A(AValue::Numeric("4".to_string())),
            ASMInstruction::A(AValue::Numeric("5".to_string())),
            ASMInstruction::A(AValue::Numeric("7".to_string())),
        ];
        let folded_sequences = HashMap::new();
        assert_eq!(
            find_best_match(&instructions, 10, &folded_sequences),
            Some(Subroutine { start_idx: 3, length: 3 })
        );
    }
}
