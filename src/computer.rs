use std::sync::{Arc, Mutex};

const DEBUG: bool = false;

pub fn bit(instruction: i16, idx: u32) -> u16 {
    (instruction as u16 & (2u16).pow(idx)) >> idx
}

fn comp_bits(instruction: i16) -> i16 {
    (instruction >> 6) & 0b1111111
}

pub struct Cpu {
    pub reg_a: i16,
    pub reg_d: i16,
    out_m: i16,
    pc: i16,
    memory_load: bool,
}

impl Cpu {
    fn execute(&mut self, instruction: i16, in_m: i16) {
        if bit(instruction, 15) == 0 {
            // A Instruction
            self.reg_a = instruction;
            self.pc += 1;
            self.memory_load = false;
        } else {
            // C Instruction
            let alu_out = match comp_bits(instruction) {
                0b0101010 => 0,
                0b0111111 => 1,
                0b0111010 => -1,
                0b0001100 => self.reg_d,
                0b0110000 => self.reg_a,
                0b0001101 => !self.reg_d,
                0b0110001 => !self.reg_a,
                0b0001111 => -self.reg_d,
                0b0110011 => -self.reg_a,
                0b0011111 => self.reg_d + 1,
                0b0110111 => self.reg_a + 1,
                0b0001110 => self.reg_d - 1,
                0b0110010 => self.reg_a - 1,
                0b0000010 => self.reg_d + self.reg_a,
                0b0010011 => self.reg_d - self.reg_a,
                0b0000111 => self.reg_a - self.reg_d,
                0b0000000 => self.reg_d & self.reg_a,
                0b0010101 => self.reg_d | self.reg_a,
                0b1110000 => in_m,
                0b1110001 => !in_m,
                0b1110011 => -in_m,
                0b1110111 => in_m + 1,
                0b1110010 => in_m - 1,
                0b1000010 => self.reg_d + in_m,
                0b1010011 => self.reg_d - in_m,
                0b1000111 => in_m - self.reg_d,
                0b1000000 => self.reg_d & in_m,
                0b1010101 => self.reg_d | in_m,
                _ => panic!("bad instruction"),
            };
            if (bit(instruction, 0) == 1 && alu_out > 0)
                || (bit(instruction, 1) == 1 && alu_out == 0)
                || (bit(instruction, 2) == 1 && alu_out < 0)
            {
                self.pc = self.reg_a;
            } else {
                self.pc += 1;
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
    pub ram: Arc<Mutex<[i16; 32768]>>,
    pub cpu: Cpu,
}

impl Computer {
    pub fn new(rom: [i16; 32768]) -> Self {
        Self {
            rom,
            ram: Arc::new(Mutex::new([0; 32768])),
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
        let prev_reg_a = self.cpu.reg_a;
        let instruction = self.rom[self.cpu.pc as usize];
        if DEBUG {
            let ram = self.ram.lock().unwrap();
            println!(
                "pc: {}, instruction: {:016b}, reg_a: {}, reg_d: {}, R0: {}, R1: {}, R2: {}, R3: {}, R4: {}, R5: {}, R6: {}",
                self.cpu.pc, instruction, self.cpu.reg_a, self.cpu.reg_d, ram[0], ram[1],ram[2],ram[3],ram[4],ram[5],ram[6]
            );
        }
        let addr = self.cpu.reg_a as usize % self.ram.lock().unwrap().len();
        let in_m = self.ram.lock().unwrap()[addr];
        self.cpu.execute(instruction, in_m);
        if self.cpu.memory_load {
            self.ram.lock().unwrap()[prev_reg_a as usize] = self.cpu.out_m;
        }
    }

    pub fn tick_until(&mut self, predicate: &dyn Fn(&Computer) -> bool) {
        let max_ticks = 10000;
        for _ in 0..=max_ticks {
            if predicate(self) {
                return;
            }
            self.tick();
        }
        panic!("predicate was not true within {} ticks", max_ticks);
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
        assert_eq!(comp_bits(0b1110101010010111), 0b0101010);
        assert_eq!(comp_bits(0b1111010101111010), 0b1010101);
    }
}
