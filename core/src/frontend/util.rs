use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use crossbeam_channel::Sender;
use rfd::FileDialog;
use sha2::{Digest, Sha256};

use crate::emulation::messages::RgbPalette;
use crate::emulation::savestate::{self, SaveState};
use crate::frontend::messages::{AsyncFrontendMessage, SavestateLoadContext};
use crate::frontend::palettes::parse_palette_from_file;

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

pub trait Hashable {
    fn hash(&self) -> u64;
}

pub trait ToBytes {
    fn to_bytes(&self) -> Vec<u8>;
}

impl ToBytes for RgbPalette {
    fn to_bytes(&self) -> Vec<u8> {
        self.colors
            .iter()
            .flatten()
            .flat_map(|d| {
                let mut bytes = [0u8; 3];
                bytes[0] = ((d >> 16) & 0xFF) as u8;
                bytes[1] = ((d >> 8) & 0xFF) as u8;
                bytes[2] = (d & 0xFF) as u8;
                bytes
            })
            .collect()
    }
}

impl ToBytes for SaveState {
    fn to_bytes(&self) -> Vec<u8> {
        bincode::serde::encode_to_vec(self, bincode::config::standard())
            .expect("Failed to serialize SaveState")
    }
}

impl Hashable for RgbPalette {
    /// Compute a fast hash of the given data for change detection.
    /// Uses FNV-1a algorithm which is fast and has good distribution.
    #[inline]
    fn hash(&self) -> u64 {
        let bytes = self.to_bytes();
        compute_hash(&bytes[..])
    }
}

/// Compute a fast hash of the given data for change detection.
/// Uses FNV-1a algorithm which is fast and has good distribution.
#[inline]
pub fn compute_hash(data: &[u8]) -> u64 {
    const FNV_OFFSET_BASIS: u64 = 0xCBF29CE484222325;
    const FNV_PRIME: u64 = 0x100000001B3;

    let mut hash = FNV_OFFSET_BASIS;
    for &byte in data {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    hash
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

pub fn pick_file(previous: PathBuf, file_type: FileType) -> Option<PathBuf> {
    FileDialog::new()
        .add_filetype_filter(file_type)
        .add_filetype_filter(FileType::All)
        .set_directory(previous)
        .pick_file()
}

pub fn save_file(previous: PathBuf, file_type: FileType) -> Option<PathBuf> {
    FileDialog::new()
        .set_directory(previous)
        .add_filetype_filter(file_type)
        .add_filetype_filter(FileType::All)
        .set_can_create_directories(true)
        .save_file()
}

pub enum FileType {
    Rom,
    Savestate,
    Palette,
    All,
}

impl FileType {
    pub fn add_filters(&self, dialog: FileDialog) -> FileDialog {
        match self {
            FileType::Rom => dialog.add_filter("NES ROM File", &["nes"]),
            FileType::Savestate => dialog.add_filter("Savestate", &["sav"]),
            FileType::Palette => dialog.add_filter("NES Palette File", &["pal"]),
            FileType::All => dialog.add_filter("All Files", &["*"]),
        }
    }
}

pub trait AddFilter {
    fn add_filetype_filter(self, file_type: FileType) -> Self;
}

impl AddFilter for FileDialog {
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
/// This reads and parses the palette file in a background thread to avoid blocking the UI.
///
/// # Arguments
/// * `sender` - Channel to send the loaded palette back to the UI thread
/// * `previous_path` - Used to set the initial directory for the file picker dialog
/// * `fallback_path` - Used as fallback if the selected file cannot be parsed
pub fn spawn_palette_picker(
    sender: &Sender<AsyncFrontendMessage>,
    previous_path: Option<&PathBuf>,
    fallback_path: Option<PathBuf>,
) {
    let sender = sender.clone();
    let prev_dir = get_parent_dir(previous_path);
    std::thread::spawn(move || {
        if let Some(path) = pick_file(prev_dir, FileType::Palette) {
            // Parse the palette file in this background thread
            let palette = parse_palette_from_file(Some(path.clone()), fallback_path);
            let _ = sender.send(AsyncFrontendMessage::PaletteLoaded(palette, Some(path)));
        }
    });
}

pub fn spawn_rom_picker(sender: &Sender<AsyncFrontendMessage>, previous_path: Option<&PathBuf>) {
    let sender = sender.clone();
    let prev_dir = get_parent_dir(previous_path);
    std::thread::spawn(move || {
        if let Some(path) = pick_file(prev_dir, FileType::Rom) {
            let _ = sender.send(AsyncFrontendMessage::LoadRom(Some(path)));
        }
    });
}

/// Spawn a save dialog for a file in a background thread.
/// The file is written asynchronously after the user selects a path.
pub fn spawn_save_dialog(
    sender: Option<&Sender<AsyncFrontendMessage>>,
    previous_path: Option<&PathBuf>,
    file_type: FileType,
    data: Vec<u8>,
) {
    let prev_dir = get_parent_dir(previous_path);
    let sender = sender.cloned();
    std::thread::spawn(move || {
        if let Some(p) = save_file(prev_dir, file_type) {
            let result = match OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(&p)
            {
                Ok(mut file) => match file.write_all(&data) {
                    Ok(_) => None,
                    Err(e) => Some(format!("Failed to write file: {}", e)),
                },
                Err(e) => Some(format!("Failed to create file: {}", e)),
            };
            // Notify completion if a sender was provided
            if let Some(sender) = sender {
                let _ = sender.send(AsyncFrontendMessage::FileSaveCompleted(result));
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
pub fn compute_file_checksum(path: &PathBuf) -> Option<[u8; 32]> {
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

/// Spawn the initial savestate file picker in a background thread.
/// After the savestate is loaded, it will check for a matching ROM.
pub fn spawn_savestate_picker(
    sender: &Sender<AsyncFrontendMessage>,
    previous_savestate_path: Option<&PathBuf>,
    previous_rom_path: Option<PathBuf>,
) {
    let sender = sender.clone();
    let prev_dir = get_parent_dir(previous_savestate_path);
    let rom_dir = previous_rom_path.and_then(|p| p.parent().map(|p| p.to_path_buf()));

    std::thread::spawn(move || {
        let path = pick_file(prev_dir, FileType::Savestate);

        if let Some(savestate_path) = path
            && let Ok(canonical_path) = savestate_path.canonicalize()
        {
            // Try to load the savestate, handle errors gracefully
            let savestate = match savestate::try_load_state(&canonical_path) {
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
                savestate_path: canonical_path,
            };

            // Check if there's a matching ROM in the last ROM directory
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

    std::thread::spawn(move || {
        let path = pick_file(prev_dir, FileType::Rom);

        if let Some(rom_path) = path
            && let Ok(canonical_path) = rom_path.canonicalize()
        {
            let _ = sender.send(AsyncFrontendMessage::RomSelectedForSavestate(
                context,
                canonical_path,
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
