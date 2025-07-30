use crate::cpu::Cpu;

#[test]
fn clc_clears_carry_flag() {
    let mut cpu = Cpu::new();
    cpu.processor_status |= 0b0000_0001;
    cpu.mem_write(0x0, 0x18);
    cpu.step();
    assert!(!cpu.get_carry_flag());
}

#[test]
fn cld_clears_decimal_flag() {
    let mut cpu = Cpu::new();
    cpu.processor_status |= 0b0000_1000;
    cpu.mem_write(0x0, 0xD8);
    cpu.step();
    assert_eq!(cpu.processor_status & 0b0000_1000, 0);
}

#[test]
fn cli_clears_interrupt_disable_flag() {
    let mut cpu = Cpu::new();
    cpu.processor_status |= 0b0000_0100;
    cpu.mem_write(0x0, 0x58);
    cpu.step();
    assert_eq!(cpu.processor_status & 0b0000_0100, 0);
}

#[test]
fn clv_clears_overflow_flag() {
    let mut cpu = Cpu::new();
    cpu.processor_status |= 0b0100_0000;
    cpu.mem_write(0x0, 0xB8);
    cpu.step();
    assert!(!cpu.get_overflow_flag());
}

#[test]
fn sec_sets_carry_flag() {
    let mut cpu = Cpu::new();
    cpu.processor_status &= !0b0000_0001;
    cpu.mem_write(0x0, 0x38);
    cpu.step();
    assert!(cpu.get_carry_flag());
}

#[test]
fn sed_sets_decimal_flag() {
    let mut cpu = Cpu::new();
    cpu.processor_status &= !0b0000_1000;
    cpu.mem_write(0x0, 0xF8);
    cpu.step();
    assert_eq!(cpu.processor_status & 0b0000_1000, 0b0000_1000);
}

#[test]
fn sei_sets_interrupt_disable_flag() {
    let mut cpu = Cpu::new();
    cpu.processor_status &= !0b0000_0100;
    cpu.mem_write(0x0, 0x78);
    cpu.step();
    assert_eq!(cpu.processor_status & 0b0000_0100, 0b0000_0100);
}
