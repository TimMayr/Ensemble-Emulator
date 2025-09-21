use crate::emulation::cpu::Cpu;

#[test]
fn test_sec() {
    let mut cpu = Cpu::test_instance();
    cpu.processor_status &= !0b0000_0001;
    cpu.mem_write(0x0, 0x38);
    cpu.step(0);
    assert!(cpu.get_carry_flag());
}
