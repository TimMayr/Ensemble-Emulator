use nesamabob::emulation::emu::{Console, Consoles};
use nesamabob::emulation::nes::Nes;
use nesamabob::frontend::{Frontends, SdlFrontend};

fn main() {
    let mut emu = Consoles::Nes(Nes::default());
    let frontend = Frontends::Sdl2(SdlFrontend::default());

    emu.load_rom(&String::from(
        "./tests/nes-test-roms/instr_test-v5/rom_singles/01-basics.nes",
    ));
    emu.reset();
    emu.run_until(&mut Some(frontend), 4312488)
        .expect("TODO: panic message");
    println!("{:02x?}", emu.get_memory_debug(Some(0x6000..=0x6100)));
}
