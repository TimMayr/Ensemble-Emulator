//! CLI module for the NES emulator.
//!
//! This module provides a comprehensive command-line interface for programmatic
//! control of the emulator. It is designed with extensibility in mind:
//!
//! ## Architecture
//!
//! The CLI is organized into several sub-modules:
//!
//! - **args** - Command-line argument definitions using clap derive macros
//! - **config** - TOML configuration file support
//! - **execution** - Execution engine with generic stop conditions
//! - **output** - Extensible output formatting system
//!
//! ## Extensibility
//!
//! ### Adding a new output format
//!
//! 1. Add a variant to `OutputFormat` enum in `args.rs`
//! 2. Implement `MemoryFormatter` trait for the new format in `output.rs`
//! 3. Register it in `OutputFormat::formatter()`
//!
//! ### Adding a new stop condition
//!
//! 1. Add a variant to `StopCondition` enum in `execution.rs`
//! 2. Add a corresponding `StopReason` variant
//! 3. Implement the check in `ExecutionConfig::check_conditions()`
//! 4. Add CLI argument in `args.rs` and builder method in `ExecutionConfig`
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
pub mod execution;
pub mod output;

pub use args::{CliArgs, OutputFormat, parse_hex_u16};
use clap::Parser;
pub use config::ConfigFile;
pub use execution::{
    ExecutionConfig, ExecutionEngine, ExecutionResult, SavestateConfig, SavestateDestination,
    SavestateSource, StopCondition, StopReason,
};
pub use output::{MemoryDump, MemoryFormatter, MemoryType, OutputWriter};

// =============================================================================
// Argument Parsing
// =============================================================================

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

// =============================================================================
// Argument Validation
// =============================================================================

/// Validate CLI arguments for consistency and completeness.
///
/// Returns an error message if the arguments are invalid.
pub fn validate_args(args: &CliArgs) -> Result<(), String> {
    validate_headless_requirements(args)?;
    validate_savestate_options(args)?;
    validate_output_format(args)?;
    Ok(())
}

fn validate_headless_requirements(args: &CliArgs) -> Result<(), String> {
    if args.headless
        && args.rom.rom.is_none()
        && args.savestate.load_state.is_none()
        && !args.savestate.state_stdin
    {
        return Err(
            "Headless mode requires a ROM (--rom), savestate (--load-state), or --state-stdin"
                .to_string(),
        );
    }
    Ok(())
}

fn validate_savestate_options(args: &CliArgs) -> Result<(), String> {
    // Can't use both state-stdin and load-state
    if args.savestate.state_stdin && args.savestate.load_state.is_some() {
        return Err("Cannot use both --state-stdin and --load-state".to_string());
    }

    // Can't use both state-stdout and save-state
    if args.savestate.state_stdout && args.savestate.save_state.is_some() {
        return Err("Cannot use both --state-stdout and --save-state".to_string());
    }

    Ok(())
}

fn validate_output_format(args: &CliArgs) -> Result<(), String> {
    let format_flags = [args.output.json, args.output.toml, args.output.binary]
        .iter()
        .filter(|&&x| x)
        .count();

    if format_flags > 1 {
        return Err(
            "Cannot specify multiple output format flags (--json, --toml, --binary)".to_string(),
        );
    }

    Ok(())
}

// =============================================================================
// Memory Range Parsing
// =============================================================================

/// Parse a memory range string in format `START-END` or `START:LENGTH`.
///
/// Both `START` and `END`/`LENGTH` should be hexadecimal values (with or without 0x prefix).
///
/// # Examples
///
/// ```
/// use nes_core::cli::parse_memory_range;
///
/// assert_eq!(
///     parse_memory_range("0x0000-0x07FF").unwrap(),
///     (0x0000, 0x07FF)
/// );
/// assert_eq!(parse_memory_range("6000:100").unwrap(), (0x6000, 0x60FF));
/// ```
pub fn parse_memory_range(range: &str) -> Result<(u16, u16), String> {
    if let Some((start_str, end_str)) = range.split_once('-') {
        let start = parse_hex_u16(start_str)?;
        let end = parse_hex_u16(end_str)?;
        Ok((start, end))
    } else if let Some((start_str, len_str)) = range.split_once(':') {
        let start = parse_hex_u16(start_str)?;
        let len = parse_hex_u16(len_str)?;
        Ok((start, start.saturating_add(len).saturating_sub(1)))
    } else {
        Err(format!(
            "Invalid memory range format: '{}'. Use START-END or START:LENGTH (e.g., 0x0000-0x07FF or 0x6000:0x100)",
            range
        ))
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_memory_range_dash() {
        assert_eq!(
            parse_memory_range("0x0000-0x07FF").unwrap(),
            (0x0000, 0x07FF)
        );
        assert_eq!(parse_memory_range("6000-7FFF").unwrap(), (0x6000, 0x7FFF));
    }

    #[test]
    fn test_parse_memory_range_colon() {
        assert_eq!(
            parse_memory_range("0x6000:0x100").unwrap(),
            (0x6000, 0x60FF)
        );
        assert_eq!(parse_memory_range("0000:10").unwrap(), (0x0000, 0x000F));
    }

    #[test]
    fn test_parse_memory_range_invalid() {
        assert!(parse_memory_range("invalid").is_err());
        assert!(parse_memory_range("").is_err());
    }
}
