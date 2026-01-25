//! CLI module for the NES emulator.
//!
//! This module provides a comprehensive command-line interface for programmatic
//! control of the emulator. It supports:
//!
//! - ROM loading and information display
//! - Savestate management (file and pipe-based)
//! - Memory read/write operations
//! - Power control (on/off, reset)
//! - Palette configuration
//! - Video/screenshot export
//! - Execution control (cycles, frames, breakpoints)
//! - TOML configuration file support
//!
//! # Example
//!
//! ```bash
//! # Basic headless run
//! nes_main --headless --rom game.nes --frames 100
//!
//! # With config file
//! nes_main --config run.toml
//!
//! # Pipe-based savestate workflow
//! nes_main -H --rom game.nes --frames 100 --state-stdout | \
//! nes_main -H --rom game.nes --state-stdin --frames 50 --save-state final.sav
//! ```
//!
//! See `docs/CLI_INTERFACE.md` for full documentation.

pub mod args;
pub mod config;

pub use args::CliArgs;
pub use config::ConfigFile;

use clap::Parser;

/// Parse CLI arguments and optionally merge with a config file.
///
/// This function:
/// 1. Parses command-line arguments using clap
/// 2. If `--config` is specified, loads and merges the TOML config file
/// 3. Returns the final merged configuration
///
/// CLI arguments always take precedence over config file values.
pub fn parse_args() -> Result<CliArgs, Box<dyn std::error::Error>> {
    let mut args = CliArgs::parse();

    // If a config file is specified, load and merge it
    if let Some(ref config_path) = args.config {
        let config = ConfigFile::load(config_path)?;
        config.merge_with_cli(&mut args);
    }

    Ok(args)
}

/// Validate CLI arguments for consistency and completeness.
///
/// Returns an error message if the arguments are invalid.
pub fn validate_args(args: &CliArgs) -> Result<(), String> {
    // Headless mode requires a ROM or savestate
    if args.headless && args.rom.rom.is_none() && args.savestate.load_state.is_none() && !args.savestate.state_stdin {
        return Err(
            "Headless mode requires a ROM (--rom), savestate (--load-state), or --state-stdin"
                .to_string(),
        );
    }

    // Can't use both state-stdin and load-state
    if args.savestate.state_stdin && args.savestate.load_state.is_some() {
        return Err("Cannot use both --state-stdin and --load-state".to_string());
    }

    // Can't use both state-stdout and save-state
    if args.savestate.state_stdout && args.savestate.save_state.is_some() {
        return Err("Cannot use both --state-stdout and --save-state".to_string());
    }

    // If no execution limit is specified in headless mode, warn
    // (but don't error - some use cases like --rom-info don't need execution)
    if args.headless
        && args.execution.cycles.is_none()
        && args.execution.frames.is_none()
        && args.execution.until_pc.is_none()
        && args.execution.until_mem.is_none()
        && !args.execution.until_hlt
        && !args.rom.rom_info
    {
        // This is a warning case - execution will run indefinitely
        // For now, we'll allow it but the engine should handle this
    }

    // Multiple output format flags
    let format_flags = [args.output.json, args.output.toml, args.output.binary]
        .iter()
        .filter(|&&x| x)
        .count();
    if format_flags > 1 {
        return Err("Cannot specify multiple output format flags (--json, --toml, --binary)".to_string());
    }

    Ok(())
}
