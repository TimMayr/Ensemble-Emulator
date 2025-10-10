use crate::emulation::cpu::Cpu;

#[test]
fn test_jmp_complete() {
    let mut cpu = Cpu::test_instance();
    cpu.mem_write(0x0, 0x4C);
    cpu.mem_write_u16(0x1, 0x1234);

    cpu.step();
    cpu.step();
    cpu.step();
    assert_eq!(cpu.program_counter, 0x1234);

    cpu.mem_write(0x1234, 0x6C);
    cpu.mem_write_u16(0x1235, 0x8345);
    cpu.mem_write_u16(0x8345, 0x9456);

    cpu.step();
    cpu.step();
    cpu.step();
    cpu.step();
    cpu.step();

    assert_eq!(cpu.program_counter, 0x9456);

    cpu.mem_write(0x9456, 0xA9);
    cpu.mem_write(0x9457, 0x66);

    cpu.step();
    cpu.step();

    assert_eq!(cpu.accumulator, 0x66);
}

#[test]
fn test_jmp_absolute() {
    let mut cpu = Cpu::test_instance();
    cpu.mem_write(0x0, 0x4C);
    cpu.mem_write_u16(0x1, 0x1234);

    cpu.step();
    cpu.step();
    cpu.step();
    assert_eq!(cpu.program_counter, 0x1234);

    cpu.mem_write(0x1234, 0xA9);
    cpu.mem_write(0x1235, 0x66);
    cpu.step();
    cpu.step();

    assert_eq!(cpu.accumulator, 0x66);
}

#[test]
fn test_jmp_indirect() {
    let mut cpu = Cpu::test_instance();
    cpu.mem_write(0x0, 0x6C);
    cpu.mem_write_u16(0x1, 0x1234);
    cpu.mem_write_u16(0x1234, 0x8345);

    cpu.step();
    cpu.step();
    cpu.step();
    cpu.step();
    cpu.step();

    assert_eq!(cpu.program_counter, 0x8345);

    cpu.mem_write(0x8345, 0xA9);
    cpu.mem_write(0x8346, 0x66);

    cpu.step();
    cpu.step();

    assert_eq!(cpu.accumulator, 0x66);
}
