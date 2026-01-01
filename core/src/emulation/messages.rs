/// Message types for communication between the frontend and emulator.
///
/// This module defines the message protocol for bidirectional communication:
/// - `FrontendMessage`: Commands sent from the frontend to the emulator
/// - `EmulatorMessage`: Notifications sent from the emulator to the frontend
/// - `ControllerEvent`: Input events for the emulated console
///
/// The message-based architecture provides clean separation between the frontend
/// and emulation logic, enabling future threading and remote control features.
use crate::emulation::emu::{TOTAL_OUTPUT_HEIGHT, TOTAL_OUTPUT_WIDTH};


// Pattern table display: 2 tables of 16x16 tiles (8px each) with 16px gap
pub const PATTERN_TABLE_WIDTH: u32 = 256 + 16; // 16*8*2 + 16px gap
pub const PATTERN_TABLE_HEIGHT: u32 = 128; // 16*8

// Nametable display: 4 nametables of 32x30 tiles (8px each) arranged 2x2
pub const NAMETABLE_WIDTH: u32 = 512; // 32*8*2
pub const NAMETABLE_HEIGHT: u32 = 480; // 30*8*2

/// Messages sent from the frontend to the emulator
#[derive(Debug, Clone)]
pub enum FrontendMessage {
    /// Request to quit the emulator
    Quit,
    /// Controller input events
    ControllerInput(ControllerEvent),
    /// Request to pause emulation
    Pause,
    /// Request to resume emulation
    Resume,
    /// Request to reset the console
    Reset,
    /// Request to step one frame
    StepFrame,
    /// Enable pattern table rendering
    EnablePatternTableRendering(bool),
    /// Enable nametable rendering
    EnableNametableRendering(bool),
    /// Request pattern table data (frontend requests it, not sent every frame)
    RequestPatternTableData,
    /// Request nametable data (frontend requests it, not sent every frame)
    RequestNametableData,
}

/// Controller input events
#[derive(Debug, Clone, Copy)]
pub enum ControllerEvent {
    IncPalette,
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
#[derive(Debug)]
pub enum EmulatorMessage {
    /// A new frame is ready to be displayed
    FrameReady(Box<[u32; (TOTAL_OUTPUT_WIDTH * TOTAL_OUTPUT_HEIGHT) as usize]>),
    /// Pattern table data is ready
    PatternTableReady(Box<[u32; (PATTERN_TABLE_WIDTH * PATTERN_TABLE_HEIGHT) as usize]>),
    /// Nametable data is ready
    NametableReady(Box<[u32; (NAMETABLE_WIDTH * NAMETABLE_HEIGHT) as usize]>),
    /// Emulator has stopped/quit
    Stopped,
}
