#[derive(Debug)]
pub struct Memory {
    mem: Vec<u8>,
    registers: [u8; 16],
    index_register: u16,
    stack: Vec<u16>,
    pc: u16,
}

impl Memory {
    pub fn new(size: usize) -> Memory {
        Memory {
            mem: vec![0; size],
            registers: [0; 16],
            index_register: 0,
            stack: vec![],
            pc: 0,
        }
    }

    pub fn write_pc(&mut self, pc: u16) {
        self.pc = pc;
    }

    pub fn decrement_pc(&mut self) {
        self.pc -= 2;
    }

    pub fn increment_pc(&mut self) {
        self.pc += 2;
    }

    pub fn read_pc(&self) -> u16 {
        self.pc
    }

    pub fn write_slice(&mut self, index: usize, data: &[u8]) {
        self.mem[index..index + data.len()].copy_from_slice(data);
    }

    pub fn read_u8(&self, index: usize) -> u8 {
        self.mem[index]
    }

    pub fn write_u8(&mut self, index: usize, data: u8) {
        self.mem[index] = data;
    }

    pub fn read_u16(&self, index: usize) -> u16 {
        u16::from_be_bytes([self.mem[index], self.mem[index + 1]])
    }

    pub fn registers(&self) -> &[u8; 16] {
        &self.registers
    }

    pub fn read_register(&self, index: usize) -> u8 {
        self.registers[index]
    }

    pub fn write_register(&mut self, index: usize, values: u8) {
        self.registers[index] = values;
    }

    pub fn read_index_register(&self) -> u16 {
        self.index_register
    }

    pub fn write_index_register(&mut self, index: u16) {
        self.index_register = index;
    }

    pub fn push_stack(&mut self, value: u16) {
        self.stack.push(value);
    }

    pub fn pop_stack(&mut self) -> u16 {
        self.stack.pop().unwrap()
    }
}

pub fn u16_to_u4_array(value: u16) -> [u8; 4] {
    [
        ((value & 0xF000) >> 12) as u8,
        ((value & 0x0F00) >> 8) as u8,
        (value & 0x00F0) as u8 >> 4,
        (value & 0x000F) as u8,
    ]
}

pub trait ToU16 {
    fn to_u16(self) -> u16;
}

impl ToU16 for (u8, u8, u8) {
    fn to_u16(self) -> u16 {
        ((self.0 as u16) << 8) + ((self.1 as u16) << 4) + self.2 as u16
    }
}

pub trait ToU8 {
    fn to_u8(self) -> u8;
}

impl ToU8 for (u8, u8) {
    fn to_u8(self) -> u8 {
        (self.0 << 4) + self.1
    }
}
