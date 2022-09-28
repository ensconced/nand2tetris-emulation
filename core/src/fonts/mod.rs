// parser for psf font file
// a description of the file format can be found here https://www.win.tue.nl/~aeb/linux/kbd/font-formats-1.html

use std::{
    collections::HashMap,
    fs,
    iter::{self, Peekable},
};

use itertools::Itertools;

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

fn safe_jack_number_string(num: i16) -> String {
    if num == -32768 {
        // This is a special case because while -32768 is a valid i16, 32768 is not.
        // If the jack compiler sees -32768, it treats that as a unary neg operation on
        // a positive int 32768. But then it will realise that 32768 is not a valid i16
        // and panic. To get around this we use an overflow.
        "32767 + 1".to_string()
    } else {
        num.to_string()
    }
}

pub fn glyphs_class() -> String {
    let glyph_map = parse_psf_file();
    let glyph_count = glyph_map.len();
    let glyph_allocations: Vec<_> = glyph_map
        .into_iter()
        // sort to keep things deterministic when iterating over hashmap
        .sorted()
        .map(|(codepoint, bitmap)| {
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
            let words = sixteen_bit_chunks.map(|chunk| safe_jack_number_string(i16::from_be_bytes(<[u8; 2]>::try_from(chunk).unwrap())));
            words
                .enumerate()
                .map(|(word_idx, word)| format!("            let bitmap[{}] = {};", 4 * (codepoint - 32) + (word_idx as u16), word))
                .join("\n")
        })
        .collect();

    format!(
        "
    class Glyphs {{
        static int arr;

        function void init() {{
            var int bitmap;

            // I'm taking advantage here of the fact that the first 32 ascii
            // characters do not have glyphs in my font.
            let bitmap = Memory.alloc({});
{}
        }}

        function int glyph(int codepoint) {{
            var int glyph_ptr;

            if (codepoint < 32 | codepoint > 127) {{
                return 0;
            }}
            return arr + 4 * (codepoint - 32);
        }}
    }}
    ",
        glyph_count * 4,
        glyph_allocations.join("\n")
    )
}

#[cfg(test)]

mod tests {
    use std::num::Wrapping;

    use crate::compilers::{compile_to_machine_code, utils::source_modules::mock_from_sources};

    use super::*;
    #[test]
    fn test_glyph_module_compiles() {
        // just check that the output compiles
        compile_to_machine_code(mock_from_sources(vec![&glyphs_class()]));
    }

    #[test]
    fn test_number_wrapping_strategy() {
        assert_eq!(Wrapping(32767_i16) + Wrapping(1), Wrapping(-32768));
    }
}
