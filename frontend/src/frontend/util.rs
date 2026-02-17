use std::fs::File;
use std::future::Future;
use std::io::Read;
use std::path::{Path, PathBuf};

use crossbeam_channel::Sender;
use ensemble_lockstep::emulation::savestate::{self};
use ensemble_lockstep::util::ToBytes;
use rfd::{AsyncFileDialog, FileHandle};
use sha2::{Digest, Sha256};

use crate::frontend::messages::{AsyncFrontendMessage, SavestateLoadContext};

/// Enum to represent errors that can occur during savestate loading UI flow
pub enum SavestateLoadError {
    /// Failed to load or parse the savestate file
    FailedToLoadSavestate,
    /// Failed to compute checksum for a ROM file
    FailedToComputeChecksum,
}

pub trait Contrastable {
    fn get_contrast(&self) -> Self;
}

impl Contrastable for egui::Color32 {
    /// Calculate foreground color (black or white) based on background luminance
    fn get_contrast(&self) -> Self {
        let bg = self.as_u32();
        let r = (bg >> 16) & 0xFF;
        let g = (bg >> 8) & 0xFF;
        let b = bg & 0xFF;

        let luminance = 0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32;

        if luminance > 128.0 {
            egui::Color32::BLACK
        } else {
            egui::Color32::WHITE
        }
    }
}

pub trait FromU32 {
    fn from_u32(d: u32) -> Self;
}

pub trait AsU32 {
    fn as_u32(&self) -> u32;
}

impl FromU32 for egui::Color32 {
    fn from_u32(d: u32) -> Self {
        egui::Color32::from_rgba_unmultiplied(
            (d >> 16) as u8,
            (d >> 8) as u8,
            d as u8,
            (d >> 24) as u8,
        )
    }
}

impl AsU32 for egui::Color32 {
    fn as_u32(&self) -> u32 {
        ((self.a() as u32) << 24)
            | ((self.r() as u32) << 16)
            | ((self.g() as u32) << 8)
            | (self.b() as u32)
    }
}
pub async fn pick_file(previous: PathBuf, file_type: FileType) -> Option<FileHandle> {
    AsyncFileDialog::new()
        .add_filetype_filter(file_type)
        .add_filetype_filter(FileType::All)
        .set_directory(previous)
        .pick_file()
        .await
}

