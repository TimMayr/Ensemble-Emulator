use nesamabob::emulation::emu::{Console, Consoles};
use nesamabob::emulation::nes::Nes;
use nesamabob::frontend::{Frontends, SdlFrontend};

fn main() -> Result<(), String> {
    let mut emu = Consoles::Nes(Nes::default());
    let frontend = Frontends::Sdl2(SdlFrontend::default());

    emu.load_rom(&String::from("./tests/Pac-Man (USA) (Namco).nes"));
    emu.reset();
    emu.run(&mut Some(frontend))
}
