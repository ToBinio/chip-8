use crate::io::{RenderContext, IO};
use async_std::stream::StreamExt;
use crossterm::cursor::{MoveTo, MoveToColumn};
use crossterm::event::{
    DisableMouseCapture, EnableMouseCapture, Event, EventStream, KeyCode, KeyEventKind,
    KeyboardEnhancementFlags, PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags,
};
use crossterm::execute;
use crossterm::style::{Print, Stylize};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use std::io::{stdout, Stdout};
use std::sync::{Arc, Mutex};

pub struct TerminalIO {
    pub pressed_keys: Arc<Mutex<Vec<KeyCode>>>,

    screen_width: usize,
    screen_height: usize,
}

impl IO for TerminalIO {
    fn width(&self) -> usize {
        self.screen_width
    }

    fn height(&self) -> usize {
        self.screen_height
    }
    fn is_code_pressed(&self, code: u8) -> bool {
        self.pressed_keys
            .lock()
            .unwrap()
            .contains(&self.map_key(code))
    }

    fn should_shutdown(&self) -> bool {
        self.is_key_pressed(KeyCode::Esc)
    }

    fn render(&self, context: RenderContext) {
        let mut stdout = stdout();

        execute!(stdout, MoveTo(0, 0), Clear(ClearType::All)).unwrap();

        self.print_registries(&context, &mut stdout);
        self.print_keyboard(&mut stdout);
        self.print_screen(&context, &mut stdout);
    }
}
impl TerminalIO {
    pub fn new(width: usize, height: usize) -> Self {
        let pressed_keys = Arc::new(Mutex::new(Vec::new()));

        let pressed_keys_clone = pressed_keys.clone();
        async_std::task::spawn(async move {
            TerminalIO::start(pressed_keys_clone).await;
        });

        TerminalIO {
            pressed_keys,
            screen_width: width,
            screen_height: height,
        }
    }
    fn map_key(&self, value: u8) -> KeyCode {
        match value {
            0x0 => KeyCode::Char('x'),
            0x1 => KeyCode::Char('1'),
            0x2 => KeyCode::Char('2'),
            0x3 => KeyCode::Char('3'),
            0x4 => KeyCode::Char('q'),
            0x5 => KeyCode::Char('w'),
            0x6 => KeyCode::Char('e'),
            0x7 => KeyCode::Char('a'),
            0x8 => KeyCode::Char('s'),
            0x9 => KeyCode::Char('d'),
            0xA => KeyCode::Char('y'),
            0xB => KeyCode::Char('c'),
            0xC => KeyCode::Char('4'),
            0xD => KeyCode::Char('r'),
            0xE => KeyCode::Char('f'),
            0xF => KeyCode::Char('v'),
            _ => {
                panic!("unhandled key 0x{:x}", value);
            }
        }
    }
    fn is_key_pressed(&self, code: KeyCode) -> bool {
        self.pressed_keys.lock().unwrap().contains(&code)
    }

    async fn start(pressed_keys: Arc<Mutex<Vec<KeyCode>>>) {
        let mut reader = EventStream::new();

        enable_raw_mode().unwrap();

        let mut stdout = stdout();
        execute!(
            stdout,
            EnableMouseCapture,
            PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES)
        )
        .unwrap();

        loop {
            let event = reader.next().await;

            match event {
                Some(Ok(event)) => {
                    if let Event::Key(key_event) = event {
                        if key_event.kind == KeyEventKind::Press {
                            let mut keys = pressed_keys.lock().unwrap();

                            if keys.contains(&key_event.code) {
                                keys.retain(|key| key != &key_event.code);
                            } else {
                                keys.push(key_event.code);
                            }
                        }
                    }

                    if event == Event::Key(KeyCode::Esc.into()) {
                        break;
                    }
                }
                Some(Err(e)) => println!("Error: {:?}\r", e),
                None => break,
            }
        }

        execute!(stdout, DisableMouseCapture, PopKeyboardEnhancementFlags).unwrap();
        disable_raw_mode().unwrap();
    }

    const REGISTRIES_WIDTH: u16 = 10;

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
            MoveTo(Self::REGISTRIES_WIDTH, 0),
            Print(format!("{}\n", context.title.bold())),
            MoveToColumn(Self::REGISTRIES_WIDTH),
            Print(format!("╭{}╮\n", "──".repeat(self.width()))),
        )
        .unwrap();

        for y in 0..self.height() {
            execute!(stdout, MoveToColumn(Self::REGISTRIES_WIDTH), Print("│")).unwrap();
            for x in 0..self.width() {
                if context.pixels[y * self.width() + x] {
                    execute!(stdout, Print("██")).unwrap();
                } else {
                    execute!(stdout, Print("  ")).unwrap();
                }
            }

            execute!(stdout, Print("│\n")).unwrap();
        }

        execute!(
            stdout,
            MoveToColumn(Self::REGISTRIES_WIDTH),
            Print(format!("╰{}╯\n", "──".repeat(self.width()))),
            MoveToColumn(0),
        )
        .unwrap()
    }

    fn print_keyboard(&self, stdout: &mut Stdout) {
        let offset = Self::REGISTRIES_WIDTH + (self.width() * 2) as u16 + 2;

        execute!(
            stdout,
            MoveTo(offset, 1),
            Print(format!("╭─{}╮\n", "──────".repeat(4))),
        )
        .unwrap();

        for y in 0..4 {
            execute!(
                stdout,
                MoveToColumn(offset),
                Print(format!("│{} │\n", " ╭───╮".repeat(4))),
                MoveToColumn(offset),
                Print("│ "),
            )
            .unwrap();

            for x in 0..4 {
                let key = y * 4 + x;

                if self.is_code_pressed(key) {
                    execute!(
                        stdout,
                        Print(format!("│ {} │ ", format!("{:X}", key).bold()))
                    )
                    .unwrap();
                } else {
                    execute!(stdout, Print(format!("│ {:X} │ ", key))).unwrap();
                }
            }
            execute!(
                stdout,
                Print("│ \n"),
                MoveToColumn(offset),
                Print(format!("│{} │\n", " ╰───╯".repeat(4))),
            )
            .unwrap();
        }

        execute!(
            stdout,
            MoveToColumn(offset),
            Print(format!("╰─{}╯\n", "──────".repeat(4))),
        )
        .unwrap()
    }
}
