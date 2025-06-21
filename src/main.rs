use std::{fs::File};

use hachi::Chip8;
use log::error;

fn main() {
    if std::env::var("RUST_LOG").is_err() {
        unsafe {
            std::env::set_var("RUST_LOG", "info");
        }
    }
    pretty_env_logger::init();

    let mut chip8 = Chip8::default();
    let file = File::open("roms/space_invaders.ch8").unwrap_or_else(|e| {
        error!("Failed to open ROM file: {}", e);
        std::process::exit(1);
    });

    if let Err(e) = chip8.load_rom_from_reader(file) {
        error!("Failed to load ROM: {}", e);
        std::process::exit(1);
    }
}