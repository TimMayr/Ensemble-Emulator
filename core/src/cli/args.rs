//! CLI argument definitions using clap derive macros.
//!
//! This module defines all command-line arguments for the NES emulator CLI.
//! Arguments are organized into logical groups matching the documentation.

use std::path::PathBuf;

use clap::{Args, Parser, ValueEnum, value_parser};

use crate::cli::config::DEFAULT_VIDEO_FPS;

/// NES Emulator CLI - A cycle-accurate NES emulator with comprehensive CLI support
#[derive(Parser, Debug, Clone, Default)]
#[command(name = "nes_main")]
#[command(version, about, long_about = None)]
#[command(after_help = "For more information, see docs/CLI_INTERFACE.md")]
pub struct CliArgs {
    /// Run in headless mode (no GUI)
    #[arg(short = 'H', long, default_value_t = false)]
    pub headless: bool,

    /// Suppress non-error output
    #[arg(short, long, default_value_t = false)]
    pub quiet: bool,

    /// Enable verbose output
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,

    /// Load configuration from TOML file
    #[arg(short, long, value_parser = value_parser!(PathBuf), value_hint = clap::ValueHint::FilePath)]
    pub config: Option<PathBuf>,

    #[command(flatten)]
    pub rom: RomArgs,

    #[command(flatten)]
    pub savestate: SavestateArgs,

    #[command(flatten)]
    pub memory: MemoryArgs,

    #[command(flatten)]
    pub power: PowerArgs,

    #[command(flatten)]
    pub palette: PaletteArgs,

    #[command(flatten)]
    pub video: VideoArgs,

    #[command(flatten)]
    pub execution: ExecutionArgs,

    #[command(flatten)]
    pub output: OutputArgs,
}

/// ROM loading arguments
#[derive(Args, Debug, Clone, Default)]
pub struct RomArgs {
    /// Path to ROM file
    #[arg(short, long, value_parser = value_parser!(PathBuf), value_hint = clap::ValueHint::FilePath)]
    pub rom: Option<PathBuf>,

    /// Print ROM information and exit
    #[arg(long, default_value_t = false)]
    pub rom_info: bool,
}

/// Savestate operation arguments
#[derive(Args, Debug, Clone, Default)]
pub struct SavestateArgs {
    /// Load savestate from file
    #[arg(short = 'l', long, value_parser = value_parser!(PathBuf), value_hint = clap::ValueHint::FilePath)]
    pub load_state: Option<PathBuf>,

    /// Save state to file on exit
    #[arg(short = 's', long, value_parser = value_parser!(PathBuf), value_hint = clap::ValueHint::FilePath)]
    pub save_state: Option<PathBuf>,

    /// Read savestate from stdin
    #[arg(long, default_value_t = false)]
    pub state_stdin: bool,

    /// Write savestate to stdout on exit
    #[arg(long, default_value_t = false)]
    pub state_stdout: bool,

    /// When to save state (exit, stop, cycle:N, pc:ADDR, frame:N)
    #[arg(long)]
    pub save_state_on: Option<String>,
}

/// Memory operation arguments
#[derive(Args, Debug, Clone, Default)]
pub struct MemoryArgs {
    /// Read CPU memory range (e.g., 0x0000-0x07FF or 0x6000:0x100)
    #[arg(long)]
    pub read_cpu: Option<String>,

    /// Read PPU memory range (e.g., 0x0000-0x1FFF)
    #[arg(long)]
    pub read_ppu: Option<String>,

    /// Dump OAM (sprite) memory
    #[arg(long, default_value_t = false)]
    pub dump_oam: bool,

    /// Dump nametables
    #[arg(long, default_value_t = false)]
    pub dump_nametables: bool,

    /// Initialize CPU memory (ADDR=VALUE or ADDR=V1,V2,...)
    #[arg(long, action = clap::ArgAction::Append)]
    pub init_cpu: Vec<String>,

    /// Initialize PPU memory (ADDR=VALUE or ADDR=V1,V2,...)
    #[arg(long, action = clap::ArgAction::Append)]
    pub init_ppu: Vec<String>,

    /// Initialize OAM memory (ADDR=VALUE or ADDR=V1,V2,...)
    #[arg(long, action = clap::ArgAction::Append)]
    pub init_oam: Vec<String>,

    /// Load init values from file (JSON/TOML/binary)
    #[arg(long, value_parser = value_parser!(PathBuf), value_hint = clap::ValueHint::FilePath)]
    pub init_file: Option<PathBuf>,
}

/// Power control arguments
#[derive(Args, Debug, Clone, Default)]
pub struct PowerArgs {
    /// Don't auto-power on after ROM load
    #[arg(long, default_value_t = false)]
    pub no_power: bool,

