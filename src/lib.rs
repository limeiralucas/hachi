use log::info;
use rand::Rng;
use std::io::{BufReader, Error, ErrorKind, Read};

const FONTSET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub struct Chip8 {
    pub registers: [u8; 16],
    pub memory: [u8; 4096],
    pub index: u16,
    pub pc: u16,
    pub stack: [u16; 16],
    pub sp: u8,
    pub keypad: [bool; 16],
    pub video: [bool; 64 * 32],
    pub opcode: u16,
}

impl Default for Chip8 {
    fn default() -> Self {
        let mut memory = [0; 4096];
        memory[0x50..0x50 + FONTSET.len()].copy_from_slice(&FONTSET);

        Self {
            registers: [0; 16],
            memory,
            index: 0,
            pc: 0x200,
            stack: [0; 16],
            sp: 0,
            keypad: [false; 16],
            video: [false; 64 * 32],
            opcode: 0,
        }
    }
}

impl Chip8 {
    pub fn load_rom_from_reader<R: Read>(&mut self, reader: R) -> Result<(), Error> {
        let buf_reader = BufReader::new(reader);

        for (index, byte) in buf_reader.bytes().enumerate() {
            let memory_address = self.pc as usize + index;
            if memory_address >= self.memory.len() {
                return Err(Error::new(ErrorKind::FileTooLarge, "ROM too large"));
            }

            self.memory[memory_address] = byte?;
        }
        info!("ROM loaded successfully");
        Ok(())
    }

    #[allow(dead_code)]
    fn rand_gen() -> u8 {
        let mut rng = rand::rng();

        rng.random_range(0..=255)
    }

    pub fn clear_display(&mut self) {
        self.video = [false; 64 * 32];
    }

    pub fn ret(&mut self) {
        self.sp -= 1;
        self.pc = self.stack[self.sp as usize];
    }

    pub fn jump(&mut self) {
        self.pc = self.opcode & 0x0FFFu16;
    }

    pub fn call(&mut self) {
        self.stack[self.sp as usize] = self.pc;
        self.sp += 1;
        self.pc = self.opcode & 0x0FFFu16;
    }

    pub fn skip_equal_vx_byte(&mut self) {
        let vx = (self.opcode & 0x0F00) >> 8;
        let byte = (self.opcode & 0x00FFu16) as u8;

        if self.registers[vx as usize] == byte {
            self.pc += 2;
        }
    }

    pub fn skip_not_equal_vx_byte(&mut self) {
        let vx = (self.opcode & 0x0F00) >> 8;
        let byte = (self.opcode & 0x00FFu16) as u8;

        if self.registers[vx as usize] != byte {
            self.pc += 2;
        }
    }

    pub fn skip_equal_vx_vy(&mut self) {
        let vx = (self.opcode & 0x0F00) >> 8;
        let vy = (self.opcode & 0x00F0) >> 4;

        if self.registers[vx as usize] == self.registers[vy as usize] {
            self.pc += 2;
        }
    }

    pub fn load_vx_byte(&mut self) {
        let vx = (self.opcode & 0x0F00) >> 8;
        let byte = (self.opcode & 0x00FFu16) as u8;

        self.registers[vx as usize] = byte;
    }

    pub fn load_vx_vy(&mut self) {
        let vx = (self.opcode & 0x0F00) >> 8;
        let vy = (self.opcode & 0x00F0) >> 4;

        self.registers[vx as usize] = self.registers[vy as usize];
    }

    pub fn add_vx_byte(&mut self) {
        let vx = (self.opcode & 0x0F00) >> 8;
        let byte = (self.opcode & 0x00FFu16) as u8;

        self.registers[vx as usize] = self.registers[vx as usize].wrapping_add(byte);
    }

    pub fn or_vx_vy(&mut self) {
        let vx = (self.opcode & 0x0F00u16) >> 8;
        let vy = (self.opcode & 0x00F0u16) >> 4;

        self.registers[vx as usize] |= self.registers[vy as usize];
    }

    pub fn and_vx_vy(&mut self) {
        let vx = (self.opcode & 0x0F00u16) >> 8;
        let vy = (self.opcode & 0x00F0u16) >> 4;

        self.registers[vx as usize] &= self.registers[vy as usize];
    }

    pub fn xor_vx_vy(&mut self) {
        let vx = (self.opcode & 0x0F00u16) >> 8;
        let vy = (self.opcode & 0x00F0u16) >> 4;

        self.registers[vx as usize] ^= self.registers[vy as usize];
    }

    pub fn add_vx_vy(&mut self) {
        let vx = (self.opcode & 0x0F00u16) >> 8;
        let vy = (self.opcode & 0x00F0u16) >> 4;

        let (result, has_overflow) =
            self.registers[vx as usize].overflowing_add(self.registers[vy as usize]);

        self.registers[vx as usize] = result;
        self.registers[0xF] = has_overflow as u8;
    }

    pub fn sub_vx_vy(&mut self) {
        let vx = (self.opcode & 0x0F00u16) >> 8;
        let vy = (self.opcode & 0x00F0u16) >> 4;

        let (result, has_borrow) =
            self.registers[vx as usize].overflowing_sub(self.registers[vy as usize]);

        self.registers[vx as usize] = result;
        self.registers[0xF] = !has_borrow as u8;
    }

    pub fn shr_vx(&mut self) {
        let vx = (self.opcode & 0x0F00u16) >> 8;
        let lsb = self.registers[vx as usize] & 0x1u8;

        self.registers[0xF] = lsb;
        self.registers[vx as usize] >>= 1;
    }

    pub fn subn_vx_vy(&mut self) {
        let vx = (self.opcode & 0x0F00u16) >> 8;
        let vy = (self.opcode & 0x00F0u16) >> 4;

        let (result, has_borrow) =
            self.registers[vy as usize].overflowing_sub(self.registers[vx as usize]);

        self.registers[vx as usize] = result;
        self.registers[0xF] = !has_borrow as u8;
    }

    pub fn shl_vx(&mut self) {
        let vx = (self.opcode & 0x0F00u16) >> 8;

        let (result, has_overflow) = self.registers[vx as usize].overflowing_mul(2);

        self.registers[0xF] = has_overflow as u8;
        self.registers[vx as usize] = result;
    }

    pub fn skip_not_equal_vx_vy(&mut self) {
        let vx = (self.opcode & 0x0F00u16) >> 8;
        let vy = (self.opcode & 0x00F0u16) >> 4;

        if self.registers[vx as usize] != self.registers[vy as usize] {
            self.pc += 2;
        }
    }
}
