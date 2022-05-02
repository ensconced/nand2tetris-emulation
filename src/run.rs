use super::computer::Computer;
use super::io::IO;
use super::programmer::get_rom;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn run(file_path: &String) {
    let computer = Arc::new(Mutex::new(Computer::new(get_rom(file_path.as_str()))));
    let comp_clone = Arc::clone(&computer);

    thread::spawn(move || loop {
        comp_clone.lock().unwrap().tick();
    });

    let mut io = IO::new();
    let mut cmp = computer.lock().unwrap();
    loop {
        io.refresh(&mut cmp.ram);
    }
}
