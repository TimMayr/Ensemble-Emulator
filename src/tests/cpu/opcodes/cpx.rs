use crate::emulation::cpu::Cpu;

#[test]
fn test_cpx_immediate() {
    let mut cpu = Cpu::test_instance();
    cpu.x_register = 0x20;
    cpu.mem_write(0x0, 0xE0);
    cpu.mem_write(0x1, 0x10);

    cpu.step(0);
    cpu.step(0);

    assert!(cpu.get_carry_flag());
    assert!(!cpu.get_zero_flag());
    assert!(!cpu.get_negative_flag());
}

#[test]
fn test_cpx_zero_page() {
    let mut cpu = Cpu::test_instance();
    cpu.x_register = 0x20;
    cpu.mem_write(0x0, 0xE4);
    cpu.mem_write(0x1, 0xC5);
    cpu.mem_write(0xC5, 0x10);

    cpu.step(0);

    assert!(cpu.get_carry_flag());
    assert!(!cpu.get_zero_flag());
    assert!(!cpu.get_negative_flag());
}

#[test]
fn test_cpx_absolute() {
    let mut cpu = Cpu::test_instance();
    cpu.x_register = 0x20;
    cpu.mem_write(0x0, 0xEC);
    cpu.mem_write_u16(0x1, 0xCDCD);
    cpu.mem_write(0xCDCD, 0x10);

    cpu.step(0);
    cpu.step(0);
    cpu.step(0);
    cpu.step(0);

    assert!(cpu.get_carry_flag());
    assert!(!cpu.get_zero_flag());
    assert!(!cpu.get_negative_flag());
}

#[test]
fn test_cpx_zero_when_equal() {
    let mut cpu = Cpu::test_instance();
    cpu.x_register = 0x20;
    cpu.mem_write(0x0, 0xE0);
    cpu.mem_write(0x1, 0x20);

    cpu.step(0);
    cpu.step(0);

    assert!(cpu.get_carry_flag());
    assert!(cpu.get_zero_flag());
    assert!(!cpu.get_negative_flag());
}

#[test]
fn test_cpx_negative_when_negative() {
    let mut cpu = Cpu::test_instance();
    cpu.x_register = 0x20;
    cpu.mem_write(0x0, 0xE0);
    cpu.mem_write(0x1, 0x30);

    cpu.step(0);
    cpu.step(0);

    assert!(!cpu.get_carry_flag());
    assert!(!cpu.get_zero_flag());
    assert!(cpu.get_negative_flag());
}
