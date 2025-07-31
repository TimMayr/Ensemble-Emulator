use crate::cpu::Cpu;

#[test]
fn test_sec() {
    let mut cpu = Cpu::new();
    cpu.processor_status &= !0b0000_0001;
    cpu.mem_write(0x0, 0x38);
    cpu.step();
    assert!(cpu.get_carry_flag());
}
