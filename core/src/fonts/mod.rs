// parser for psf font file
// a description of the file format can be found here https://www.win.tue.nl/~aeb/linux/kbd/font-formats-1.html

use std::{
    collections::HashMap,
    fs,
    iter::{self, Peekable},
};

use itertools::Itertools;

use crate::compiler::assembler::parser::{ASMInstruction, AValue};

#[derive(PartialEq, Debug)]
struct GlyphUnicodeInfo {
    individual_codepoints: Vec<u16>,
    codepoint_sequences: Vec<Vec<u16>>,
}

#[derive(PartialEq, Debug)]
pub struct Glyph {
    bitmap: [u8; 9],
    individual_codepoints: Vec<u16>,
    codepoint_sequences: Vec<Vec<u16>>,
}

const GLYPH_COUNT_IS_512_MASK: u8 = 0x01;
const HAS_UNICODE_TABLE_MASK: u8 = 0x02;

fn take_glyphs(bytes: &mut impl Iterator<Item = u8>, glyph_count: usize, glyph_height: usize) -> Vec<[u8; 9]> {
    let glyphs: Vec<_> = bytes.take(glyph_count * glyph_height).collect();
    glyphs.chunks(glyph_height).map(|sl| <[u8; 9]>::try_from(sl).unwrap()).collect()
}

fn take_codepoints(codepoints: &mut Peekable<impl Iterator<Item = u16>>) -> Vec<u16> {
    let mut result = Vec::new();
    while let Some(&codepoint) = codepoints.peek() {
        if codepoint == 0xFFFF || codepoint == 0xFFFE {
            return result;
        } else {
            result.push(codepoints.next().unwrap());
        }
    }
    result
}

fn maybe_take_codepoint_sequence(codepoints: &mut Peekable<impl Iterator<Item = u16>>) -> Option<Vec<u16>> {
    if let Some(&codepoint) = codepoints.peek() {
        if codepoint == 0xFFFE {
            codepoints.next();
            Some(take_codepoints(codepoints))
        } else {
            None
        }
    } else {
        None
    }
}

fn take_codepoint_sequences(codepoints: &mut Peekable<impl Iterator<Item = u16>>) -> Vec<Vec<u16>> {
    let mut result = Vec::new();
    while let Some(seq) = maybe_take_codepoint_sequence(codepoints) {
        result.push(seq);
    }
    result
}

fn take_term(codepoints: &mut Peekable<impl Iterator<Item = u16>>) {
    if let Some(&codepoint) = codepoints.peek() {
        if codepoint == 0xFFFF {
            codepoints.next();
        } else {
            panic!("expected 0xFFFF");
        }
    } else {
        panic!("expected 0xFFFF");
    }
}

fn take_glyph_unicode_description(codepoints: &mut Peekable<impl Iterator<Item = u16>>) -> GlyphUnicodeInfo {
    let individual_codepoints = take_codepoints(codepoints);
    let codepoint_sequences = take_codepoint_sequences(codepoints);
    take_term(codepoints);
    GlyphUnicodeInfo {
        individual_codepoints,
        codepoint_sequences,
    }
}

fn take_unicode_info(bytes: &mut impl Iterator<Item = u8>, glyph_count: usize) -> Vec<GlyphUnicodeInfo> {
    // All the remaining bytes can be considered in pairs as u16 codepoints.
    let bytes_vec: Vec<_> = bytes.collect();
    let mut codepoints = bytes_vec
        .chunks(2)
        .map(|byte_pair| u16::from_le_bytes(<[u8; 2]>::try_from(byte_pair).unwrap()))
        .peekable();

    iter::repeat_with(|| take_glyph_unicode_description(&mut codepoints))
        .take(glyph_count)
        .collect()
}

fn parse_psf_file() -> HashMap<u16, [u8; 9]> {
    let mut bytes = fs::read("./fonts/zap-vga09.psf").unwrap().into_iter();
    let magic0 = bytes.next().unwrap();
    assert_eq!(magic0, 0x36);
    let magic1 = bytes.next().unwrap();
    assert_eq!(magic1, 0x04);

    let mode_byte = bytes.next().unwrap();
    let glyph_height = bytes.next().unwrap() as usize;

    let glyph_count_is_512 = mode_byte & GLYPH_COUNT_IS_512_MASK;
    let has_unicode_table = mode_byte & HAS_UNICODE_TABLE_MASK;
    assert!(has_unicode_table != 0);

    let glyph_count = if glyph_count_is_512 == 0 { 256 } else { 512 } as usize;

    let glyphs = take_glyphs(&mut bytes, glyph_count, glyph_height);

    let unicode_info = take_unicode_info(&mut bytes, glyph_count);

    let glyphs = iter::zip(glyphs, unicode_info).map(
        |(
            bitmap,
            GlyphUnicodeInfo {
                individual_codepoints,
                codepoint_sequences,
            },
        )| Glyph {
            bitmap,
            individual_codepoints,
            codepoint_sequences,
        },
    );

    let mut result = HashMap::new();

    for glyph in glyphs {
        for codepoint in glyph.individual_codepoints.into_iter().filter(|&codepoint| codepoint < 128) {
            result.insert(codepoint, glyph.bitmap);
        }
    }

    result
}

