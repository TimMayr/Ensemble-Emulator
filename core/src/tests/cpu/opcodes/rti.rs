use crate::emulation::cpu::Cpu;

#[test]
fn test_rti() {
    let mut cpu = Cpu::test_instance();
    cpu.processor_status |= 0b00000001;
    cpu.stack_push(0x10);
    cpu.stack_push(0x80);
    cpu.stack_push(cpu.processor_status);

    cpu.mem_write(0x0, 0x40);

    cpu.step(0);
    cpu.step(0);
    cpu.step(0);
    cpu.step(0);
    cpu.step(0);
    cpu.step(0);

    assert_eq!(cpu.program_counter, 0x1080)
}
