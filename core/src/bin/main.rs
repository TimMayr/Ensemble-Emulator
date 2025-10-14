use std::time::Instant;

use nes_core::emulation::emu::{Console, Consoles};
use nes_core::emulation::nes::Nes;
#[cfg(feature = "sdl2")]
use nes_core::frontend::sdl_frontend::SdlFrontend;
use nes_core::frontend::Frontends;

#[cfg(feature = "sdl2")]
fn main() {
    let mut emu = Consoles::Nes(Nes::default());
    let mut frontend = Frontends::Sdl2(SdlFrontend::default());

    emu.load_rom(&String::from("./core/tests/Mario Bros. (World).nes"));
    emu.power();

    let start = Instant::now();

    emu.run_until(&mut frontend, u128::MAX)
        .expect("TODO: panic message");

    println!("{:?}", start.elapsed());
}

#[cfg(not(feature = "sdl2"))]
fn main() {
    let mut emu = Consoles::Nes(Nes::default());

    let mut frontend = Frontends::default();

    emu.load_rom(&String::from(
        "./core/tests/nes-test-roms/oam_read/oam_read.nes",
    ));
    emu.power();

    let start = Instant::now();

    let Consoles::Nes(ref mut emu) = emu;

    // for i in 0..u128::MAX {
    //     emu.step(&mut frontend, u128::MAX).expect("panic message");
    //     let val = emu.get_memory_debug(Some(0x6000..=0x6000))[0][0];
    //
    //     if val == 0x81 {
    //         for _ in 0..8_000_000 {
    //             emu.step(&mut frontend, u128::MAX).expect("panic message");
    //         }
    //
    //         emu.reset();
    //     }
    //
    //     if i > 4_000_000 && val != 0x80 && val != 0x81 {
    //         break;
    //     }
    // }

    emu.run_until(&mut frontend, 300_000_000)
        .expect("panic message");

    println!("{:?}", start.elapsed());

    let mem = &emu.get_memory_debug(Some(0x6000..=0x6200))[0];

    for (i, n) in mem.iter().enumerate() {
        if i % 32 == 0 {
            if i > 0 {
                println!();
            }
            print!("    ");
        }
        print!("{:02X}, ", n);
    }
    println!();
}
