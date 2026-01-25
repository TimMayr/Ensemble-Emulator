//! Output formatting system for CLI memory dumps.
//!
//! This module provides an extensible output format system using traits.
//! Adding a new output format requires:
//! 1. Add a variant to `OutputFormat` enum
//! 2. Implement the `MemoryFormatter` trait for the new format
//! 3. Register it in `OutputFormat::formatter()`
//!
//! # Example: Adding a new format
//!
//! ```rust,ignore
//! // 1. Add enum variant in args.rs
//! pub enum OutputFormat {
//!     Hex,
//!     Json,
//!     Toml,
//!     Binary,
//!     Xml,  // New format
//! }
//!
//! // 2. Implement the formatter
//! pub struct XmlFormatter;
//!
//! impl MemoryFormatter for XmlFormatter {
//!     fn format(&self, dump: &MemoryDump) -> Result<Vec<u8>, String> {
//!         // ... format as XML ...
//!     }
//!
//!     fn file_extension(&self) -> &'static str {
//!         "xml"
//!     }
//! }
//!
//! // 3. Register in OutputFormat::formatter()
//! OutputFormat::Xml => Box::new(XmlFormatter),
//! ```

use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};

use serde::Serialize;

use super::args::OutputFormat;

// =============================================================================
// Memory Dump Data Structure
// =============================================================================

/// A memory dump with metadata.
///
/// This is the common data structure passed to all formatters.
#[derive(Debug, Clone)]
pub struct MemoryDump {
    /// Type of memory (cpu, ppu, oam, nametables)
    pub mem_type: MemoryType,
    /// Start address
    pub start_addr: u16,
    /// End address
    pub end_addr: u16,
    /// Raw memory bytes
    pub data: Vec<u8>,
}

/// Type of memory being dumped
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryType {
    /// CPU address space
    Cpu,
    /// PPU address space
    Ppu,
    /// OAM (sprite) memory
    Oam,
    /// Nametables
    Nametables,
}

impl MemoryType {
    /// Get string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            MemoryType::Cpu => "cpu",
            MemoryType::Ppu => "ppu",
            MemoryType::Oam => "oam",
            MemoryType::Nametables => "nametables",
        }
    }
}

impl std::fmt::Display for MemoryType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl MemoryDump {
    /// Create a new memory dump
    pub fn new(mem_type: MemoryType, start_addr: u16, data: Vec<u8>) -> Self {
        let end_addr = if data.is_empty() {
            start_addr
        } else {
            start_addr.saturating_add((data.len() - 1) as u16)
        };
        Self {
            mem_type,
            start_addr,
            end_addr,
            data,
        }
    }

    /// Create CPU memory dump
    pub fn cpu(start_addr: u16, data: Vec<u8>) -> Self {
        Self::new(MemoryType::Cpu, start_addr, data)
    }

    /// Create PPU memory dump
    pub fn ppu(start_addr: u16, data: Vec<u8>) -> Self {
        Self::new(MemoryType::Ppu, start_addr, data)
    }

    /// Create OAM memory dump
    pub fn oam(data: Vec<u8>) -> Self {
        Self::new(MemoryType::Oam, 0, data)
    }

    /// Create nametables memory dump
    pub fn nametables(data: Vec<u8>) -> Self {
        Self::new(MemoryType::Nametables, 0x2000, data)
    }
}

// =============================================================================
// Formatter Trait
// =============================================================================

/// Trait for memory dump formatters.
///
/// Implement this trait to add a new output format.
pub trait MemoryFormatter: Send + Sync {
    /// Format a memory dump into bytes ready to be written.
    fn format(&self, dump: &MemoryDump) -> Result<Vec<u8>, String>;

    /// Get the file extension for this format (without leading dot).
    fn file_extension(&self) -> &'static str;

    /// Whether this format is human-readable (affects stdout display).
    fn is_text(&self) -> bool {
        true
    }
}

