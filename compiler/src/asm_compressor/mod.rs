// use std::{collections::HashMap, hash::Hash};

// use super::assembler::parser::ASMInstruction;

// const SUBROUTINE_OVERHEADS: usize = 3;
// const SUBROUTINE_CALL_OVERHEADS: usize = 4;

// struct AnalysisResult {
//     // start_idx mapped onto length
//     subroutines: HashMap<usize, usize>,
//     // start_idx mapped onto start_idx of subroutine
//     subroutine_call_sites: HashMap<usize, usize>,
// }

// fn analyse_sequence(sequence: &[ASMInstruction]) -> AnalysisResult {
//     todo!()
// }

// fn add_subroutine(
//     original_sequence: &[ASMInstruction],
//     compressed_sequence: &mut [ASMInstruction],
//     subroutine_start_idx: usize,
//     subroutine_length: usize,
// ) {
//     todo!()
// }

// fn add_subroutine_call(
//     original_sequence: &[ASMInstruction],
//     compressed_sequence: &mut [ASMInstruction],
//     subroutine_start_idx: usize,
//     subroutine_length: usize,
// ) {
//     todo!()
// }

// fn compress_sequence(sequence: Vec<ASMInstruction>) -> Vec<ASMInstruction> {
//     let AnalysisResult {
//         subroutines,
//         subroutine_call_sites,
//     } = analyse_sequence(&sequence);
//     let mut result: Vec<ASMInstruction> = vec![];
//     let mut steps_remaining_within_subroutine = 0;
//     let mut steps_remaining_within_subroutine_call_site = 0;
//     for (i, instruction) in sequence.iter().enumerate() {
//         if let Some(&subroutine_length) = subroutines.get(&i) {
//             steps_remaining_within_subroutine = subroutine_length;
//             add_subroutine(&sequence, &mut result, i, subroutine_length);
//         } else if steps_remaining_within_subroutine > 0 {
//             steps_remaining_within_subroutine -= 1;
//         } else if let Some(subroutine_start_idx) = subroutine_call_sites.get(&i) {
//             let &subroutine_length = subroutines
//                 .get(subroutine_start_idx)
//                 .expect("failed to find subroutine at subroutine call site");
//             steps_remaining_within_subroutine_call_site = subroutine_length;
//             add_subroutine_call(&sequence, &mut result, i, subroutine_length);
//         } else if steps_remaining_within_subroutine_call_site > 0 {
//             steps_remaining_within_subroutine_call_site -= 1;
//         } else {
//             result.push(instruction.clone());
//         }
//     }
//     result
// }

// // fn find_existing_matching_subroutine<'a, 'b>(
// //     instructions: &'a [ASMInstruction],
// //     index: usize,
// //     subroutines: &'b Subroutines,
// // ) -> Option<&'b Subroutine> {
// //     if let Some(subroutine) = subroutines.get(&index) {
// //         let subroutine_instructions = instructions.iter().skip(subroutine.start_idx).take(subroutine.length);
// //         if instructions
// //             .iter()
// //             .skip(index)
// //             .zip(subroutine_instructions)
// //             .all(|(this_instruction, that_instruction)| this_instruction == that_instruction)
// //         {
// //             return Some(subroutine);
// //         }
// //     }
// //     None
// // }

// // fn find_all_matching_sequences<'a>(
// //     instructions: &'a [ASMInstruction],
// //     index: usize,
// //     subroutines: &'a Subroutines,
// // ) -> impl Iterator<Item = Subroutine> + 'a {
// //     (0..index).filter_map(move |start_idx| {
// //         let match_len = instructions
// //             .iter()
// //             .skip(start_idx)
// //             .take(index - start_idx)
// //             .zip(instructions.iter().enumerate().skip(index))
// //             .take_while(|(other_instruction, (idx, instruction))| instruction == other_instruction && !folded_sequences.contains_key(idx))
// //             .count();
// //         if match_len == 0 {
// //             None
// //         } else {
// //             Some(Subroutine {
// //                 start_idx,
// //                 length: match_len,
// //             })
// //         }
// //     })
// // }

