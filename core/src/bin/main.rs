use std::path::PathBuf;
use std::time::Instant;

use clap::{Parser, value_parser};
use egui::TextBuffer;
use nes_core::emulation::nes::Nes;
use nes_core::frontend::egui_frontend;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    ///Run in headless mode
    #[arg(short = 'H', long, default_value_t = false)]
    headless: bool,

    ///Rom File to load
    #[arg(value_parser = value_parser!(PathBuf), value_hint = clap::ValueHint::FilePath, short, long)]
    rom: Option<PathBuf>,

    ///RGB Palette to load
    #[arg(value_parser = value_parser!(PathBuf), value_hint = clap::ValueHint::FilePath, short, long)]
    palette: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();

    let res = if args.headless {
        start_headless(args.rom)
    } else {
        start_egui(args.rom, args.palette)
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

fn start_egui(rom: Option<PathBuf>, palette: Option<PathBuf>) -> Result<(), String> {
    if let Err(e) = egui_frontend::run(rom, palette) {
        Err(e.to_string())
    } else {
        Ok(())
    }
}

fn start_headless(rom: Option<PathBuf>) -> Result<(), String> {
    let mut emu = Nes::default();


    if rom.is_none() {
        panic!("Headless runs must specify the rom file using the -r (--rom) argument")
    }

    emu.load_rom(&rom.unwrap().to_string_lossy().take());
    emu.power();

    let start = Instant::now();

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
