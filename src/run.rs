use super::computer::Computer;
use super::io::IO;
use super::programmer::get_rom;
use std::thread;

pub fn run(file_path: &String) {
    let mut computer = Computer::new(get_rom(file_path.as_str()));
    let cloned_ram = computer.ram.clone();

    thread::spawn(move || loop {
        computer.tick();
    });

    let mut io = IO::new();
    loop {
        io.refresh(&cloned_ram);
    }
}