// =============================================================================
// Built-in Formatters
// =============================================================================

/// Hexadecimal dump formatter (traditional hex dump format)
pub struct HexFormatter;

impl MemoryFormatter for HexFormatter {
    fn format(&self, dump: &MemoryDump) -> Result<Vec<u8>, String> {
        let mut output = String::new();
        for (i, chunk) in dump.data.chunks(16).enumerate() {
            let line = format!(
                "{:04X}: {}\n",
                dump.start_addr as usize + i * 16,
                chunk
                    .iter()
                    .map(|b| format!("{:02X}", b))
                    .collect::<Vec<_>>()
                    .join(" ")
            );
            output.push_str(&line);
        }
        Ok(output.into_bytes())
    }

    fn file_extension(&self) -> &'static str {
        "hex"
    }
}

/// Raw binary formatter
pub struct BinaryFormatter;

impl MemoryFormatter for BinaryFormatter {
    fn format(&self, dump: &MemoryDump) -> Result<Vec<u8>, String> {
        Ok(dump.data.clone())
    }

    fn file_extension(&self) -> &'static str {
        "bin"
    }

    fn is_text(&self) -> bool {
        false
    }
}

/// JSON formatter
pub struct JsonFormatter;

/// Structure for JSON/TOML serialization
#[derive(Serialize)]
struct MemoryDumpOutput {
    memory_dump: MemoryDumpData,
}

#[derive(Serialize)]
struct MemoryDumpData {
    #[serde(rename = "type")]
    mem_type: String,
    start: String,
    end: String,
    data: Vec<String>,
}

impl MemoryFormatter for JsonFormatter {
    fn format(&self, dump: &MemoryDump) -> Result<Vec<u8>, String> {
        let data_hex: Vec<String> = dump.data.iter().map(|b| format!("0x{:02X}", b)).collect();

        let output = MemoryDumpOutput {
            memory_dump: MemoryDumpData {
                mem_type: dump.mem_type.to_string(),
                start: format!("0x{:04X}", dump.start_addr),
                end: format!("0x{:04X}", dump.end_addr),
                data: data_hex,
            },
        };

        let json_str = serde_json::to_string_pretty(&output)
            .map_err(|e| format!("Failed to serialize JSON: {}", e))?;

        Ok(format!("{}\n", json_str).into_bytes())
    }

    fn file_extension(&self) -> &'static str {
        "json"
    }
}

/// TOML formatter
pub struct TomlFormatter;

impl MemoryFormatter for TomlFormatter {
    fn format(&self, dump: &MemoryDump) -> Result<Vec<u8>, String> {
        let data_hex: Vec<String> = dump.data.iter().map(|b| format!("0x{:02X}", b)).collect();

        let output = MemoryDumpOutput {
            memory_dump: MemoryDumpData {
                mem_type: dump.mem_type.to_string(),
                start: format!("0x{:04X}", dump.start_addr),
                end: format!("0x{:04X}", dump.end_addr),
                data: data_hex,
            },
        };

        let toml_str = toml::to_string_pretty(&output)
            .map_err(|e| format!("Failed to serialize TOML: {}", e))?;

        Ok(format!("{}\n", toml_str).into_bytes())
    }

    fn file_extension(&self) -> &'static str {
        "toml"
    }
}

// =============================================================================
// OutputFormat Extensions
// =============================================================================

impl OutputFormat {
    /// Get the formatter for this output format.
    ///
    /// To add a new format, add a variant to the enum and a case here.
    pub fn formatter(&self) -> Box<dyn MemoryFormatter> {
        match self {
            OutputFormat::Hex => Box::new(HexFormatter),
            OutputFormat::Json => Box::new(JsonFormatter),
            OutputFormat::Toml => Box::new(TomlFormatter),
            OutputFormat::Binary => Box::new(BinaryFormatter),
        }
    }

