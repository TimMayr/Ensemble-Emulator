use crate::cpu::Cpu;

#[test]
fn test_clv() {
    let mut cpu = Cpu::new();
    cpu.processor_status |= 0b0100_0000;
    cpu.mem_write(0x0, 0xB8);
    cpu.step();
    assert!(!cpu.get_overflow_flag());
}
