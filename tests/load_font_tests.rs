use hachi::Chip8;

#[test]
fn test_chip8_initializes_with_font_in_memory() {
    // Expected font set data (16 characters, 5 bytes each)
    let expected_font = [
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

    // Create a new Chip8 instance
    let chip8 = Chip8::default();

    // Font should be loaded starting at address 0x50
    let font_start_address = 0x50;

    // Verify each byte of the font data
    for (i, &expected_byte) in expected_font.iter().enumerate() {
        let actual_byte = chip8.memory[font_start_address + i];
        assert_eq!(
            actual_byte, expected_byte,
            "Font byte at offset {} should be 0x{:02X}, but found 0x{:02X}",
            i, expected_byte, actual_byte
        );
    }

    // Verify font data doesn't overwrite other memory areas
    // Memory before font should be zero
    for addr in 0..font_start_address {
        assert_eq!(
            chip8.memory[addr], 0,
            "Memory before font (at 0x{:02X}) should be zero",
            addr
        );
    }

    // Memory after font (until ROM area) should be zero
    let font_end_address = font_start_address + expected_font.len();
    for addr in font_end_address..0x200 {
        assert_eq!(
            chip8.memory[addr], 0,
            "Memory after font (at 0x{:02X}) should be zero",
            addr
        );
    }
}
