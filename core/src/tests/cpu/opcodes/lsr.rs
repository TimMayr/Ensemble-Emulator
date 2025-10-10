use crate::emulation::cpu::Cpu;

#[test]
fn test_lsr_complete() {
    let mut cpu = Cpu::test_instance();
    cpu.mem_write(0x0, 0x4A);

    cpu.accumulator = 0b1000_0001;
    cpu.step();
    cpu.step();

    assert_eq!(cpu.accumulator, 0b0100_0000);
    assert!(cpu.get_carry_flag());
    assert!(!cpu.get_zero_flag());
    assert!(!cpu.get_negative_flag());

    cpu.mem_write(0x1, 0x46);
    cpu.mem_write(0x2, 0x0046);
    cpu.mem_write(0x0046, 0b0100_0000);

    cpu.step();
    cpu.step();
    cpu.step();
    cpu.step();
    cpu.step();

    assert_eq!(cpu.mem_read(0x0046), 0b0010_0000);
    assert!(!cpu.get_carry_flag());
    assert!(!cpu.get_zero_flag());
    assert!(!cpu.get_negative_flag());

    cpu.mem_write(0x3, 0x56);
    cpu.mem_write(0x4, 0x56);
    cpu.mem_write(0x57, 0b1111_1111);

    cpu.x_register = 0x01;

    cpu.step();
    cpu.step();
    cpu.step();
    cpu.step();
    cpu.step();
    cpu.step();
    assert_eq!(cpu.mem_read(0x57), 0b0111_1111);
    assert!(cpu.get_carry_flag());
    assert!(!cpu.get_zero_flag());
    assert!(!cpu.get_negative_flag());

    cpu.mem_write(0x5, 0x4E);
    cpu.mem_write_u16(0x6, 0x1234);
    cpu.mem_write(0x1234, 0b0000_0010);

    cpu.step();
    cpu.step();
    cpu.step();
    cpu.step();
    cpu.step();
    cpu.step();

    assert_eq!(cpu.mem_read(0x1234), 0b0000_0001);
    assert!(!cpu.get_carry_flag());
    assert!(!cpu.get_zero_flag());
    assert!(!cpu.get_negative_flag());

    cpu.mem_write(0x8, 0x5E);
    cpu.mem_write_u16(0x9, 0x8000);
    cpu.mem_write(0x8001, 0b0000_0001);
    cpu.x_register = 0x01;

    cpu.step();
    cpu.step();
    cpu.step();
    cpu.step();
    cpu.step();
    cpu.step();
    cpu.step();

    assert_eq!(cpu.mem_read(0x8001), 0);
    assert!(cpu.get_carry_flag());
    assert!(cpu.get_zero_flag());
    assert!(!cpu.get_negative_flag());

    cpu.mem_write(0xB, 0x4A);
    cpu.accumulator = 0b0011_1110;

    cpu.step();
    cpu.step();

    assert!(!cpu.get_carry_flag());
    assert!(!cpu.get_negative_flag());
    assert!(!cpu.get_zero_flag());

    cpu.mem_write(0xC, 0x4A);
    cpu.accumulator = 0b1000_0011;

    cpu.step();
    cpu.step();

    assert!(cpu.get_carry_flag());
    assert!(!cpu.get_negative_flag());
    assert!(!cpu.get_zero_flag());

    cpu.mem_write(0xD, 0x4A);
    cpu.accumulator = 0b1000_0011;

    cpu.step();
    cpu.step();

    assert!(cpu.get_carry_flag());
    assert!(!cpu.get_negative_flag());
    assert!(!cpu.get_zero_flag());

    cpu.mem_write(0xE, 0x4A);
    cpu.accumulator = 0b0000_0001;

    cpu.step();
    cpu.step();

    assert!(cpu.get_carry_flag());
    assert!(!cpu.get_negative_flag());
    assert!(cpu.get_zero_flag());
}

#[test]
fn test_lsr_accumulator() {
    let mut cpu = Cpu::test_instance();
    cpu.mem_write(0x0, 0x4A);

    cpu.accumulator = 0b1000_0001;
    cpu.step();
    cpu.step();

    assert_eq!(cpu.accumulator, 0b0100_0000);
    assert!(cpu.get_carry_flag());
    assert!(!cpu.get_zero_flag());
    assert!(!cpu.get_negative_flag());
}