pub async fn save_file(previous: PathBuf, file_type: FileType) -> Option<FileHandle> {
    AsyncFileDialog::new()
        .set_directory(previous)
        .add_filetype_filter(file_type)
        .add_filetype_filter(FileType::All)
        .set_can_create_directories(true)
        .save_file()
        .await
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum FileType {
    Rom,
    Savestate,
    Palette,
    All,
}

impl FileType {
    pub fn add_filters(&self, dialog: AsyncFileDialog) -> AsyncFileDialog {
        match self {
            FileType::Rom => dialog.add_filter("NES ROM File", &[self.get_default_extension()]),
            FileType::Savestate => {
                dialog.add_filter("Savestate", &[self.get_default_extension(), "json"])
            }
            FileType::Palette => {
                dialog.add_filter("NES Palette File", &[self.get_default_extension()])
            }
            FileType::All => dialog.add_filter("All Files", &["*"]),
        }
    }

    pub fn get_default_extension(&self) -> &str {
        match self {
            FileType::Rom => "nes",
            FileType::Savestate => "sav",
            FileType::Palette => "pal",
            FileType::All => "",
        }
    }
}

pub trait AddFilter {
    fn add_filetype_filter(self, file_type: FileType) -> Self;
}

impl AddFilter for AsyncFileDialog {
    fn add_filetype_filter(self, file_type: FileType) -> Self { file_type.add_filters(self) }
}

/// Extract the parent directory from an optional path, or return a default empty path.
/// This is commonly used to set the initial directory for file dialogs.
pub fn get_parent_dir(path: Option<&PathBuf>) -> PathBuf {
    path.and_then(|p| p.parent())
        .map(|p| p.to_path_buf())
        .unwrap_or_default()
}

/// Spawn a file picker for palette files that loads the palette asynchronously.
///
/// This reads and parses the palette file asynchronously to avoid blocking the UI.
///
/// # Arguments
/// * `sender` - Channel to send the loaded palette back to the UI thread
/// * `previous_path` - Used to set the initial directory for the file picker dialog
/// * `_fallback_path` - Unused, kept for API compatibility (fallback is now handled internally)
pub fn spawn_palette_picker(
    sender: &Sender<AsyncFrontendMessage>,
    previous_path: Option<&PathBuf>,
    _fallback_path: Option<PathBuf>,
) {
    let sender = sender.clone();
    let prev_dir = get_parent_dir(previous_path);
    spawn_async(async move {
        if let Some(handle) = pick_file(prev_dir, FileType::Palette).await {
            // Read the file contents from the handle
            let data = handle.read().await;
            let palette = ensemble_lockstep::emulation::screen_renderer::parse_palette_from_bytes(
                data,
            );
            let _ = sender.send(AsyncFrontendMessage::PaletteLoaded(palette, None));
        }
    });
}

pub fn spawn_rom_picker(sender: &Sender<AsyncFrontendMessage>, previous_path: Option<&PathBuf>) {
    let sender = sender.clone();
    let prev_dir = get_parent_dir(previous_path);
    spawn_async(async move {
        if let Some(handle) = pick_file(prev_dir, FileType::Rom).await {
            #[cfg(not(target_arch = "wasm32"))]
            let path = handle.path().to_path_buf();
            #[cfg(target_arch = "wasm32")]
            let path = PathBuf::from(handle.file_name());
            let _ = sender.send(AsyncFrontendMessage::LoadRom(Some(path)));
        }
    });
}

/// Spawn a save dialog for a file asynchronously.
/// The file is written after the user selects a path.
pub fn spawn_save_dialog(
    sender: Option<&Sender<AsyncFrontendMessage>>,
    previous_path: Option<&PathBuf>,
    file_type: FileType,
    data: Box<dyn ToBytes + Send>,
) {
    let prev_dir = get_parent_dir(previous_path);
    let sender = sender.cloned();
    spawn_async(async move {
        if let Some(handle) = save_file(prev_dir, file_type).await {
            // Get filename for format detection
            let filename = handle.file_name();
            let format = PathBuf::from(&filename)
                .extension()
                .map(|ext| ext.to_string_lossy().to_string());

            // Write data using the file handle
            let bytes = data.to_bytes(format);
            let result = handle.write(&bytes).await;

            // Notify completion if a sender was provided
            let error = result.err().map(|e| format!("Failed to write file: {}", e));
            if let Some(sender) = sender {
                let _ = sender.send(AsyncFrontendMessage::FileSaveCompleted(error));
            }
        }
    });
}

pub fn color_radio<Value: PartialEq>(
    ui: &mut egui::Ui,
    current: &mut Value,
    alternative: Value,
    color: egui::Color32,
) -> egui::Response {
    let size = egui::vec2(20.0, 20.0);
    let selected = *current == alternative;

    let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());

    if ui.is_rect_visible(rect) {
        let painter = ui.painter();

        // Draw the colored square
        painter.rect_filled(rect, 2.0, color);

        // Draw selection border when selected
        if selected {
            painter.rect_stroke(
                rect,
                2.0,
                egui::Stroke::new(2.0, ui.visuals().selection.stroke.color),
                egui::StrokeKind::Inside,
            );
        }
    }

    if response.clicked() {
        *current = alternative;
    }

    response
}

/// Compute SHA256 checksum for a file
pub fn compute_file_checksum(path: &Path) -> Option<[u8; 32]> {
    let mut file = File::open(path).ok()?;
    let mut data = Vec::new();
    file.read_to_end(&mut data).ok()?;

    let mut hasher = Sha256::new();
    hasher.update(&data);
    Some(hasher.finalize().into())
}

/// Check if a ROM file matches the expected filename and checksum
pub fn find_matching_rom_in_dir(
    dir: &Path,
    expected_filename: &str,
    expected_checksum: &[u8; 32],
) -> Option<PathBuf> {
    let rom_path = dir.join(expected_filename);
    if rom_path.exists()
        && let Some(checksum) = compute_file_checksum(&rom_path)
        && checksum == *expected_checksum
    {
        return Some(rom_path);
    }
    None
}

