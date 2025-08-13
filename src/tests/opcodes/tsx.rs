use crate::cpu::Cpu;

#[test]
fn test_tsx_simple() {
    let mut cpu = Cpu::test_instance();
    cpu.stack_pointer = 0x66;

    cpu.mem_write(0x0, 0xBA);
    cpu.step();

    assert_eq!(0x66, cpu.x_register);
    assert!(!cpu.get_zero_flag());
    assert!(!cpu.get_negative_flag());
}

#[test]
fn test_tsx_complicated() {
    let mut cpu = Cpu::test_instance();
    cpu.stack_pointer = 0x66;

    cpu.mem_write(0x0, 0xBA);
    cpu.step();

    assert_eq!(0x66, cpu.x_register);
    assert!(!cpu.get_zero_flag());
    assert!(!cpu.get_negative_flag());

    cpu.stack_pointer = 0x00;

    cpu.mem_write(0x1, 0xBA);
    cpu.step();

    assert_eq!(0x00, cpu.x_register);
    assert!(cpu.get_zero_flag());
    assert!(!cpu.get_negative_flag());

    cpu.stack_pointer = 0x80;

    cpu.mem_write(0x2, 0xBA);
    cpu.step();

    assert_eq!(0x80, cpu.x_register);
    assert!(!cpu.get_zero_flag());
    assert!(cpu.get_negative_flag());

    //Increment stack counter by reading from stack
    cpu.mem_write(0x3, 0x28);
    cpu.step();

    cpu.mem_write(0x4, 0xBA);
    cpu.step();

    //Validate that incremented stack counter is loaded correctly
    assert_eq!(0x81, cpu.x_register);
    assert!(!cpu.get_zero_flag());
    assert!(cpu.get_negative_flag());
}
