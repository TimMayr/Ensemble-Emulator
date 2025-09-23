use crate::emulation::cpu::Cpu;

#[test]
fn test_sei() {
    let mut cpu = Cpu::test_instance();
    cpu.processor_status &= !0b0000_0100;
    cpu.mem_write(0x0, 0x78);
    cpu.step(0);
    cpu.step(0);
    assert_eq!(cpu.processor_status & 0b0000_0100, 0b0000_0100);
}
