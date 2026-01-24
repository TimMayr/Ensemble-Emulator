use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use crate::emulation::messages::RgbPalette;

static DEFAULT_PALETTE: &[u8] = include_bytes!("../../assets/2C02G_wiki.pal");
pub fn parse_palette_from_file(path: Option<PathBuf>, fallback: Option<PathBuf>) -> RgbPalette {
    let path = path.or(fallback);

    let data = if let Some(path) = path {
        if let Ok(ref mut file) = File::open(path) {
            let mut data = Vec::new();
            if file.read_to_end(&mut data).is_ok() {
                data.into_boxed_slice()
            } else {
                DEFAULT_PALETTE.into()
            }
        } else {
            DEFAULT_PALETTE.into()
        }
    } else {
        DEFAULT_PALETTE.into()
    };

    let mut colors = [[0u32; 64]; 8];

    for (palette_index, palette) in colors.iter_mut().enumerate() {
        for (color_index, color) in palette.iter_mut().enumerate() {
            let base_index = ((palette_index * 64) + color_index) * 3;
            let read_color = if let (Some(r), Some(g), Some(b)) = (
                data.get(base_index),
                data.get(base_index + 1),
                data.get(base_index + 2),
            ) {
                (0xFFu32 << 24) | ((*r as u32) << 16) | ((*g as u32) << 8) | (*b as u32)
            } else {
                (0xFFu32 << 24)
                    | ((DEFAULT_PALETTE[base_index] as u32) << 16)
                    | ((DEFAULT_PALETTE[base_index + 1] as u32) << 8)
                    | (DEFAULT_PALETTE[base_index + 2] as u32)
            };

            *color = read_color;
        }
    }

    RgbPalette {
        colors,
    }
}
