use std::{
    num::Wrapping,
    sync::{Arc, Mutex, MutexGuard},
};

use wasm_bindgen::prelude::wasm_bindgen;

pub fn bit(instruction: u16, idx: u32) -> u16 {
    (instruction & (2u16).pow(idx)) >> idx
}

fn comp_bits(instruction: u16) -> u16 {
    (instruction >> 6) & 0b1111111
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct Cpu {
    pub reg_a: u16,
    pub reg_d: u16,
    out_m: u16,
    pub pc: u16,
    memory_load: bool,
}

impl Cpu {
    fn execute(&mut self, instruction: u16, in_m: u16) {
        if bit(instruction, 15) == 0 {
            // A Instruction
            self.reg_a = instruction;
            self.pc += 1;
            self.memory_load = false;
        } else {
            // C Instruction
            let alu_out = match comp_bits(instruction) {
                0b0101010 => Wrapping(0),
                0b0111111 => Wrapping(1),
                0b0111010 => Wrapping(-1_i16 as u16),
                0b0001100 => Wrapping(self.reg_d),
                0b0110000 => Wrapping(self.reg_a),
                0b0001101 => Wrapping(!self.reg_d),
                0b0110001 => !Wrapping(self.reg_a),
                0b0001111 => -Wrapping(self.reg_d),
                0b0110011 => -Wrapping(self.reg_a),
                0b0011111 => Wrapping(self.reg_d) + Wrapping(1),
                0b0110111 => Wrapping(self.reg_a) + Wrapping(1),
                0b0001110 => Wrapping(self.reg_d) - Wrapping(1),
                0b0110010 => Wrapping(self.reg_a) - Wrapping(1),
                0b0000010 => Wrapping(self.reg_d) + Wrapping(self.reg_a),
                0b0010011 => Wrapping(self.reg_d) - Wrapping(self.reg_a),
                0b0000111 => Wrapping(self.reg_a) - Wrapping(self.reg_d),
                0b0000000 => Wrapping(self.reg_d) & Wrapping(self.reg_a),
                0b0010101 => Wrapping(self.reg_d) | Wrapping(self.reg_a),
                0b1110000 => Wrapping(in_m),
                0b1110001 => !Wrapping(in_m),
                0b1110011 => -Wrapping(in_m),
                0b1110111 => Wrapping(in_m) + Wrapping(1),
                0b1110010 => Wrapping(in_m) - Wrapping(1),
                0b1000010 => Wrapping(self.reg_d) + Wrapping(in_m),
                0b1010011 => Wrapping(self.reg_d) - Wrapping(in_m),
                0b1000111 => Wrapping(in_m) - Wrapping(self.reg_d),
                0b1000000 => Wrapping(self.reg_d) & Wrapping(in_m),
                0b1010101 => Wrapping(self.reg_d) | Wrapping(in_m),
                _ => panic!("bad instruction"),
            };
            if (bit(instruction, 0) == 1 && (alu_out.0 as i16) > 0)
                || (bit(instruction, 1) == 1 && alu_out == Wrapping(0))
                || (bit(instruction, 2) == 1 && (alu_out.0 as i16) < 0)
            {
                self.pc = self.reg_a;
            } else {
                self.pc += 1;
            }
            self.memory_load = bit(instruction, 3) == 1;
            if self.memory_load {
                self.out_m = alu_out.0;
            }
            if bit(instruction, 4) == 1 {
                self.reg_d = alu_out.0;
            }
            if bit(instruction, 5) == 1 {
                self.reg_a = alu_out.0;
            }
        }
    }
}

#[wasm_bindgen]
pub enum WordDisplayBase {
    Binary,
    Decimal,
    BinaryBlocks,
}

#[wasm_bindgen]
pub fn get_ram_word(ram: &Ram, addr: usize, display_base: WordDisplayBase) -> String {
    let word = (*ram.0.lock().unwrap())[addr];
    match display_base {
        WordDisplayBase::Binary => format!("{:016b}", word),
        WordDisplayBase::BinaryBlocks => format!("{:016b}", word).replace('0', " ").replace('1', "█"),
        WordDisplayBase::Decimal => format!("{}", word),
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct Ram(Arc<Mutex<[u16; 32768]>>);

impl Ram {
    pub fn lock(&self) -> MutexGuard<[u16; 32768]> {
        self.0.lock().unwrap()
    }
}

#[wasm_bindgen(getter_with_clone)]
pub struct Computer {
    rom: [u16; 32768],
    pub ram: Ram,
    pub cpu: Cpu,
}

#[wasm_bindgen]
pub fn tick(computer: &mut Computer) {
    let prev_reg_a = computer.cpu.reg_a;
    let instruction = computer.rom[computer.cpu.pc as usize];
    let addr = computer.cpu.reg_a as usize % computer.ram.lock().len();
    let in_m = computer.ram.lock()[addr];
    computer.cpu.execute(instruction, in_m);
    if computer.cpu.memory_load {
        computer.ram.lock()[prev_reg_a as usize] = computer.cpu.out_m;
    }
}

pub fn tick_until(computer: &mut Computer, predicate: &dyn Fn(&Computer) -> bool) {
    let max_ticks: usize = 4_000_000_000;
    for _ in 0..=max_ticks {
        if predicate(computer) {
            return;
        }
        tick(computer);
    }
    panic!("predicate was not true within {} ticks", max_ticks);
}

#[wasm_bindgen]
pub fn tick_to_breakpoint(computer: &mut Computer, breakpoint: u16) {
    tick_until(computer, &|comp| comp.cpu.pc == breakpoint)
}

#[wasm_bindgen]
pub fn tick_to_some_breakpoint(computer: &mut Computer, breakpoints: &[u16]) {
    tick(computer);
    tick_until(computer, &|comp| breakpoints.iter().any(|breakpoint| comp.cpu.pc == *breakpoint))
}

impl Computer {
    pub fn new(rom: [u16; 32768]) -> Self {
        Self {
            rom,
            ram: Ram(Arc::new(Mutex::new([0; 32768]))),
            cpu: Cpu {
                reg_a: 0,
                reg_d: 0,
                pc: 0,
                out_m: 0,
                memory_load: false,
            },
        }
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
