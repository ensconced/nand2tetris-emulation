use super::computer::Computer;
use super::generate_rom;
use super::io::IO;
use std::thread;

pub fn run(file_path: &str) {
    let mut computer = Computer::new(generate_rom::from_file(file_path));
    let cloned_ram = computer.ram.clone();

    thread::spawn(move || loop {
        computer.tick();
    });

    let mut io = IO::new();
    loop {
        io.refresh(&cloned_ram);
    }
}
