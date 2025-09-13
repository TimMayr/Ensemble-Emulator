use crate::cpu::Cpu;
use crate::mem::{Memory, Rom};

#[test]
fn test_cpu_ram_writeable_and_readable() {
    let mut cpu = Cpu::new();
    cpu.mem_write(0x10, 0x20);
    assert_eq!(cpu.mem_read(0x10), 0x20)
}

#[test]
fn test_cpu_ram_mirroring() {
    let mut cpu = Cpu::new();
    cpu.mem_write(0x10, 0x10);
    assert_eq!(cpu.mem_read(0x810), 0x10);

    cpu.mem_write(0x820, 0x20);
    assert_eq!(cpu.mem_read(0x20), 0x20);

    cpu.mem_write(0x830, 0x30);
    assert_eq!(cpu.mem_read(0x830), 0x30);
}

#[test]
fn test_rom_readable() {
    let mut cpu = Cpu::new();

    //Create and initialize new Rom
    let mut rom = Rom::new(0xBFE0);
    rom.init(0x10, 0x20);

    //Attach new Rom memory device to cpu
    cpu.memory.add_memory(0x4020..=0xFFFF, Box::new(rom));

    assert_eq!(cpu.mem_read(0x4030), 0x20);
}

#[test]
fn test_rom_non_writeable() {
    let mut cpu = Cpu::new();

    //Attach new Rom memory device to cpu
    cpu.memory
        .add_memory(0x4020..=0xFFFF, Box::new(Rom::new(0xBFE0)));

    cpu.mem_write(0x4030, 0x20);

    assert_eq!(cpu.mem_read(0x4030), 0x00);
}
