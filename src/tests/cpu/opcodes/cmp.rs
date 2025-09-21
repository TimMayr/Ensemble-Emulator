use crate::emulation::cpu::Cpu;

#[test]
fn test_cmp_immediate() {
    let mut cpu = Cpu::test_instance();
    cpu.accumulator = 0x20;
    cpu.mem_write(0x0, 0xC9);
    cpu.mem_write(0x1, 0x10);

    cpu.step(0);

    assert!(cpu.get_carry_flag());
    assert!(!cpu.get_zero_flag());
    assert!(!cpu.get_negative_flag());
}

#[test]
fn test_cmp_zero_page() {
    let mut cpu = Cpu::test_instance();
    cpu.accumulator = 0x20;
    cpu.mem_write(0x0, 0xC5);
    cpu.mem_write(0x1, 0xC5);
    cpu.mem_write(0xC5, 0x10);

    cpu.step(0);

    assert!(cpu.get_carry_flag());
    assert!(!cpu.get_zero_flag());
    assert!(!cpu.get_negative_flag());
}

#[test]
fn test_cmp_zero_page_x() {
    let mut cpu = Cpu::test_instance();
    cpu.accumulator = 0x20;
    cpu.x_register = 0x10;
    cpu.mem_write(0x0, 0xD5);
    cpu.mem_write(0xD5, 0xC5);

    cpu.step(0);

    assert!(cpu.get_carry_flag());
    assert!(!cpu.get_zero_flag());
    assert!(!cpu.get_negative_flag());
}

#[test]
fn test_cmp_absolute() {
    let mut cpu = Cpu::test_instance();
    cpu.accumulator = 0x20;
    cpu.mem_write(0x0, 0xCD);
    cpu.mem_write_u16(0x1, 0xCDCD);
    cpu.mem_write(0xCDCD, 0x10);

    cpu.step(0);

    assert!(cpu.get_carry_flag());
    assert!(!cpu.get_zero_flag());
    assert!(!cpu.get_negative_flag());
}

#[test]
fn test_cmp_absolute_x() {
    let mut cpu = Cpu::test_instance();
    cpu.accumulator = 0x20;
    cpu.x_register = 0x10;

    cpu.mem_write(0x0, 0xDD);
    cpu.mem_write_u16(0x1, 0xDDCD);
    cpu.mem_write(0xDDDD, 0x10);

    cpu.step(0);

    assert!(cpu.get_carry_flag());
    assert!(!cpu.get_zero_flag());
    assert!(!cpu.get_negative_flag());
}

#[test]
fn test_cmp_absolute_y() {
    let mut cpu = Cpu::test_instance();
    cpu.accumulator = 0x20;
    cpu.y_register = 0x10;

    cpu.mem_write(0x0, 0xD9);
    cpu.mem_write_u16(0x1, 0xDDCD);
    cpu.mem_write(0xDDDD, 0x10);

    cpu.step(0);

    assert!(cpu.get_carry_flag());
    assert!(!cpu.get_zero_flag());
    assert!(!cpu.get_negative_flag());
}

#[test]
fn test_cmp_indirect_x() {
    let mut cpu = Cpu::test_instance();
    cpu.accumulator = 0x20;
    cpu.x_register = 0x10;
    cpu.mem_write(0x0, 0xC1);
    cpu.mem_write(0x1, 0x33);
    cpu.mem_write_u16(0x43, 0x8343);
    cpu.mem_write(0x8343, 0x10);

    cpu.step(0);

    assert!(cpu.get_carry_flag());
    assert!(!cpu.get_zero_flag());
    assert!(!cpu.get_negative_flag());
}

#[test]
fn test_cmp_indirect_y() {
    let mut cpu = Cpu::test_instance();
    cpu.accumulator = 0x20;
    cpu.y_register = 10;
    cpu.mem_write(0x0, 0xD1);
    cpu.mem_write(0x1, 0x33);
    cpu.mem_write_u16(0x33, 0x8333);
    cpu.mem_write(0x8343, 0x10);
    cpu.step(0);

    assert!(cpu.get_carry_flag());
    assert!(!cpu.get_zero_flag());
    assert!(!cpu.get_negative_flag());
}

#[test]
fn test_cmp_zero_when_equal() {
    let mut cpu = Cpu::test_instance();
    cpu.accumulator = 0x20;
    cpu.mem_write(0x0, 0xC9);
    cpu.mem_write(0x1, 0x20);

    cpu.step(0);

    assert!(cpu.get_carry_flag());
    assert!(cpu.get_zero_flag());
    assert!(!cpu.get_negative_flag());
}

#[test]
fn test_cmp_negative_when_negative() {
    let mut cpu = Cpu::test_instance();
    cpu.accumulator = 0x20;
    cpu.mem_write(0x0, 0xC9);
    cpu.mem_write(0x1, 0x30);

    cpu.step(0);

    assert!(!cpu.get_carry_flag());
    assert!(!cpu.get_zero_flag());
    assert!(cpu.get_negative_flag());
}
