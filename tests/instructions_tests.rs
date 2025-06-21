use hachi::Chip8;

#[test]
fn test_clear_display() {
    let mut chip8 = Chip8::default();
    
    chip8.video.fill(true);
    
    let initial_pc = chip8.pc;
    let initial_sp = chip8.sp;
    let initial_memory = chip8.memory;
    
    chip8.clear_display();
    
    let expected_display = [false; 64 * 32];
    assert_eq!(chip8.video, expected_display, "Display should be completely cleared");
    
    // Ensure other state remains unchanged
    assert_eq!(chip8.pc, initial_pc, "Program counter should not change");
    assert_eq!(chip8.sp, initial_sp, "Stack pointer should not change");
    assert_eq!(chip8.memory, initial_memory, "Memory should not change");
}

#[test]
fn test_ret() {
    let mut chip8 = Chip8::default();

    chip8.pc = 0x100;
    chip8.sp = 1;
    chip8.stack[0] = 0x200;

    chip8.ret();

    assert_eq!(chip8.pc, 0x200);
    assert_eq!(chip8.sp, 0);
}

#[test]
fn test_jump() {
    let mut chip8 = Chip8::default();

    chip8.opcode = 0x1A59;

    chip8.jump();

    assert_eq!(chip8.pc, 0x0A59);
}

#[test]
fn test_call() {
    let mut chip8 = Chip8::default();

    chip8.pc = 0x500;
    chip8.opcode = 0x2A59;

    chip8.call();

    assert_eq!(chip8.pc, 0x0A59);
    assert_eq!(chip8.sp, 1);
    assert_eq!(chip8.stack[0], 0x500);
}

#[test]
fn test_skip_equal_vx_byte_should_skip() {
    let mut chip8 = Chip8::default();

    chip8.pc = 0x3000;
    chip8.registers[0] = 0x55;
    chip8.opcode = 0x3055;

    chip8.skip_equal_vx_byte();

    assert_eq!(chip8.pc, 0x3002, "Expected program counter to be 0x{:04X}, got 0x{:04X}", 0x3002, chip8.pc);
}

#[test]
fn test_skip_equal_vx_byte_should_not_skip() {
    let mut chip8 = Chip8::default();

    chip8.pc = 0x3000;
    chip8.registers[0] = 0x55;
    chip8.opcode = 0x3056;

    chip8.skip_equal_vx_byte();

    assert_eq!(chip8.pc, 0x3000, "Expected program counter to be 0x{:04X}, got 0x{:04X}", 0x3000, chip8.pc);
}

#[test]
fn test_skip_not_equal_vx_byte_should_skip() {
    let mut chip8 = Chip8::default();

    chip8.pc = 0x4000;
    chip8.registers[0] = 0x55;
    chip8.opcode = 0x4056;

    chip8.skip_not_equal_vx_byte();

    assert_eq!(chip8.pc, 0x4002, "Expected program counter to be 0x{:04X}, got 0x{:04X}", 0x4002, chip8.pc);
}

#[test]
fn test_skip_not_equal_vx_byte_should_not_skip() {
    let mut chip8 = Chip8::default();

    chip8.pc = 0x4000;
    chip8.registers[0] = 0x55;
    chip8.opcode = 0x4055;

    chip8.skip_not_equal_vx_byte();

    assert_eq!(chip8.pc, 0x4000, "Expected program counter to be 0x{:04X}, got 0x{:04X}", 0x4000, chip8.pc);
}

#[test]
fn test_skip_equal_vx_vy_should_skip() {
    let mut chip8 = Chip8::default();

    chip8.pc = 0x5000;
    chip8.registers[0] = 0x55;
    chip8.registers[1] = 0x55;
    chip8.opcode = 0x5010;

    chip8.skip_equal_vx_vy();

    assert_eq!(chip8.pc, 0x5002, "Expected program counter to be 0x{:04X}, got 0x{:04X}", 0x5002, chip8.pc);
}

#[test]
fn test_skip_equal_vx_vy_should_not_skip() {
    let mut chip8 = Chip8::default();

    chip8.pc = 0x5000;
    chip8.registers[0] = 0x55;
    chip8.registers[1] = 0x56;
    chip8.opcode = 0x5010;

    chip8.skip_equal_vx_vy();

    assert_eq!(chip8.pc, 0x5000, "Expected program counter to be 0x{:04X}, got 0x{:04X}", 0x5000, chip8.pc);
}

#[test]
fn test_load_vx_byte() {
    let mut chip8 = Chip8::default();

    chip8.opcode = 0x6A55;

    chip8.load_vx_byte();

    assert_eq!(chip8.registers[0xA], 0x55);
}