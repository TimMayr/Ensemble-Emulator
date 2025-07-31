use crate::cpu::Cpu;

#[test]
fn test_cpy_immediate() {
    let mut cpu = Cpu::new();
    cpu.y_register = 0x20;
    cpu.mem_write(0x0, 0xC0);
    cpu.mem_write(0x1, 0x10);

    cpu.step();

    assert!(cpu.get_carry_flag());
    assert!(!cpu.get_zero_flag());
    assert!(!cpu.get_negative_flag());
}

#[test]
fn test_cpy_zero_page() {
    let mut cpu = Cpu::new();
    cpu.y_register = 0x20;
    cpu.mem_write(0x0, 0xC4);
    cpu.mem_write(0x1, 0xC5);
    cpu.mem_write(0xC5, 0x10);

    cpu.step();

    assert!(cpu.get_carry_flag());
    assert!(!cpu.get_zero_flag());
    assert!(!cpu.get_negative_flag());
}

#[test]
fn test_cpy_absolute() {
    let mut cpu = Cpu::new();
    cpu.y_register = 0x20;
    cpu.mem_write(0x0, 0xCC);
    cpu.mem_write_u16(0x1, 0xCDCD);
    cpu.mem_write(0xCDCD, 0x10);

    cpu.step();

    assert!(cpu.get_carry_flag());
    assert!(!cpu.get_zero_flag());
    assert!(!cpu.get_negative_flag());
}

#[test]
fn test_cpy_zero_when_equal() {
    let mut cpu = Cpu::new();
    cpu.y_register = 0x20;
    cpu.mem_write(0x0, 0xC0);
    cpu.mem_write(0x1, 0x20);

    cpu.step();

    assert!(cpu.get_carry_flag());
    assert!(cpu.get_zero_flag());
    assert!(!cpu.get_negative_flag());
}

#[test]
fn test_cpy_negative_when_negative() {
    let mut cpu = Cpu::new();
    cpu.y_register = 0x20;
    cpu.mem_write(0x0, 0xC0);
    cpu.mem_write(0x1, 0x30);

    cpu.step();

    assert!(!cpu.get_carry_flag());
    assert!(!cpu.get_zero_flag());
    assert!(cpu.get_negative_flag());
}
