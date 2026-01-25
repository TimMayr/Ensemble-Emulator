use std::path::PathBuf;
use std::process::ExitCode;
use std::time::Instant;

use egui::TextBuffer;
use nes_core::cli::{self, CliArgs};
use nes_core::emulation::nes::Nes;
use nes_core::emulation::rom::RomFile;
use nes_core::frontend::egui_frontend;

fn main() -> ExitCode {
    let args = match cli::parse_args() {
        Ok(args) => args,
        Err(e) => {
            eprintln!("Error parsing arguments: {}", e);
            return ExitCode::from(2);
        }
    };

    if let Err(e) = cli::validate_args(&args) {
        eprintln!("Invalid arguments: {}", e);
        return ExitCode::from(2);
    }

    let res = if args.headless {
        start_headless(&args)
    } else {
        start_egui(&args)
    };

    match res {
        Ok(_) => {
            if !args.quiet {
                println!("Emulator finished execution");
            }
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("Emulator finished with error: \"{}\"", e);
            ExitCode::from(1)
        }
    }
}

fn start_egui(args: &CliArgs) -> Result<(), String> {
    let rom = args.rom.rom.clone();
    let palette = args.palette.palette.clone();

    if let Err(e) = egui_frontend::run(rom, palette) {
        Err(e.to_string())
    } else {
        Ok(())
    }
}

fn start_headless(args: &CliArgs) -> Result<(), String> {
    let mut emu = Nes::default();

    // Handle ROM info display
    if args.rom.rom_info {
        if let Some(ref rom_path) = args.rom.rom {
            return print_rom_info(rom_path);
        } else {
            return Err("--rom-info requires --rom to be specified".to_string());
        }
    }

    // Load ROM
    if let Some(ref rom_path) = args.rom.rom {
        emu.load_rom(&rom_path.to_string_lossy().take());
    } else if args.savestate.load_state.is_none() && !args.savestate.state_stdin {
        return Err("Headless runs must specify the ROM file using the -r (--rom) argument".to_string());
    }

    // Power on (unless --no-power is specified)
    if !args.power.no_power {
        emu.power();
    }

    // Handle reset
    if args.power.reset {
        emu.reset();
    }

    let start = Instant::now();

    // Determine execution limit
    let target_cycles = if let Some(cycles) = args.execution.cycles {
        cycles
    } else if let Some(frames) = args.execution.frames {
        // Approximately 357366 master cycles per frame
        frames as u128 * 357366
    } else {
        // Default: run for a reasonable amount (same as before)
        700_119_365
    };

    let res = emu.run_until(target_cycles);
    let res = if let Err(e) = res { Err(e) } else { Ok(()) };

    if args.verbose {
        println!("Execution time: {:?}", start.elapsed());
    }

    // Handle memory dump output
    if let Some(ref range) = args.memory.read_cpu {
        output_memory_dump(&emu, range, args)?;
    } else if !args.quiet {
        // Default behavior: dump test results area
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
    }

    res
}

fn print_rom_info(rom_path: &PathBuf) -> Result<(), String> {
    let rom = RomFile::load(&rom_path.to_string_lossy().to_string());

    println!("ROM Information:");
    println!("  File: {}", rom_path.display());
    if let Some(ref name) = rom.name {
        println!("  Name: {}", name);
    }
    println!("  Mapper: {}", rom.mapper_number);
    println!("  PRG ROM: {} KB", rom.prg_memory.prg_rom_size / 1024);
    println!("  CHR ROM: {} KB", rom.chr_memory.chr_rom_size / 1024);
    println!("  PRG RAM: {} KB (battery-backed: {})",
             rom.prg_memory.prg_ram_size / 1024,
             if rom.is_battery_backed { "yes" } else { "no" });
    println!("  Mirroring: {}",
             if rom.hardwired_nametable_layout { "Vertical" } else { "Horizontal" });
    println!("  Checksum (SHA-256): {}",
             rom.data_checksum.iter().map(|b| format!("{:02x}", b)).collect::<String>());

    Ok(())
}

fn output_memory_dump(emu: &Nes, range: &str, args: &CliArgs) -> Result<(), String> {
    // Parse range: either START-END or START:LENGTH
    let (start, end) = parse_memory_range(range)?;

    let mem = &emu.get_memory_debug(Some(start..=end))[0];

    match args.output.effective_format() {
        nes_core::cli::args::OutputFormat::Hex => {
            for (i, chunk) in mem.chunks(16).enumerate() {
                print!("{:04X}: ", start as usize + i * 16);
                for byte in chunk {
                    print!("{:02X} ", byte);
                }
                println!();
            }
        }
        nes_core::cli::args::OutputFormat::Binary => {
            use std::io::Write;
            if let Some(ref path) = args.output.output {
                std::fs::write(path, mem).map_err(|e| e.to_string())?;
            } else {
                std::io::stdout().write_all(mem).map_err(|e| e.to_string())?;
            }
        }
        nes_core::cli::args::OutputFormat::Json => {
            let json = serde_json::json!({
                "memory_dump": {
                    "type": "cpu",
                    "start": format!("0x{:04X}", start),
                    "end": format!("0x{:04X}", end),
                    "data": mem.iter().map(|b| *b).collect::<Vec<_>>()
                }
            });
            println!("{}", serde_json::to_string_pretty(&json).unwrap_or_default());
        }
        nes_core::cli::args::OutputFormat::Toml => {
            println!("[memory_dump]");
            println!("type = \"cpu\"");
            println!("start = \"0x{:04X}\"", start);
            println!("end = \"0x{:04X}\"", end);
            println!("data = {:?}", mem);
        }
    }

    Ok(())
}

fn parse_memory_range(range: &str) -> Result<(u16, u16), String> {
    if let Some((start_str, end_str)) = range.split_once('-') {
        let start = parse_hex(start_str)?;
        let end = parse_hex(end_str)?;
        Ok((start, end))
    } else if let Some((start_str, len_str)) = range.split_once(':') {
        let start = parse_hex(start_str)?;
        let len = parse_hex(len_str)?;
        Ok((start, start.saturating_add(len).saturating_sub(1)))
    } else {
        Err(format!("Invalid memory range format: {}. Use START-END or START:LENGTH", range))
    }
}

fn parse_hex(s: &str) -> Result<u16, String> {
    let s = s.trim();
    let s = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")).unwrap_or(s);
    u16::from_str_radix(s, 16).map_err(|e| format!("Invalid hex value '{}': {}", s, e))
}
