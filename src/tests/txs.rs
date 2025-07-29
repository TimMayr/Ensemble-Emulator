#[cfg(test)]
mod txs {
    use crate::cpu::Cpu;

    #[test]
    fn test_txs_simple() {
        let mut cpu = Cpu::new();
        cpu.x_register = 0x66;

        cpu.mem_write(0x0, 0x9A);
        cpu.step();

        assert_eq!(0x66, cpu.stack_pointer);
    }
}
