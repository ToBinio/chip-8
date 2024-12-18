use crate::io::IO;
use crate::Platform;

#[derive(Debug)]

pub struct Gpu {
    pixels: Vec<bool>,
}

impl Gpu {
    pub fn new(platform: Platform) -> Gpu {
        Gpu {
            pixels: vec![false; platform.width() * platform.height()],
        }
    }

    pub fn flip_pixel(&mut self, platform: Platform, x: usize, y: usize) -> bool {
        //todo - dont hard code sizes
        if x >= platform.width() || y >= platform.height() {
            return false;
        }

        self.pixels[y * platform.width() + x] = !self.pixels[y * platform.width() + x];

        !self.pixels[y * platform.width() + x]
    }

    pub fn clear(&mut self) {
        self.pixels.fill(false);
    }

    pub fn pixels(&self) -> &[bool] {
        &self.pixels
    }
}
