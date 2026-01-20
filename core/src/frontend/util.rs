use std::path::PathBuf;

use rfd::FileDialog;

pub trait FromU32 {
    fn from_u32(d: u32) -> Self;
}

impl FromU32 for egui::Color32 {
    fn from_u32(d: u32) -> Self {
        egui::Color32::from_rgb((d >> 16) as u8, (d >> 8) as u8, d as u8)
    }
}

pub fn pick_rom(previous: PathBuf) -> Option<PathBuf> {
    FileDialog::new()
        .add_filter("NES ROM File", &["nes"])
        .set_directory(previous)
        .pick_file()
}

pub fn pick_palette(previous: PathBuf) -> Option<PathBuf> {
    FileDialog::new()
        .add_filter("NES Palette File", &["pal"])
        .set_directory(previous)
        .pick_file()
}
