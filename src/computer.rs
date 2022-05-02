const DEBUG: bool = false;

pub fn bit(instruction: i16, idx: u32) -> u16 {
    (instruction as u16 & (2u16).pow(idx)) >> idx
}

fn comp_bits(instruction: i16) -> i16 {
    (instruction >> 6) & 0b1_111111
}

struct Cpu {
    reg_a: i16,
    reg_d: i16,
    out_m: i16,
    pc: i16,
    memory_load: bool,
}

impl Cpu {
    fn execute(&mut self, instruction: i16, in_m: i16) {
        if bit(instruction, 15) == 0 {
            // A Instruction
            self.reg_a = instruction;
            self.pc = self.pc + 1;
            self.memory_load = false;
        } else {
            // C Instruction
            let alu_out = match comp_bits(instruction) {
                0b0_101010 => 0,
                0b0_111111 => 1,
                0b0_111010 => -1,
                0b0_001100 => self.reg_d,
                0b0_110000 => self.reg_a,
                0b0_001101 => !self.reg_d,
                0b0_110001 => !self.reg_a,
                0b0_001111 => -self.reg_d,
                0b0_110011 => -self.reg_a,
                0b0_011111 => self.reg_d + 1,
                0b0_110111 => self.reg_a + 1,
                0b0_001110 => self.reg_d - 1,
                0b0_110010 => self.reg_a - 1,
                0b0_000010 => self.reg_d + self.reg_a,
                0b0_010011 => self.reg_d - self.reg_a,
                0b0_000111 => self.reg_a - self.reg_d,
                0b0_000000 => self.reg_d & self.reg_a,
                0b0_010101 => self.reg_d | self.reg_a,
                0b1_110000 => in_m,
                0b1_110001 => !in_m,
                0b1_110011 => -in_m,
                0b1_110111 => in_m + 1,
                0b1_110010 => in_m - 1,
                0b1_000010 => self.reg_d + in_m,
                0b1_010011 => self.reg_d - in_m,
                0b1_000111 => in_m - self.reg_d,
                0b1_000000 => self.reg_d & in_m,
                0b1_010101 => self.reg_d | in_m,
                _ => panic!("bad instruction"),
            };
            if (bit(instruction, 0) == 1 && alu_out > 0)
                || (bit(instruction, 1) == 1 && alu_out == 0)
                || (bit(instruction, 2) == 1 && alu_out < 0)
            {
                self.pc = self.reg_a;
            } else {
                self.pc = self.pc + 1;
            }
            self.memory_load = bit(instruction, 3) == 1;
            if self.memory_load {
                self.out_m = alu_out;
            }
            if bit(instruction, 4) == 1 {
                self.reg_d = alu_out;
            }
            if bit(instruction, 5) == 1 {
                self.reg_a = alu_out;
            }
        }
    }
}
pub struct Computer {
    rom: [i16; 32768],
    pub ram: [i16; 32768],
    cpu: Cpu,
}

impl Computer {
    pub fn new(rom: [i16; 32768]) -> Self {
        Self {
            rom,
            ram: [0; 32768],
            cpu: Cpu {
                reg_a: 0,
                reg_d: 0,
                pc: 0,
                out_m: 0,
                memory_load: false,
            },
        }
    }
    pub fn tick(&mut self) {
        let instruction = self.rom[self.cpu.pc as usize];
        if DEBUG {
            println!(
                "pc: {}, instruction: {:016b}, reg_a: {}, reg_d: {}, R0: {}, R1: {}",
                self.cpu.pc, instruction, self.cpu.reg_a, self.cpu.reg_d, self.ram[0], self.ram[1]
            );
        }
        let in_m = self.ram[self.cpu.reg_a as usize % self.ram.len()];
        self.cpu.execute(instruction, in_m);
        if self.cpu.memory_load {
            self.ram[self.cpu.reg_a as usize] = self.cpu.out_m;
        }
    }
    pub fn screen_output(&self) -> &[i16] {
        &self.ram[16384..24576]
    }
    pub fn kbd_output(&mut self) -> &mut i16 {
        &mut self.ram[24576]
    }
}

#[cfg(test)]
#[allow(overflowing_literals, unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_get_bit() {
        assert_eq!(bit(0b0000000000000000, 0), 0);
        assert_eq!(bit(0b0000000000000001, 0), 1);
        assert_eq!(bit(0b0000000000000010, 1), 1);
        assert_eq!(bit(0b0000000000000100, 2), 1);
        assert_eq!(bit(0b0000000000001000, 3), 1);
        assert_eq!(bit(0b0000000000010000, 4), 1);
        assert_eq!(bit(0b0000000000011111, 4), 1);
        assert_eq!(bit(0b0000000000010001, 3), 0);
    }

    #[test]
    fn test_get_comp_bits() {
        assert_eq!(comp_bits(0b1110_101010_010_111), 0b0_101010);
        assert_eq!(comp_bits(0b1111_010101_111_010), 0b1_010101);
    }
}
