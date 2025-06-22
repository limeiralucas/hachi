use hachi::Chip8;

// Macro for hex values with configurable formatting
macro_rules! assert_hex_equal {
    // 8-bit hex format (default)
    ($name:expr, $expected:expr, $actual:expr) => {
        assert_eq!(
            $actual, $expected,
            "Expected {} to be 0x{:02X}, got 0x{:02X}",
            $name, $expected, $actual
        );
    };
    // 16-bit hex format
    ($name:expr, $expected:expr, $actual:expr, 16) => {
        assert_eq!(
            $actual, $expected,
            "Expected {} to be 0x{:04X}, got 0x{:04X}",
            $name, $expected, $actual
        );
    };
}

#[test]
fn test_clear_display() {
    let mut chip8 = Chip8 {
        video: [true; 64 * 32],
        ..Default::default()
    };

    let initial_pc = chip8.pc;
    let initial_sp = chip8.sp;
    let initial_memory = chip8.memory;

    chip8.clear_display();

    let expected_display = [false; 64 * 32];
    assert_eq!(
        chip8.video, expected_display,
        "Display should be completely cleared"
    );

    // Ensure other state remains unchanged
    assert_hex_equal!("program counter", initial_pc, chip8.pc, 16);
    assert_hex_equal!("stack pointer", initial_sp, chip8.sp);
    assert_eq!(chip8.memory, initial_memory, "Memory should not change");
}

#[test]
fn test_ret() {
    let mut chip8 = Chip8 {
        pc: 0x100,
        sp: 1,
        stack: {
            let mut stack = [0; 16];
            stack[0] = 0x200;
            stack
        },
        ..Default::default()
    };

    chip8.ret();

    assert_hex_equal!("program counter", 0x200, chip8.pc, 16);
    assert_hex_equal!("stack pointer", 0, chip8.sp);
}

#[test]
fn test_jump() {
    let mut chip8 = Chip8 {
        opcode: 0x1A59,
        ..Default::default()
    };

    chip8.jump();

    assert_hex_equal!("program counter", 0x0A59, chip8.pc, 16);
}

#[test]
fn test_call() {
    let mut chip8 = Chip8 {
        pc: 0x500,
        opcode: 0x2A59,
        ..Default::default()
    };

    chip8.call();

    assert_hex_equal!("program counter", 0x0A59, chip8.pc, 16);
    assert_hex_equal!("stack pointer", 1, chip8.sp);
    assert_hex_equal!("stack[0]", 0x500, chip8.stack[0], 16);
}

#[test]
fn test_skip_equal_vx_byte_should_skip() {
    let mut chip8 = Chip8 {
        pc: 0x3000,
        registers: {
            let mut registers = [0; 16];
            registers[0] = 0x55;
            registers
        },
        opcode: 0x3055,
        ..Default::default()
    };

    chip8.skip_equal_vx_byte();

    assert_hex_equal!("program counter", 0x3002, chip8.pc, 16);
}

#[test]
fn test_skip_equal_vx_byte_should_not_skip() {
    let mut chip8 = Chip8 {
        pc: 0x3000,
        registers: {
            let mut registers = [0; 16];
            registers[0] = 0x55;
            registers
        },
        opcode: 0x3056,
        ..Default::default()
    };

    chip8.skip_equal_vx_byte();

    assert_hex_equal!("program counter", 0x3000, chip8.pc, 16);
}

#[test]
fn test_skip_not_equal_vx_byte_should_skip() {
    let mut chip8 = Chip8 {
        pc: 0x4000,
        registers: {
            let mut registers = [0; 16];
            registers[0] = 0x55;
            registers
        },
        opcode: 0x4056,
        ..Default::default()
    };

    chip8.skip_not_equal_vx_byte();

    assert_hex_equal!("program counter", 0x4002, chip8.pc, 16);
}

