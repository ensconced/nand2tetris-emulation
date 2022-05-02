use crate::computer::bit;
use minifb::{Scale, ScaleMode, Window, WindowOptions};

const WORD_SIZE: usize = 16;
const WIDTH: usize = 512;
const HEIGHT: usize = 256;

pub struct Display {
    window: Window,
    buffer: [u32; WIDTH * HEIGHT],
}

impl Display {
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
        }
    }

    pub fn refresh(&mut self, screen_mem: &[i16]) {
        for (pixel_idx, pixel) in self.buffer.iter_mut().enumerate() {
            let word_idx = pixel_idx / WORD_SIZE;
            let word = screen_mem[word_idx];
            let bit_position_in_word = pixel_idx % 16;
            *pixel = if bit(word, bit_position_in_word as u32) == 0 {
                0xff000000
            } else {
                0xffffffff
            }
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        self.window
            .update_with_buffer(&self.buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}
