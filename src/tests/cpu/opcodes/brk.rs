use crate::emulation::cpu::Cpu;

#[test]
fn test_brk_storing() {
    let mut cpu = Cpu::test_instance();
    cpu.processor_status |= 0b00000001;
    cpu.mem_write_u16(0xFFFE, 0xFF00);

    cpu.step();

    assert_eq!(cpu.mem_read_u16(0x01FE), 0x2u16);
    assert_eq!(cpu.mem_read(0x01FD), 0b00110001);
}

#[test]
fn test_brk_interrupt_vector() {
    let mut cpu = Cpu::test_instance();
    cpu.processor_status |= 0b00000001;
    cpu.mem_write_u16(0xFFFE, 0xFF00);
    cpu.mem_write(0xFF00, 0xA9);
    cpu.mem_write(0xFF01, 0x66);

    cpu.step();

    assert_eq!(cpu.mem_read_u16(0x01FE), 0x2u16);
    assert_eq!(cpu.mem_read(0x01FD), 0b00110001);

    cpu.step();

    assert_eq!(cpu.accumulator, 0x66)
}
