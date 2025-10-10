use crate::emulation::cpu::Cpu;
use crate::emulation::mem::{Memory, MemoryDevice, Rom};

#[test]
fn test_brk_vector() {
    let mut cpu = Cpu::new();

    // Create and initialize new Rom
    let mut rom = Rom::new(0xBFE0);

    // Set Break Handler vector to 0x4020
    rom.init(0xBFDE, 0x20);
    rom.init(0xBFDF, 0x40);

    // Load 0x20 to acc from 4020
    rom.init(0x0, 0xA9);
    rom.init(0x1, 0x20);

    // Trigger Break at cpu start
    cpu.mem_write(0x0, 0x0);

    // Attach new Rom memory device to cpu
    cpu.memory.add_memory(0x4020..=0xFFFF, Memory::Rom(rom));

    cpu.step();
    cpu.step();
    cpu.step();
    cpu.step();
    cpu.step();
    cpu.step();
    cpu.step();

    cpu.step();
    cpu.step();
    assert_eq!(cpu.accumulator, 0x20)
}
