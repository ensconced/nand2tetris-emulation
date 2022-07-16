// parser for psf font file
// a description of the file format can be found here https://www.win.tue.nl/~aeb/linux/kbd/font-formats-1.html

use std::{
    fs,
    iter::{self, Peekable},
};

#[derive(PartialEq, Debug)]
struct GlyphUnicodeInfo {
    individual_codepoints: Vec<u16>,
    codepoint_sequences: Vec<Vec<u16>>,
}

#[derive(PartialEq, Debug)]
pub struct PSF {
    glyphs: Vec<[u8; 9]>,
    unicode_info: Vec<GlyphUnicodeInfo>,
}

const GLYPH_COUNT_IS_512_MASK: u8 = 0x01;
const HAS_UNICODE_TABLE_MASK: u8 = 0x02;

fn take_glyphs(
    bytes: &mut impl Iterator<Item = u8>,
    glyph_count: usize,
    glyph_height: usize,
) -> Vec<[u8; 9]> {
    let glyphs: Vec<_> = bytes.take(glyph_count * glyph_height).collect();
    glyphs
        .chunks(glyph_height)
        .map(|sl| <[u8; 9]>::try_from(sl).unwrap())
        .collect()
}

fn take_u16(bytes: &mut impl Iterator<Item = u8>) -> Option<u16> {
    bytes.next().map(|less_sig_byte| {
        let more_sig_byte = bytes.next().unwrap();
        u16::from_le_bytes([less_sig_byte, more_sig_byte])
    })
}

fn take_codepoints_up_to(bytes: &mut impl Iterator<Item = u8>, end: u16) -> Vec<u16> {
    let mut result = Vec::new();
    while let Some(codepoint) = take_u16(bytes) {
        if codepoint == end {
            return result;
        } else {
            result.push(codepoint);
        }
    }
    result
}

fn take_codepoints(codepoints: &mut iter::Peekable<impl Iterator<Item = u16>>) -> Vec<u16> {
    let mut result = Vec::new();
    while let Some(&codepoint) = codepoints.peek() {
        if codepoint == 0xFFFF || codepoint == 0xFFFE {
            return result;
        } else {
            result.push(codepoints.next().unwrap());
        }
    }
    return result;
}

fn maybe_take_codepoint_sequence(
    codepoints: &mut iter::Peekable<impl Iterator<Item = u16>>,
) -> Option<Vec<u16>> {
    if let Some(&codepoint) = codepoints.peek() {
        if codepoint == 0xFFFE {
            codepoints.next();
            Some(take_codepoints(codepoints))
        } else {
            panic!("expected 0xFFFE");
        }
    } else {
        None
    }
}

fn take_codepoint_sequences(
    codepoints: &mut iter::Peekable<impl Iterator<Item = u16>>,
) -> Vec<Vec<u16>> {
    let mut result = Vec::new();
    while let Some(seq) = maybe_take_codepoint_sequence(codepoints) {
        result.push(seq);
    }
    result
}

fn take_glyph_unicode_description(
    codepoints: &mut iter::Peekable<impl Iterator<Item = u16>>,
) -> GlyphUnicodeInfo {
    let individual_codepoints = take_codepoints(codepoints);
    let codepoint_sequences = take_codepoint_sequences(codepoints);
    GlyphUnicodeInfo {
        individual_codepoints,
        codepoint_sequences,
    }
}

fn take_unicode_info(
    bytes: &mut impl Iterator<Item = u8>,
    glyph_count: usize,
) -> Vec<GlyphUnicodeInfo> {
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

pub fn parse_psf_file() -> PSF {
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

    PSF {
        glyphs,
        unicode_info,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_psf_file() {
        let psf = parse_psf_file();
        for glyph in psf.glyphs {
            for line in glyph {
                for bit_idx in (0..8).rev() {
                    if 2_u8.pow(bit_idx) & line == 0 {
                        print!(" ");
                    } else {
                        print!("█");
                    }
                }
                println!();
            }
        }
    }
}
