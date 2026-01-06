use std::path::PathBuf;
use std::time::Instant;

use clap::{Parser, value_parser};
use egui::TextBuffer;
use nes_core::emulation::emu::{Console, Consoles};
use nes_core::emulation::nes::Nes;
use nes_core::frontend::egui_frontend;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    ///Run in headless mode
    #[arg(short = 'H', long, default_value_t = false)]
    headless: bool,

    ///Rom File to load
    #[arg(value_parser = value_parser!(PathBuf), value_hint = clap::ValueHint::FilePath)]
    file: PathBuf,
}

fn main() {
    let args = Args::parse();

    let res = if args.headless {
        start_headless(args.file)
    } else {
        start_egui(args.file)
    };

    match res {
        Ok(_) => {
            println!("Emulator finished execution")
        }
        Err(t) => {
            println!("Emulator finished with error: \"{}\"", t)
        }
    }
}

fn start_egui(file: PathBuf) -> Result<(), String> {
    if let Err(e) = egui_frontend::run(file) {
        Err(e.to_string())
    } else {
        Ok(())
    }
}

fn start_headless(file: PathBuf) -> Result<(), String> {
    let mut emu = Consoles::Nes(Nes::default());

    emu.load_rom(&file.to_string_lossy().take());
    emu.power();

    let start = Instant::now();

    let Consoles::Nes(ref mut emu) = emu;
    let res = emu.run_until(700_119_365);

    let res = if let Err(e) = res { Err(e) } else { Ok(()) };

    println!("{:?}", start.elapsed());

    let mem = &emu.get_memory_debug(Some(0x6000..=0x6500))[0];

    for (i, n) in mem.iter().enumerate() {
        if i % 32 == 0 {
            if i > 0 {
                println!();
            }
            print!("    ");
        }
        print!("{:02X} ", n);
    }
    println!();

    res
}
