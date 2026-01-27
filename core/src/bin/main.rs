//! NES Emulator CLI entry point.
//!
//! This is the main entry point for the NES emulator. It supports both
//! GUI mode (using egui) and headless mode for programmatic control.
//!
//! See `docs/CLI_INTERFACE.md` for full CLI documentation.

use std::path::Path;
use std::process::ExitCode;
use std::time::Instant;
use image::{ImageBuffer, Rgba};
use nes_core::cli::{
    self, CliArgs, ExecutionConfig, ExecutionEngine, MemoryDump, MemoryType, OutputWriter,
    SavestateConfig, StopReason,
};
use nes_core::cli::args::VideoFormat;
use nes_core::emulation::nes::Nes;
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
                eprintln!("Emulator finished execution");
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

    let start = Instant::now();

    // Build execution and savestate configs from CLI args
    let exec_config = ExecutionConfig::from_cli_args(args);
    let savestate_config = SavestateConfig::from_cli_args(args);

    // Create and configure the execution engine
    let mut engine = ExecutionEngine::new()
        .with_config(exec_config)
        .with_savestate_config(savestate_config);

    // Load ROM
    if let Some(ref rom_path) = args.rom.rom {
        engine.load_rom(rom_path)?;
    }

    // Load savestate if configured
    engine.load_savestate()?;

    // Power on (unless --no-power is specified)
    if !args.power.no_power {
        engine.power_on();
    }

    // Handle reset
    if args.power.reset {
        engine.reset();
    }

    // Run emulation
    let result = engine.run()?;

    // Print execution summary if verbose
    if args.verbose {
        eprintln!("Execution time: {:?}", start.elapsed());
        eprintln!("Total cycles: {}", result.total_cycles);
        eprintln!("Total frames: {}", result.total_frames);
        eprintln!("Stop reason: {:?}", result.stop_reason);
    }

    // Save savestate if configured
    engine.save_savestate()?;

    // Output memory dumps
    output_results(engine.emulator(), args)?;
    save_video(engine.frames, args)?

    // Check for error stop reason
    match result.stop_reason {
        StopReason::Error(e) => Err(e),
        _ => Ok(()),
    }
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

// =============================================================================
// Output Functions (using the output module abstraction)
// =============================================================================

/// Output results based on CLI args using the output module abstraction.
fn output_results(emu: &Nes, args: &CliArgs) -> Result<(), String> {
    // Reset the output writer state for this run
    OutputWriter::reset();

    // Create the output writer with configured format and destination
    let writer = OutputWriter::new(args.output.output.clone(), args.output.effective_format());

    // Process each requested memory dump
    if let Some(ref range) = args.memory.read_cpu {
        let dump = create_cpu_dump(emu, range)?;
        writer.write(&dump)?;
    }

    if let Some(ref range) = args.memory.read_ppu {
        let dump = create_ppu_dump(emu, range)?;
        writer.write(&dump)?;
    }

    if args.memory.dump_oam {
        let dump = create_oam_dump(emu);
        writer.write(&dump)?;
    }

    if args.memory.dump_nametables {
        let dump = create_nametables_dump(emu);
        writer.write(&dump)?;
    }

    Ok(())
}

fn save_video(frames: &Vec<Vec<u32>>, args: &CliArgs) -> Result<(), String> {
    if let Some(video_path) = args.video.video {
        match args.video.video_format {
            VideoFormat::Raw => todo!(),
            VideoFormat::Ppm => {}
            VideoFormat::Png => {}
            VideoFormat::Mp4 => {}
        }
    }
}

fn get_image(data: &Vec<u32>)-> Result<(), image::ImageError> {

}

// =============================================================================
// Memory Dump Creation
// =============================================================================

/// Create a CPU memory dump from the emulator
fn create_cpu_dump(emu: &Nes, range: &str) -> Result<MemoryDump, String> {
    let (start, end) = cli::parse_memory_range(range)?;
    let mem = &emu.get_memory_debug(Some(start..=end))[0];
    Ok(MemoryDump::new(MemoryType::Cpu, start, mem.to_vec()))
}

/// Create a PPU memory dump from the emulator
fn create_ppu_dump(emu: &Nes, range: &str) -> Result<MemoryDump, String> {
    let (start, end) = cli::parse_memory_range(range)?;
    let mem = &emu.get_memory_debug(Some(start..=end))[1];
    Ok(MemoryDump::new(MemoryType::Ppu, start, mem.to_vec()))
}

/// Create an OAM memory dump from the emulator
fn create_oam_dump(emu: &Nes) -> MemoryDump {
    let mem = emu.ppu.borrow().oam.get_memory_debug(None);
    MemoryDump::oam(mem)
}

/// Create a nametables memory dump from the emulator
fn create_nametables_dump(emu: &Nes) -> MemoryDump {
    let mem = emu
        .ppu
        .borrow()
        .memory
        .get_memory_debug(Some(0x2000..=0x2FFF));
    MemoryDump::nametables(mem)
}

// =============================================================================
// ROM Info
// =============================================================================

fn print_rom_info(rom_path: &Path) -> Result<(), String> {
    let path_str = rom_path.to_string_lossy().to_string();
    let rom = RomFile::load(&path_str);

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
