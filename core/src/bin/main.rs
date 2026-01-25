//! NES Emulator CLI entry point.
//!
//! This is the main entry point for the NES emulator. It supports both
//! GUI mode (using egui) and headless mode for programmatic control.
//!
//! See `docs/CLI_INTERFACE.md` for full CLI documentation.

use std::path::PathBuf;
use std::process::ExitCode;
use std::time::Instant;

use nes_core::cli::{self, CliArgs};
use nes_core::emulation::nes::{MASTER_CYCLES_PER_FRAME, Nes};
use nes_core::emulation::rom::RomFile;
use nes_core::frontend::egui_frontend;

// =============================================================================
// Exit Codes (as documented in CLI_INTERFACE.md)
// =============================================================================

const EXIT_SUCCESS: u8 = 0;
const EXIT_GENERAL_ERROR: u8 = 1;
const EXIT_INVALID_ARGS: u8 = 2;
// const EXIT_ROM_LOAD_FAILED: u8 = 3;  // Reserved for future use
// const EXIT_SAVESTATE_FAILED: u8 = 4; // Reserved for future use
// const EXIT_IO_ERROR: u8 = 5;         // Reserved for future use
// const EXIT_TIMEOUT: u8 = 6;          // Reserved for future use

// =============================================================================
// Main Entry Point
// =============================================================================

fn main() -> ExitCode {
    let args = match cli::parse_args() {
        Ok(args) => args,
        Err(e) => {
            eprintln!("Error parsing arguments: {}", e);
            return ExitCode::from(EXIT_INVALID_ARGS);
        }
    };

    if let Err(e) = cli::validate_args(&args) {
        eprintln!("Invalid arguments: {}", e);
        return ExitCode::from(EXIT_INVALID_ARGS);
    }

    let result = if args.headless {
        run_headless(&args)
    } else {
        run_gui(&args)
    };

    match result {
        Ok(_) => {
            if !args.quiet {
                println!("Emulator finished execution");
            }
            ExitCode::from(EXIT_SUCCESS)
        }
        Err(e) => {
            eprintln!("Emulator finished with error: \"{}\"", e);
            ExitCode::from(EXIT_GENERAL_ERROR)
        }
    }
}

// =============================================================================
// GUI Mode
// =============================================================================

fn run_gui(args: &CliArgs) -> Result<(), String> {
    egui_frontend::run(args.rom.rom.clone(), args.palette.palette.clone())
        .map_err(|e| e.to_string())
}

// =============================================================================
// Headless Mode
// =============================================================================

fn run_headless(args: &CliArgs) -> Result<(), String> {
    // Handle ROM info display (exits early)
    if args.rom.rom_info {
        return handle_rom_info(args);
    }

    // Initialize emulator
    let mut emu = setup_emulator(args)?;

    // Run emulation
    let execution_result = execute_emulation(&mut emu, args);

    // Output results
    output_results(&emu, args)?;

    execution_result
}

/// Handle --rom-info flag
fn handle_rom_info(args: &CliArgs) -> Result<(), String> {
    let rom_path = args
        .rom
        .rom
        .as_ref()
        .ok_or("--rom-info requires --rom to be specified")?;
    print_rom_info(rom_path)
}

/// Set up the emulator with ROM, power state, etc.
fn setup_emulator(args: &CliArgs) -> Result<Nes, String> {
    let mut emu = Nes::default();

    // Load ROM
    if let Some(ref rom_path) = args.rom.rom {
        emu.load_rom(rom_path);
    } else if args.savestate.load_state.is_none() && !args.savestate.state_stdin {
        return Err(
            "Headless runs must specify the ROM file using the -r (--rom) argument".to_string(),
        );
    }

    // Power on (unless --no-power is specified)
    if !args.power.no_power {
        emu.power();
    }

    // Handle reset
    if args.power.reset {
        emu.reset();
    }

    Ok(emu)
}

