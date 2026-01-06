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
pub const PATTERN_TABLE_WIDTH: usize = 256 + 16; // 16*8*2 + 16px gap
pub const PATTERN_TABLE_HEIGHT: usize = 128; // 16*8

// Nametable display: 4 nametables of 32x30 tiles (8px each) arranged 2x2
pub const NAMETABLE_WIDTH: usize = 512; // 32*8*2
pub const NAMETABLE_HEIGHT: usize = 480; // 30*8*2

pub const SPRITE_COUNT: usize = 64;
pub const SPRITE_WIDTH: usize = 8;

pub const TOTAL_OUTPUT_WIDTH: usize = 256;
pub const TOTAL_OUTPUT_HEIGHT: usize = 240;

/// Messages sent from the frontend to the emulator
#[derive(Debug, Clone)]
pub enum FrontendMessage {
    /// Request to quit the emulator
    Quit,
    /// Controller input events
    ControllerInput(ControllerEvent),
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
    RequestSpriteData,
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
pub enum EmulatorMessage {
    /// A new frame is ready to be displayed
    FrameReady(Vec<u32>),
    /// Pattern table data is ready
    PatternTableReady(Box<PatternTableViewerData>),
    /// Nametable data is ready
    NametableReady(Vec<u32>),
    /// Emulator has stopped/quit
    Stopped,
    SpritesReady(([Box<[u32]>; SPRITE_COUNT], usize)),
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct SpriteViewerData {
    pub sprites: [SpriteData; 64],
    pub sprite_height: u8,
    pub palette: PaletteData,
}
#[derive(Copy, Clone, PartialEq, Eq)]

pub struct PatternTableViewerData {
    pub left: PatternTableData,
    pub right: PatternTableData,
    pub palette: PaletteData,
}
#[derive(Copy, Clone, PartialEq, Eq)]

pub struct PatternTableData {
    pub tiles: [TileData; 256],
}
#[derive(Copy, Clone, PartialEq, Eq)]

pub struct SpriteData {
    pub tile: TileData,
    pub tile_2: Option<TileData>,
    pub y_pos: usize,
    pub x_pos: usize,
    pub attributes: SpriteAttributes,
}
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct SpriteAttributes {
    pub palette_index: u8,
    pub priority: bool,
    pub flip_x: bool,
    pub flip_y: bool,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct TileData {
    pub address: u16,
    pub plane_0: u64,
    pub plane_1: u64,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct PaletteData {
    pub colors: [u32; 4],
}
