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

    fn should_shutdown(&self) -> bool;

    fn render(&self, context: RenderContext);
}
