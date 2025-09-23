use crate::emulation::cpu::Cpu;

#[test]
fn test_sty_complete() {
    let mut cpu = Cpu::test_instance();
    cpu.y_register = 0x66;

    cpu.mem_write(0x0, 0x84);
    cpu.mem_write(0x1, 0x0084);

    cpu.step(0);
    cpu.step(0);
    cpu.step(0);
    assert_eq!(cpu.mem_read(0x0084), 0x66);

    cpu.mem_write(0x2, 0x94);
    cpu.mem_write(0x3, 0x0094);
    cpu.x_register = 0x0010;

    cpu.step(0);
    cpu.step(0);
    cpu.step(0);
    cpu.step(0);
    assert_eq!(cpu.mem_read(0x00A4), 0x66);

    cpu.mem_write(0x4, 0x8C);
    cpu.mem_write_u16(0x5, 0x8C8C);

    cpu.step(0);
    cpu.step(0);
    cpu.step(0);
    cpu.step(0);
    assert_eq!(cpu.mem_read(0x8C8C), 0x66);
}

#[test]
fn test_sty_zero_page() {
    let mut cpu = Cpu::test_instance();
    cpu.y_register = 0x66;

    cpu.mem_write(0x0, 0x84);
    cpu.mem_write(0x1, 0x0084);

    cpu.step(0);
    cpu.step(0);
    cpu.step(0);
    assert_eq!(cpu.mem_read(0x0084), 0x66);
}

#[test]
fn test_sty_zero_page_y() {
    let mut cpu = Cpu::test_instance();
    cpu.y_register = 0x66;

    cpu.mem_write(0x0, 0x94);
    cpu.mem_write(0x1, 0x0094);
    cpu.x_register = 0x0010;

    cpu.step(0);
    cpu.step(0);
    cpu.step(0);
    cpu.step(0);
    assert_eq!(cpu.mem_read(0x00A4), 0x66);
}

#[test]
fn test_sty_absolute() {
    let mut cpu = Cpu::test_instance();
    cpu.y_register = 0x66;

    cpu.mem_write(0x0, 0x8C);
    cpu.mem_write_u16(0x1, 0x8C8C);

    cpu.step(0);
    cpu.step(0);
    cpu.step(0);
    cpu.step(0);
    assert_eq!(cpu.mem_read(0x8C8C), 0x66);
}