    /// Reset after loading
    #[arg(long, default_value_t = false)]
    pub reset: bool,
}

/// Palette configuration arguments
#[derive(Args, Debug, Clone, Default)]
pub struct PaletteArgs {
    /// Path to .pal RGB palette file
    #[arg(short, long, value_parser = value_parser!(PathBuf), value_hint = clap::ValueHint::FilePath)]
    pub palette: Option<PathBuf>,

    /// Use built-in palette by name (2C02G, composite)
    #[arg(long)]
    pub palette_builtin: Option<BuiltinPalette>,
}

/// Built-in palette options
#[derive(Debug, Clone, Copy, ValueEnum, Default)]
pub enum BuiltinPalette {
    /// Standard 2C02G palette (default)
    #[default]
    #[value(name = "2C02G")]
    Nes2C02G,
    /// NTSC composite simulation
    Composite,
}

/// Video/screenshot export arguments
#[derive(Args, Debug, Clone, Default)]
pub struct VideoArgs {
    /// Save screenshot on exit
    #[arg(long, value_parser = value_parser!(PathBuf), value_hint = clap::ValueHint::FilePath)]
    pub screenshot: Option<PathBuf>,

    /// When to capture screenshot (exit, stop, cycle:N, pc:ADDR, frame:N)
    #[arg(long)]
    pub screenshot_on: Option<String>,

    /// Record video to file
    #[arg(long, value_parser = value_parser!(PathBuf), value_hint = clap::ValueHint::FilePath)]
    pub video_path: Option<PathBuf>,

    /// Video output format
    #[arg(long, default_value = "raw")]
    pub video_format: VideoFormat,

    /// Video frame rate
    #[arg(long, default_value_t = DEFAULT_VIDEO_FPS)]
    pub video_fps: f64,

    /// Video output resolution (native, 2x, 3x, 4x, 720p, 1080p, 4k, or WIDTHxHEIGHT)
    #[arg(long)]
    pub video_scale: Option<String>,

    /// Export nametable visualization as screenshot
    #[arg(long, value_parser = value_parser!(PathBuf), value_hint = clap::ValueHint::FilePath)]
    pub export_nametables: Option<PathBuf>,

    /// Export nametable visualization as video
    #[arg(long, value_parser = value_parser!(PathBuf), value_hint = clap::ValueHint::FilePath)]
    pub export_nametables_video: Option<PathBuf>,

    /// Export pattern table visualization as screenshot
    #[arg(long, value_parser = value_parser!(PathBuf), value_hint = clap::ValueHint::FilePath)]
    pub export_pattern_tables: Option<PathBuf>,

    /// Export pattern table visualization as video
    #[arg(long, value_parser = value_parser!(PathBuf), value_hint = clap::ValueHint::FilePath)]
    pub export_pattern_tables_video: Option<PathBuf>,

    /// Export sprite visualization as screenshot
    #[arg(long, value_parser = value_parser!(PathBuf), value_hint = clap::ValueHint::FilePath)]
    pub export_sprites: Option<PathBuf>,

    /// Export sprite visualization as video
    #[arg(long, value_parser = value_parser!(PathBuf), value_hint = clap::ValueHint::FilePath)]
    pub export_sprites_video: Option<PathBuf>,
}

/// Video format options
#[derive(Debug, Clone, Copy, ValueEnum, Default, PartialEq, Eq)]
pub enum VideoFormat {
    /// Raw RGBA frames (for piping to FFmpeg)
    #[default]
    Raw,
    /// PPM image sequence
    Ppm,
    /// PNG image sequence
    Png,
    /// MP4 video
    Mp4,
}

/// Execution control arguments
#[derive(Args, Debug, Clone, Default)]
pub struct ExecutionArgs {
    /// Run for N master cycles
    #[arg(long)]
    pub cycles: Option<u128>,

    /// Run for N frames
    #[arg(short, long)]
    pub frames: Option<u64>,

    /// Run until PC reaches address (hex, e.g., 0x8500)
    #[arg(long, value_parser = parse_hex_u16)]
    pub until_pc: Option<u16>,

    /// Run until specific opcode executes (hex, e.g., 0x02)
    #[arg(long, value_parser = parse_hex_u8)]
    pub until_opcode: Option<u8>,

    /// Run until memory condition (e.g., 0x6000==0x80)
    #[arg(long)]
    pub until_mem: Option<Vec<String>>,

    /// Run until HLT (illegal halt) instruction
    #[arg(long, default_value_t = false)]
    pub until_hlt: bool,

    /// Single-step mode
    #[arg(long, default_value_t = false)]
    pub step: bool,

