use std::cell::RefCell;
use std::rc::Rc;

use crate::emulation::cpu::Cpu;
use crate::emulation::nes::Nes;
use crate::emulation::ppu::Ppu;

#[test]
fn test_03_immediate() {
    let cpu = Cpu {
        ane_constant: 0xFF,
        ..Cpu::default()
    };

    let ppu = Ppu::default();
    let mut emu = Nes::new(cpu, Rc::new(RefCell::new(ppu)));

    emu.load_rom(&String::from(
        "./tests/nes-test-roms/instr_test-v5/rom_singles/03-immediate.nes",
    ));

    emu.reset();
    emu.run_until(41064917).expect("Error while running test");

    let whole_mem = emu.get_memory_debug(Some(0x6000..=0x601A));
    let cpu_mem = whole_mem[0].as_slice();

    let expected = [
        0x00, 0xDE, 0xB0, 0x61, 0x0A, 0x30, 0x33, 0x2D, 0x69, 0x6D, 0x6D, 0x65, 0x64, 0x69, 0x61,
        0x74, 0x65, 0x0A, 0x0A, 0x50, 0x61, 0x73, 0x73, 0x65, 0x64, 0x0A, 0x00,
    ];

    assert_eq!(cpu_mem[0], 0);
    assert_eq!(&cpu_mem[..expected.len()], &expected);
}
