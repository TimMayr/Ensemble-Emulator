use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;

use nesamabob::emulation::cpu::Cpu;
use nesamabob::emulation::emu::{Console, Consoles};
use nesamabob::emulation::nes::Nes;
use nesamabob::emulation::ppu::Ppu;
#[allow(unused_imports)]
use nesamabob::frontend::{Frontends, SdlFrontend};

fn main() {
    let cpu = Cpu::default();
    let ppu = Ppu::default();
    let mut emu = Consoles::Nes(Nes::new(cpu, Rc::new(RefCell::new(ppu)), None));

    let mut frontend = Frontends::default();
    let mut frontend = Frontends::Sdl2(SdlFrontend::default());

    emu.load_rom(&String::from("./tests/Mario Bros. (World).nes"));
    emu.reset();

    let start = Instant::now();
    emu.run_until(&mut frontend, 200000000)
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
