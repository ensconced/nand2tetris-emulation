use std::{thread, time::Instant};

use crate::computer::{tick, Computer, Ram};

pub trait IO {
    fn refresh(&mut self, ram: &Ram);
}

pub fn run(mut computer: Computer, io: &mut dyn IO) {
    let cloned_ram = computer.ram.clone();

    let mut count = 0u64;
    let start_time = Instant::now();
    thread::spawn(move || loop {
        tick(&mut computer);
        count += 1;
        if count == 1_00_000_0000 {
            dbg!(Instant::now().duration_since(start_time));
        }
    });

    loop {
        io.refresh(&cloned_ram);
    }
}
