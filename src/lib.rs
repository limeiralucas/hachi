use std::{io::{Error, ErrorKind, Read}};
use log::info;

pub struct Registers {
    pub v0: u8,
    pub v1: u8,
    pub v2: u8,
    pub v3: u8,
    pub v4: u8,
    pub v5: u8,
    pub v6: u8,
    pub v7: u8,
    pub v8: u8,
    pub v9: u8,
    pub va: u8,
    pub vb: u8,
    pub vc: u8,
    pub vd: u8,
    pub ve: u8,
    pub vf: u8,
}

impl Default for Registers {
    fn default() -> Self {
        Self {
            v0: 0,
            v1: 0,
            v2: 0,
            v3: 0,
            v4: 0,
            v5: 0,
            v6: 0,
            v7: 0,
            v8: 0,
            v9: 0,
            va: 0,
            vb: 0,
            vc: 0,
            vd: 0,
            ve: 0,
            vf: 0,
        }
    }
}

pub struct Chip8 {
    pub registers: Registers,
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
        Self {
            registers: Registers::default(),
            memory: [0; 4096],
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
        for (index, byte) in reader.bytes().enumerate() {
            let memory_address = self.pc as usize + index;
            if memory_address >= self.memory.len() {
                return Err(Error::new(ErrorKind::FileTooLarge, "ROM too large"));
            }

            self.memory[memory_address] = byte?;
        }
        info!("ROM loaded successfully");
        Ok(())
    }
} 