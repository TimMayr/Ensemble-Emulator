use std::path::PathBuf;

pub enum AsyncFrontendMessage {
    LoadPalette(Option<PathBuf>),
    LoadRom(Option<PathBuf>),
    RefreshPalette,
}
