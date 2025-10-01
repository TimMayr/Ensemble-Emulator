use std::time::Instant;

use nesamabob::emulation::emu::{Console, Consoles};
use nesamabob::emulation::nes::Nes;
use nesamabob::frontend::{Frontends, SdlFrontend};

fn main() {
    let mut emu = Consoles::Nes(Nes::default());
    // emu.set_trace_log_path(Some(String::from("./trace-log.log")));
    let mut frontend = Frontends::Sdl2(SdlFrontend::default());

    emu.load_rom(&String::from(
        "./tests/nes-test-roms/instr_test-v5/rom_singles/03-immediate.nes",
    ));
    emu.reset();

    let start = Instant::now();
    emu.run_until(&mut frontend, 50145737)
        .expect("TODO: panic message");

    println!("{:?}", start.elapsed());

    let mem = &emu.get_memory_debug(Some(0x6000..=0x6100))[0];

    for (i, n) in mem.iter().enumerate() {
        // print 16 per row, like `Debug`
        if i % 32 == 0 {
            if i > 0 {
                println!();
            }
            print!("    "); // indentation like `Debug`
        }
        print!("{:02X}, ", n);
    }
    println!();
}
