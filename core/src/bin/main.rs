#[cfg(not(feature = "sdl2"))]
use std::time::Instant;

use nes_core::emulation::emu::{Console, Consoles};
use nes_core::emulation::nes::Nes;
#[allow(unused_imports)]
use nes_core::frontend::Frontends;
#[cfg(feature = "sdl2")]
use nes_core::frontend::sdl_frontend::SdlFrontend;

#[cfg(feature = "sdl2")]
fn main() {
    let mut emu = Consoles::Nes(Nes::default());
    let mut frontend = Frontends::Sdl2(SdlFrontend::default());

    emu.load_rom(&String::from("./core/tests/Mario Bros. (World).nes"));
    emu.reset();
    emu.run(&mut frontend).expect("TODO: panic message");
}

#[cfg(not(feature = "sdl2"))]
fn main() {
    let mut emu = Consoles::Nes(Nes::default());

    let mut frontend = Frontends::default();

    emu.load_rom(&String::from(
        "./core/tests/nes-test-roms/ppu_vbl_nmi/rom_singles/04-nmi_control.nes",
    ));
    emu.reset();

    let start = Instant::now();
    emu.run_until(&mut frontend, 12523621)
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
