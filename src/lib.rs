use crate::clock::Clock;
use crate::gpu::Gpu;
use crate::io::{RenderContext, IO};
use crate::memory::Memory;
use crate::memory::ToU16;
use crate::memory::ToU8;
use rand::{thread_rng, Rng, RngCore};
use std::ops::Not;

pub mod clock;
pub mod gpu;
pub mod io;
pub mod memory;

pub mod programs;

#[derive(Debug)]
pub struct Emulator {
    program_name: String,
    memory: Memory,
    display: Gpu,
    clock: Clock,
}

impl Emulator {
    pub fn new(program: Vec<u8>, program_name: String, io: &dyn IO) -> Emulator {
        let mut memory = Memory::new(4096);

        memory.write_slice(0x200, &program);
        memory.write_pc(0x200);

        Emulator {
            program_name,
            memory,
            display: Gpu::new(io),
            clock: Clock::default(),
        }
    }

    pub fn tick(&mut self, io: &dyn IO) {
        self.run_instruction(io);
        self.clock.tick();
    }

    fn get_render_context(&self) -> RenderContext {
        RenderContext {
            title: &self.program_name,
            registries: self.memory.registers(),
            pixels: self.display.pixels(),
        }
    }

    fn run_instruction(&mut self, io: &dyn IO) {
        let instruction = self.memory.read_u16(self.memory.read_pc() as usize);
        let instruction_parts = memory::u16_to_u4_array(instruction);
        self.memory.increment_pc();

        println!("unimplemented instruction: {:#06x}", instruction);

        match (
            instruction_parts[0],
            instruction_parts[1],
            instruction_parts[2],
            instruction_parts[3],
        ) {
            (0x0, 0x0, 0xE, 0x0) | (0x0, 0x0, 0xF, 0xF) => {
                self.display.clear();
            }
            (0x0, 0x0, 0xE, 0xE) => {
                let stack = self.memory.pop_stack();
                self.memory.write_pc(stack)
            }
            (0x1, a, b, c) => {
                self.memory.write_pc((a, b, c).to_u16());
            }
            (0x2, a, b, c) => {
                self.memory.push_stack(self.memory.read_pc());
                self.memory.write_pc((a, b, c).to_u16())
            }
            (0x3, x, a, b) => {
                if self.memory.read_register(x as usize) == (a, b).to_u8() {
                    self.memory.increment_pc();
                }
            }
            (0x4, x, a, b) => {
                if self.memory.read_register(x as usize) != (a, b).to_u8() {
                    self.memory.increment_pc();
                }
            }
            (0x5, x, y, 0) => {
                if self.memory.read_register(x as usize) == self.memory.read_register(y as usize) {
                    self.memory.increment_pc();
                }
            }
            (0x6, x, a, b) => {
                self.memory.write_register(x as usize, (a, b).to_u8());
            }
            (0x7, x, a, b) => {
                let current = self.memory.read_register(x as usize);
                let to_add = (a, b).to_u8();

                let result = (current as u16 + to_add as u16) & 255;

                self.memory.write_register(x as usize, result as u8);
            }
            (0x8, x, y, 0x0) => {
                self.memory
                    .write_register(x as usize, self.memory.read_register(y as usize));
            }
            (0x8, x, y, 0x1) => {
                let x_val = self.memory.read_register(x as usize);
                let y_val = self.memory.read_register(y as usize);

                self.memory.write_register(x as usize, x_val | y_val);
            }
            (0x8, x, y, 0x2) => {
                let x_val = self.memory.read_register(x as usize);
                let y_val = self.memory.read_register(y as usize);

                self.memory.write_register(x as usize, x_val & y_val);
            }
            (0x8, x, y, 0x3) => {
                let x_val = self.memory.read_register(x as usize);
                let y_val = self.memory.read_register(y as usize);

                self.memory.write_register(x as usize, x_val ^ y_val);
            }
            (0x8, x, y, 0x4) => {
                let x_val = self.memory.read_register(x as usize);
                let y_val = self.memory.read_register(y as usize);

                self.memory
                    .write_register(x as usize, u8::wrapping_add(x_val, y_val));

                let result = x_val as u16 + y_val as u16;
                self.memory
                    .write_register(0xF, if result > 255 { 1 } else { 0 });
            }
            (0x8, x, y, 0x5) => {
                let x_val = self.memory.read_register(x as usize);
                let y_val = self.memory.read_register(y as usize);

                self.memory
                    .write_register(x as usize, u8::wrapping_sub(x_val, y_val));

                self.memory
                    .write_register(0xF, if x_val >= y_val { 1 } else { 0 });
            }
            (0x8, x, y, 0x6) => {
                let mut y_val = self.memory.read_register(y as usize);

                let rest = y_val & 0x01;
                y_val >>= 1;

                self.memory.write_register(x as usize, y_val);

                self.memory.write_register(0xF, rest);
            }
            (0x8, x, y, 0x7) => {
                let x_val = self.memory.read_register(x as usize);
                let y_val = self.memory.read_register(y as usize);

                self.memory
                    .write_register(x as usize, u8::wrapping_sub(y_val, x_val));

                self.memory
                    .write_register(0xF, if y_val >= x_val { 1 } else { 0 });
            }
            (0x8, x, y, 0xe) => {
                let mut y_val = self.memory.read_register(y as usize);

                let rest = (y_val & 0x80) >> 7;
                y_val <<= 1;

                self.memory.write_register(x as usize, y_val);

                self.memory.write_register(0xF, rest);
            }
            (0x9, x, y, 0) => {
                if self.memory.read_register(x as usize) != self.memory.read_register(y as usize) {
                    self.memory.increment_pc();
                }
            }
            (0xA, a, b, c) => {
                self.memory.write_index_register((a, b, c).to_u16());
            }
            (0xB, a, b, c) => {
                let offset = self.memory.read_register(0);
                self.memory.write_pc((a, b, c).to_u16() + offset as u16);
            }
            (0xD, x, y, n) => {
                //todo move to gpu

                let x = self.memory.read_register(x as usize) as usize % io.width();
                let y = self.memory.read_register(y as usize) as usize % io.height();

                let mut current_pointer = self.memory.read_index_register();

                let mut was_turned_off = false;

                for y_off in 0..n as usize {
                    let mut to_render = self.memory.read_u8(current_pointer as usize);

                    for x_off in 0..8 {
                        if (to_render & 0x01) == 0x01 {
                            if self.display.flip_pixel(x + 7 - x_off, y + y_off) {
                                was_turned_off = true;
                            }
                        }

                        to_render >>= 1
                    }

                    current_pointer += 1;
                }

                self.memory
                    .write_register(0xF, if was_turned_off { 1 } else { 0 });
            }
            (0xC, x, a, b) => {
                let nn = (a, b).to_u8();
                let mut rng = thread_rng();

                self.memory.write_register(x as usize, rng.gen::<u8>() & nn);
            }
            (0xE, x, 0x9, 0xE) => {
                if io.is_code_pressed(self.memory.read_register(x as usize)) {
                    self.memory.increment_pc();
                }
            }
            (0xE, x, 0xA, 0x1) => {
                if io
                    .is_code_pressed(self.memory.read_register(x as usize))
                    .not()
                {
                    self.memory.increment_pc();
                }
            }
            (0xF, x, 0x0, 0x7) => {
                self.memory
                    .write_register(x as usize, self.clock.delay_timer());
            }
            (0xF, x, 0x0, 0xA) => {
                self.memory.decrement_pc();

                let just_pressed = io.get_just_pressed();

                if let Some(key) = just_pressed.get(0) {
                    self.memory.write_register(x as usize, *key);
                    self.memory.increment_pc();
                }
            }
            (0xF, x, 0x1, 0x5) => {
                self.clock
                    .set_delay_timer(self.memory.read_register(x as usize));
            }
            (0xF, x, 0x1, 0x8) => {
                self.clock
                    .set_sound_timer(self.memory.read_register(x as usize));
            }
            (0xF, x, 0x1, 0xE) => {
                let x = self.memory.read_register(x as usize);
                let index = self.memory.read_index_register();

                self.memory.write_index_register(index + x as u16);
            }
            (0xF, x, 0x3, 0x3) => {
                let x = self.memory.read_register(x as usize);

                let index = self.memory.read_index_register() as usize;

                let first_digit = x / 100;
                let second_digit = (x / 10) % 10;
                let third_digit = x % 10;

                self.memory.write_u8(index, first_digit);
                self.memory.write_u8(index + 1, second_digit);
                self.memory.write_u8(index + 2, third_digit);
            }
            (0xF, x, 0x5, 0x5) => {
                for register in 0..=x {
                    self.memory.write_u8(
                        self.memory.read_index_register() as usize + register as usize,
                        self.memory.read_register(register as usize),
                    )
                }
            }
            (0xF, x, 0x6, 0x5) => {
                for register in 0..=x {
                    self.memory.write_register(
                        register as usize,
                        self.memory.read_u8(
                            self.memory.read_index_register() as usize + register as usize,
                        ),
                    );
                }
            }
            (_, _, _, _) => {
                panic!("unimplemented instruction: {:#06x}", instruction);
            }
        }
    }
}
