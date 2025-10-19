use crate::emulation::emu::{Console, Consoles};
use crate::emulation::nes::Nes;
use crate::frontend::Frontends;

#[test]
fn test_12_jmp_jsr() {
    let mut emu = Consoles::Nes(Nes::default());
    emu.load_rom(&String::from(
        "./tests/nes-test-roms/instr_test-v5/rom_singles/12-jmp_jsr.nes",
    ));
    emu.reset();
    emu.run_until(&mut Frontends::default(), &8049153)
        .expect("Error while running test");

    let whole_mem = emu.get_memory_debug(Some(0x6000..=0x6018));
    let cpu_mem = whole_mem[0].as_slice();

    let expected = [
        0x00, 0xDE, 0xB0, 0x61, 0x0A, 0x31, 0x32, 0x2D, 0x6A, 0x6D, 0x70, 0x5F, 0x6A, 0x73, 0x72,
        0x0A, 0x0A, 0x50, 0x61, 0x73, 0x73, 0x65, 0x64, 0x0A, 0x00,
    ];

    assert_eq!(cpu_mem[0], 0);
    assert_eq!(&cpu_mem[..expected.len()], &expected);
}
