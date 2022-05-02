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

        // Limit to max ~60 fps update rate
        window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

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

        // // let size = self.window.get_size();

        // let mut image_data: [u32; 131072] = [0; 131072];

        // let black: u32 = 0xff000000;
        // let white: u32 = 0xffffffff;

        // for image_data_idx in 0..131072 {
        //     image_data[image_data_idx] = if (image_data_idx % 10) % 2 == 0 {
        //         black
        //     } else {
        //         white
        //     };
        // }
        // for row_idx in 0..HEIGHT {
        //     for col_idx in 0..WIDTH {
        //         let word_idx = (col_idx + row_idx * WIDTH) / WORD_SIZE;
        //         let word = screen_mem[word_idx];
        //         let is_black = row_idx % 16 == 0; // bit(word, col_idx as u32 % WORD_SIZE as u32) == 0;
        //         image_data[image_data_idx] = if is_black { black } else { white };
        //         image_data_idx = image_data_idx + 1;
        //     }
        // }
    }
}