/// Spawn the initial savestate file picker asynchronously.
/// After the savestate is loaded, it will check for a matching ROM.
pub fn spawn_savestate_picker(
    sender: &Sender<AsyncFrontendMessage>,
    previous_savestate_path: Option<&PathBuf>,
    previous_rom_path: Option<PathBuf>,
) {
    let sender = sender.clone();
    let prev_dir = get_parent_dir(previous_savestate_path);
    let rom_dir = previous_rom_path.and_then(|p| p.parent().map(|p| p.to_path_buf()));

    spawn_async(async move {
        let handle = pick_file(prev_dir, FileType::Savestate).await;

        if let Some(handle) = handle {
            // Read savestate data from the file handle
            let data = handle.read().await;
            
            #[cfg(not(target_arch = "wasm32"))]
            let savestate_path = handle.path().to_path_buf();
            #[cfg(target_arch = "wasm32")]
            let savestate_path = PathBuf::from(handle.file_name());

            // Try to parse the savestate from the data
            let savestate = match savestate::try_load_state_from_bytes(&data) {
                Some(s) => s,
                None => {
                    // Failed to load savestate - send error notification
                    let _ = sender.send(AsyncFrontendMessage::SavestateLoadFailed(
                        SavestateLoadError::FailedToLoadSavestate,
                    ));
                    return;
                }
            };
            let context = SavestateLoadContext {
                savestate,
                savestate_path: savestate_path.clone(),
            };

            // Check if there's a matching ROM in the last ROM directory (native only)
            #[cfg(not(target_arch = "wasm32"))]
            if let Some(ref rom_dir) = rom_dir
                && let Some(ref rom_name) = context.savestate.rom_file.name
                && let Some(matching_rom) = find_matching_rom_in_dir(
                    rom_dir,
                    rom_name,
                    &context.savestate.rom_file.data_checksum,
                )
            {
                // Found a matching ROM - ask user if they want to use it
                let _ = sender.send(AsyncFrontendMessage::ShowMatchingRomDialog(
                    Box::new(context),
                    matching_rom,
                ));
                return;
            }
            
            // Silence unused warning for rom_dir on WASM
            #[cfg(target_arch = "wasm32")]
            let _ = rom_dir;

            // No matching ROM found, send context for next step
            let _ = sender.send(AsyncFrontendMessage::SavestateLoaded(Box::new(context)));
        }
    });
}

/// Spawn a file picker to select a ROM for a savestate
pub fn spawn_rom_picker_for_savestate(
    sender: &Sender<AsyncFrontendMessage>,
    context: Box<SavestateLoadContext>,
    previous_rom_path: Option<&PathBuf>,
) {
    let sender = sender.clone();
    let prev_dir = get_parent_dir(previous_rom_path);

    spawn_async(async move {
        let handle = pick_file(prev_dir, FileType::Rom).await;

        if let Some(handle) = handle {
            #[cfg(not(target_arch = "wasm32"))]
            let rom_path = handle.path().to_path_buf();
            #[cfg(target_arch = "wasm32")]
            let rom_path = PathBuf::from(handle.file_name());
            
            let _ = sender.send(AsyncFrontendMessage::RomSelectedForSavestate(
                context,
                rom_path,
            ));
        }
    });
}

pub fn rom_display_name(path: &Path, sha256: &[u8; 32]) -> String {
    match path.file_stem().and_then(|s| s.to_str()) {
        Some(name) if !name.is_empty() => name.to_owned(),
        _ => format!("Unknown ROM ({})", short_hash_hex(sha256)),
    }
}
pub fn short_hash_hex(hash: &[u8; 32]) -> String {
    use std::fmt::Write;
    let mut s = String::with_capacity(12);
    for b in &hash[..6] {
        write!(&mut s, "{:02x}", b).unwrap();
    }
    s
}

pub fn append_to_filename(path: &Path, suffix: &str, overwrite: usize) -> PathBuf {
    let parent = path.parent().unwrap_or_else(|| Path::new(""));
    let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
    let ext = path.extension().and_then(|e| e.to_str());

    let cut = stem.len().saturating_sub(overwrite);
    let mut name = String::from(&stem[..cut]);
    name.push_str(suffix);

    match ext {
        Some(ext) => parent.join(format!("{name}.{ext}")),
        None => parent.join(name),
    }
}

#[cfg(target_arch = "wasm32")]
fn spawn_async<F: Future<Output = ()> + 'static>(f: F) { wasm_bindgen_futures::spawn_local(f); }

#[cfg(not(target_arch = "wasm32"))]
fn spawn_async<F: Future<Output = ()> + Send + 'static>(f: F) { tokio::spawn(f); }
