use crate::emulation::cpu::Cpu;

#[test]
fn test_lda_complete() {
    let mut cpu = Cpu::test_instance();
    cpu.mem_write(0x0, 0xA9);
    cpu.mem_write(0x1, 0x65);

    cpu.step();
    cpu.step();
    assert_eq!(cpu.accumulator, 0x65);

    cpu.mem_write(0x2, 0xA5);
    cpu.mem_write(0x3, 0x00A5);
    cpu.mem_write(0x00A5, 0x66);

    cpu.step();
    cpu.step();
    cpu.step();
    assert_eq!(cpu.accumulator, 0x66);

    cpu.mem_write(0x4, 0xB5);
    cpu.mem_write(0x5, 0x00B5);
    cpu.x_register = 0x0010;
    cpu.mem_write(0x00C5, 0x67);

    cpu.step();
    cpu.step();
    cpu.step();
    cpu.step();
    assert_eq!(cpu.accumulator, 0x67);

    cpu.mem_write(0x6, 0xAD);
    cpu.mem_write_u16(0x7, 0xADAD);
    cpu.mem_write(0xADAD, 0x68);

    cpu.step();
    cpu.step();
    cpu.step();
    cpu.step();
    assert_eq!(cpu.accumulator, 0x68);

    cpu.mem_write(0x9, 0xBD);
    cpu.mem_write_u16(0xA, 0xBDBD);
    cpu.x_register = 0x0010;
    cpu.mem_write(0xBDCD, 0x69);

    cpu.step();
    cpu.step();
    cpu.step();
    cpu.step();
    assert_eq!(cpu.accumulator, 0x69);

    cpu.mem_write(0xC, 0xB9);
    cpu.mem_write_u16(0xD, 0xB9B9);
    cpu.y_register = 0x0010;
    cpu.mem_write(0xB9C9, 0x6A);

    cpu.step();
    cpu.step();
    cpu.step();
    cpu.step();
    assert_eq!(cpu.accumulator, 0x6A);

    cpu.mem_write(0xF, 0xA1);
    cpu.mem_write(0x10, 0x00A1);
    cpu.x_register = 0x0010;
    cpu.mem_write_u16(0xB1, 0xA1A1);
    cpu.mem_write(0xA1A1, 0x6B);

    cpu.step();
    cpu.step();
    cpu.step();
    cpu.step();
    cpu.step();
    cpu.step();
    assert_eq!(cpu.accumulator, 0x6B);

    cpu.mem_write(0x11, 0xB1);
    cpu.mem_write(0x12, 0x00B1);
    cpu.mem_write_u16(0x00B1, 0xB1B1);
    cpu.y_register = 0x0010;
    cpu.mem_write(0xB1C1, 0x6C);

    cpu.step();
    cpu.step();
    cpu.step();
    cpu.step();
    cpu.step();
    assert_eq!(cpu.accumulator, 0x6C);

    cpu.mem_write(0x13, 0xA9);
    cpu.mem_write(0x14, 0x6D);

    cpu.step();
    cpu.step();
    assert_eq!(cpu.accumulator, 0x6D);
    assert!(!cpu.get_negative_flag());
    assert!(!cpu.get_zero_flag());

    cpu.mem_write(0x15, 0xA9);
    cpu.mem_write(0x16, 0x0);

    cpu.step();
    cpu.step();
    assert_eq!(cpu.accumulator, 0x0);
    assert!(!cpu.get_negative_flag());
    assert!(cpu.get_zero_flag());

    cpu.mem_write(0x17, 0xA9);
    cpu.mem_write(0x18, 0x80);

    cpu.step();
    cpu.step();
    assert_eq!(cpu.accumulator, 0x80);
    assert!(cpu.get_negative_flag());
    assert!(!cpu.get_zero_flag());
}

#[test]
fn test_lda_immediate() {
    let mut cpu = Cpu::test_instance();
    cpu.mem_write(0x0, 0xA9);
    cpu.mem_write(0x1, 0x66);

    cpu.step();
    cpu.step();
    assert_eq!(cpu.accumulator, 0x66);
}

#[test]
fn test_lda_zero_page() {
    let mut cpu = Cpu::test_instance();
    cpu.mem_write(0x0, 0xA5);
    cpu.mem_write(0x1, 0x00A5);
    cpu.mem_write(0x00A5, 0x66);

    cpu.step();
    cpu.step();
    cpu.step();
    assert_eq!(cpu.accumulator, 0x66);
}

