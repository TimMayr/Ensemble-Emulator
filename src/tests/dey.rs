use crate::cpu::Cpu;

#[test]
fn test_dey_complete() {
    let mut cpu = Cpu::new();
    cpu.mem_write(0x0, 0xE8);

    cpu.step();

    assert_eq!(cpu.x_register, 0x1);
    assert!(!cpu.get_zero_flag());
    assert!(!cpu.get_negative_flag());

    cpu.mem_write(0x1, 0xE8);
    cpu.x_register = 0xFF;

    cpu.step();

    assert_eq!(cpu.x_register, 0x0);
    assert!(cpu.get_zero_flag());
    assert!(!cpu.get_negative_flag());

    cpu.mem_write(0x2, 0xE8);
    cpu.x_register = 0x7F;

    cpu.step();

    assert_eq!(cpu.x_register, 0x80);
    assert!(!cpu.get_zero_flag());
    assert!(cpu.get_negative_flag());
}

#[test]
fn test_dey_zero_page() {
    let mut cpu = Cpu::new();
    cpu.mem_write(0x0, 0xE8);

    cpu.step();

    assert_eq!(cpu.x_register, 0x1)
}

#[test]
fn test_dey_flags_none_when_none() {
    let mut cpu = Cpu::new();
    cpu.mem_write(0x0, 0xE8);
    cpu.step();

    assert_eq!(cpu.x_register, 0x1);
    assert!(!cpu.get_zero_flag());
    assert!(!cpu.get_negative_flag());
}

#[test]
fn test_dey_flags_zero_when_zero() {
    let mut cpu = Cpu::new();
    cpu.mem_write(0x0, 0xE8);
    cpu.x_register = 0xFF;

    cpu.step();

    assert_eq!(cpu.x_register, 0x0);
    assert!(cpu.get_zero_flag());
    assert!(!cpu.get_negative_flag());
}

#[test]
fn test_dey_flags_negative_when_negative() {
    let mut cpu = Cpu::new();
    cpu.mem_write(0x0, 0xE8);
    cpu.x_register = 0x7F;

    cpu.step();

    assert_eq!(cpu.x_register, 0x80);
    assert!(!cpu.get_zero_flag());
    assert!(cpu.get_negative_flag());
}