#[test]
fn test_lsr_zero_page() {
    let mut cpu = Cpu::test_instance();
    cpu.mem_write(0x0, 0x46);
    cpu.mem_write(0x1, 0x10);
    cpu.mem_write(0x10, 0b0100_0000);

    cpu.step();
    cpu.step();
    cpu.step();
    cpu.step();
    cpu.step();

    assert_eq!(cpu.mem_read(0x10), 0b0010_0000);
    assert!(!cpu.get_carry_flag());
    assert!(!cpu.get_zero_flag());
    assert!(!cpu.get_negative_flag());
}

#[test]
fn test_lsr_zero_page_x() {
    let mut cpu = Cpu::test_instance();
    cpu.mem_write(0x0, 0x56);
    cpu.mem_write(0x1, 0x20);
    cpu.x_register = 0x01;

    cpu.mem_write(0x21, 0b1111_1111);

    cpu.step();
    cpu.step();
    cpu.step();
    cpu.step();
    cpu.step();
    cpu.step();

    assert_eq!(cpu.mem_read(0x21), 0b0111_1111);
    assert!(cpu.get_carry_flag());
    assert!(!cpu.get_zero_flag());
    assert!(!cpu.get_negative_flag());
}

#[test]
fn test_lsr_absolute() {
    let mut cpu = Cpu::test_instance();
    cpu.mem_write(0x0, 0x4E);
    cpu.mem_write_u16(0x1, 0x1234);
    cpu.mem_write(0x1234, 0b0000_0010);

    cpu.step();
    cpu.step();
    cpu.step();
    cpu.step();
    cpu.step();
    cpu.step();

    assert_eq!(cpu.mem_read(0x1234), 0b0000_0001);
    assert!(!cpu.get_carry_flag());
    assert!(!cpu.get_zero_flag());
    assert!(!cpu.get_negative_flag());
}

#[test]
fn test_lsr_absolute_x() {
    let mut cpu = Cpu::test_instance();
    cpu.mem_write(0x0, 0x5E);
    cpu.mem_write_u16(0x1, 0x8000);
    cpu.mem_write(0x8001, 0b0000_0001);

    cpu.x_register = 0x01;
    cpu.step();
    cpu.step();
    cpu.step();
    cpu.step();
    cpu.step();
    cpu.step();
    cpu.step();
    assert_eq!(cpu.mem_read(0x8001), 0);
    assert!(cpu.get_carry_flag());
    assert!(cpu.get_zero_flag());
    assert!(!cpu.get_negative_flag());
}

#[test]
fn test_lsr_flags_none_when_none() {
    let mut cpu = Cpu::test_instance();
    cpu.mem_write(0x0, 0x4A);
    cpu.accumulator = 0b0011_1110;

    cpu.step();
    cpu.step();

    assert!(!cpu.get_carry_flag());
    assert!(!cpu.get_negative_flag());
    assert!(!cpu.get_zero_flag());
}

#[test]
fn test_lsr_flags_only_carry_when_carry() {
    let mut cpu = Cpu::test_instance();
    cpu.mem_write(0x0, 0x4A);
    cpu.accumulator = 0b1000_0011;

    cpu.step();
    cpu.step();

    assert!(cpu.get_carry_flag());
    assert!(!cpu.get_negative_flag());
    assert!(!cpu.get_zero_flag());
}

#[test]
fn test_lsr_flags_only_zero_when_zero() {
    let mut cpu = Cpu::test_instance();
    cpu.mem_write(0x0, 0x4A);
    cpu.accumulator = 0b0000_0000;

    cpu.step();
    cpu.step();

    assert!(cpu.get_zero_flag());
    assert!(!cpu.get_carry_flag());
    assert!(!cpu.get_negative_flag());
}

#[test]
fn test_lsr_flags_carry_and_zero_when_carry_and_zero() {
    let mut cpu = Cpu::test_instance();
    cpu.mem_write(0x0, 0x4A);
    cpu.accumulator = 0b0000_0001;

    cpu.step();
    cpu.step();

    assert!(cpu.get_carry_flag());
    assert!(!cpu.get_negative_flag());
    assert!(cpu.get_zero_flag());
}
