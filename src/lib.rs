use std::{io::{Error, ErrorKind, Read}};
use log::info;
use rand::Rng;

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
	0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

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
        let mut memory = [0; 4096];
        memory[0x50..0x50 + FONTSET.len()].copy_from_slice(&FONTSET);

        Self {
            registers: Registers::default(),
            memory: memory,
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

    fn rand_gen() -> u8 {
        let mut rng = rand::rng();

        return rng.random_range(0..=255);
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
} 