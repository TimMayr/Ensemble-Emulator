use crate::emulation::cpu::Cpu;

#[test]
fn test_jsr_absolute() {
    let mut cpu = Cpu::test_instance();
    cpu.mem_write(0x0, 0x20);
    cpu.mem_write_u16(0x1, 0x1234);

    cpu.step(0);
    cpu.step(0);
    cpu.step(0);
    cpu.step(0);
    cpu.step(0);
    cpu.step(0);
    assert_eq!(cpu.program_counter, 0x1234);
    assert_eq!(cpu.mem_read_u16(0x01FE), 0x2u16);

    cpu.mem_write(0x1234, 0xA9);
    cpu.mem_write(0x1235, 0x66);
    cpu.step(0);
    cpu.step(0);

    assert_eq!(cpu.accumulator, 0x66);
}
