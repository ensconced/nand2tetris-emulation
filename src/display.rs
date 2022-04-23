use crate::computer::bit;
use minifb::{Window, WindowOptions};
use raqote::{DrawOptions, DrawTarget, PathBuilder, SolidSource, Source};

const WIDTH: usize = 1000;
const HEIGHT: usize = 1000;

pub struct Display {
    window: Window,
    dt: DrawTarget,
}

impl Display {
    pub fn new() -> Self {
        let window = Window::new(
            "LED output",
            WIDTH,
            HEIGHT,
            WindowOptions {
                ..WindowOptions::default()
            },
        )
        .unwrap();
        let size = window.get_size();
        let dt = DrawTarget::new(size.0 as i32, size.1 as i32);
        Self { window, dt }
    }

    pub fn refresh(&mut self, val: i16) {
        let size = self.window.get_size();
        self.dt.clear(SolidSource::from_unpremultiplied_argb(
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
            self.dt.fill(
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

        self.window
            .update_with_buffer(self.dt.get_data(), size.0, size.1)
            .unwrap();
    }
}
