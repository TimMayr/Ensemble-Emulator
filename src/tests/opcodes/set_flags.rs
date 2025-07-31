use crate::cpu::Cpu;

#[test]
fn test_clc() {
    let mut cpu = Cpu::new();
    cpu.processor_status |= 0b0000_0001;
    cpu.mem_write(0x0, 0x18);
    cpu.step();
    assert!(!cpu.get_carry_flag());
}

#[test]
fn test_cld() {
    let mut cpu = Cpu::new();
    cpu.processor_status |= 0b0000_1000;
    cpu.mem_write(0x0, 0xD8);
    cpu.step();
    assert_eq!(cpu.processor_status & 0b0000_1000, 0);
}

#[test]
fn test_cli() {
    let mut cpu = Cpu::new();
    cpu.processor_status |= 0b0000_0100;
    cpu.mem_write(0x0, 0x58);
    cpu.step();
    assert_eq!(cpu.processor_status & 0b0000_0100, 0);
}

#[test]
fn test_clv() {
    let mut cpu = Cpu::new();
    cpu.processor_status |= 0b0100_0000;
    cpu.mem_write(0x0, 0xB8);
    cpu.step();
    assert!(!cpu.get_overflow_flag());
}

#[test]
fn test_sec() {
    let mut cpu = Cpu::new();
    cpu.processor_status &= !0b0000_0001;
    cpu.mem_write(0x0, 0x38);
    cpu.step();
    assert!(cpu.get_carry_flag());
}

#[test]
fn test_sed() {
    let mut cpu = Cpu::new();
    cpu.processor_status &= !0b0000_1000;
    cpu.mem_write(0x0, 0xF8);
    cpu.step();
    assert_eq!(cpu.processor_status & 0b0000_1000, 0b0000_1000);
}

#[test]
fn test_sei() {
    let mut cpu = Cpu::new();
    cpu.processor_status &= !0b0000_0100;
    cpu.mem_write(0x0, 0x78);
    cpu.step();
    assert_eq!(cpu.processor_status & 0b0000_0100, 0b0000_0100);
}