pub fn glyphs_asm() -> Vec<ASMInstruction> {
    let glyph_map = parse_psf_file();
    let glyph_loading_asm = glyph_map
        .into_iter()
        // sort to keep things deterministic when iterating over hashmap
        .sorted()
        .flat_map(|(codepoint, bitmap)| {
            if codepoint < 32 {
                panic!("unexpected glyph for codepoint < 32");
            }
            if codepoint > 127 {
                panic!("unexpected glyph for codepoint > 127");
            }
            // The height of the glyphs is nominally 9, but the bottom line of
            // each glyph is actually always blank, at least for the subset of
            // glyphs that I'm using. This means I can ignore the remainder here
            // when converting the bytes into 16-bit chunks.
            let sixteen_bit_chunks = bitmap.chunks_exact(2);
            let words: Vec<_> = sixteen_bit_chunks
                .map(|chunk| i16::from_be_bytes(<[u8; 2]>::try_from(chunk).unwrap()))
                .collect();
            words.into_iter().flat_map(|word| {
                fn load_and_increment_address() -> impl Iterator<Item = ASMInstruction> {
                    vec![
                        ASMInstruction::A(AValue::Symbolic("R7".to_string())),
                        ASMInstruction::C {
                            expr: "M+1".to_string(),
                            dest: Some("AM".to_string()),
                            jump: None,
                        },
                    ]
                    .into_iter()
                }

                let alu_constants = vec![-1, 0, 1];

                let instructions: Vec<_> = if alu_constants.contains(&word) {
                    load_and_increment_address()
                        .chain(vec![ASMInstruction::C {
                            expr: word.to_string(),
                            dest: Some("M".to_string()),
                            jump: None,
                        }])
                        .collect()
                } else if let Some(alu_constant) = alu_constants.iter().find(|&&alu_const| !alu_const == word) {
                    load_and_increment_address()
                        .chain(vec![
                            ASMInstruction::C {
                                expr: alu_constant.to_string(),
                                dest: Some("M".to_string()),
                                jump: None,
                            },
                            ASMInstruction::C {
                                expr: "!M".to_string(),
                                dest: Some("M".to_string()),
                                jump: None,
                            },
                        ])
                        .collect()
                } else if let Some(alu_constant) = alu_constants.iter().find(|&&alu_const| -alu_const == word) {
                    load_and_increment_address()
                        .chain(vec![
                            ASMInstruction::C {
                                expr: alu_constant.to_string(),
                                dest: Some("M".to_string()),
                                jump: None,
                            },
                            ASMInstruction::C {
                                expr: "-M".to_string(),
                                dest: Some("M".to_string()),
                                jump: None,
                            },
                        ])
                        .collect()
                } else if word < 0 {
                    vec![
                        ASMInstruction::A(AValue::Numeric((!word).to_string())),
                        ASMInstruction::C {
                            expr: "!A".to_string(),
                            dest: Some("D".to_string()),
                            jump: None,
                        },
                    ]
                    .into_iter()
                    .chain(load_and_increment_address())
                    .chain(vec![ASMInstruction::C {
                        expr: "D".to_string(),
                        dest: Some("M".to_string()),
                        jump: None,
                    }])
                    .collect()
                } else {
                    vec![
                        ASMInstruction::A(AValue::Numeric((word).to_string())),
                        ASMInstruction::C {
                            expr: "A".to_string(),
                            dest: Some("D".to_string()),
                            jump: None,
                        },
                    ]
                    .into_iter()
                    .chain(load_and_increment_address())
                    .chain(vec![ASMInstruction::C {
                        expr: "D".to_string(),
                        dest: Some("M".to_string()),
                        jump: None,
                    }])
                    .collect()
                };
                instructions
            })
        });

    let prelude = vec![
        ASMInstruction::A(AValue::Symbolic("GLYPHS".to_string())),
        ASMInstruction::C {
            expr: "A-1".to_string(),
            dest: Some("D".to_string()),
            jump: None,
        },
        ASMInstruction::A(AValue::Symbolic("R7".to_string())),
        ASMInstruction::C {
            expr: "D".to_string(),
            dest: Some("M".to_string()),
            jump: None,
        },
    ];
    prelude.into_iter().chain(glyph_loading_asm).collect()
}
