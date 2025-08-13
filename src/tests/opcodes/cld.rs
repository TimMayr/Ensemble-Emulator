use crate::cpu::Cpu;

#[test]
fn test_cld() {
    let mut cpu = Cpu::test_instance();
    cpu.processor_status |= 0b0000_1000;
    cpu.mem_write(0x0, 0xD8);
    cpu.step();
    assert_eq!(cpu.processor_status & 0b0000_1000, 0);
}
