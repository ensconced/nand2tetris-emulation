pub fn from_string(source: String) -> [i16; 32768] {
    let mut rom = [0; 32768];
    for (idx, line) in source.lines().enumerate() {
        rom[idx] = u16::from_str_radix(line, 2).unwrap() as i16;
    }
    rom
}
