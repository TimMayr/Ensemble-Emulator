use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use crate::emulation::messages::{RgbColor, RgbPalette};

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

    let mut colors: [[RgbColor; 64]; 8] = [[(0, 0, 0); 64]; 8];

    for (palette_index, palette) in colors.iter_mut().enumerate() {
        for (color_index, color) in palette.iter_mut().enumerate() {
            let base_index = ((palette_index * 64) + color_index) * 3;
            let read_color: RgbColor = if let (Some(r), Some(g), Some(b)) = (
                data.get(base_index),
                data.get(base_index + 1),
                data.get(base_index + 2),
            ) {
                (*r, *g, *b)
            } else {
                (
                    DEFAULT_PALETTE[base_index],
                    DEFAULT_PALETTE[base_index + 1],
                    DEFAULT_PALETTE[base_index + 2],
                )
            };

            *color = read_color;
        }
    }

    RgbPalette {
        colors,
    }
}
