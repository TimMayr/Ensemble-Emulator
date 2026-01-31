//! Headless mode execution for the NES emulator CLI.
//!
//! This module contains the main headless execution logic, including:
//! - ROM info display
//! - Emulation execution with stop conditions
//! - Streaming video export
//! - Memory initialization
//! - Screenshot and video saving

use std::path::Path;
use std::time::Instant;

use crate::cli::{
    apply_memory_init, apply_memory_init_config, is_ffmpeg_available, CliArgs, ExecutionConfig,
    ExecutionEngine, ExecutionResult, MemoryInit, MemoryInitConfig, OutputWriter, SavestateConfig,
    StopReason, StreamingVideoEncoder, VideoFormat, VideoResolution,
};
use crate::emulation::messages::RgbColor;
use crate::emulation::nes::Nes;
use crate::emulation::rom::RomFile;

use super::memory_dumps::{create_cpu_dump, create_nametables_dump, create_oam_dump, create_palette_dump, create_ppu_dump};
use super::screenshot::{save_screenshot, save_single_screenshot};

// =============================================================================
// NES Constants
// =============================================================================

/// NES native width in pixels
pub const NES_WIDTH: u32 = 256;
/// NES native height in pixels  
pub const NES_HEIGHT: u32 = 240;

// =============================================================================
// Headless Execution
// =============================================================================

/// Run the emulator in headless mode.
///
/// This is the main entry point for headless execution. It handles:
/// - ROM info display (if --rom-info is specified)
/// - Emulation setup and execution
/// - Streaming video export (if video output is configured)
/// - Memory dumps and screenshots
pub fn run_headless(args: &CliArgs) -> Result<(), String> {
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

    // Apply memory initialization (after power on, before execution)
    apply_memory_initialization(engine.emulator_mut(), args)?;

    // Determine if we should use streaming video export
    // Streaming mode: frames are written directly to encoder during execution
    // This significantly reduces memory usage for long recordings
    let use_streaming = args.video.video_path.is_some();

    let result = if use_streaming {
        run_with_streaming_video(&mut engine, args)?
    } else {
        // Standard buffered mode
        engine.run()?
    };

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

    // Save screenshot (only if we have frames - in buffered mode)
    if !use_streaming {
        save_screenshot(&engine.frames, args)?;
    }

    // Video was already saved in streaming mode, skip in buffered mode if already done
    if !use_streaming {
        save_video(&engine.frames, args)?;
    }

    // Check for error stop reason
    match result.stop_reason {
        StopReason::Error(e) => Err(e),
        _ => Ok(()),
    }
}

// =============================================================================
// ROM Info
// =============================================================================

/// Handle --rom-info flag
fn handle_rom_info(args: &CliArgs) -> Result<(), String> {
    let rom_path = args
        .rom
        .rom
        .as_ref()
        .ok_or("--rom-info requires --rom to be specified")?;
    print_rom_info(rom_path)
}