#[test]
fn test_lda_zero_page_x() {
    let mut cpu = Cpu::test_instance();
    cpu.mem_write(0x0, 0xB5);
    cpu.mem_write(0x1, 0x00B5);
    cpu.x_register = 0x0010;
    cpu.mem_write(0x00C5, 0x66);

    cpu.step();
    cpu.step();
    cpu.step();
    cpu.step();
    assert_eq!(cpu.accumulator, 0x66);
}

#[test]
fn test_lda_absolute() {
    let mut cpu = Cpu::test_instance();
    cpu.mem_write(0x0, 0xAD);
    cpu.mem_write_u16(0x1, 0xADAD);
    cpu.mem_write(0xADAD, 0x66);

    cpu.step();
    cpu.step();
    cpu.step();
    cpu.step();
    assert_eq!(cpu.accumulator, 0x66);
}

#[test]
fn test_lda_absolute_x() {
    let mut cpu = Cpu::test_instance();
    cpu.mem_write(0x0, 0xBD);
    cpu.mem_write_u16(0x1, 0xBDBD);
    cpu.x_register = 0x0010;
    cpu.mem_write(0xBDCD, 0x66);

    cpu.step();
    cpu.step();
    cpu.step();
    cpu.step();
    assert_eq!(cpu.accumulator, 0x66);
}

#[test]
fn test_lda_absolute_y() {
    let mut cpu = Cpu::test_instance();
    cpu.mem_write(0x0, 0xB9);
    cpu.mem_write_u16(0x1, 0xB9B9);
    cpu.y_register = 0x0010;
    cpu.mem_write(0xB9C9, 0x66);

    cpu.step();
    cpu.step();
    cpu.step();
    cpu.step();
    assert_eq!(cpu.accumulator, 0x66);
}

#[test]
fn test_lda_indirect_x() {
    let mut cpu = Cpu::test_instance();
    cpu.mem_write(0x0, 0xA1);
    cpu.mem_write(0x1, 0x00A1);
    cpu.x_register = 0x0010;
    cpu.mem_write_u16(0xB1, 0xA1A1);
    cpu.mem_write(0xA1A1, 0x66);

    cpu.step();
    cpu.step();
    cpu.step();
    cpu.step();
    cpu.step();
    cpu.step();
    assert_eq!(cpu.accumulator, 0x66);
}

#[test]
fn test_lda_indirect_y() {
    let mut cpu = Cpu::test_instance();
    cpu.mem_write(0x0, 0xB1);
    cpu.mem_write(0x1, 0x00B1);
    cpu.mem_write_u16(0x00B1, 0xB1B1);
    cpu.y_register = 0x0010;
    cpu.mem_write(0xB1C1, 0x66);

    cpu.step();
    cpu.step();
    cpu.step();
    cpu.step();
    cpu.step();
    assert_eq!(cpu.accumulator, 0x66);
}

#[test]
fn test_lda_flags_none_when_none() {
    let mut cpu = Cpu::test_instance();
    cpu.mem_write(0x0, 0xA9);
    cpu.mem_write(0x1, 0x66);

    cpu.step();
    cpu.step();
    assert_eq!(cpu.accumulator, 0x66);
    assert!(!cpu.get_negative_flag());
    assert!(!cpu.get_zero_flag());
}

#[test]
fn test_lda_flags_only_zero_when_zero() {
    let mut cpu = Cpu::test_instance();
    cpu.mem_write(0x0, 0xA9);
    cpu.mem_write(0x1, 0x0);

    cpu.step();
    cpu.step();
    assert_eq!(cpu.accumulator, 0x0);
    assert!(!cpu.get_negative_flag());
    assert!(cpu.get_zero_flag());
}

#[test]
fn test_lda_flags_only_negative_when_negative() {
    let mut cpu = Cpu::test_instance();
    cpu.mem_write(0x0, 0xA9);
    cpu.mem_write(0x1, 0x80);

    cpu.step();
    cpu.step();
    assert_eq!(cpu.accumulator, 0x80);
    assert!(cpu.get_negative_flag());
    assert!(!cpu.get_zero_flag());
}
