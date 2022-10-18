use std::{
    num::Wrapping,
    sync::{Arc, Mutex, MutexGuard},
};

use wasm_bindgen::prelude::wasm_bindgen;

pub enum DebugMode {
    #[allow(dead_code)]
    Verbose,
    #[allow(dead_code)]
    Heap,
    None,
}

pub fn bit(instruction: u16, idx: u32) -> u16 {
    (instruction & (2u16).pow(idx)) >> idx
}

fn comp_bits(instruction: u16) -> u16 {
    (instruction >> 6) & 0b1111111
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Wrappedi16(Wrapping<i16>);

// TODO - temp for refactoring - remove me
#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Wrappedu16(Wrapping<u16>);

impl Wrappedi16 {
    fn new(i: u16) -> Self {
        Self(Wrapping(i as i16))
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct Cpu {
    pub reg_a: Wrappedi16,
    pub reg_d: Wrappedi16,
    out_m: Wrappedi16,
    pub pc: i16,
    memory_load: bool,
}

impl Cpu {
    fn execute(&mut self, instruction: u16, in_m: Wrapping<i16>) {
        if bit(instruction, 15) == 0 {
            // A Instruction
            self.reg_a = Wrappedi16::new(instruction);
            self.pc += 1;
            self.memory_load = false;
        } else {
            // C Instruction
            let alu_out = match comp_bits(instruction) {
                0b0101010 => Wrapping(0),
                0b0111111 => Wrapping(1),
                0b0111010 => Wrapping(-1),
                0b0001100 => self.reg_d.0,
                0b0110000 => self.reg_a.0,
                0b0001101 => !self.reg_d.0,
                0b0110001 => !self.reg_a.0,
                0b0001111 => -self.reg_d.0,
                0b0110011 => -self.reg_a.0,
                0b0011111 => self.reg_d.0 + Wrapping(1),
                0b0110111 => self.reg_a.0 + Wrapping(1),
                0b0001110 => self.reg_d.0 - Wrapping(1),
                0b0110010 => self.reg_a.0 - Wrapping(1),
                0b0000010 => self.reg_d.0 + self.reg_a.0,
                0b0010011 => self.reg_d.0 - self.reg_a.0,
                0b0000111 => self.reg_a.0 - self.reg_d.0,
                0b0000000 => self.reg_d.0 & self.reg_a.0,
                0b0010101 => self.reg_d.0 | self.reg_a.0,
                0b1110000 => in_m,
                0b1110001 => !in_m,
                0b1110011 => -in_m,
                0b1110111 => in_m + Wrapping(1),
                0b1110010 => in_m - Wrapping(1),
                0b1000010 => self.reg_d.0 + in_m,
                0b1010011 => self.reg_d.0 - in_m,
                0b1000111 => in_m - self.reg_d.0,
                0b1000000 => self.reg_d.0 & in_m,
                0b1010101 => self.reg_d.0 | in_m,
                _ => panic!("bad instruction"),
            };
            if (bit(instruction, 0) == 1 && alu_out > Wrapping(0))
                || (bit(instruction, 1) == 1 && alu_out == Wrapping(0))
                || (bit(instruction, 2) == 1 && alu_out < Wrapping(0))
            {
                self.pc = (self.reg_a.0).0;
            } else {
                self.pc += 1;
            }
            self.memory_load = bit(instruction, 3) == 1;
            if self.memory_load {
                self.out_m = Wrappedi16::new(alu_out.0 as u16);
            }
            if bit(instruction, 4) == 1 {
                self.reg_d = Wrappedi16::new(alu_out.0 as u16);
            }
            if bit(instruction, 5) == 1 {
                self.reg_a = Wrappedi16::new(alu_out.0 as u16);
            }
        }
    }
}

#[wasm_bindgen]
pub enum RamFormat {
    binary,
    decimal,
}

#[wasm_bindgen]
pub fn get_formatted_ram(ram: &Ram, format: RamFormat) -> String {
    let v: Vec<_> = (*ram.0.lock().unwrap())
        .into_iter()
        .map(|word| match format {
            RamFormat::binary => format!("{word:016b}"),
            RamFormat::decimal => format!("{word}"),
        })
        .collect();

    v.join("\n")
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct Ram(Arc<Mutex<[i16; 32768]>>);

impl Ram {
    pub fn lock(&self) -> MutexGuard<[i16; 32768]> {
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
    let addr = (computer.cpu.reg_a.0).0 as usize % computer.ram.lock().len();
    let in_m = Wrapping(computer.ram.lock()[addr]);
    computer.cpu.execute(instruction, in_m);
    if computer.cpu.memory_load {
        computer.ram.lock()[(prev_reg_a.0).0 as usize] = (computer.cpu.out_m.0).0;
    }
}

impl Computer {
    pub fn new(rom: [u16; 32768]) -> Self {
        Self {
            rom,
            ram: Ram(Arc::new(Mutex::new([0; 32768]))),
            cpu: Cpu {
                reg_a: Wrappedi16::new(0),
                reg_d: Wrappedi16::new(0),
                pc: 0,
                out_m: Wrappedi16::new(0),
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
