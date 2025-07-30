use crate::cpu::Cpu;

#[test]
fn test_nop() {
    let mut cpu = Cpu::new();
    cpu.processor_status |= 0b00000001;
    cpu.accumulator = 0x66;

    cpu.mem_write(0x0, 0xEA);
    cpu.step();

    assert_eq!(cpu.accumulator, 0x66);
    assert_eq!(cpu.processor_status, 0b00000001);
}
