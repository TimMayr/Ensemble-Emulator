use std::path::PathBuf;

use crate::emulation::messages::{ControllerEvent, RgbPalette};
use crate::emulation::savestate::SaveState;
use crate::frontend::util::SavestateLoadError;

/// Visual/frontend-only events that are processed synchronously via a deque.
///
/// These events only affect the frontend UI state and don't communicate with
/// the emulator. They are pushed to a deque and processed on each update call,
/// avoiding channel overhead for internal frontend operations.
pub enum FrontendEvent {
    /// Change the window title
    ChangeWindowTitle(String),
    /// Refresh palette textures
    RefreshPalette,
}

/// Messages for async/deferred frontend operations.
///
/// These messages are processed by EguiApp and allow UI components to request
/// operations without directly sending FrontendMessages to the emulator.
/// This consolidates all emulator communication logic in one place.
pub enum AsyncFrontendMessage {
    EmuRelay(RelayType, Option<PathBuf>),
    /// Palette file was loaded asynchronously - includes the parsed palette data and path
    PaletteLoaded(RgbPalette, PathBuf),
    /// User has selected a savestate file, now need to verify/select ROM
    SavestateLoaded(Box<SavestateLoadContext>),
    /// Show dialog asking if user wants to load the found matching ROM
    ShowMatchingRomDialog(Box<SavestateLoadContext>, PathBuf),
    /// User chose to use the matching ROM that was found
    UseMatchingRom(Box<SavestateLoadContext>, PathBuf),
    /// User chose to manually select a ROM for the savestate
    ManuallySelectRom(Box<SavestateLoadContext>),
    /// User selected a ROM file for the savestate
    RomSelectedForSavestate(Box<SavestateLoadContext>, PathBuf),
    /// Show checksum mismatch warning dialog
    ShowChecksumMismatchDialog(Box<SavestateLoadContext>, PathBuf),
    /// User chose to try loading with mismatched checksum anyway
    LoadSavestateAnyway(Box<SavestateLoadContext>, PathBuf),
    /// User chose to select another ROM after checksum mismatch
    SelectAnotherRom(Box<SavestateLoadContext>),
    /// An error occurred while loading the savestate
    SavestateLoadFailed(SavestateLoadError),
    /// An error occurred while verifying the ROM
    RomVerificationFailed(Box<SavestateLoadContext>, SavestateLoadError),
    /// File save completed (success or error message)
    FileSaveCompleted(Option<String>),
    Quickload,
    Quicksave,
    LoadRom(PathBuf),

    // =========================================================================
    // Consolidated emulator operations
    // These replace direct FrontendMessage sends from UI components
    // =========================================================================

    /// Power on the console (updates is_powered config)
    PowerOn,
    /// Power off the console (updates is_powered config)
    PowerOff,
    /// Reset the console (soft reset)
    Reset,
    /// Create a manual savestate
    CreateSavestate,
    /// Set the RGB palette and refresh tile textures
    SetPalette(RgbPalette),
    /// Write to PPU palette RAM and request updated palette data
    WritePpuPalette { address: u16, value: u8 },
    /// Write to PPU pattern table and request updated tile data
    WritePpuPattern {
        addr_0: u16,
        value_0: u8,
        addr_1: u16,
        value_1: u8,
    },
    /// Send controller input to the emulator
    ControllerInput(ControllerEvent),
}

/// Context for the multistep savestate loading process
#[derive(Clone)]
pub struct SavestateLoadContext {
    pub savestate: SaveState,
    pub savestate_path: PathBuf,
}

pub enum RelayType {
    LoadRom,
    LoadPalette,
}
