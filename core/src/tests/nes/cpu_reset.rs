use crate::emulation::emu::{Console, Consoles};
use crate::emulation::nes::Nes;
use crate::frontend::Frontends;

#[test]
fn test_ram_after_reset() {
    let mut emu = Consoles::Nes(Nes::default());
    emu.load_rom(&String::from(
        "./tests/nes-test-roms/cpu_reset/ram_after_reset.nes",
    ));
    emu.reset();

    for i in 0..u128::MAX {
        emu.step(&mut Frontends::default())
            .expect("TODO: panic message");

        let val = emu.get_memory_debug(Some(0x6000..=0x6000))[0][0];

        if val == 0x81 {
            for _ in 0..4_000_000 {
                emu.step(&mut Frontends::default())
                    .expect("TODO: panic message");
            }
            emu.reset();
        }

        if i > 4_000_000 && val != 0x80 && val != 0x81 {
            break;
        }
    }

    let whole_mem = emu.get_memory_debug(Some(0x6000..=0x6032));
    let cpu_mem = whole_mem[0].as_slice();

    let expected = [
        0x00, 0xDE, 0xB0, 0x61, 0x0A, 0x72, 0x61, 0x6D, 0x5F, 0x61, 0x66, 0x74, 0x65, 0x72, 0x5F,
        0x72, 0x65, 0x73, 0x65, 0x74, 0x0A, 0x0A, 0x50, 0x61, 0x73, 0x73, 0x65, 0x64, 0x0A, 0x00,
        0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x0A, 0x64, 0x69, 0x73, 0x61, 0x70, 0x70, 0x65, 0x61,
        0x72, 0x73, 0x0A, 0x0A, 0x0A, 0x00,
    ];

    assert_eq!(cpu_mem[0], 0);
    assert_eq!(&cpu_mem[..expected.len()], &expected);
}
