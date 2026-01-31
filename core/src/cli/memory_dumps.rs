//! Memory dump creation functions for the CLI.
//!
//! This module provides functions to create memory dumps from the emulator state.

use crate::cli::{parse_memory_range, MemoryDump, MemoryType};
use crate::emulation::nes::Nes;

// =============================================================================
// Memory Dump Creation
// =============================================================================

/// Create a CPU memory dump from the emulator.
pub fn create_cpu_dump(emu: &Nes, range: &str) -> Result<MemoryDump, String> {
    let (start, end) = parse_memory_range(range)?;
    let mem = &emu.get_memory_debug(Some(start..=end))[0];
    Ok(MemoryDump::new(MemoryType::Cpu, start, mem.to_vec()))
}

/// Create a PPU memory dump from the emulator.
pub fn create_ppu_dump(emu: &Nes, range: &str) -> Result<MemoryDump, String> {
    let (start, end) = parse_memory_range(range)?;
    let mem = &emu.get_memory_debug(Some(start..=end))[1];
    Ok(MemoryDump::new(MemoryType::Ppu, start, mem.to_vec()))
}

/// Create an OAM memory dump from the emulator.
pub fn create_oam_dump(emu: &Nes) -> MemoryDump {
    let mem = emu.ppu.borrow().oam.get_memory_debug(None);
    MemoryDump::oam(mem)
}

/// Create a nametables memory dump from the emulator.
pub fn create_nametables_dump(emu: &Nes) -> MemoryDump {
    let mem = emu
        .ppu
        .borrow()
        .memory
        .get_memory_debug(Some(0x2000..=0x2FFF));
    MemoryDump::nametables(mem)
}

/// Create a palette RAM memory dump from the emulator.
pub fn create_palette_dump(emu: &Nes) -> MemoryDump {
    // Palette RAM is at PPU addresses $3F00-$3F1F (32 bytes)
    let mem = emu
        .ppu
        .borrow()
        .memory
        .get_memory_debug(Some(0x3F00..=0x3F1F));
    MemoryDump::palette_ram(mem)
}
