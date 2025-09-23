use crate::emulation::cpu::Cpu;

#[test]
fn test_sta_complete() {
    let mut cpu = Cpu::test_instance();
    cpu.accumulator = 0x66;

    cpu.mem_write(0x0, 0x85);
    cpu.mem_write(0x1, 0x0085);

    cpu.step(0);
    cpu.step(0);
    cpu.step(0);
    assert_eq!(cpu.mem_read(0x0085), 0x66);

    cpu.mem_write(0x2, 0x95);
    cpu.mem_write(0x3, 0x0095);
    cpu.x_register = 0x0010;

    cpu.step(0);
    cpu.step(0);
    cpu.step(0);
    cpu.step(0);
    assert_eq!(cpu.mem_read(0x00A5), 0x66);

    cpu.mem_write(0x4, 0x8D);
    cpu.mem_write_u16(0x5, 0x8D8D);

    cpu.step(0);
    cpu.step(0);
    cpu.step(0);
    cpu.step(0);
    assert_eq!(cpu.mem_read(0x8D8D), 0x66);

    cpu.mem_write(0x7, 0x9D);
    cpu.mem_write_u16(0x8, 0x9D9D);
    cpu.x_register = 0x0010;

    cpu.step(0);
    assert_eq!(cpu.mem_read(0x9DAD), 0x66);

    cpu.mem_write(0xA, 0x99);
    cpu.mem_write_u16(0xB, 0x9999);
    cpu.y_register = 0x0010;

    cpu.step(0);
    assert_eq!(cpu.mem_read(0x99A9), 0x66);

    cpu.mem_write(0xD, 0x81);
    cpu.mem_write(0xE, 0x0081);
    cpu.x_register = 0x0010;
    cpu.mem_write_u16(0x91, 0x8181);

    cpu.step(0);
    assert_eq!(cpu.mem_read(0x8181), 0x66);

    cpu.mem_write(0xF, 0x91);
    cpu.mem_write(0x10, 0x0091);
    cpu.mem_write_u16(0x0091, 0x9191);
    cpu.y_register = 0x0010;

    cpu.step(0);
    assert_eq!(cpu.mem_read(0x91A1), 0x66);
}

#[test]
fn test_sta_zero_page() {
    let mut cpu = Cpu::test_instance();
    cpu.accumulator = 0x66;

    cpu.mem_write(0x0, 0x85);
    cpu.mem_write(0x1, 0x0085);

    cpu.step(0);
    cpu.step(0);
    cpu.step(0);
    assert_eq!(cpu.mem_read(0x0085), 0x66);
}

#[test]
fn test_sta_zero_page_x() {
    let mut cpu = Cpu::test_instance();
    cpu.accumulator = 0x66;

    cpu.mem_write(0x0, 0x95);
    cpu.mem_write(0x1, 0x0095);
    cpu.x_register = 0x0010;

    cpu.step(0);
    cpu.step(0);
    cpu.step(0);
    cpu.step(0);
    assert_eq!(cpu.mem_read(0x00A5), 0x66);
}

#[test]
fn test_sta_absolute() {
    let mut cpu = Cpu::test_instance();
    cpu.accumulator = 0x66;

    cpu.mem_write(0x0, 0x8D);
    cpu.mem_write_u16(0x1, 0x8D8D);

    cpu.step(0);
    cpu.step(0);
    cpu.step(0);
    cpu.step(0);
    assert_eq!(cpu.mem_read(0x8D8D), 0x66);
}

#[test]
fn test_sta_absolute_x() {
    let mut cpu = Cpu::test_instance();
    cpu.accumulator = 0x66;

    cpu.mem_write(0x0, 0x9D);
    cpu.mem_write_u16(0x1, 0x9D9D);
    cpu.x_register = 0x0010;

    cpu.step(0);
    assert_eq!(cpu.mem_read(0x9DAD), 0x66);
}

#[test]
fn test_sta_absolute_y() {
    let mut cpu = Cpu::test_instance();
    cpu.accumulator = 0x66;

    cpu.mem_write(0x0, 0x99);
    cpu.mem_write_u16(0x1, 0x9999);
    cpu.y_register = 0x0010;

    cpu.step(0);
    assert_eq!(cpu.mem_read(0x99A9), 0x66);
}

#[test]
fn test_sta_indirect_x() {
    let mut cpu = Cpu::test_instance();
    cpu.accumulator = 0x66;

    cpu.mem_write(0x0, 0x81);
    cpu.mem_write(0x1, 0x0081);
    cpu.x_register = 0x0010;
    cpu.mem_write_u16(0x91, 0x8181);

    cpu.step(0);
    assert_eq!(cpu.mem_read(0x8181), 0x66);
}

#[test]
fn test_sta_indirect_y() {
    let mut cpu = Cpu::test_instance();
    cpu.accumulator = 0x66;

    cpu.mem_write(0x0, 0x91);
    cpu.mem_write(0x1, 0x0091);
    cpu.mem_write_u16(0x0091, 0x9191);
    cpu.y_register = 0x0010;

    cpu.step(0);
    assert_eq!(cpu.mem_read(0x91A1), 0x66);
}
