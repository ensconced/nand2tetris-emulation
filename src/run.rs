use super::computer::Computer;
use super::display::Display;
use super::programmer::get_rom;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::SystemTime;

pub fn run(file_path: &String) {
    let computer = Arc::new(Mutex::new(Computer::new(get_rom(file_path.as_str()))));
    let comp_clone = Arc::clone(&computer);

    thread::spawn(move || loop {
        comp_clone.lock().unwrap().tick();
    });

    let mut display = Display::new();
    let mut last_draw_time = SystemTime::now();
    loop {
        let time = SystemTime::now();
        if let Ok(t) = time.duration_since(last_draw_time) {
            if t.as_millis() >= 16 {
                display.refresh(computer.lock().unwrap().screen_output());
                last_draw_time = time;
            }
        }
    }
}
