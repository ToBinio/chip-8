use async_std::stream::StreamExt;
use crossterm::cursor::position;
use crossterm::event::{
    DisableMouseCapture, EnableMouseCapture, Event, EventStream, KeyCode, KeyEvent, KeyEventKind,
    KeyboardEnhancementFlags, PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags,
};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io::stdout;
use std::sync::{Arc, Mutex};

pub struct Keyboard {
    pub pressed_keys: Arc<Mutex<Vec<KeyCode>>>,
}

impl Keyboard {
    pub fn new() -> Self {
        Keyboard {
            pressed_keys: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn map_key(&self, value: u8) -> KeyCode {
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

    pub fn is_key_pressed(&self, code: KeyCode) -> bool {
        self.pressed_keys.lock().unwrap().contains(&code)
    }

    pub async fn print_events(pressed_keys: Arc<Mutex<Vec<KeyCode>>>) {
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
                        match key_event.kind {
                            KeyEventKind::Press => {
                                let mut keys = pressed_keys.lock().unwrap();

                                if keys.contains(&key_event.code) {
                                    keys.retain(|key| key != &key_event.code);
                                } else {
                                    keys.push(key_event.code);
                                }
                            }
                            _ => {}
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
}
