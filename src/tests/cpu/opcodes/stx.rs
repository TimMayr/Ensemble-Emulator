use crate::emulation::cpu::Cpu;

#[test]
fn test_stx_complete() {
    let mut cpu = Cpu::test_instance();
    cpu.x_register = 0x66;

    cpu.mem_write(0x0, 0x86);
    cpu.mem_write(0x1, 0x0086);

    cpu.step(0);
    cpu.step(0);
    cpu.step(0);
    assert_eq!(cpu.mem_read(0x0086), 0x66);

    cpu.mem_write(0x2, 0x96);
    cpu.mem_write(0x3, 0x0096);
    cpu.y_register = 0x0010;

    cpu.step(0);
    cpu.step(0);
    cpu.step(0);
    cpu.step(0);
    assert_eq!(cpu.mem_read(0x00A6), 0x66);

    cpu.mem_write(0x4, 0x8E);
    cpu.mem_write_u16(0x5, 0x8E8E);

    cpu.step(0);
    cpu.step(0);
    cpu.step(0);
    cpu.step(0);
    assert_eq!(cpu.mem_read(0x8E8E), 0x66);
}

#[test]
fn test_stx_zero_page() {
    let mut cpu = Cpu::test_instance();
    cpu.x_register = 0x66;

    cpu.mem_write(0x0, 0x86);
    cpu.mem_write(0x1, 0x0086);

    cpu.step(0);
    cpu.step(0);
    cpu.step(0);
    assert_eq!(cpu.mem_read(0x0086), 0x66);
}

#[test]
fn test_stx_zero_page_y() {
    let mut cpu = Cpu::test_instance();
    cpu.x_register = 0x66;

    cpu.mem_write(0x0, 0x96);
    cpu.mem_write(0x1, 0x0096);
    cpu.y_register = 0x0010;

    cpu.step(0);
    cpu.step(0);
    cpu.step(0);
    cpu.step(0);
    assert_eq!(cpu.mem_read(0x00A6), 0x66);
}

#[test]
fn test_stx_absolute() {
    let mut cpu = Cpu::test_instance();
    cpu.x_register = 0x66;

    cpu.mem_write(0x0, 0x8E);
    cpu.mem_write_u16(0x1, 0x8E8E);

    cpu.step(0);
    cpu.step(0);
    cpu.step(0);
    cpu.step(0);
    assert_eq!(cpu.mem_read(0x8E8E), 0x66);
}
