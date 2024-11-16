use crossterm::cursor::MoveTo;
use crossterm::execute;
use crossterm::style::{Print, Stylize};
use crossterm::terminal::{Clear, ClearType};
use std::io::stdout;

pub struct Display {
    pixels: Vec<bool>,

    width: usize,
    height: usize,
}

pub struct RenderContext {
    pub title: String,
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

    pub fn print(&self, context: RenderContext) {
        let mut stdout = stdout();

        execute!(
            stdout,
            Clear(ClearType::All),
            MoveTo(0, 0),
            Print(format!("{}\n", context.title.bold())),
            Print(format!("╭{}╮\n", "──".repeat(self.width))),
        )
        .unwrap();

        for y in 0..self.height {
            execute!(stdout, Print("│")).unwrap();
            for x in 0..self.width {
                if self.pixels[y * self.width + x] {
                    execute!(stdout, Print("██")).unwrap();
                } else {
                    execute!(stdout, Print("  ")).unwrap();
                }
            }

            execute!(stdout, Print("│\n")).unwrap();
        }

        execute!(stdout, Print(format!("╰{}╯\n", "──".repeat(self.width))),).unwrap()
    }
}
