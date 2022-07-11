use super::computer::bit;
use minifb::{Key, Scale, ScaleMode, Window, WindowOptions};
use std::sync::{Arc, Mutex};
use std::time::SystemTime;

const WORD_SIZE: usize = 16;
const WIDTH: usize = 512;
const HEIGHT: usize = 256;

pub struct IO {
    window: Window,
    buffer: [u32; WIDTH * HEIGHT],
    last_draw_time: SystemTime,
}

fn kbd_output(keys: Vec<Key>) -> i16 {
    keys.into_iter().next().map_or(0, |key| match key {
        Key::Key0 => 0x30,
        Key::Key1 => 0x31,
        Key::Key2 => 0x32,
        Key::Key3 => 0x33,
        Key::Key4 => 0x34,
        Key::Key5 => 0x35,
        Key::Key6 => 0x36,
        Key::Key7 => 0x37,
        Key::Key8 => 0x38,
        Key::Key9 => 0x39,
        Key::A => 0x41,
        Key::B => 0x42,
        Key::C => 0x43,
        Key::D => 0x44,
        Key::E => 0x45,
        Key::F => 0x46,
        Key::G => 0x47,
        Key::H => 0x48,
        Key::I => 0x49,
        Key::J => 0x4A,
        Key::K => 0x4B,
        Key::L => 0x4C,
        Key::M => 0x4D,
        Key::N => 0x4E,
        Key::O => 0x4F,
        Key::P => 0x50,
        Key::Q => 0x51,
        Key::R => 0x52,
        Key::S => 0x53,
        Key::T => 0x54,
        Key::U => 0x55,
        Key::V => 0x56,
        Key::W => 0x57,
        Key::X => 0x58,
        Key::Y => 0x59,
        Key::Z => 0x5A,
        Key::F1 => 141,
        Key::F2 => 142,
        Key::F3 => 143,
        Key::F4 => 144,
        Key::F5 => 145,
        Key::F6 => 146,
        Key::F7 => 147,
        Key::F8 => 148,
        Key::F9 => 149,
        Key::F10 => 150,
        Key::F11 => 151,
        Key::F12 => 152,
        Key::Left => 130,
        Key::Up => 131,
        Key::Right => 132,
        Key::Down => 133,
        Key::Enter => 128,
        Key::Backspace => 129,
        Key::Home => 134,
        Key::End => 135,
        Key::PageUp => 136,
        Key::PageDown => 137,
        Key::Insert => 138,
        Key::Delete => 139,
        Key::Escape => 140,
        Key::Apostrophe => 0x27,
        Key::Backquote => 0x60,
        Key::Backslash => 0x5C,
        Key::Comma => 0x2C,
        Key::Equal => 0x3D,
        Key::LeftBracket => 0x28,
        Key::Minus => 0x2D,
        Key::Period => 0x2E,
        Key::RightBracket => 0x29,
        Key::Semicolon => 0x3B,
        Key::Slash => 0x2F,
        Key::Space => 0x20,
        Key::Tab => 0x9,
        _ => 0,
    })
}

impl IO {
    pub fn new() -> Self {
        let mut window = Window::new(
            "Display",
            WIDTH,
            HEIGHT,
            WindowOptions {
                borderless: true,
                title: true,
                resize: true,
                scale: Scale::FitScreen,
                scale_mode: ScaleMode::AspectRatioStretch,
                ..WindowOptions::default()
            },
        )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });
        window.set_background_color(0xdb, 0xdb, 0xdb);

        Self {
            window,
            buffer: [0; WIDTH * HEIGHT],
            last_draw_time: SystemTime::now(),
        }
    }

    pub fn refresh(&mut self, ram: &Arc<Mutex<[i16; 32768]>>) {
        let time = SystemTime::now();
        if let Ok(t) = time.duration_since(self.last_draw_time) {
            if t.as_millis() >= 16 {
                for (pixel_idx, pixel) in self.buffer.iter_mut().enumerate() {
                    let word_idx = pixel_idx / WORD_SIZE;
                    let word = ram.lock().unwrap()[word_idx + 16384];
                    let bit_position_in_word = 15 - (pixel_idx % 16);
                    *pixel = if bit(word, bit_position_in_word as u32) == 0 {
                        0xff000000
                    } else {
                        0xffffffff
                    }
                }
                self.last_draw_time = time;
            }
            ram.lock().unwrap()[26624] = kbd_output(self.window.get_keys());
            self.window
                .update_with_buffer(&self.buffer, WIDTH, HEIGHT)
                .unwrap();
        }
    }
}
