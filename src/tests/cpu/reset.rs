use crate::emulation::cpu::Cpu;
use crate::emulation::mem::{Memory, MemoryDevice, Rom};

#[test]
fn test_reset_vector() {
    let mut cpu = Cpu::new();

    //Create and initialize new Rom
    let mut rom = Rom::new(0xBFE0);
    //Set Reset vector to 0x4020
    rom.init(0xBFDC, 0x20);
    rom.init(0xBFDD, 0x40);
    //Load 0x20 to acc from 4020
    rom.init(0x0, 0xA9);
    rom.init(0x1, 0x20);

    //Attach new Rom memory device to cpu
    cpu.memory.add_memory(0x4020..=0xFFFF, Memory::Rom(rom));

    cpu.reset();
    cpu.step();
    assert_eq!(cpu.accumulator, 0x20)
}
