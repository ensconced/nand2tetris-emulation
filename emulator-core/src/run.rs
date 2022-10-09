use std::thread;

use crate::computer::{tick, Computer, Ram};

pub trait IO {
    fn refresh(&mut self, ram: &Ram);
}

pub fn run(mut computer: Computer, io: &mut dyn IO) {
    let cloned_ram = computer.ram.clone();

    thread::spawn(move || loop {
        tick(&mut computer);
    });

    loop {
        io.refresh(&cloned_ram);
    }
}
