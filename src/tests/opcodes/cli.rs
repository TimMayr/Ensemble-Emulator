use crate::cpu::Cpu;

#[test]
fn test_cli() {
    let mut cpu = Cpu::test_instance();
    cpu.processor_status |= 0b0000_0100;
    cpu.mem_write(0x0, 0x58);
    cpu.step();
    assert_eq!(cpu.processor_status & 0b0000_0100, 0);
}
