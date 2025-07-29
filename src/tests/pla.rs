#[cfg(test)]
mod pla {
    use crate::cpu::Cpu;

    #[test]
    fn test_pla_implied_simple() {
        let mut cpu = Cpu::new();
        //Init acc
        cpu.accumulator = 0x66;

        //Write acc to Stack
        cpu.mem_write(0x0, 0x48);

        cpu.step();

        //Reset acc
        cpu.accumulator = 0x00;

        //Load acc from stack
        cpu.mem_write(0x1, 0x68);

        cpu.step();
        //Validate that acc has been loaded correctly
        assert_eq!(cpu.accumulator, 0x66);
        //Validate that stack pointer is correct
        assert_eq!(cpu.stack_pointer, 0xFF)
    }

    #[test]
    fn test_pla_implied_complicated() {
        let mut cpu = Cpu::new();
        //Init acc
        cpu.accumulator = 0x0;

        //Write acc to stack
        cpu.mem_write(0x0, 0x48);
        cpu.step();

        cpu.accumulator = 0x80;

        //Write acc to stack
        cpu.mem_write(0x1, 0x48);
        cpu.step();

        cpu.accumulator = 0x70;

        //Write acc to stack
        cpu.mem_write(0x2, 0x48);
        cpu.step();

        //Load acc from stack
        cpu.mem_write(0x3, 0x68);
        cpu.step();

        //Validate that correct value was loaded
        assert_eq!(cpu.accumulator, 0x70);
        //Validate that correct flags have been set
        assert_eq!(cpu.get_zero_flag(), false);
        assert_eq!(cpu.get_negative_flag(), false);

        //Load acc from stack
        cpu.mem_write(0x4, 0x68);
        cpu.step();

        //Validate that correct value was loaded
        assert_eq!(cpu.accumulator, 0x80);
        //Validate that correct flags have been set
        assert_eq!(cpu.get_zero_flag(), false);
        assert_eq!(cpu.get_negative_flag(), true);

        //Load acc from stack
        cpu.mem_write(0x5, 0x68);
        cpu.step();

        //Validate that correct value was loaded
        assert_eq!(cpu.accumulator, 0x00);
        //Validate that correct flags have been set
        assert_eq!(cpu.get_zero_flag(), true);
        assert_eq!(cpu.get_negative_flag(), false);
    }
}
