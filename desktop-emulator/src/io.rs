use emulator_core::{
    computer::{bit, Ram},
    run::IO,
};
use minifb::{Key, Scale, ScaleMode, Window, WindowOptions};
use std::{array, time::SystemTime};

const WORD_SIZE: usize = 16;
const SCREEN_WIDTH: usize = 512;
const SCREEN_HEIGHT: usize = 256;

pub struct DesktopIO {
    screen_window: Window,
    led_window: Window,
    screen_buffer: [u32; SCREEN_WIDTH * SCREEN_HEIGHT],
    leds: u16,
    last_draw_time: SystemTime,
}

fn get_key_scancode(key: &Key) -> u16 {
    match *key {
        // digits are offset from ascii values by 48
        Key::Key0 => 1,
        Key::Key1 => 2,
        Key::Key2 => 3,
        Key::Key3 => 4,
        Key::Key4 => 5,
        Key::Key5 => 6,
        Key::Key6 => 7,
        Key::Key7 => 8,
        Key::Key8 => 9,
        Key::Key9 => 10,
        // letters are offset from the uppercase ascii by 54
        Key::A => 11,
        Key::B => 12,
        Key::C => 13,
        Key::D => 14,
        Key::E => 15,
        Key::F => 16,
        Key::G => 17,
        Key::H => 18,
        Key::I => 19,
        Key::J => 20,
        Key::K => 21,
        Key::L => 22,
        Key::M => 23,
        Key::N => 24,
        Key::O => 25,
        Key::P => 26,
        Key::Q => 27,
        Key::R => 28,
        Key::S => 29,
        Key::T => 30,
        Key::U => 31,
        Key::V => 32,
        Key::W => 33,
        Key::X => 34,
        Key::Y => 35,
        Key::Z => 36,
        Key::Left => 37,
        Key::Up => 38,
        Key::Right => 39,
        Key::Down => 40,
        Key::Enter => 41,
        Key::Backspace => 42,
        Key::Apostrophe => 43,
        Key::Backquote => 44,
        Key::Backslash => 45,
        Key::Comma => 46,
        Key::Equal => 47,
        Key::LeftBracket => 48,
        Key::RightBracket => 49,
        Key::Minus => 50,
        Key::Period => 51,
        Key::Semicolon => 52,
        Key::Slash => 53,
        Key::Space => 54,
        Key::Tab => 55,
        _ => 0,
    }
}

fn kbd_output(keys: Vec<Key>) -> u16 {
    let non_modifier_key = keys.get(0).map(get_key_scancode).unwrap_or(0);
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
                for (pixel_idx, pixel) in self.screen_buffer.iter_mut().enumerate() {
                    let word_idx = pixel_idx / WORD_SIZE;
                    let word = ram.lock()[word_idx + 18432];
                    let bit_position_in_word = 15 - (pixel_idx % 16);
                    *pixel = if bit(word, bit_position_in_word as u32) == 0 {
                        0xff000000
                    } else {
                        0xffffffff
                    }
                }

                self.leds = ram.lock()[30425];
                self.last_draw_time = time;
            }
            // Note that if `update_with_buffer` is called on one screen, it must be called on all
            // screens: https://github.com/emoon/rust_minifb/issues/343#issuecomment-1918601505
            self.screen_window
                .update_with_buffer(&self.screen_buffer, SCREEN_WIDTH, SCREEN_HEIGHT)
                .unwrap();

            let led_buffer: [u32; 16] =
                array::from_fn(|i| 15 - i as u32).map(|i| if 2u16.pow(i) & self.leds == 0 { 0xff000000u32 } else { 0xff00ff00u32 });
            self.led_window.update_with_buffer(&led_buffer, 16, 1).unwrap();

            ram.lock()[26624] = kbd_output(self.screen_window.get_keys());
        }
    }
}

impl DesktopIO {
    pub fn new() -> Self {
        let mut screen_window = Window::new(
            "Screen",
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
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
        screen_window.set_background_color(0xdb, 0xdb, 0xdb);

        let mut led_window = Window::new(
            "LEDs",
            16,
            1,
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
        led_window.set_background_color(0xdb, 0xdb, 0xdb);

        Self {
            screen_window,
            led_window,
            screen_buffer: [0; SCREEN_WIDTH * SCREEN_HEIGHT],
            leds: 0,
            last_draw_time: SystemTime::now(),
        }
    }
}
