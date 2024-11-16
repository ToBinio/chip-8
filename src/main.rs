use crate::display::Display;
use crate::memory::Memory;
use crate::memory::ToU16;
use crate::memory::ToU8;
use std::thread::sleep;
use std::time::Duration;
use std::{env, fs};

mod display;
mod memory;

struct Emulator {
    memory: Memory,
    display: Display,
    pc: u16,
}

impl Emulator {
    fn new(program: Vec<u8>) -> Emulator {
        let mut memory = Memory::new(4096);
        memory.write_slice(0x200, &program);

        Emulator {
            memory,
            pc: 0x200,
            display: Display::new(64, 32),
        }
    }

    pub fn run(&mut self) {
        loop {
            //todo find end of program...
            self.tick();
            sleep(Duration::from_millis(2));
        }
    }

    fn tick(&mut self) {
        let instruction = self.memory.read_u16(self.pc as usize);
        let instruction_parts = memory::u16_to_u4_array(instruction);
        self.pc += 2;

        match (
            instruction_parts[0],
            instruction_parts[1],
            instruction_parts[2],
            instruction_parts[3],
        ) {
            (0x0, 0x0, 0xE, 0x0) => {
                self.display.clear();
            }
            (0x0, 0x0, 0xE, 0xE) => self.pc = self.memory.pop_stack(),
            (0x1, a, b, c) => {
                self.pc = (a, b, c).to_u16();
            }
            (0x2, a, b, c) => {
                self.memory.push_stack(self.pc);
                self.pc = (a, b, c).to_u16();
            }
            (0x3, x, a, b) => {
                if (self.memory.read_register(x as usize) == (a, b).to_u8()) {
                    self.pc += 2;
                }
            }
            (0x4, x, a, b) => {
                if (self.memory.read_register(x as usize) != (a, b).to_u8()) {
                    self.pc += 2;
                }
            }
            (0x5, x, y, 0) => {
                if (self.memory.read_register(x as usize) == self.memory.read_register(y as usize))
                {
                    self.pc += 2;
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

                let result = x_val as u16 | y_val as u16;

                self.memory
                    .write_register(0xF, if result > 255 { 1 } else { 0 });

                self.memory
                    .write_register(x as usize, (result % 255) as u8);
            }

            (0x8, x, y, 0x5) => {
                let x_val = self.memory.read_register(x as usize);
                let y_val = self.memory.read_register(y as usize);

                self.memory
                    .write_register(0xF, if x_val > y_val { 1 } else { 0 });

                self.memory
                    .write_register(x as usize, u8::wrapping_sub(x_val, y_val));
            }

            (0x8, x, y, 0x7) => {
                let x_val = self.memory.read_register(x as usize);
                let y_val = self.memory.read_register(y as usize);

                self.memory
                    .write_register(0xF, if y_val > x_val { 1 } else { 0 });

                self.memory
                    .write_register(x as usize, u8::wrapping_sub(y_val, x_val));
            }

            (0x9, x, y, 0) => {
                if (self.memory.read_register(x as usize) != self.memory.read_register(y as usize))
                {
                    self.pc += 2;
                }
            }
            (0xA, a, b, c) => {
                self.memory.write_index_register((a, b, c).to_u16());
            }
            (0xD, x, y, n) => {
                let x = self.memory.read_register(x as usize) as usize % self.display.width();
                let y = self.memory.read_register(y as usize) as usize % self.display.height();

                let mut current_pointer = self.memory.read_index_register();

                for y_off in 0..n as usize {
                    let mut to_render = self.memory.read_u8(current_pointer as usize);

                    for x_off in 0..8 {
                        if (to_render & 0x01) == 0x01 {
                            self.display.flip_pixel(x + 8 - x_off, y + y_off)
                        }

                        to_render = to_render >> 1
                    }

                    current_pointer += 1;
                }

                self.display.print();

                //todo set VF if something something got turned off
            }
            (_, _, _, _) => {
                panic!("unimplemented instruction: {:#06x}", instruction);
            }
        }
    }
}

fn main() {
    let program_path = env::args().skip(1).next();

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

    let mut emulator = Emulator::new(program);
    emulator.run();
}
