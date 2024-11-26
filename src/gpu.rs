use web_sys::console::log_1;
use crate::io::IO;
#[derive(Debug)]

pub struct Gpu {
    pixels: Vec<bool>,
}

impl Gpu {
    pub fn new(io: &dyn IO) -> Gpu {
        Gpu {
            pixels: vec![false; io.width() * io.height()],
        }
    }

    pub fn flip_pixel(&mut self, x: usize, y: usize) {
        self.pixels[y * 64 + x] = !self.pixels[y * 64 + x];
    }

    pub fn clear(&mut self) {
        self.pixels.fill(false);
    }

    pub fn pixels(&self) -> &[bool] {
        &self.pixels
    }
}
