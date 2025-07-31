use crate::cpu::Cpu;

#[test]
fn beq_fail() {
    let mut cpu = Cpu::new();
    cpu.mem_write(0x0, 0xEA);
    cpu.mem_write(0x1, 0xEA);
    cpu.mem_write(0x2, 0xEA);
    cpu.mem_write(0x3, 0xF0);
    cpu.mem_write(0x4, 0x12);

    cpu.step();
    cpu.step();
    cpu.step();
    cpu.step();

    assert_eq!(cpu.program_counter, 0x5);
}

#[test]
fn beq_success() {
    let mut cpu = Cpu::new();
    cpu.processor_status |= 0b00000010;
    cpu.mem_write(0x0, 0xEA);
    cpu.mem_write(0x1, 0xEA);
    cpu.mem_write(0x2, 0xEA);
    cpu.mem_write(0x3, 0xF0);
    cpu.mem_write(0x4, 0x12);

    cpu.step();
    cpu.step();
    cpu.step();
    cpu.step();

    assert_eq!(cpu.program_counter, 0x16);

    cpu.mem_write(0x16, 0xA9);
    cpu.mem_write(0x17, 0x66);

    cpu.step();

    assert_eq!(cpu.accumulator, 0x66);
}
