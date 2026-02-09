use std::path::PathBuf;

use lockstep_ensemble::emulation::ppu::{EmulatorFetchable, RgbColor, RgbPalette};
use lockstep_ensemble::emulation::rom::RomFile;
use lockstep_ensemble::emulation::savestate::SaveState;

/// Message types for communication between the frontend and emulator.
///
/// This module defines the message protocol for bidirectional communication:
/// - `FrontendMessage`: Commands sent from the frontend to the emulator
/// - `EmulatorMessage`: Notifications sent from the emulator to the frontend
/// - `ControllerEvent`: Input events for the emulated console
///
/// The message-based architecture provides clean separation between the frontend
/// and emulation logic, enabling future threading and remote control features.
// Pattern table display: 2 tables of 16x16 tiles (8px each) with 16px gap
/// Convert an ARGB u32 color to an RGB tuple.
#[inline]
pub const fn argb_to_rgb(argb: u32) -> RgbColor {
    (
        ((argb >> 16) & 0xFF) as u8, // R
        ((argb >> 8) & 0xFF) as u8,  // G
        (argb & 0xFF) as u8,         // B
    )
}

/// Convert an RGB tuple to an ARGB u32 color (with alpha = 0xFF).
#[inline]
pub const fn rgb_to_argb(rgb: RgbColor) -> u32 {
    0xFF00_0000 | ((rgb.0 as u32) << 16) | ((rgb.1 as u32) << 8) | (rgb.2 as u32)
}

/// Messages sent from the frontend to the emulator
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum FrontendMessage {
    /// Request to quit the emulator
    Quit,
    /// Controller input events
    ControllerInput(ControllerEvent),
    /// Request to reset the console
    Reset,
    Power,
    PowerOff,
    /// Request to step one frame
    StepFrame,
    RequestDebugData(EmulatorFetchable),
    SetPalette(Box<RgbPalette>),
    LoadRom(PathBuf),
    WritePpu(u16, u8),
    WriteCpu(u16, u8),
    CreateSaveState(SaveType),
    LoadSaveState(Box<SaveState>),
}

#[derive(Debug, Default, Eq, PartialEq, Copy, Clone, Hash)]
pub enum SaveType {
    #[default]
    Manual,
    Quicksave,
    Autosave,
}

/// Controller input events
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ControllerEvent {
    Left,
    Right,
    Up,
    Down,
    Start,
    Select,
    A,
    B,
}

/// Messages sent from the emulator to the frontend
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum EmulatorMessage {
    FrameReady(Vec<RgbColor>),
    /// Emulator has stopped/quit
    Stopped,
    DebugData(EmulatorFetchable),
    SaveState(Box<SaveState>, SaveType),
    RomLoaded(Option<RomFile>),
}
