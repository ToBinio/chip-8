use chip_8::io::terminal_io::TerminalIO;
use chip_8::Emulator;
use std::{env, fs};

fn main() {
    let program_path = env::args().nth(1);

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

    let mut emulator = Emulator::new(program, program_path, TerminalIO::new(64, 32));
    emulator.run();
}
