use std::path::PathBuf;

use crate::emulation::savestate::SaveState;
use crate::frontend::palettes::parse_palette_from_file;

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

pub const TILE_COUNT: usize = 512;
pub const PALETTE_COUNT: usize = 8;
pub const NAMETABLE_COUNT: usize = 4;
pub const NAMETABLE_ROWS: usize = 30;
pub const NAMETABLE_COLS: usize = 32;
pub const PATTERN_TABLE_SIZE: usize = 256;

/// RGB color represented as a tuple of (R, G, B) bytes.
/// This is more memory-efficient than u32 ARGB (3 bytes vs 4 bytes per pixel).
pub type RgbColor = (u8, u8, u8);

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
#[derive(Debug, Clone)]
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
    CreateSaveState,
    LoadSaveState(Box<SaveState>),
}

/// Controller input events
#[derive(Debug, Clone, Copy)]
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
pub enum EmulatorMessage {
    FrameReady(Vec<RgbColor>),
    /// Emulator has stopped/quit
    Stopped,
    DebugData(EmulatorFetchable),
    SaveState(Box<SaveState>),
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum EmulatorFetchable {
    Palettes(Option<Box<PaletteData>>),
    Tiles(Option<Box<[TileData; TILE_COUNT]>>),
    Nametables(Option<Box<NametableData>>),
}

impl EmulatorFetchable {
    #[inline]
    pub fn get_empty(emulator_fetchable: &EmulatorFetchable) -> EmulatorFetchable {
        match emulator_fetchable {
            EmulatorFetchable::Palettes(_) => EmulatorFetchable::Palettes(None),
            EmulatorFetchable::Tiles(_) => EmulatorFetchable::Tiles(None),
            EmulatorFetchable::Nametables(_) => EmulatorFetchable::Nametables(None),
        }
    }

    /// Returns true if this fetchable should only be fetched when the emulator
    /// notifies that the data has changed (passive), rather than on a regular
    /// interval (active).
    ///
    /// Passive fetches reduce CPU overhead for data that rarely changes.
    #[inline]
    pub fn is_passive(&self) -> bool {
        matches!(
            self,
            EmulatorFetchable::Palettes(_) | EmulatorFetchable::Tiles(_)
        )
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct NametableData {
    pub tiles: [[u16; NAMETABLE_ROWS * NAMETABLE_COLS]; NAMETABLE_COUNT],
    pub palettes: [[u8; 64]; NAMETABLE_COUNT],
}

// #[derive(Copy, Clone, PartialEq, Eq, Hash)]
// pub struct SpriteViewerData {
//     pub sprites: [SpriteData; 64],
//     pub sprite_height: u8,
//     pub palette: PaletteData,
// }

// #[derive(Copy, Clone, PartialEq, Eq, Hash)]
// pub struct SpriteData {
//     pub tile: u16,
//     pub tile_2: Option<u16>,
//     pub y_pos: usize,
//     pub x_pos: usize,
//     pub attributes: SpriteAttributes,
// }
//
// #[derive(Clone, Copy, PartialEq, Eq, Hash)]
// pub struct SpriteAttributes {
//     pub palette_index: u8,
//     pub priority: bool,
//     pub flip_x: bool,
//     pub flip_y: bool,
// }

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct TileData {
    pub address: u16,
    pub plane_0: u64,
    pub plane_1: u64,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct PaletteData {
    pub colors: [[u8; 4]; 8],
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct RgbPalette {
    pub colors: [[RgbColor; 64]; 8],
}

impl Default for RgbPalette {
    fn default() -> Self { parse_palette_from_file(None, None) }
}
