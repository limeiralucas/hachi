use std::{env, fs::File};

use hachi::Chip8;
use log::error;

fn main() {
    if std::env::var("RUST_LOG").is_err() {
        unsafe {
            std::env::set_var("RUST_LOG", "info");
        }
    }
    pretty_env_logger::init();

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        error!("Invalid arguments. Usage: hachi <rom-filepath>");
        std::process::exit(1);
    }
    let rom_filepath = &args[1];

    let mut chip8 = Chip8::default();
    let file = File::open(rom_filepath).unwrap_or_else(|e| {
        error!("Failed to open ROM file: {}", e);
        std::process::exit(1);
    });

    if let Err(e) = chip8.load_rom_from_reader(file) {
        error!("Failed to load ROM: {}", e);
        std::process::exit(1);
    }
}