#[test]
fn test_skip_not_equal_vx_byte_should_not_skip() {
    let mut chip8 = Chip8 {
        pc: 0x4000,
        registers: {
            let mut registers = [0; 16];
            registers[0] = 0x55;
            registers
        },
        opcode: 0x4055,
        ..Default::default()
    };

    chip8.skip_not_equal_vx_byte();

    assert_hex_equal!("program counter", 0x4000, chip8.pc, 16);
}

#[test]
fn test_skip_equal_vx_vy_should_skip() {
    let mut chip8 = Chip8 {
        pc: 0x5000,
        registers: {
            let mut registers = [0; 16];
            registers[0] = 0x55;
            registers[1] = 0x55;
            registers
        },
        opcode: 0x5010,
        ..Default::default()
    };

    chip8.skip_equal_vx_vy();

    assert_hex_equal!("program counter", 0x5002, chip8.pc, 16);
}

#[test]
fn test_skip_equal_vx_vy_should_not_skip() {
    let mut chip8 = Chip8 {
        pc: 0x5000,
        registers: {
            let mut registers = [0; 16];
            registers[0] = 0x55;
            registers[1] = 0x56;
            registers
        },
        opcode: 0x5010,
        ..Default::default()
    };

    chip8.skip_equal_vx_vy();

    assert_hex_equal!("program counter", 0x5000, chip8.pc, 16);
}

#[test]
fn test_load_vx_byte() {
    let mut chip8 = Chip8 {
        opcode: 0x6A55,
        ..Default::default()
    };

    chip8.load_vx_byte();

    assert_hex_equal!("register A", 0x55, chip8.registers[0xA]);
}

#[test]
fn test_load_vx_vy() {
    let mut chip8 = Chip8 {
        opcode: 0x8AB0,
        registers: {
            let mut registers = [0; 16];
            registers[0xA] = 0x05;
            registers[0xB] = 0xF2;
            registers
        },
        ..Default::default()
    };

    chip8.load_vx_vy();

    assert_hex_equal!("register A", 0xF2, chip8.registers[0xA]);
    assert_hex_equal!("register B", 0xF2, chip8.registers[0xB]);
}

#[test]
fn test_add_vx_byte() {
    let mut chip8 = Chip8 {
        registers: {
            let mut registers = [0; 16];
            registers[0xA] = 0x55;
            registers
        },
        opcode: 0x7A12,
        ..Default::default()
    };

    chip8.add_vx_byte();

    assert_hex_equal!("register A", 0x67, chip8.registers[0xA]);
}

#[test]
fn test_add_vx_byte_overflow() {
    let mut chip8 = Chip8 {
        registers: {
            let mut registers = [0; 16];
            registers[0xA] = 0xFF;
            registers
        },
        opcode: 0x7A02,
        ..Default::default()
    };

    chip8.add_vx_byte();

    assert_hex_equal!("register A", 0x01, chip8.registers[0xA]);
}

#[test]
fn test_or_vx_vy() {
    let mut chip8 = Chip8 {
        opcode: 0x8AB1,
        registers: {
            let mut registers = [0; 16];
            registers[0xA] = 0x01;
            registers[0xB] = 0x10;
            registers
        },
        ..Default::default()
    };

    chip8.or_vx_vy();

    assert_hex_equal!("register A", 0x11, chip8.registers[0xA]);
}

#[test]
fn test_and_vx_vy() {
    let mut chip8 = Chip8 {
        opcode: 0x8AB2,
        registers: {
            let mut registers = [0; 16];
            registers[0xA] = 0x11;
            registers[0xB] = 0x10;
            registers
        },
        ..Default::default()
    };

    chip8.and_vx_vy();

    assert_hex_equal!("register A", 0x10, chip8.registers[0xA]);
}

#[test]
fn test_xor_vx_vy() {
    let mut chip8 = Chip8 {
        opcode: 0x8AB3,
        registers: {
            let mut registers = [0; 16];
            registers[0xA] = 0x11;
            registers[0xB] = 0x10;
            registers
        },
        ..Default::default()
    };

    chip8.xor_vx_vy();

    assert_hex_equal!("register A", 0x01, chip8.registers[0xA]);
}

