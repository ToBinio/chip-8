use crate::io::IO;

pub struct GPU {
    pixels: Vec<bool>,
}

impl GPU {
    pub fn new(io: &IO) -> GPU {
        GPU {
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
