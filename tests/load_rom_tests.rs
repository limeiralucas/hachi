use hachi::Chip8;
use std::io::{Cursor, ErrorKind};

#[test]
fn test_load_rom_from_reader_success() {
    // Create test ROM data
    let test_data = [0xA2, 0x2A, 0x60, 0x0C, 0x61, 0x08];
    let cursor = Cursor::new(test_data);
    
    let mut chip8 = Chip8::default();
    let initial_pc = chip8.pc;
    
    // Load the test ROM from memory
    let result = chip8.load_rom_from_reader(cursor);
    assert!(result.is_ok(), "Loading ROM should succeed");
    
    // Verify the ROM was loaded at the correct memory location
    for (i, &byte) in test_data.iter().enumerate() {
        assert_eq!(chip8.memory[initial_pc as usize + i], byte, 
                  "Byte at position {} should be 0x{:02X}", i, byte);
    }
}

#[test]
fn test_load_rom_from_reader_preserves_other_memory() {
    // Create test ROM data
    let test_data = [0xFF, 0xEE];
    let cursor = Cursor::new(test_data);
    
    let mut chip8 = Chip8::default();
    let initial_pc = chip8.pc;
    
    // Set some memory before loading ROM
    chip8.memory[0] = 0xAA;
    chip8.memory[4095] = 0xBB;
    
    // Load the test ROM from memory
    let result = chip8.load_rom_from_reader(cursor);
    assert!(result.is_ok(), "Loading ROM should succeed");
    
    // Verify ROM was loaded correctly
    assert_eq!(chip8.memory[initial_pc as usize], 0xFF);
    assert_eq!(chip8.memory[initial_pc as usize + 1], 0xEE);
    
    // Verify other memory was preserved
    assert_eq!(chip8.memory[0], 0xAA);
    assert_eq!(chip8.memory[4095], 0xBB);
}

#[test]
fn test_load_rom_from_reader_empty_data() {
    // Test with empty ROM data
    let cursor = Cursor::new(Vec::new());
    
    let mut chip8 = Chip8::default();
    let initial_memory = chip8.memory;
    
    // Load empty ROM
    let result = chip8.load_rom_from_reader(cursor);
    assert!(result.is_ok(), "Loading empty ROM should succeed");
    
    // Memory should be unchanged
    assert_eq!(chip8.memory, initial_memory);
}

#[test]
fn test_load_rom_from_reader_memory_overflow() {
    // Create ROM data that would overflow memory
    // Vec makes sense here because we're using the repeat pattern vec![0xFF; 4096]
    let oversized_data = vec![0xFF; 4096]; // This will definitely overflow
    let cursor = Cursor::new(oversized_data);
    
    let mut chip8 = Chip8::default();
    
    // This should fail with memory overflow error
    let result = chip8.load_rom_from_reader(cursor);
    assert!(result.is_err(), "Loading oversized ROM should fail");
    
    let error = result.unwrap_err();
    assert_eq!(error.kind(), ErrorKind::FileTooLarge, 
              "Error should be FileTooLarge, got: {:?}", error.kind());
    assert!(error.to_string().contains("ROM too large"), 
           "Error message should contain 'ROM too large', got: {}", error);
} 