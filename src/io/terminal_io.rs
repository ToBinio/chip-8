use crate::io::{map_key, RenderContext, IO};
use crate::Emulator;
use async_std::stream::StreamExt;
use crossterm::cursor::{Hide, MoveTo, MoveToColumn, Show};
use crossterm::event::{
    DisableMouseCapture, EnableMouseCapture, Event, EventStream, KeyCode, KeyEventKind,
    KeyboardEnhancementFlags, PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags,
};
use crossterm::style::{Print, Stylize};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use crossterm::{execute, queue};
use std::io::{stdout, Stdout, Write};
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;
use std::{env, fs};

pub fn run() {
    let program_path = env::args().nth(1);

    let program_path = match program_path {
        Some(program_path) => program_path,
        None => {
            println!("Please specify a program path");
            return;
        }
    };

    println!("{}", program_path);
    let program = fs::read(&program_path);

    let program = match program {
        Ok(program) => program,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };

    let io = TerminalIO::new(64, 32);
    let emulator = Emulator::new(program, program_path, &io);

    TerminalIO::start(io, emulator);
}

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
}
impl TerminalIO {
    pub fn new(width: usize, height: usize) -> Self {
        TerminalIO {
            pressed_keys: Arc::new(Mutex::new(Vec::new())),
            screen_width: width,
            screen_height: height,
        }
    }
    fn map_key(&self, value: u8) -> KeyCode {
        KeyCode::Char(map_key(value))
    }
    fn is_key_pressed(&self, code: KeyCode) -> bool {
        self.pressed_keys.lock().unwrap().contains(&code)
    }

    pub fn start(terminal_io: TerminalIO, mut emulator: Emulator) {
        let pressed_keys = terminal_io.pressed_keys.clone();

        let pressed_keys_clone = pressed_keys.clone();
        async_std::task::spawn(async move {
            TerminalIO::start_listening(pressed_keys_clone).await;
        });

        let mut stdout = stdout();
        execute!(stdout, MoveTo(0, 0), Clear(ClearType::All)).unwrap();

        while !terminal_io.is_key_pressed(KeyCode::Esc) {
            emulator.tick(&terminal_io);
            terminal_io.render(emulator.get_render_context());
            sleep(Duration::from_millis(10));
        }

        execute!(stdout, MoveTo(0, 0), Clear(ClearType::All)).unwrap();
    }

    async fn start_listening(pressed_keys: Arc<Mutex<Vec<KeyCode>>>) {
        let mut reader = EventStream::new();

        enable_raw_mode().unwrap();

        let mut stdout = stdout();
        execute!(
            stdout,
            Hide,
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

        execute!(
            stdout,
            Show,
            DisableMouseCapture,
            PopKeyboardEnhancementFlags
        )
        .unwrap();
        disable_raw_mode().unwrap();
    }

    const REGISTRIES_WIDTH: u16 = 10;

    fn render(&self, context: RenderContext) {
        let mut stdout = stdout();

        self.print_registries(&context, &mut stdout);
        self.print_keyboard(&mut stdout);
        self.print_screen(&context, &mut stdout);

        stdout.flush().unwrap()
    }

    fn print_registries(&self, context: &RenderContext, stdout: &mut Stdout) {
        queue!(stdout, MoveTo(0, 1), Print("Registers\n"),).unwrap();

        for register in context.registries {
            queue!(
                stdout,
                MoveToColumn(0),
                Print(format!("{:#04x}\n", register))
            )
            .unwrap();
        }
    }

    fn print_screen(&self, context: &RenderContext, stdout: &mut Stdout) {
        queue!(
            stdout,
            MoveTo(Self::REGISTRIES_WIDTH, 0),
            Print(format!("{}\n", context.title.bold())),
            MoveToColumn(Self::REGISTRIES_WIDTH),
            Print(format!("╭{}╮\n", "──".repeat(self.width()))),
        )
        .unwrap();

        for y in 0..self.height() {
            queue!(stdout, MoveToColumn(Self::REGISTRIES_WIDTH), Print("│")).unwrap();
            for x in 0..self.width() {
                if context.pixels[y * self.width() + x] {
                    queue!(stdout, Print("██")).unwrap();
                } else {
                    queue!(stdout, Print("  ")).unwrap();
                }
            }

            queue!(stdout, Print("│\n")).unwrap();
        }

        queue!(
            stdout,
            MoveToColumn(Self::REGISTRIES_WIDTH),
            Print(format!("╰{}╯\n", "──".repeat(self.width()))),
            MoveToColumn(0),
        )
        .unwrap()
    }

    fn print_keyboard(&self, stdout: &mut Stdout) {
        let offset = Self::REGISTRIES_WIDTH + (self.width() * 2) as u16 + 2;

        queue!(
            stdout,
            MoveTo(offset, 1),
            Print(format!("╭─{}╮\n", "──────".repeat(4))),
        )
        .unwrap();

        for y in 0..4 {
            queue!(
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
                    queue!(
                        stdout,
                        Print(format!("│ {} │ ", format!("{:X}", key).bold()))
                    )
                    .unwrap();
                } else {
                    queue!(stdout, Print(format!("│ {:X} │ ", key))).unwrap();
                }
            }
            queue!(
                stdout,
                Print("│ \n"),
                MoveToColumn(offset),
                Print(format!("│{} │\n", " ╰───╯".repeat(4))),
            )
            .unwrap();
        }

        queue!(
            stdout,
            MoveToColumn(offset),
            Print(format!("╰─{}╯\n", "──────".repeat(4))),
        )
        .unwrap()
    }
}
