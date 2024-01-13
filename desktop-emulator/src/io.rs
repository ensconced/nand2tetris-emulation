use emulator_core::{
    computer::{bit, Ram},
    run::IO,
};
use minifb::{Key, Scale, ScaleMode, Window, WindowOptions};
use std::time::SystemTime;

const WORD_SIZE: usize = 16;
const WIDTH: usize = 512;
const HEIGHT: usize = 256;

pub struct DesktopIO {
    window: Window,
    buffer: [u32; WIDTH * HEIGHT],
    last_draw_time: SystemTime,
}

const NON_MODIFIER_KEYS: [Key; 55] = [
    Key::Key0,
    Key::Key1,
    Key::Key2,
    Key::Key3,
    Key::Key4,
    Key::Key5,
    Key::Key6,
    Key::Key7,
    Key::Key8,
    Key::Key9,
    Key::A,
    Key::B,
    Key::C,
    Key::D,
    Key::E,
    Key::F,
    Key::G,
    Key::H,
    Key::I,
    Key::J,
    Key::K,
    Key::L,
    Key::M,
    Key::N,
    Key::O,
    Key::P,
    Key::Q,
    Key::R,
    Key::S,
    Key::T,
    Key::U,
    Key::V,
    Key::W,
    Key::X,
    Key::Y,
    Key::Z,
    Key::Left,
    Key::Up,
    Key::Right,
    Key::Down,
    Key::Enter,
    Key::Backspace,
    Key::Apostrophe,
    Key::Backquote,
    Key::Backslash,
    Key::Comma,
    Key::Equal,
    Key::LeftBracket,
    Key::RightBracket,
    Key::Minus,
    Key::Period,
    Key::Semicolon,
    Key::Slash,
    Key::Space,
    Key::Tab,
];

fn kbd_output(keys: Vec<Key>) -> u16 {
    let non_modifier_key = keys.get(0).map_or(0, |key| {
        let key_idx = NON_MODIFIER_KEYS.iter().position(|some_key| some_key == key).map(|idx| idx + 1);
        key_idx.unwrap_or(0) as u16
    });
    let shift_is_down = keys.contains(&Key::LeftShift) || keys.contains(&Key::RightShift);
    let ctrl_is_down = keys.contains(&Key::LeftCtrl) || keys.contains(&Key::RightCtrl);
    let alt_is_down = keys.contains(&Key::LeftAlt) || keys.contains(&Key::RightAlt);
    let super_is_down = keys.contains(&Key::LeftSuper) || keys.contains(&Key::RightSuper);
    let modifier_flags = ((ctrl_is_down as u16) << 3) | ((alt_is_down as u16) << 2) | ((super_is_down as u16) << 1) | (shift_is_down as u16);
    (modifier_flags << 12) | non_modifier_key
}

impl Default for DesktopIO {
    fn default() -> Self {
        Self::new()
    }
}

impl IO for DesktopIO {
    fn refresh(&mut self, ram: &Ram) {
        let time = SystemTime::now();
        if let Ok(t) = time.duration_since(self.last_draw_time) {
            if t.as_millis() >= 16 {
                for (pixel_idx, pixel) in self.buffer.iter_mut().enumerate() {
                    let word_idx = pixel_idx / WORD_SIZE;
                    let word = ram.lock()[word_idx + 18432];
                    let bit_position_in_word = 15 - (pixel_idx % 16);
                    *pixel = if bit(word as u16, bit_position_in_word as u32) == 0 {
                        0xff000000
                    } else {
                        0xffffffff
                    }
                }
                self.last_draw_time = time;
            }
            ram.lock()[26624] = kbd_output(self.window.get_keys());
            self.window.update_with_buffer(&self.buffer, WIDTH, HEIGHT).unwrap();
        }
    }
}

impl DesktopIO {
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
}
