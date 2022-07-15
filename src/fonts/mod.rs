// parser for psf font file
// a description of the file format can be found here https://www.win.tue.nl/~aeb/linux/kbd/font-formats-1.html

use std::fs;

#[derive(PartialEq, Debug)]
struct UnicodeInfo {}

#[derive(PartialEq, Debug)]
pub struct PSF {
    glyphs: Vec<[u8; 9]>,
    unicode_info: Option<UnicodeInfo>,
}

const GLYPH_COUNT_IS_512_MASK: u8 = 0x01;
const HAS_UNICODE_TABLE_MASK: u8 = 0x02;

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
    assert_eq!(has_unicode_table, 0);

    let glyph_count = if glyph_count_is_512 == 0 { 256 } else { 512 } as usize;

    let glyphs: Vec<_> = bytes.take(glyph_count * glyph_height).collect();

    PSF {
        glyphs: glyphs
            .chunks(glyph_height)
            .map(|sl| <[u8; 9]>::try_from(sl).unwrap())
            .collect(),
        unicode_info: None,
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
