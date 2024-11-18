use crossterm::cursor::{MoveTo, MoveToColumn};
use crossterm::execute;
use crossterm::style::{Print, Stylize};
use crossterm::terminal::{Clear, ClearType};
use std::io::{stdout, Stdout};

pub struct Display {
    pixels: Vec<bool>,

    width: usize,
    height: usize,
}

pub struct RenderContext {
    pub title: String,
    pub registries: [u8; 16],
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

    const screen_offset: u16 = 10;

    pub fn print(&self, context: RenderContext) {
        let mut stdout = stdout();

        execute!(stdout, Clear(ClearType::All),).unwrap();

        self.print_registries(&context, &mut stdout);
        self.print_screen(&context, &mut stdout);
    }

    fn print_registries(&self, context: &RenderContext, stdout: &mut Stdout) {
        execute!(stdout, MoveTo(0, 1), Print("Registers\n"),).unwrap();

        for register in context.registries {
            execute!(
                stdout,
                MoveToColumn(0),
                Print(format!("{:#04x}\n", register))
            )
            .unwrap();
        }
    }

    fn print_screen(&self, context: &RenderContext, stdout: &mut Stdout) {
        execute!(
            stdout,
            MoveTo(Self::screen_offset, 0),
            Print(format!("{}\n", context.title.clone().bold())),
            MoveToColumn(Self::screen_offset),
            Print(format!("╭{}╮\n", "──".repeat(self.width))),
        )
        .unwrap();

        for y in 0..self.height {
            execute!(stdout, MoveToColumn(Self::screen_offset), Print("│")).unwrap();
            for x in 0..self.width {
                if self.pixels[y * self.width + x] {
                    execute!(stdout, Print("██")).unwrap();
                } else {
                    execute!(stdout, Print("  ")).unwrap();
                }
            }

            execute!(stdout, Print("│\n")).unwrap();
        }

        execute!(
            stdout,
            MoveToColumn(Self::screen_offset),
            Print(format!("╰{}╯\n", "──".repeat(self.width))),
            MoveToColumn(0),
        )
        .unwrap()
    }
}
