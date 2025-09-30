use crate::emulation::emu::{Console, Consoles};
use crate::emulation::nes::Nes;

#[test]
fn test_05_zp_xy() {
    let mut emu = Consoles::Nes(Nes::default());
    emu.load_rom(&String::from(
        "./tests/nes-test-roms/instr_test-v5/rom_singles/10-branches.nes",
    ));
    emu.reset();
    emu.run_until(&mut None, 11653057)
        .expect("Error while running test");

    let whole_mem = emu.get_memory_debug(Some(0x6000..=0x6019));
    let cpu_mem = whole_mem[0].as_slice();

    let expected = [
        0x00, 0xDE, 0xB0, 0x61, 0x0A, 0x31, 0x30, 0x2D, 0x62, 0x72, 0x61, 0x6E, 0x63, 0x68, 0x65,
        0x73, 0x0A, 0x0A, 0x50, 0x61, 0x73, 0x73, 0x65, 0x64, 0x0A, 0x00,
    ];

    assert_eq!(cpu_mem[0], 0);
    assert_eq!(&cpu_mem[..expected.len()], &expected);
}
