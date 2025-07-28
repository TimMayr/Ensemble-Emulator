#[cfg(test)]
mod ldx {
    use crate::cpu::Cpu;

    #[test]
    fn test_ldx_complete() {
        let mut cpu = Cpu::new();
        cpu.mem_write(0x0, 0xA2);
        cpu.mem_write(0x1, 0x66);

        cpu.step();
        assert_eq!(cpu.x_register, 0x66);

        cpu.mem_write(0x2, 0xA6);
        cpu.mem_write(0x3, 0x00A6);
        cpu.mem_write(0x00A6, 0x66);

        cpu.step();
        assert_eq!(cpu.x_register, 0x66);

        cpu.mem_write(0x4, 0xB6);
        cpu.mem_write(0x5, 0x00B6);
        cpu.y_register = 0x0010;
        cpu.mem_write(0x00C6, 0x66);

        cpu.step();
        assert_eq!(cpu.x_register, 0x66);

        cpu.mem_write(0x6, 0xAE);
        cpu.mem_write_u16(0x7, 0xAEAE);
        cpu.mem_write(0xAEAE, 0x66);

        cpu.step();
        assert_eq!(cpu.x_register, 0x66);

        cpu.mem_write(0x9, 0xBE);
        cpu.mem_write_u16(0xA, 0xBEBE);
        cpu.y_register = 0x0010;
        cpu.mem_write(0xBECE, 0x66);

        cpu.step();
        assert_eq!(cpu.x_register, 0x66);
    }

    #[test]
    fn test_ldx_immediate() {
        let mut cpu = Cpu::new();
        cpu.mem_write(0x0, 0xA2);
        cpu.mem_write(0x1, 0x66);

        cpu.step();
        assert_eq!(cpu.x_register, 0x66);
    }

    #[test]
    fn test_ldx_zero_page() {
        let mut cpu = Cpu::new();
        cpu.mem_write(0x0, 0xA6);
        cpu.mem_write(0x1, 0x00A6);
        cpu.mem_write(0x00A6, 0x66);

        cpu.step();
        assert_eq!(cpu.x_register, 0x66);
    }

    #[test]
    fn test_ldx_zero_page_y() {
        let mut cpu = Cpu::new();
        cpu.mem_write(0x0, 0xB6);
        cpu.mem_write(0x1, 0x00B6);
        cpu.y_register = 0x0010;
        cpu.mem_write(0x00C6, 0x66);

        cpu.step();
        assert_eq!(cpu.x_register, 0x66);
    }

    #[test]
    fn test_ldx_absolute() {
        let mut cpu = Cpu::new();
        cpu.mem_write(0x0, 0xAE);
        cpu.mem_write_u16(0x1, 0xAEAE);
        cpu.mem_write(0xAEAE, 0x66);

        cpu.step();
        assert_eq!(cpu.x_register, 0x66);
    }

    #[test]
    fn test_ldx_absolute_y() {
        let mut cpu = Cpu::new();
        cpu.mem_write(0x0, 0xBE);
        cpu.mem_write_u16(0x1, 0xBEBE);
        cpu.y_register = 0x0010;
        cpu.mem_write(0xBECE, 0x66);

        cpu.step();
        assert_eq!(cpu.x_register, 0x66);
    }
}