#[test]
fn test_add_vx_vy_without_overflow() {
    let mut chip8 = Chip8 {
        opcode: 0x8AB4,
        registers: {
            let mut registers = [0; 16];
            registers[0xA] = 0x10;
            registers[0xB] = 0x01;
            registers
        },
        ..Default::default()
    };

    chip8.add_vx_vy();

    assert_hex_equal!("register A", 0x11, chip8.registers[0xA]);
    assert_hex_equal!("register F", 0, chip8.registers[0xF]);
}

#[test]
fn test_add_vx_vy_with_overflow() {
    let mut chip8 = Chip8 {
        opcode: 0x8AB4,
        registers: {
            let mut registers = [0; 16];
            registers[0xA] = 0xFF;
            registers[0xB] = 0x11;
            registers
        },
        ..Default::default()
    };

    chip8.add_vx_vy();

    assert_hex_equal!("register A", 0x10, chip8.registers[0xA]);
    assert_hex_equal!("register F", 1, chip8.registers[0xF]);
}

#[test]
fn test_sub_vx_vy_with_overflow() {
    let mut chip8 = Chip8 {
        opcode: 0x8AB5,
        registers: {
            let mut registers = [0; 16];
            registers[0xA] = 0x00;
            registers[0xB] = 0x01;
            registers
        },
        ..Default::default()
    };

    chip8.sub_vx_vy();

    assert_hex_equal!("register A", 0xFF, chip8.registers[0xA]);
    assert_hex_equal!("register F", 0, chip8.registers[0xF]);
}

#[test]
fn test_sub_vx_vy_without_overflow() {
    let mut chip8 = Chip8 {
        opcode: 0x8AB5,
        registers: {
            let mut registers = [0; 16];
            registers[0xA] = 0x0A;
            registers[0xB] = 0x01;
            registers
        },
        ..Default::default()
    };

    chip8.sub_vx_vy();

    assert_hex_equal!("register A", 0x09, chip8.registers[0xA]);
    assert_hex_equal!("register F", 1, chip8.registers[0xF]);
}

#[test]
fn test_shr_vx_with_remainder() {
    let mut chip8 = Chip8 {
        opcode: 0x8A06,
        registers: {
            let mut registers = [0; 16];
            registers[0xA] = 0x05;
            registers
        },
        ..Default::default()
    };

    chip8.shr_vx();

    assert_hex_equal!("register A", 0x02, chip8.registers[0xA]);
    assert_hex_equal!("register F", 1, chip8.registers[0xF]);
}

#[test]
fn test_shr_vx_without_remainder() {
    let mut chip8 = Chip8 {
        opcode: 0x8A06,
        registers: {
            let mut registers = [0; 16];
            registers[0xA] = 0x04;
            registers
        },
        ..Default::default()
    };

    chip8.shr_vx();

    assert_hex_equal!("register A", 0x02, chip8.registers[0xA]);
    assert_hex_equal!("register F", 0, chip8.registers[0xF]);
}

#[test]
fn test_subn_vx_vy_with_overflow() {
    let mut chip8 = Chip8 {
        opcode: 0x8AB7,
        registers: {
            let mut registers = [0; 16];
            registers[0xA] = 0x0B;
            registers[0xB] = 0x0A;
            registers
        },
        ..Default::default()
    };

    chip8.subn_vx_vy();

    assert_hex_equal!("register A", 0xFF, chip8.registers[0xA]);
    assert_hex_equal!("register F", 0, chip8.registers[0xF]);
}

#[test]
fn test_subn_vx_vy_without_overflow() {
    let mut chip8 = Chip8 {
        opcode: 0x8AB7,
        registers: {
            let mut registers = [0; 16];
            registers[0xA] = 0x09;
            registers[0xB] = 0x0A;
            registers
        },
        ..Default::default()
    };

    chip8.subn_vx_vy();

    assert_hex_equal!("register A", 0x01, chip8.registers[0xA]);
    assert_hex_equal!("register F", 1, chip8.registers[0xF]);
}
