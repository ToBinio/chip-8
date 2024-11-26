use crate::io::{RenderContext, IO};
use crate::Emulator;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use web_sys::console::{log, log_1};

#[wasm_bindgen]
pub struct WebIO {
    width: usize,
    height: usize,
}

#[wasm_bindgen(module = "/js/io.ts")]
extern "C" {
    fn render(context: JsValue);
}

#[wasm_bindgen]
pub fn start() {
    log_1(&"asd".into());
    let program = include_bytes!("../../programs/ibm-logo.ch8").to_vec();

    log_1(&"asd2".into());
    let mut emulator = Emulator::new(program, "IBM".to_string(), WebIO::new(64, 32));

    log_1(&"asd3".into());
    emulator.run();
}

impl IO for WebIO {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn is_code_pressed(&self, code: u8) -> bool {
        false
    }

    fn should_shutdown(&self) -> bool {
        false
    }

    fn render(&self, context: RenderContext) {
        render(serde_wasm_bindgen::to_value(&context).unwrap())
    }
}

impl WebIO {
    pub fn new(height: usize, width: usize) -> WebIO {
        WebIO { height, width }
    }
}
