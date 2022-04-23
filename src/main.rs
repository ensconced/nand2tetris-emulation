mod computer;

use computer::computer::{bit, Computer};
use minifb::{Window, WindowOptions};
use raqote::{DrawOptions, DrawTarget, PathBuilder, SolidSource, Source};
use std::fs;
use std::time::SystemTime;

const WIDTH: usize = 1000;
const HEIGHT: usize = 1000;

fn display_led_output(window: &mut Window, dt: &mut DrawTarget, val: i16) {
    let size = window.get_size();
    dt.clear(SolidSource::from_unpremultiplied_argb(
        0xff, 0xff, 0xff, 0xff,
    ));
    let led_width = size.0 as f32 / 16 as f32;
    let padding = 5.0;

    for i in 0..16 {
        let mut pb = PathBuilder::new();
        pb.rect(
            i as f32 * led_width + padding,
            padding,
            led_width - padding * 2.,
            led_width - padding * 2.,
        );
        let path = pb.finish();
        let val = bit(val, 15 - i);
        dt.fill(
            &path,
            &Source::Solid(SolidSource::from_unpremultiplied_argb(
                0xff,
                0,
                if val == 1 { 0xff } else { 0 },
                0,
            )),
            &DrawOptions::new(),
        );
    }

    window
        .update_with_buffer(dt.get_data(), size.0, size.1)
        .unwrap();
}

fn main() {
    let mut window = Window::new(
        "LED output",
        WIDTH,
        HEIGHT,
        WindowOptions {
            ..WindowOptions::default()
        },
    )
    .unwrap();
    let size = window.get_size();
    let mut dt = DrawTarget::new(size.0 as i32, size.1 as i32);

    let program = fs::read_to_string("./programs/blinky").unwrap();
    let mut clean_lines: Vec<String> = program
        .lines()
        .map(|line| {
            let clean_line: String = line
                .chars()
                .take_while(|ch| ch.is_ascii_digit() || ch.is_whitespace())
                .collect();
            clean_line
        })
        .filter(|line| line.len() > 0)
        .collect();

    let mut rom: [i16; 32768] = [0; 32768];
    for (idx, line) in clean_lines.iter_mut().enumerate() {
        line.retain(|ch| !ch.is_whitespace());
        let instruction = u16::from_str_radix(line, 2).unwrap() as i16;
        rom[idx] = instruction;
    }
    let mut computer = Computer::new(rom);

    let mut last_draw_time = SystemTime::now();
    loop {
        computer.tick();
        let time = SystemTime::now();
        if let Ok(t) = time.duration_since(last_draw_time) {
            if t.as_millis() >= 16 {
                display_led_output(&mut window, &mut dt, computer.led_output());
                last_draw_time = time;
            }
        }
    }
}
