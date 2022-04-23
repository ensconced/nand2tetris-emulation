use std::fs;

pub fn get_rom(file_path: &str) -> [i16; 32768] {
    let program = fs::read_to_string(file_path).unwrap();
    let mut clean_lines: Vec<String> = program
        .lines()
        .map(|line| {
            let clean_line: String = line
                .chars()
                .take_while(|ch| ch.is_ascii_digit() || ch.is_whitespace())
                .collect();
            clean_line
        })
        .filter(|line| line.len() > 0)
        .collect();

    let mut rom: [i16; 32768] = [0; 32768];
    for (idx, line) in clean_lines.iter_mut().enumerate() {
        line.retain(|ch| !ch.is_whitespace());
        let instruction = u16::from_str_radix(line, 2).unwrap() as i16;
        rom[idx] = instruction;
    }
    rom
}