// // fn find_best_match<'a, 'b>(instructions: &'a [ASMInstruction], index: usize, subroutines: &'b Subroutines) -> Option<&'b Subroutine> {
// //     find_existing_matching_subroutine(instructions, index, subroutines).or_else(|| {
// //         find_all_matching_sequences(instructions, index, subroutines)
// //             .into_iter()
// //             .max_by_key(|seq| seq.length)
// //     })
// // }

// // fn apply_compression_at(instructions: &[ASMInstruction], index: usize, folded_sequences: &mut Subroutines) {
// //     if let Some(Subroutine { start_idx, length }) = find_best_match(instructions, index, folded_sequences) {}
// // }

// // #[cfg(test)]
// // mod tests {
// //     use std::collections::HashMap;

// //     use crate::{
// //         asm_compressor::{find_best_match, Subroutine},
// //         assembler::parser::{ASMInstruction, AValue},
// //     };

// //     use super::find_all_matching_sequences;

// //     #[test]
// //     fn test_find_all_matching_sequences() {
// //         let instructions = vec![
// //             ASMInstruction::A(AValue::Numeric("1".to_string())),
// //             ASMInstruction::A(AValue::Numeric("2".to_string())),
// //             ASMInstruction::A(AValue::Numeric("3".to_string())),
// //             ASMInstruction::A(AValue::Numeric("4".to_string())),
// //             ASMInstruction::A(AValue::Numeric("5".to_string())),
// //             ASMInstruction::A(AValue::Numeric("7".to_string())),
// //             ASMInstruction::A(AValue::Numeric("4".to_string())),
// //             ASMInstruction::A(AValue::Numeric("5".to_string())),
// //             ASMInstruction::A(AValue::Numeric("1".to_string())),
// //             ASMInstruction::A(AValue::Numeric("2".to_string())),
// //             ASMInstruction::A(AValue::Numeric("4".to_string())),
// //             ASMInstruction::A(AValue::Numeric("5".to_string())),
// //             ASMInstruction::A(AValue::Numeric("7".to_string())),
// //         ];
// //         let folded_sequences = HashMap::new();
// //         assert_eq!(
// //             find_all_matching_sequences(&instructions, 10, &folded_sequences).collect::<Vec<_>>(),
// //             vec![Subroutine { start_idx: 3, length: 3 }, Subroutine { start_idx: 6, length: 2 }]
// //         );
// //     }

// //     fn test_find_best_match() {
// //         let instructions = vec![
// //             ASMInstruction::A(AValue::Numeric("1".to_string())),
// //             ASMInstruction::A(AValue::Numeric("2".to_string())),
// //             ASMInstruction::A(AValue::Numeric("3".to_string())),
// //             ASMInstruction::A(AValue::Numeric("4".to_string())),
// //             ASMInstruction::A(AValue::Numeric("5".to_string())),
// //             ASMInstruction::A(AValue::Numeric("7".to_string())),
// //             ASMInstruction::A(AValue::Numeric("4".to_string())),
// //             ASMInstruction::A(AValue::Numeric("5".to_string())),
// //             ASMInstruction::A(AValue::Numeric("1".to_string())),
// //             ASMInstruction::A(AValue::Numeric("2".to_string())),
// //             ASMInstruction::A(AValue::Numeric("4".to_string())),
// //             ASMInstruction::A(AValue::Numeric("5".to_string())),
// //             ASMInstruction::A(AValue::Numeric("7".to_string())),
// //         ];
// //         let folded_sequences = HashMap::new();
// //         assert_eq!(
// //             find_best_match(&instructions, 10, &folded_sequences),
// //             Some(Subroutine { start_idx: 3, length: 3 })
// //         );
// //     }
// // }
