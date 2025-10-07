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
    emu.reset();

    let start = Instant::now();

    emu.run_until(&mut frontend, 1846387 * 12)
        .expect("TODO: panic message");

    println!("{:?}", start.elapsed());
}

#[cfg(not(feature = "sdl2"))]
fn main() {
    let mut emu = Consoles::Nes(Nes::default());

    let mut frontend = Frontends::default();

    emu.load_rom(&String::from("./core/tests/Mario Bros. (World).nes"));
    emu.reset();

    let start = Instant::now();
    emu.run_until(&mut frontend, 1846387 * 120)
        .expect("TODO: panic message");

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
