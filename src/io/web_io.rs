use crate::io::{RenderContext, IO};
use crate::programs::Program;
use crate::Emulator;
use std::io::empty;
use std::sync::{LazyLock, Mutex, OnceLock};
use strum::IntoEnumIterator;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use web_sys::console::{log, log_1};
use web_sys::js_sys::Reflect::get;

#[wasm_bindgen]
#[derive(Debug)]
pub struct WebIO {
    width: usize,
    height: usize,
}

#[wasm_bindgen(module = "/js/io.ts")]
extern "C" {}

static EMULATOR: OnceLock<Mutex<Emulator>> = OnceLock::new();
static IO: OnceLock<Mutex<WebIO>> = OnceLock::new();

#[wasm_bindgen]
pub fn get_programs() -> Vec<JsValue> {
    Program::iter()
        .map(|value| serde_wasm_bindgen::to_value(&value).unwrap())
        .collect()
}

#[wasm_bindgen]
pub fn init(program: JsValue) {
    let program = serde_wasm_bindgen::from_value::<Program>(program)
        .unwrap()
        .source();
    let io = WebIO::new(64, 32);
    let emulator = Emulator::new(program, "IBM".to_string(), &io);

    match (IO.get(), EMULATOR.get()) {
        (Some(io_lock), Some(emulator_lock)) => {
            *io_lock.lock().unwrap() = io;
            *emulator_lock.lock().unwrap() = emulator;
        }
        (_, _) => {
            IO.set(Mutex::new(io)).unwrap();
            EMULATOR.set(Mutex::new(emulator)).unwrap();
        }
    }
}

#[wasm_bindgen]
pub fn tick() {
    let io = IO.get().unwrap().lock().unwrap();
    EMULATOR.get().unwrap().lock().unwrap().tick(&*io);
}

#[wasm_bindgen]
pub fn get_render_context() -> JsValue {
    let emulator = EMULATOR.get().unwrap().lock().unwrap();

    serde_wasm_bindgen::to_value(&emulator.get_render_context()).unwrap()
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
}

impl WebIO {
    pub fn new(width: usize, height: usize) -> WebIO {
        WebIO { height, width }
    }
}
