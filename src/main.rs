use std::{fs::File, io::Read};
use log::{error, info};

fn main() {
    // Initialize logger with default level if RUST_LOG is not set
    if std::env::var("RUST_LOG").is_err() {
        unsafe {
            std::env::set_var("RUST_LOG", "info");
        }
    }
    pretty_env_logger::init();

    let mut chip8 = Chip8::default();
    chip8.load_rom("roms/space_invaders.ch8");
}

struct Registers {
    v0: u8,
    v1: u8,
    v2: u8,
    v3: u8,
    v4: u8,
    v5: u8,
    v6: u8,
    v7: u8,
    v8: u8,
    v9: u8,
    va: u8,
    vb: u8,
    vc: u8,
    vd: u8,
    ve: u8,
    vf: u8,
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

struct Chip8 {
    registers: Registers,
    memory: [u8; 4096],
    index: u16,
    pc: u16,
    stack: [u16; 16],
    sp: u8,
    keypad: [bool; 16],
    video: [bool; 64 * 32],
    opcode: u16,
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
    fn load_rom(&mut self, filename: &str) {
        let file = File::open(filename).unwrap_or_else(|_| {
            error!("Failed to open ROM file: {} is invalid or not found", filename);
            std::process::exit(1);
        });

        for (index, byte) in file.bytes().enumerate() {
            self.memory[self.pc as usize + index] = byte.unwrap();
        }
        info!("ROM loaded successfully");
    }
}