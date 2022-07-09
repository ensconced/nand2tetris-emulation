use itertools::Itertools;

use std::{
    num::Wrapping,
    sync::{Arc, Mutex},
    thread::current,
};

use tabled::{Style, Table, Tabled};

enum DebugMode {
    Verbose,
    Heap,
    None,
}

const DEBUG_MODE: DebugMode = DebugMode::None;

fn display_heap(heap: &[i16]) {
    let groups = heap.iter().group_by(|x| **x == 0);
    print!("[");
    for (is_zero, group) in groups.into_iter() {
        let v: Vec<_> = group.collect();
        if is_zero && v.len() > 1 {
            print!("0 x {}, ", v.len());
        } else {
            for elem in v.into_iter() {
                print!("{}, ", elem);
            }
        }
    }
    println!("]")
}

pub fn bit(instruction: i16, idx: u32) -> u16 {
    (instruction as u16 & (2u16).pow(idx)) >> idx
}

fn comp_bits(instruction: i16) -> i16 {
    (instruction >> 6) & 0b1111111
}

pub struct Cpu {
    pub reg_a: Wrapping<i16>,
    pub reg_d: Wrapping<i16>,
    out_m: Wrapping<i16>,
    pub pc: i16,
    memory_load: bool,
}

impl Cpu {
    fn execute(&mut self, instruction: i16, in_m: Wrapping<i16>) {
        if bit(instruction, 15) == 0 {
            // A Instruction
            self.reg_a = Wrapping(instruction);
            self.pc += 1;
            self.memory_load = false;
        } else {
            // C Instruction
            let alu_out = match comp_bits(instruction) {
                0b0101010 => Wrapping(0),
                0b0111111 => Wrapping(1),
                0b0111010 => Wrapping(-1),
                0b0001100 => self.reg_d,
                0b0110000 => self.reg_a,
                0b0001101 => !self.reg_d,
                0b0110001 => !self.reg_a,
                0b0001111 => -self.reg_d,
                0b0110011 => -self.reg_a,
                0b0011111 => self.reg_d + Wrapping(1),
                0b0110111 => self.reg_a + Wrapping(1),
                0b0001110 => self.reg_d - Wrapping(1),
                0b0110010 => self.reg_a - Wrapping(1),
                0b0000010 => self.reg_d + self.reg_a,
                0b0010011 => self.reg_d - self.reg_a,
                0b0000111 => self.reg_a - self.reg_d,
                0b0000000 => self.reg_d & self.reg_a,
                0b0010101 => self.reg_d | self.reg_a,
                0b1110000 => in_m,
                0b1110001 => !in_m,
                0b1110011 => -in_m,
                0b1110111 => in_m + Wrapping(1),
                0b1110010 => in_m - Wrapping(1),
                0b1000010 => self.reg_d + in_m,
                0b1010011 => self.reg_d - in_m,
                0b1000111 => in_m - self.reg_d,
                0b1000000 => self.reg_d & in_m,
                0b1010101 => self.reg_d | in_m,
                _ => panic!("bad instruction"),
            };
            if (bit(instruction, 0) == 1 && alu_out > Wrapping(0))
                || (bit(instruction, 1) == 1 && alu_out == Wrapping(0))
                || (bit(instruction, 2) == 1 && alu_out < Wrapping(0))
            {
                self.pc = self.reg_a.0;
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

#[derive(Tabled, PartialEq)]
struct DebugInfo {
    pc: i16,
    sp: i16,
    lcl: i16,
    arg: i16,
    this: i16,
    that: i16,
    temp: String,
    stack: String,
    heap: String,
}

impl DebugInfo {
    fn display(&self) {
        let rows = vec![self];
        println!("{}", Table::new(rows).with(Style::blank()));
        println!();
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
                reg_a: Wrapping(0),
                reg_d: Wrapping(0),
                pc: 0,
                out_m: Wrapping(0),
                memory_load: false,
            },
        }
    }
    pub fn tick(&mut self) {
        let prev_reg_a = self.cpu.reg_a;
        let instruction = self.rom[self.cpu.pc as usize];
        match DEBUG_MODE {
            DebugMode::Verbose => {
                let ram = self.ram.lock().unwrap();
                let sp = ram[0];
                let stack = if sp >= 256 {
                    &ram[256..ram[0] as usize]
                } else {
                    &[]
                };
                let heap = &ram[2048..2148]; // just show start of heap
                let temp = format!("{:?}", &ram[5..=12]);
                println!("{:?}", &ram[16..26]);
                let debug_info = DebugInfo {
                    pc: self.cpu.pc,
                    sp: ram[0],
                    lcl: ram[1],
                    arg: ram[2],
                    this: ram[3],
                    that: ram[4],
                    stack: format!("{:?}", stack),
                    heap: format!("{:?}", heap),
                    temp: format!("{:?}", temp),
                };
                debug_info.display();
            }
            DebugMode::Heap => {
                let ram = self.ram.lock().unwrap();
                display_heap(&ram[2048..16384]);
            }
            DebugMode::None => {}
        }
        let addr = self.cpu.reg_a.0 as usize % self.ram.lock().unwrap().len();
        let in_m = Wrapping(self.ram.lock().unwrap()[addr]);
        self.cpu.execute(instruction, in_m);
        if self.cpu.memory_load {
            self.ram.lock().unwrap()[prev_reg_a.0 as usize] = self.cpu.out_m.0;
        }
    }

    pub fn tick_until(&mut self, predicate: &dyn Fn(&Computer) -> bool) {
        let max_ticks: usize = 10_000_000_000;
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
