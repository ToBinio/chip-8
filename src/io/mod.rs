use serde::Serialize;

#[cfg(feature = "cli")]
pub mod terminal_io;

#[cfg(feature = "wasm")]
mod web_io;

#[derive(Serialize)]
pub struct RenderContext<'a> {
    pub title: &'a str,
    pub registries: &'a [u8; 16],
    pub pixels: &'a [bool],
}

pub trait IO {
    fn width(&self) -> usize;

    fn height(&self) -> usize;

    fn is_code_pressed(&self, code: u8) -> bool;
}

pub fn map_key(value: u8) -> char {
    match value {
        0x0 => 'x',
        0x1 => '1',
        0x2 => '2',
        0x3 => '3',
        0x4 => 'q',
        0x5 => 'w',
        0x6 => 'e',
        0x7 => 'a',
        0x8 => 's',
        0x9 => 'd',
        0xA => 'y',
        0xB => 'c',
        0xC => '4',
        0xD => 'r',
        0xE => 'f',
        0xF => 'v',
        _ => {
            panic!("unhandled key 0x{:x}", value);
        }
    }
}
