pub enum Program {
    Chip8Logo,
    Corax,
    Flags,
    IBM,
    Keypad,
}

impl Program {
    pub fn source(&self) -> Vec<u8> {
        match self {
            Program::Chip8Logo => include_bytes!("../programs/chip8-logo.ch8").to_vec(),
            Program::Corax => include_bytes!("../programs/corax.ch8").to_vec(),
            Program::Flags => include_bytes!("../programs/flags.ch8").to_vec(),
            Program::IBM => include_bytes!("../programs/ibm-logo.ch8").to_vec(),
            Program::Keypad => include_bytes!("../programs/keypad.ch8").to_vec(),
        }
    }
}