/// Print detailed ROM information.
pub fn print_rom_info(rom_path: &Path) -> Result<(), String> {
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

// =============================================================================
// Streaming Video Export
// =============================================================================

/// Run emulation with streaming video export.
///
/// This mode writes frames directly to the encoder as they're generated,
/// instead of buffering all frames in memory. This significantly reduces
/// memory usage for long recordings.
fn run_with_streaming_video(
    engine: &mut ExecutionEngine,
    args: &CliArgs,
) -> Result<ExecutionResult, String> {
    let video_path = args.video.video_path.as_ref().unwrap();

    // Check if format requires FFmpeg and warn if not available
    if args.video.video_format == VideoFormat::Mp4 && !is_ffmpeg_available() {
        return Err("MP4 export requires FFmpeg to be installed. \
             Use --video-format png or --video-format ppm for self-contained export."
            .to_string());
    }

    // Parse video resolution
    let resolution = VideoResolution::parse(args.video.video_scale.as_ref().unwrap())
        .map_err(|e| format!("Invalid video scale: {}", e))?;

    let (dst_width, dst_height) = resolution.dimensions(NES_WIDTH, NES_HEIGHT);
    let fps = args.video.video_fps;

    // Print export info
    if !args.quiet {
        print_video_export_info(video_path, args, &resolution, dst_width, dst_height, true);
    }

    // Create streaming encoder
    let mut encoder = StreamingVideoEncoder::new(
        args.video.video_format,
        video_path,
        NES_WIDTH,
        NES_HEIGHT,
        &resolution,
        fps,
    )
    .map_err(|e| format!("Failed to create video encoder: {}", e))?;

    // Run with streaming video export
    let result = engine.run_with_video_encoder(&mut encoder)?;

    // Finalize encoder
    encoder
        .finish()
        .map_err(|e| format!("Failed to finalize video: {}", e))?;

    if !args.quiet {
        eprintln!("Exported {} frames successfully", encoder.frames_written());
    }

    // Handle screenshot in streaming mode (save last frame)
    if args.video.screenshot.is_some() {
        let last_frame = engine.emulator().get_pixel_buffer();
        save_single_screenshot(&last_frame, args)?;
    }

    Ok(result)
}

// =============================================================================
// Buffered Video Export
// =============================================================================

/// Save recorded frames to video file (buffered mode).
fn save_video(frames: &[Vec<RgbColor>], args: &CliArgs) -> Result<(), String> {
    if let Some(ref video_path) = args.video.video_path {
        // Check if format requires FFmpeg and warn if not available
        if args.video.video_format == VideoFormat::Mp4 && !is_ffmpeg_available() {
            return Err("MP4 export requires FFmpeg to be installed. \
                 Use --video-format png or --video-format ppm for self-contained export."
                .to_string());
        }

        if frames.is_empty() {
            eprintln!("Warning: No frames to export");
            return Ok(());
        }

        // Parse video resolution
        let resolution = VideoResolution::parse(&args.video.video_scale.clone().unwrap())
            .map_err(|e| format!("Invalid video scale: {}", e))?;

        let (dst_width, dst_height) = resolution.dimensions(NES_WIDTH, NES_HEIGHT);
        let fps = args.video.video_fps;

        if !args.quiet {
            print_video_export_info_with_frames(
                frames.len(),
                video_path,
                args,
                &resolution,
                dst_width,
                dst_height,
            );
        }

        // Use streaming encoder for proper scaling support
        let mut encoder = StreamingVideoEncoder::new(
            args.video.video_format,
            video_path,
            NES_WIDTH,
            NES_HEIGHT,
            &resolution,
            fps,
        )
        .map_err(|e| format!("Failed to create video encoder: {}", e))?;

        for frame in frames {
            encoder.write_frame(frame).map_err(|e| e.to_string())?;
        }

        encoder.finish().map_err(|e| e.to_string())?;

        if !args.quiet {
            eprintln!("Exported {} frames successfully", encoder.frames_written());
        }
    }

    Ok(())
}

// =============================================================================
// Video Export Helpers
// =============================================================================

/// Print video export information (streaming mode).
fn print_video_export_info(
    video_path: &Path,
    args: &CliArgs,
    resolution: &VideoResolution,
    dst_width: u32,
    dst_height: u32,
    streaming: bool,
) {
    let mode_str = if streaming { " [streaming mode]" } else { "" };
    
    if *resolution == VideoResolution::Native {
        eprintln!(
            "Exporting to {} as {:?} ({}x{}){}...",
            video_path.display(),
            args.video.video_format,
            NES_WIDTH,
            NES_HEIGHT,
            mode_str
        );
    } else if matches!(args.video.video_format, VideoFormat::Mp4) {
        eprintln!(
            "Exporting to {} as {:?} ({}x{} → {}x{} via FFmpeg nearest-neighbor){}...",
            video_path.display(),
            args.video.video_format,
            NES_WIDTH,
            NES_HEIGHT,
            dst_width,
            dst_height,
            mode_str
        );
    } else {
        eprintln!(
            "Exporting to {} as {:?} ({}x{}){}...",
            video_path.display(),
            args.video.video_format,
            NES_WIDTH,
            NES_HEIGHT,
            if streaming {
                " [streaming mode, scaling only supported for MP4]"
            } else {
                ""
            }
        );
    }
}

/// Print video export information with frame count (buffered mode).
fn print_video_export_info_with_frames(
    frame_count: usize,
    video_path: &Path,
    args: &CliArgs,
    resolution: &VideoResolution,
    dst_width: u32,
    dst_height: u32,
) {
    if *resolution == VideoResolution::Native {
        eprintln!(
            "Exporting {} frames to {} as {:?} ({}x{})...",
            frame_count,
            video_path.display(),
            args.video.video_format,
            NES_WIDTH,
            NES_HEIGHT
        );
    } else if matches!(args.video.video_format, VideoFormat::Mp4) {
        eprintln!(
            "Exporting {} frames to {} as {:?} ({}x{} → {}x{} via FFmpeg nearest-neighbor)...",
            frame_count,
            video_path.display(),
            args.video.video_format,
            NES_WIDTH,
            NES_HEIGHT,
            dst_width,
            dst_height
        );
    } else {
        eprintln!(
            "Exporting {} frames to {} as {:?} ({}x{})...",
            frame_count,
            video_path.display(),
            args.video.video_format,
            NES_WIDTH,
            NES_HEIGHT
        );
    }
}

// =============================================================================
// Memory Initialization
// =============================================================================

/// Apply memory initialization based on CLI args.
pub fn apply_memory_initialization(emu: &mut Nes, args: &CliArgs) -> Result<(), String> {
    // Parse CPU memory init commands
    let cpu_inits: Vec<MemoryInit> = args
        .memory
        .init_cpu
        .iter()
        .map(|s| MemoryInit::parse(s))
        .collect::<Result<Vec<_>, _>>()?;

    // Parse PPU memory init commands
    let ppu_inits: Vec<MemoryInit> = args
        .memory
        .init_ppu
        .iter()
        .map(|s| MemoryInit::parse(s))
        .collect::<Result<Vec<_>, _>>()?;

    // Parse OAM memory init commands
    let oam_inits: Vec<MemoryInit> = args
        .memory
        .init_oam
        .iter()
        .map(|s| MemoryInit::parse(s))
        .collect::<Result<Vec<_>, _>>()?;

    // Apply direct init commands
    if !cpu_inits.is_empty() || !ppu_inits.is_empty() || !oam_inits.is_empty() {
        apply_memory_init(emu, &cpu_inits, &ppu_inits, &oam_inits);
        if args.verbose {
            eprintln!(
                "Applied memory init: {} CPU, {} PPU, {} OAM operations",
                cpu_inits.len(),
                ppu_inits.len(),
                oam_inits.len()
            );
        }
    }

    // Load init config from file if specified
    if let Some(ref init_file) = args.memory.init_file {
        let config = MemoryInitConfig::load_from_file(init_file)?;
        apply_memory_init_config(emu, &config);
        if args.verbose {
            eprintln!(
                "Applied memory init from file: {} CPU, {} PPU, {} OAM addresses",
                config.cpu.len(),
                config.ppu.len(),
                config.oam.len()
            );
        }
    }

    Ok(())
}

// =============================================================================
// Memory Dump Output
// =============================================================================

/// Output results based on CLI args using the output module abstraction.
pub fn output_results(emu: &Nes, args: &CliArgs) -> Result<(), String> {
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

    if args.memory.dump_palette {
        let dump = create_palette_dump(emu);
        writer.write(&dump)?;
    }

    Ok(())
}
