use crate::emulation::emu::{Console, Consoles};
use crate::emulation::nes::Nes;

#[test]
fn test_01_basic() {
    let mut emu = Consoles::Nes(Nes::default());
    emu.load_rom(&String::from(
        "./tests/nes-test-roms/instr_test-v5/rom_singles/01-basics.nes",
    ));
    emu.reset();
    emu.run_until(&mut None, 4312500)
        .expect("Error while running test");

    let whole_mem = emu.get_memory_debug(Some(0x6000..=0x6017));
    let cpu_mem = whole_mem[0].as_slice();

    let expected = [
        0x0, 0xDE, 0xB0, 0x61, 0x0A, 0x30, 0x31, 0x2D, 0x62, 0x61, 0x73, 0x69, 0x63, 0x73, 0x0A,
        0x0A, 0x50, 0x61, 0x73, 0x73, 0x65, 0x64, 0x0A, 0x0,
    ];

    assert_eq!(cpu_mem[0], 0);
    assert_eq!(&cpu_mem[..expected.len()], &expected);
}
