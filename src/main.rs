use std::thread::sleep;
use std::time::Duration;
use std::{env, fs};

struct Emulator {
    memory: Vec<u8>,
    pc: u16,
    //64x32
    display: Vec<bool>,
    registers: [u8; 16],
    index_register: u16,
}

impl Emulator {
    fn new(program: Vec<u8>) -> Emulator {
        let mut memory = vec![0u8; 4096];
        memory[0x200..0x200 + program.len()].copy_from_slice(&program);

        Emulator {
            memory,
            pc: 0x200,
            display: vec![true; 64 * 32],
            registers: [0; 16],
            index_register: 0,
        }
    }

    pub fn run(&mut self) {
        loop {
            //todo find end of program...
            self.tick();
            sleep(Duration::from_millis(100));
        }
    }

    fn tick(&mut self) {
        let instruction = self.get_current_instruction();
        self.pc += 2;

        match (
            instruction[0],
            instruction[1],
            instruction[2],
            instruction[3],
        ) {
            (0x0, 0x0, 0xE, 0x0) => {
                self.display.fill(false);
            }
            (0x1, a, b, c) => {
                self.pc = ((a as u16) << 8) + ((b as u16) << 4) + c as u16;
            }
            (0x6, x, a, b) => {
                self.registers[x as usize] = (a << 4) + b;
            }
            (0x7, x, a, b) => {
                self.registers[x as usize] += (a << 4) + b;
            }
            (0xA, a, b, c) => {
                self.index_register = ((a as u16) << 8) + ((b as u16) << 4) + c as u16;
            }
            (0xD, x, y, n) => {
                let x = self.registers[x as usize] as usize % 64;
                let y = self.registers[y as usize] as usize % 32;

                let mut current_pointer = self.index_register as usize;

                for y_off in 0..n as usize {
                    let mut to_render = self.memory[current_pointer];

                    for x_off in 0..8 {
                        if (to_render & 0x01) == 0x01 {
                            self.flip_pixel(x + 8 - x_off, y + y_off)
                        }

                        to_render = to_render >> 1
                    }

                    current_pointer += 1;
                }

                self.render();

                //todo set VF if something something got turned off
            }
            (_, _, _, _) => {
                panic!(
                    "unimplemented instruction: {}",
                    fmt_instruction(&instruction)
                );
            }
        }
    }

    fn flip_pixel(&mut self, x: usize, y: usize) {
        self.display[y * 64 + x] = !self.display[y * 64 + x];
    }

    fn render(&mut self) {
        for y in 0..32 {
            for x in 0..64 {
                if self.display[y * 64 + x] {
                    print!("█")
                } else {
                    print!(".")
                }
            }

            println!();
        }

        println!();
        println!();
    }

    fn get_current_instruction(&self) -> [u8; 4] {
        let first_byte = self.memory[self.pc as usize];
        let second_byte = self.memory[self.pc as usize + 1];

        [
            (first_byte & 0xF0) >> 4,
            first_byte & 0x0F,
            (second_byte & 0xF0) >> 4,
            second_byte & 0x0F,
        ]
    }
}

fn fmt_instruction(instruction: &[u8; 4]) -> String {
    format!(
        "{:#06x}",
        instruction[3] as u16
            + ((instruction[2] as u16) << 4)
            + ((instruction[1] as u16) << 8)
            + ((instruction[0] as u16) << 12)
    )
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
