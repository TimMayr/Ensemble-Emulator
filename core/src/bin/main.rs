use std::time::Instant;

use nes_core::emulation::emu::{Console, Consoles};
use nes_core::emulation::nes::Nes;
use nes_core::frontend::Frontends;
#[cfg(feature = "sdl2")]
use nes_core::frontend::sdl_frontend::SdlFrontend;

#[cfg(feature = "sdl2")]
fn main() {
    let mut emu = Consoles::Nes(Nes::default());
    let mut frontend = Frontends::Sdl2(SdlFrontend::default());

    emu.load_rom(&String::from("./core/tests/Mario Bros. (World).nes"));
    emu.reset();

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
        "./core/tests/nes-test-roms/cpu_reset/registers.nes",
    ));

    let start = Instant::now();

    for i in 0..u128::MAX {
        emu.step(&mut frontend).expect("TODO: panic message");

        let val = emu.get_memory_debug(Some(0x6000..=0x6000))[0][0];

        if val == 0x81 {
            for _ in 0..4_000_000 {
                emu.step(&mut frontend).expect("TODO: panic message");
            }
            emu.reset();
        }

        if i > 80_000_000 && val != 0x80 && val != 0x81 {
            break;
        }
    }

    // emu.run_until(&mut frontend, 750_000_000)
    //     .expect("panic message");

    let Consoles::Nes(ref mut nes) = emu;

    println!("{:?}", start.elapsed());

    let mem = &nes.get_memory_debug(Some(0x6000..=0x6100))[0];

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
