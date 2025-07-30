use crate::cpu::Cpu;

#[test]
fn test_asl_complete() {
    let mut cpu = Cpu::new();
    cpu.mem_write(0x0, 0x0A);

    cpu.accumulator = 0b1000_0001;
    cpu.step();

    assert_eq!(cpu.accumulator, 0b0000_0010);
    assert!(cpu.get_carry_flag());
    assert!(!cpu.get_zero_flag());
    assert!(!cpu.get_negative_flag());

    cpu.mem_write(0x1, 0x06);
    cpu.mem_write(0x2, 0xA6);
    cpu.mem_write(0xA6, 0b0100_0000);

    cpu.step();

    assert_eq!(cpu.mem_read(0xA6), 0b1000_0000);
    assert!(!cpu.get_carry_flag());
    assert!(!cpu.get_zero_flag());
    assert!(cpu.get_negative_flag());

    cpu.mem_write(0x3, 0x16);
    cpu.mem_write(0x4, 0x20);
    cpu.mem_write(0x21, 0b1111_1111);

    cpu.x_register = 0x01;

    cpu.step();
    assert_eq!(cpu.mem_read(0x21), 0b1111_1110);
    assert!(cpu.get_carry_flag());
    assert!(!cpu.get_zero_flag());
    assert!(cpu.get_negative_flag());

    cpu.mem_write(0x5, 0x0E);
    cpu.mem_write_u16(0x6, 0x1234);
    cpu.mem_write(0x1234, 0b0000_0001);

    cpu.step();

    assert_eq!(cpu.mem_read(0x1234), 0b0000_0010);
    assert!(!cpu.get_carry_flag());
    assert!(!cpu.get_zero_flag());
    assert!(!cpu.get_negative_flag());

    cpu.mem_write(0x8, 0x1E);
    cpu.mem_write_u16(0x9, 0x2000);
    cpu.mem_write(0x2001, 0b1000_0000);

    cpu.x_register = 0x01;
    cpu.step();
    assert_eq!(cpu.mem_read(0x2001), 0);
    assert!(cpu.get_carry_flag());
    assert!(cpu.get_zero_flag());
    assert!(!cpu.get_negative_flag());

    cpu.mem_write(0xB, 0x0A);
    cpu.accumulator = 0b0011_1111;

    cpu.step();

    assert!(!cpu.get_carry_flag());
    assert!(!cpu.get_negative_flag());
    assert!(!cpu.get_zero_flag());

    cpu.mem_write(0xC, 0x0A);
    cpu.accumulator = 0b1000_0010;

    cpu.step();

    assert!(cpu.get_carry_flag());
    assert!(!cpu.get_negative_flag());
    assert!(!cpu.get_zero_flag());

    cpu.mem_write(0xD, 0x0A);
    cpu.accumulator = 0b0000_0000;

    cpu.step();

    assert!(cpu.get_zero_flag());
    assert!(!cpu.get_carry_flag());
    assert!(!cpu.get_negative_flag());

    cpu.mem_write(0xE, 0x0A);
    cpu.accumulator = 0b0100_0010;

    cpu.step();

    assert!(!cpu.get_carry_flag());
    assert!(cpu.get_negative_flag());
    assert!(!cpu.get_zero_flag());

    cpu.mem_write(0xF, 0x0A);
    cpu.accumulator = 0b1100_0000;

    cpu.step();

    assert!(cpu.get_carry_flag());
    assert!(cpu.get_negative_flag());
    assert!(!cpu.get_zero_flag());

    cpu.mem_write(0x10, 0x0A);
    cpu.accumulator = 0b1000_0000;

    cpu.step();

    assert!(cpu.get_carry_flag());
    assert!(!cpu.get_negative_flag());
    assert!(cpu.get_zero_flag());
}

#[test]
fn test_asl_accumulator() {
    let mut cpu = Cpu::new();
    cpu.mem_write(0x0, 0x0A);

    cpu.accumulator = 0b1000_0001;
    cpu.step();

    assert_eq!(cpu.accumulator, 0b0000_0010);
    assert!(cpu.get_carry_flag());
    assert!(!cpu.get_zero_flag());
    assert!(!cpu.get_negative_flag());
}

