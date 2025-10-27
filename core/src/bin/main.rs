use std::time::Instant;

#[cfg(feature = "frontend")]
use nes_core::app::App;
use nes_core::emulation::emu::Console;

#[cfg(feature = "frontend")]
fn main() {
    let mut app = App::default();

    app.emulator
        .lock()
        .unwrap()
        .load_rom(&"./core/tests/Pac-Man (USA) (Namco).nes".to_string());

    let start = Instant::now();

    app.run();

    println!("{:?}", start.elapsed());
}

#[cfg(not(feature = "frontend"))]
fn main() {
    let mut emu = Consoles::Nes(Nes::default());

    emu.load_rom(&String::from(
        "./core/tests/nes-test-roms/oam_stress/oam_stress.nes",
    ));
    emu.power();

    let start = Instant::now();

    let Consoles::Nes(ref mut emu) = emu;

    // for i in 0..u128::MAX {
    //     emu.step(&mut app, u128::MAX).expect("panic message");
    //     let val = emu.get_memory_debug(Some(0x6000..=0x6000))[0][0];
    //
    //     if val == 0x81 {
    //         for _ in 0..8_000_000 {
    //             emu.step(&mut app, u128::MAX).expect("panic message");
    //         }
    //
    //         emu.reset();
    //     }
    //
    //     if i > 4_000_000 && val != 0x80 && val != 0x81 {
    //         break;
    //     }
    // }

    emu.run_until(700_119_365).expect("panic message");

    println!("{:?}", start.elapsed());

    let mem = &emu.get_memory_debug(Some(0x6000..=0x6500))[0];

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
