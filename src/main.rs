mod assembler;
mod computer;
mod display;
mod programmer;

use computer::Computer;
use display::Display;
use programmer::get_rom;
use std::time::SystemTime;

fn main() {
    let mut display = Display::new();
    let mut computer = Computer::new(get_rom("./programs/blinky"));

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