#[test]
fn test_asl_zero_page() {
    let mut cpu = Cpu::new();
    cpu.mem_write(0x0, 0x06);
    cpu.mem_write(0x1, 0x10);
    cpu.mem_write(0x10, 0b0100_0000);

    cpu.step();

    assert_eq!(cpu.mem_read(0x10), 0b1000_0000);
    assert!(!cpu.get_carry_flag());
    assert!(!cpu.get_zero_flag());
    assert!(cpu.get_negative_flag());
}

#[test]
fn test_asl_zero_page_x() {
    let mut cpu = Cpu::new();
    cpu.mem_write(0x0, 0x16);
    cpu.mem_write(0x1, 0x20);
    cpu.mem_write(0x21, 0b1111_1111);

    cpu.x_register = 0x01;

    cpu.step();
    assert_eq!(cpu.mem_read(0x21), 0b1111_1110);
    assert!(cpu.get_carry_flag());
    assert!(!cpu.get_zero_flag());
    assert!(cpu.get_negative_flag());
}

#[test]
fn test_asl_absolute() {
    let mut cpu = Cpu::new();
    cpu.mem_write(0x0, 0x0E);
    cpu.mem_write_u16(0x1, 0x1234);
    cpu.mem_write(0x1234, 0b0000_0001);

    cpu.step();

    assert_eq!(cpu.mem_read(0x1234), 0b0000_0010);
    assert!(!cpu.get_carry_flag());
    assert!(!cpu.get_zero_flag());
    assert!(!cpu.get_negative_flag());
}

#[test]
fn test_asl_absolute_x() {
    let mut cpu = Cpu::new();
    cpu.mem_write(0x0, 0x1E);
    cpu.mem_write_u16(0x1, 0x2000);
    cpu.mem_write(0x2001, 0b1000_0000);

    cpu.x_register = 0x01;
    cpu.step();
    assert_eq!(cpu.mem_read(0x2001), 0);
    assert!(cpu.get_carry_flag());
    assert!(cpu.get_zero_flag());
    assert!(!cpu.get_negative_flag());
}

#[test]
fn test_asl_flags_none_when_none() {
    let mut cpu = Cpu::new();
    cpu.mem_write(0x0, 0x0A);
    cpu.accumulator = 0b0011_1111;

    cpu.step();

    assert!(!cpu.get_carry_flag());
    assert!(!cpu.get_negative_flag());
    assert!(!cpu.get_zero_flag());
}

#[test]
fn test_asl_flags_only_carry_when_carry() {
    let mut cpu = Cpu::new();
    cpu.mem_write(0x0, 0x0A);
    cpu.accumulator = 0b1000_0010;

    cpu.step();

    assert!(cpu.get_carry_flag());
    assert!(!cpu.get_negative_flag());
    assert!(!cpu.get_zero_flag());
}

#[test]
fn test_asl_flags_only_zero_when_zero() {
    let mut cpu = Cpu::new();
    cpu.mem_write(0x0, 0x0A);
    cpu.accumulator = 0b0000_0000;

    cpu.step();

    assert!(cpu.get_zero_flag());
    assert!(!cpu.get_carry_flag());
    assert!(!cpu.get_negative_flag());
}

#[test]
fn test_asl_flags_only_negative_when_negative() {
    let mut cpu = Cpu::new();
    cpu.mem_write(0x0, 0x0A);
    cpu.accumulator = 0b0100_0010;

    cpu.step();

    assert!(!cpu.get_carry_flag());
    assert!(cpu.get_negative_flag());
    assert!(!cpu.get_zero_flag());
}

#[test]
fn test_asl_flags_carry_and_negative_when_carry_and_negative() {
    let mut cpu = Cpu::new();
    cpu.mem_write(0x0, 0x0A);
    cpu.accumulator = 0b1100_0000;

    cpu.step();

    assert!(cpu.get_carry_flag());
    assert!(cpu.get_negative_flag());
    assert!(!cpu.get_zero_flag());
}

#[test]
fn test_asl_flags_carry_and_zero_when_carry_and_zero() {
    let mut cpu = Cpu::new();
    cpu.mem_write(0x0, 0x0A);
    cpu.accumulator = 0b1000_0000;

    cpu.step();

    assert!(cpu.get_carry_flag());
    assert!(!cpu.get_negative_flag());
    assert!(cpu.get_zero_flag());
}