/// Execute emulation for the specified duration/conditions
fn execute_emulation(emu: &mut Nes, args: &CliArgs) -> Result<(), String> {
    let start = Instant::now();

    // Determine execution limit
    let target_cycles = calculate_target_cycles(args);

    // Run emulation
    let result = emu.run_until(target_cycles);

    if args.verbose {
        println!("Execution time: {:?}", start.elapsed());
        println!("Target cycles: {}", target_cycles);
    }

    // Convert ExecutionFinishedType to simple Result
    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

/// Calculate target cycles based on CLI args
fn calculate_target_cycles(args: &CliArgs) -> u128 {
    if let Some(cycles) = args.execution.cycles {
        cycles
    } else if let Some(frames) = args.execution.frames {
        frames as u128 * MASTER_CYCLES_PER_FRAME as u128
    } else {
        // Default: run for 60 frames (1 second at 60fps)
        60 * MASTER_CYCLES_PER_FRAME as u128
    }
}

// =============================================================================
// Output Functions
// =============================================================================

/// Output results based on CLI args
fn output_results(emu: &Nes, args: &CliArgs) -> Result<(), String> {
    if let Some(ref range) = args.memory.read_cpu {
        output_cpu_memory(emu, range, args)?;
    } else if let Some(ref range) = args.memory.read_ppu {
        output_ppu_memory(emu, range, args)?;
    } else if args.memory.dump_oam {
        output_oam(emu, args)?;
    } else if args.memory.dump_nametables {
        output_nametables(emu, args)?;
    } else if !args.quiet {
        // Default behavior: dump test results area
        output_default_memory_dump(emu);
    }

    Ok(())
}

/// Output CPU memory dump
fn output_cpu_memory(emu: &Nes, range: &str, args: &CliArgs) -> Result<(), String> {
    let (start, end) = cli::parse_memory_range(range)?;
    let mem = &emu.get_memory_debug(Some(start..=end))[0];
    format_and_output_memory(mem, start, "cpu", args)
}

/// Output PPU memory dump
/// TODO: Implement proper PPU memory access when available
fn output_ppu_memory(emu: &Nes, range: &str, args: &CliArgs) -> Result<(), String> {
    let (start, end) = cli::parse_memory_range(range)?;
    // FIXME: This uses CPU memory accessor as placeholder. Need PPU-specific accessor.
    // PPU memory (pattern tables, nametables, palette) requires direct PPU bus access.
    let mem = &emu.get_memory_debug(Some(start..=end))[0];
    format_and_output_memory(mem, start, "ppu", args)
}

/// Output OAM dump
/// TODO: Implement proper OAM access when available
fn output_oam(_emu: &Nes, _args: &CliArgs) -> Result<(), String> {
    // FIXME: OAM is PPU internal memory (256 bytes), not accessible via CPU bus.
    // Need to add method to Nes/PPU to expose OAM data for debugging.
    Err("OAM dump not yet implemented - requires PPU-specific accessor".to_string())
}

/// Output nametables dump
/// TODO: Implement proper nametable access when available
fn output_nametables(_emu: &Nes, _args: &CliArgs) -> Result<(), String> {
    // FIXME: Nametables are at PPU address 0x2000-0x2FFF, not CPU address space.
    // CPU 0x2000-0x2007 are PPU registers, not nametable data.
    // Need to add method to Nes/PPU to expose VRAM for debugging.
    Err("Nametable dump not yet implemented - requires PPU-specific accessor".to_string())
}

/// Default memory dump (test results area)
fn output_default_memory_dump(emu: &Nes) {
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

/// Format and output memory data in the specified format
fn format_and_output_memory(
    mem: &[u8],
    start_addr: u16,
    mem_type: &str,
    args: &CliArgs,
) -> Result<(), String> {
    use cli::args::OutputFormat;

    match args.output.effective_format() {
        OutputFormat::Hex => output_hex(mem, start_addr),
        OutputFormat::Binary => output_binary(mem, args),
        OutputFormat::Json => output_json(mem, start_addr, mem_type),
        OutputFormat::Toml => output_toml(mem, start_addr, mem_type),
    }
}

fn output_hex(mem: &[u8], start_addr: u16) -> Result<(), String> {
    for (i, chunk) in mem.chunks(16).enumerate() {
        print!("{:04X}: ", start_addr as usize + i * 16);
        for byte in chunk {
            print!("{:02X} ", byte);
        }
        println!();
    }
    Ok(())
}

fn output_binary(mem: &[u8], args: &CliArgs) -> Result<(), String> {
    use std::io::Write;

    if let Some(ref path) = args.output.output {
        std::fs::write(path, mem).map_err(|e| e.to_string())
    } else {
        std::io::stdout()
            .write_all(mem)
            .map_err(|e| e.to_string())
    }
}

fn output_json(mem: &[u8], start_addr: u16, mem_type: &str) -> Result<(), String> {
    let end_addr = start_addr.saturating_add(mem.len() as u16).saturating_sub(1);
    let json = serde_json::json!({
        "memory_dump": {
            "type": mem_type,
            "start": format!("0x{:04X}", start_addr),
            "end": format!("0x{:04X}", end_addr),
            "data": mem.to_vec()
        }
    });
    println!(
        "{}",
        serde_json::to_string_pretty(&json).unwrap_or_default()
    );
    Ok(())
}

fn output_toml(mem: &[u8], start_addr: u16, mem_type: &str) -> Result<(), String> {
    let end_addr = start_addr.saturating_add(mem.len() as u16).saturating_sub(1);
    println!("[memory_dump]");
    println!("type = \"{}\"", mem_type);
    println!("start = \"0x{:04X}\"", start_addr);
    println!("end = \"0x{:04X}\"", end_addr);
    println!("data = {:?}", mem);
    Ok(())
}

// =============================================================================
// ROM Info
// =============================================================================

fn print_rom_info(rom_path: &PathBuf) -> Result<(), String> {
    let rom = RomFile::load(rom_path);

    println!("ROM Information:");
    println!("  File: {}", rom_path.display());
    if let Some(ref name) = rom.name {
        println!("  Name: {}", name);
    }
    println!("  Mapper: {}", rom.mapper_number);
    println!("  PRG ROM: {} KB", rom.prg_memory.prg_rom_size / 1024);
    println!("  CHR ROM: {} KB", rom.chr_memory.chr_rom_size / 1024);
    println!(
        "  PRG RAM: {} KB (battery-backed: {})",
        rom.prg_memory.prg_ram_size / 1024,
        if rom.is_battery_backed { "yes" } else { "no" }
    );
    println!(
        "  Mirroring: {}",
        if rom.hardwired_nametable_layout {
            "Vertical"
        } else {
            "Horizontal"
        }
    );
    println!(
        "  Checksum (SHA-256): {}",
        rom.data_checksum
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>()
    );

    Ok(())
}
