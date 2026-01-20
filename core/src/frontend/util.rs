use std::path::PathBuf;

use rfd::FileDialog;

pub trait Contrastable {
    fn get_contrast(&self) -> Self;
}

impl Contrastable for egui::Color32 {
    fn get_contrast(&self) -> Self {
        let bg = self.as_u32();
        /// Calculate foreground color (black or white) based on background luminance
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
        egui::Color32::from_rgb((d >> 16) as u8, (d >> 8) as u8, d as u8)
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
