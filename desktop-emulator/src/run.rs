use std::thread;

use crate::io::IO;
use emulator_core::{computer::Computer, generate_rom};

pub fn run(machine_code: String) {
    let mut computer = Computer::new(generate_rom::from_string(machine_code));
    let cloned_ram = computer.ram.clone();

    thread::spawn(move || loop {
        computer.tick();
    });

    let mut io = IO::new();
    loop {
        io.refresh(&cloned_ram);
    }
}
