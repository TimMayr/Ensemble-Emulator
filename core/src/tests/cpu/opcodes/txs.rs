use crate::emulation::cpu::Cpu;

#[test]
fn test_txs_simple() {
    let mut cpu = Cpu::test_instance();
    cpu.x_register = 0x66;

    cpu.mem_write(0x0, 0x9A);
    cpu.step(0);
    cpu.step(0);

    assert_eq!(0x66, cpu.stack_pointer);
}
