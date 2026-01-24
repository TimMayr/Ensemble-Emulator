use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

use crossbeam_channel::Sender;
use rfd::FileDialog;
use rkyv::rancor::BoxedError;

use crate::emulation::messages::RgbPalette;
use crate::emulation::savestate::SaveState;
use crate::frontend::messages::{AsyncFrontendMessage, RelayType};

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
        rkyv::to_bytes::<BoxedError>(self)
            .expect("Failed to serialize SaveState")
            .to_vec()
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

pub fn spawn_file_picker(
    sender: &Sender<AsyncFrontendMessage>,
    previous_path: Option<&PathBuf>,
    file_type: FileType,
    message: RelayType,
) {
    let sender = sender.clone();
    let prev_dir = get_parent_dir(previous_path);
    std::thread::spawn(move || {
        let path = pick_file(prev_dir, file_type);
        let _ = sender.send(AsyncFrontendMessage::EmuRelay(message, path));
    });
}

/// Spawn a save dialog for a palette file in a background thread.
pub fn spawn_save_dialog(previous_path: Option<&PathBuf>, file_type: FileType, data: Vec<u8>) {
    let prev_dir = get_parent_dir(previous_path);
    std::thread::spawn(move || {
        if let Some(p) = save_file(prev_dir, file_type)
            && let Ok(mut file) = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(p)
        {
            let _ = file.write_all(&data);
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
