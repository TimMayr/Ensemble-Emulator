use crate::emulation::cpu::Cpu;

#[test]
fn test_rti() {
    let mut cpu = Cpu::test_instance();
    cpu.processor_status |= 0b00000001;
    cpu.mem_write_u16(0xFFFE, 0xFF00);
    cpu.mem_write(0xFF00, 0xA9);
    cpu.mem_write(0xFF01, 0x66);
    cpu.mem_write(0xFF02, 0xF8);
    cpu.mem_write(0xFF03, 0x40);
    cpu.mem_write(0x2, 0xA2);
    cpu.mem_write(0x3, 0x55);
    cpu.mem_write(0x4, 0x78);

    cpu.step(0);
    cpu.step(0);
    cpu.step(0);
    cpu.step(0);
    cpu.step(0);
    cpu.step(0);

    assert_eq!(cpu.accumulator, 0x66);
    assert_eq!(cpu.x_register, 0x55);
    assert_eq!(cpu.processor_status, 0b00000101);
}