    /// Enable instruction trace to file
    #[arg(long, value_parser = value_parser!(PathBuf), value_hint = clap::ValueHint::FilePath)]
    pub trace: Option<PathBuf>,

    /// Set breakpoint at address (can be specified multiple times)
    #[arg(long, value_parser = parse_hex_u16, action = clap::ArgAction::Append)]
    pub breakpoint: Vec<u16>,
}

/// Output control arguments
#[derive(Args, Debug, Clone, Default)]
pub struct OutputArgs {
    /// Output file for memory dumps
    #[arg(short, long, value_parser = value_parser!(PathBuf), value_hint = clap::ValueHint::FilePath)]
    pub output: Option<PathBuf>,

    /// Output format (hex, JSON, toml, binary)
    #[arg(long, default_value = "hex")]
    pub output_format: OutputFormat,

    /// Output in JSON format (shorthand for --output-format JSON)
    #[arg(long, default_value_t = false)]
    pub json: bool,

    /// Output in TOML format (shorthand for --output-format toml)
    #[arg(long, default_value_t = false)]
    pub toml: bool,

    /// Output in binary format (shorthand for --output-format binary)
    #[arg(long, default_value_t = false)]
    pub binary: bool,
}

/// Output format options
#[derive(Debug, Clone, Copy, ValueEnum, Default, PartialEq, Eq)]
pub enum OutputFormat {
    /// Hexadecimal dump
    #[default]
    Hex,
    /// JSON format
    Json,
    /// TOML format
    Toml,
    /// Raw binary
    Binary,
}

impl std::str::FromStr for OutputFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "hex" => Ok(OutputFormat::Hex),
            "json" => Ok(OutputFormat::Json),
            "toml" => Ok(OutputFormat::Toml),
            "binary" => Ok(OutputFormat::Binary),
            _ => Err(format!(
                "Unknown output format: '{}'. Valid options: hex, json, toml, binary",
                s
            )),
        }
    }
}

impl std::str::FromStr for VideoFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "raw" => Ok(VideoFormat::Raw),
            "ppm" => Ok(VideoFormat::Ppm),
            "png" => Ok(VideoFormat::Png),
            "mp4" => Ok(VideoFormat::Mp4),
            _ => Err(format!(
                "Unknown video format: '{}'. Valid options: raw, ppm, png, mp4",
                s
            )),
        }
    }
}

impl std::str::FromStr for BuiltinPalette {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "2c02g" => Ok(BuiltinPalette::Nes2C02G),
            "composite" => Ok(BuiltinPalette::Composite),
            _ => Err(format!(
                "Unknown palette: '{}'. Valid options: 2C02G, composite",
                s
            )),
        }
    }
}

/// Parse a hexadecimal u16 value (with or without 0x prefix)
pub fn parse_hex_u16(s: &str) -> Result<u16, String> {
    let s = s
        .strip_prefix("0x")
        .or_else(|| s.strip_prefix("0X"))
        .unwrap_or(s);
    u16::from_str_radix(s, 16).map_err(|e| format!("Invalid hex value '{}': {}", s, e))
}

/// Parse a hexadecimal u8 value (with or without 0x prefix)
pub fn parse_hex_u8(s: &str) -> Result<u8, String> {
    let s = s
        .strip_prefix("0x")
        .or_else(|| s.strip_prefix("0X"))
        .unwrap_or(s);
    u8::from_str_radix(s, 16).map_err(|e| format!("Invalid hex value '{}': {}", s, e))
}

impl OutputArgs {
    /// Get the effective output format, considering shorthand flags
    pub fn effective_format(&self) -> OutputFormat {
        if self.json {
            OutputFormat::Json
        } else if self.toml {
            OutputFormat::Toml
        } else if self.binary {
            OutputFormat::Binary
        } else {
            self.output_format
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hex_u16() {
        assert_eq!(parse_hex_u16("0x8500").unwrap(), 0x8500);
        assert_eq!(parse_hex_u16("0X8500").unwrap(), 0x8500);
        assert_eq!(parse_hex_u16("8500").unwrap(), 0x8500);
        assert_eq!(parse_hex_u16("FFFF").unwrap(), 0xFFFF);
        assert_eq!(parse_hex_u16("0x0").unwrap(), 0);
    }

    #[test]
    fn test_parse_hex_u8() {
        assert_eq!(parse_hex_u8("0x02").unwrap(), 0x02);
        assert_eq!(parse_hex_u8("FF").unwrap(), 0xFF);
        assert_eq!(parse_hex_u8("0").unwrap(), 0);
    }

    #[test]
    fn test_verify_cli() {
        use clap::CommandFactory;
        CliArgs::command().debug_assert();
    }
}
