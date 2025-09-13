use crate::cpu::Cpu;

#[test]
fn test_rts() {
    let mut cpu = Cpu::test_instance();
    cpu.mem_write(0x0, 0x20);
    cpu.mem_write_u16(0x1, 0x1234);
    cpu.mem_write(0x3, 0xA9);
    cpu.mem_write(0x4, 0x55);

    cpu.step();
    assert_eq!(cpu.program_counter, 0x1234);
    assert_eq!(cpu.mem_read_u16(0x01FE), 0x2u16.swap_bytes());

    cpu.mem_write(0x1234, 0xA9);
    cpu.mem_write(0x1235, 0x66);
    cpu.step();

    assert_eq!(cpu.accumulator, 0x66);

    cpu.mem_write(0x1236, 0x60);
    cpu.step();
    cpu.step();

    assert_eq!(cpu.accumulator, 0x55);
}
