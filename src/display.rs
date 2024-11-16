pub struct Display {
    pixels: Vec<bool>,

    width: usize,
    height: usize,
}

impl Display {
    pub fn new(width: usize, height: usize) -> Display {
        Display {
            pixels: vec![false; width * height],
            width,
            height,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn flip_pixel(&mut self, x: usize, y: usize) {
        self.pixels[y * 64 + x] = !self.pixels[y * 64 + x];
    }

    pub fn clear(&mut self) {
        self.pixels.fill(false);
    }

    pub fn print(&mut self) {
        for y in 0..32 {
            for x in 0..64 {
                if self.pixels[y * 64 + x] {
                    print!("█")
                } else {
                    print!(".")
                }
            }

            println!();
        }

        println!();
        println!();
    }
}
