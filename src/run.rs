use super::computer::Computer;
use super::display::Display;
use super::programmer::get_rom;
use std::time::SystemTime;

pub fn run(file_path: &String) {
    let mut display = Display::new();
    let mut computer = Computer::new(get_rom(file_path.as_str()));

    let mut last_draw_time = SystemTime::now();
    loop {
        computer.tick();
        let time = SystemTime::now();
        if let Ok(t) = time.duration_since(last_draw_time) {
            if t.as_millis() >= 16 {
                display.refresh(computer.led_output());
                last_draw_time = time;
            }
        }
    }
}