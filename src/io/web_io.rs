use crate::io::{char_to_key, key_to_char, IO};
use crate::programs::Program;
use crate::{Emulator, Platform};
use std::ops::Not;
use std::sync::{Mutex, OnceLock};
use strum::IntoEnumIterator;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use web_sys::console::log_1;

#[wasm_bindgen]
#[derive(Debug)]
pub struct WebIO {
    pressed_keys: Vec<char>,
    just_pressed_keys: Vec<char>,
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
    let io = WebIO::new();
    let emulator = Emulator::new(program, "IBM".to_string(), Platform::SuperChip, &io);

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
    let mut io = IO.get().unwrap().lock().unwrap();
    EMULATOR.get().unwrap().lock().unwrap().tick(&*io);
    io.just_pressed_keys.clear();
}

#[wasm_bindgen]
pub fn get_render_context() -> JsValue {
    let emulator = EMULATOR.get().unwrap().lock().unwrap();

    serde_wasm_bindgen::to_value(&emulator.get_render_context()).unwrap()
}

#[wasm_bindgen]
pub fn on_key_down(key: char) {
    let mut io = IO.get().unwrap().lock().unwrap();
    if io.pressed_keys.contains(&key).not() {
        io.pressed_keys.push(key);
    }

    io.just_pressed_keys.push(key);
}

#[wasm_bindgen]
pub fn on_key_up(key: char) {
    let mut io = IO.get().unwrap().lock().unwrap();
    io.pressed_keys.retain(|&x| x != key);
}

impl IO for WebIO {
    fn is_code_pressed(&self, code: u8) -> bool {
        let Some(key) = key_to_char(code) else {
            return true;
        };

        self.pressed_keys.contains(&key)
    }

    fn get_just_pressed(&self) -> Vec<u8> {
        self.just_pressed_keys
            .iter()
            .filter_map(|char| char_to_key(char.clone()))
            .collect()
    }
}

impl WebIO {
    pub fn new() -> WebIO {
        WebIO {
            pressed_keys: vec![],
            just_pressed_keys: vec![],
        }
    }
}
