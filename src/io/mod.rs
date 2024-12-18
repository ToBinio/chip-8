use crate::Platform;
use serde::Serialize;

#[cfg(feature = "cli")]
pub mod terminal_io;

#[cfg(feature = "wasm")]
mod web_io;

#[derive(Serialize)]
pub struct RenderContext<'a> {
    pub platform: Platform,
    pub title: &'a str,
    pub registries: &'a [u8; 16],
    pub pixels: &'a [bool],
}

pub trait IO {
    fn is_code_pressed(&self, code: u8) -> bool;

    fn get_just_pressed(&self) -> Vec<u8>;
}

pub fn key_to_char(value: u8) -> Option<char> {
    match value {
        0x0 => Some('x'),
        0x1 => Some('1'),
        0x2 => Some('2'),
        0x3 => Some('3'),
        0x4 => Some('q'),
        0x5 => Some('w'),
        0x6 => Some('e'),
        0x7 => Some('a'),
        0x8 => Some('s'),
        0x9 => Some('d'),
        0xA => Some('y'),
        0xB => Some('c'),
        0xC => Some('4'),
        0xD => Some('r'),
        0xE => Some('f'),
        0xF => Some('v'),
        _ => {
            println!("Unknown key value: 0x{:x}", value);
            None
        }
    }
}

pub fn char_to_key(value: char) -> Option<u8> {
    match value {
        'x' => Some(0x0),
        '1' => Some(0x1),
        '2' => Some(0x2),
        '3' => Some(0x3),
        'q' => Some(0x4),
        'w' => Some(0x5),
        'e' => Some(0x6),
        'a' => Some(0x7),
        's' => Some(0x8),
        'd' => Some(0x9),
        'y' => Some(0xA),
        'c' => Some(0xB),
        '4' => Some(0xC),
        'r' => Some(0xD),
        'f' => Some(0xE),
        'v' => Some(0xF),
        _ => {
            println!("Unknown key value: {}", value);
            None
        }
    }
}
