use std::path::PathBuf;

pub enum AsyncFrontendMessage {
    EmuRelay(RelayType, Option<PathBuf>),
    RefreshPalette,
}

pub enum RelayType {
    LoadRom,
    LoadPalette,
    LoadSaveState,
}