    /// Get the file extension for this format.
    pub fn extension(&self) -> &'static str {
        match self {
            OutputFormat::Hex => "hex",
            OutputFormat::Json => "json",
            OutputFormat::Toml => "toml",
            OutputFormat::Binary => "bin",
        }
    }
}

// =============================================================================
// Output Writer
// =============================================================================

/// Track whether the output file has been initialized (created/truncated).
/// Note: This is intentionally global to support multiple OutputWriter instances
/// writing to the same file in append mode within a single CLI run.
static OUTPUT_FILE_INITIALIZED: AtomicBool = AtomicBool::new(false);

/// Manages output writing with support for file and stdout.
pub struct OutputWriter {
    /// Output file path (None for stdout)
    path: Option<PathBuf>,
    /// Output format
    format: OutputFormat,
}

impl OutputWriter {
    /// Create a new output writer.
    pub fn new(path: Option<PathBuf>, format: OutputFormat) -> Self {
        Self { path, format }
    }

    /// Reset the output file state (call at start of output session).
    pub fn reset() {
        OUTPUT_FILE_INITIALIZED.store(false, Ordering::SeqCst);
    }

    /// Write a memory dump using the configured format.
    pub fn write(&self, dump: &MemoryDump) -> Result<(), String> {
        let formatter = self.format.formatter();
        let data = formatter.format(dump)?;
        self.write_bytes(&data)
    }

    /// Write raw bytes to the output.
    fn write_bytes(&self, data: &[u8]) -> Result<(), String> {
        let mut writer = self.get_writer()?;
        writer.write_all(data).map_err(|e| e.to_string())
    }

    /// Get the output writer (file or stdout).
    fn get_writer(&self) -> Result<Box<dyn Write>, String> {
        if let Some(ref path) = self.path {
            let is_first_write = !OUTPUT_FILE_INITIALIZED.swap(true, Ordering::SeqCst);

            let file = if is_first_write {
                File::create(path)
            } else {
                OpenOptions::new().append(true).open(path)
            }
            .map_err(|e| format!("Failed to open output file: {}", e))?;

            Ok(Box::new(file))
        } else {
            Ok(Box::new(std::io::stdout()))
        }
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_formatter() {
        let dump = MemoryDump::cpu(0x0000, vec![0x00, 0x01, 0x02, 0x03]);
        let formatter = HexFormatter;
        let output = formatter.format(&dump).unwrap();
        let text = String::from_utf8(output).unwrap();
        assert!(text.contains("0000: 00 01 02 03"));
    }

    #[test]
    fn test_binary_formatter() {
        let data = vec![0xDE, 0xAD, 0xBE, 0xEF];
        let dump = MemoryDump::cpu(0x1000, data.clone());
        let formatter = BinaryFormatter;
        let output = formatter.format(&dump).unwrap();
        assert_eq!(output, data);
    }

    #[test]
    fn test_json_formatter() {
        let dump = MemoryDump::cpu(0x0000, vec![0xFF]);
        let formatter = JsonFormatter;
        let output = formatter.format(&dump).unwrap();
        let text = String::from_utf8(output).unwrap();
        assert!(text.contains("\"type\": \"cpu\""));
        assert!(text.contains("0xFF"));
    }

    #[test]
    fn test_toml_formatter() {
        let dump = MemoryDump::cpu(0x0000, vec![0xFF]);
        let formatter = TomlFormatter;
        let output = formatter.format(&dump).unwrap();
        let text = String::from_utf8(output).unwrap();
        assert!(text.contains("type = \"cpu\""));
    }

    #[test]
    fn test_output_format_formatter() {
        assert_eq!(OutputFormat::Hex.extension(), "hex");
        assert_eq!(OutputFormat::Json.extension(), "json");
        assert_eq!(OutputFormat::Toml.extension(), "toml");
        assert_eq!(OutputFormat::Binary.extension(), "bin");
    }
}
