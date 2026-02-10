use serde_big_array::BigArray;
use std::fmt::{Debug, Formatter};
use ensemble_lockstep::emulation::screen_renderer::{RgbColor, RgbPalette, ScreenRenderer};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct FlatPalette {
    #[serde(with = "BigArray")]
    palette: [RgbColor; 64*8],
}

impl From<RgbPalette> for FlatPalette {
    fn from(palette: RgbPalette) -> Self {
        let mut flat = [RgbColor::default(); 8 * 64];

        for color in 0..64 {
            for emph in 0..8 {
                flat[color | (emph << 6)] = palette.colors[emph][color];
            }
        }

        Self {
            palette: flat,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct LookupPaletteRenderer {
    palette: FlatPalette,
    pub image: Vec<RgbColor>,
}

impl Default for LookupPaletteRenderer {
    fn default() -> Self { Self::new() }
}

impl LookupPaletteRenderer {
    fn new() -> Self {
        Self {
            palette: RgbPalette::default().into(),
            image: vec![],
        }
    }

    fn load_palette(&mut self, rgb_palette: RgbPalette) { self.palette = rgb_palette.into(); }
}

impl Debug for LookupPaletteRenderer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("LookupPaletteRenderer")
    }
}

impl ScreenRenderer for LookupPaletteRenderer {
    fn buffer_to_image(&mut self, buffer: &[u16]) -> &[RgbColor] {
        if self.image.len() != buffer.len() {
            self.image = Vec::with_capacity(buffer.len());
        };

        self.image.clear();
        for x in buffer.iter() {
            self.image
                .push(self.palette.palette[(*x as usize) & ((8 * 64) - 1)])
        }

        &self.image
    }
}