use std::path::PathBuf;

use crate::emulation::messages::RgbPalette;
use crate::emulation::savestate::SaveState;
use crate::frontend::util::SavestateLoadError;

pub enum AsyncFrontendMessage {
    EmuRelay(RelayType, Option<PathBuf>),
    RefreshPalette,
    /// Palette file was loaded asynchronously - includes the parsed palette data and path
    PaletteLoaded(Box<RgbPalette>, PathBuf),
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
