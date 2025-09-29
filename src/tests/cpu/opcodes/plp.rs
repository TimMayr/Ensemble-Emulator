use crate::emulation::cpu::Cpu;

#[test]
fn test_plp_implied_simple() {
    let mut cpu = Cpu::test_instance();

    // Load 0 into acc to trigger Zero flag
    cpu.mem_write(0x0, 0xA9);
    cpu.mem_write(0x1, 0x00);
    cpu.step(0);
    cpu.step(0);

    // Write Processor Status with Set Zero flag to Stack
    cpu.mem_write(0x2, 0x08);
    cpu.step(0);
    cpu.step(0);
    cpu.step(0);

    // Load 0x66 to acc to reset Zero flag
    cpu.mem_write(0x3, 0xA9);
    cpu.mem_write(0x4, 0x66);
    cpu.step(0);
    cpu.step(0);

    // Validate that Zero flag was reset
    assert_eq!(cpu.processor_status, 0b00000000);

    // Load saved Processor Status from Stack
    cpu.mem_write(0x5, 0x28);
    cpu.step(0);
    cpu.step(0);
    cpu.step(0);
    cpu.step(0);

    // Validate that Processor Status was loaded correctly
    assert_eq!(cpu.processor_status, 0b00000010);
    // Validate that Stack Point is correct
    assert_eq!(cpu.stack_pointer, 0xFF)
}

#[test]
fn test_plp_implied_complicated() {
    let mut cpu = Cpu::test_instance();

    // Load 0 into acc to trigger Zero flag
    cpu.mem_write(0x0, 0xA9);
    cpu.mem_write(0x1, 0x00);

    // Write Processor Status with Set Zero flag to Stack
    cpu.mem_write(0x2, 0x08);

    cpu.step(0);
    cpu.step(0);

    cpu.step(0);
    cpu.step(0);
    cpu.step(0);

    // Load 0x80 into acc to trigger Negative flag
    cpu.mem_write(0x3, 0xA9);
    cpu.mem_write(0x4, 0x80);

    // Write Processor Status with Set Negative flag to Stack
    cpu.mem_write(0x5, 0x08);

    cpu.step(0);
    cpu.step(0);

    cpu.step(0);
    cpu.step(0);
    cpu.step(0);

    // Load 0x66 to acc to reset Negative flag
    cpu.mem_write(0x6, 0xA9);
    cpu.mem_write(0x7, 0x66);

    // Write Processor Status without flags to Stack
    cpu.mem_write(0x8, 0x08);

    cpu.step(0);
    cpu.step(0);

    cpu.step(0);
    cpu.step(0);
    cpu.step(0);

    // Load Processor Status from Stack
    cpu.mem_write(0x9, 0x28);
    cpu.step(0);
    cpu.step(0);
    cpu.step(0);
    cpu.step(0);

    // Validate that loaded Processor Status doesn't have flags set
    assert_eq!(cpu.processor_status, 0b00000000);

    // Load Processor Status from Stack
    cpu.mem_write(0xA, 0x28);
    cpu.step(0);
    cpu.step(0);
    cpu.step(0);
    cpu.step(0);

    // Validate that loaded Processor Status has Negative flag set
    assert_eq!(cpu.processor_status, 0b10000000);

    cpu.mem_write(0xB, 0x28);
    cpu.step(0);
    cpu.step(0);
    cpu.step(0);
    cpu.step(0);

    // Validate that loaded Processor Status has Zero flag set
    assert_eq!(cpu.processor_status, 0b00000010);
    // Validate that Stack pointer is correct
    assert_eq!(cpu.stack_pointer, 0xFF)
}
