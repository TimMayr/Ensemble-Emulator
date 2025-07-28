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
        cpu.mem_write(0x00A6, 0x67);

        cpu.step();
        assert_eq!(cpu.x_register, 0x67);

        cpu.mem_write(0x4, 0xB6);
        cpu.mem_write(0x5, 0x00B6);
        cpu.y_register = 0x0010;
        cpu.mem_write(0x00C6, 0x68);

        cpu.step();
        assert_eq!(cpu.x_register, 0x68);

        cpu.mem_write(0x6, 0xAE);
        cpu.mem_write_u16(0x7, 0xAEAE);
        cpu.mem_write(0xAEAE, 0x69);

        cpu.step();
        assert_eq!(cpu.x_register, 0x69);

        cpu.mem_write(0x9, 0xBE);
        cpu.mem_write_u16(0xA, 0xBEBE);
        cpu.y_register = 0x0010;
        cpu.mem_write(0xBECE, 0x6A);

        cpu.step();
        assert_eq!(cpu.x_register, 0x6A);

        cpu.mem_write(0xC, 0xA2);
        cpu.mem_write(0xD, 0x77);

        cpu.step();
        assert_eq!(cpu.x_register, 0x77);
        assert_eq!(cpu.get_negative_flag(), false);
        assert_eq!(cpu.get_zero_flag(), false);

        cpu.mem_write(0xE, 0xA2);
        cpu.mem_write(0xF, 0x0);

        cpu.step();
        assert_eq!(cpu.x_register, 0x0);
        assert_eq!(cpu.get_negative_flag(), false);
        assert_eq!(cpu.get_zero_flag(), true);

        cpu.mem_write(0x10, 0xA2);
        cpu.mem_write(0x11, 0x80);

        cpu.step();
        assert_eq!(cpu.x_register, 0x80);
        assert_eq!(cpu.get_negative_flag(), true);
        assert_eq!(cpu.get_zero_flag(), false);
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

    #[test]
    fn test_ldx_flags_none_when_none() {
        let mut cpu = Cpu::new();
        cpu.mem_write(0x0, 0xA2);
        cpu.mem_write(0x1, 0x66);

        cpu.step();
        assert_eq!(cpu.get_negative_flag(), false);
        assert_eq!(cpu.get_zero_flag(), false);
    }

    #[test]
    fn test_ldx_flags_only_zero_when_zero() {
        let mut cpu = Cpu::new();
        cpu.mem_write(0x0, 0xA2);
        cpu.mem_write(0x1, 0x0);

        cpu.step();
        assert_eq!(cpu.get_negative_flag(), false);
        assert_eq!(cpu.get_zero_flag(), true);
    }

    #[test]
    fn test_ldx_flags_only_negative_when_negative() {
        let mut cpu = Cpu::new();
        cpu.mem_write(0x0, 0xA2);
        cpu.mem_write(0x1, 0x80);

        cpu.step();
        assert_eq!(cpu.get_negative_flag(), true);
        assert_eq!(cpu.get_zero_flag(), false);
    }
}
