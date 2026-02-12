use serde_big_array::BigArray;
use std::fmt::{Debug, Formatter};
use ensemble_lockstep::emulation::screen_renderer::{RgbColor, RgbPalette, ScreenRenderer};
use serde::{Deserialize, Serialize};

/// A flat palette structure optimized for lookup-table based rendering.
/// 
/// The NES PPU outputs a 9-bit value for each pixel:
/// - Bits 0-5 (6 bits): Color index (0-63 from the NES palette)
/// - Bits 6-8 (3 bits): Emphasis bits (R, G, B emphasis from PPU mask register)
/// 
/// This flat structure allows O(1) lookup using the raw index value.
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

/// Lookup table-based palette renderer.
/// 
/// This renderer uses a precomputed lookup table for fast color conversion.
/// It is the default renderer for the NES emulator.
/// 
/// # Usage
/// 
/// ```ignore
/// use ensemble_gown::LookupPaletteRenderer;
/// 
/// let mut renderer = LookupPaletteRenderer::default();
/// renderer.set_palette(my_palette);
/// let rgb_colors = renderer.buffer_to_image(&pixel_indices);
/// ```
#[derive(Serialize, Deserialize)]
pub struct LookupPaletteRenderer {
    palette: FlatPalette,
    image: Vec<RgbColor>,
}

impl Default for LookupPaletteRenderer {
    fn default() -> Self { Self::new() }
}

impl LookupPaletteRenderer {
    /// Create a new renderer with the default palette
    pub fn new() -> Self {
        Self {
            palette: RgbPalette::default().into(),
            image: vec![],
        }
    }

    /// Load a custom RGB palette for rendering
    pub fn set_palette(&mut self, rgb_palette: RgbPalette) { 
        self.palette = rgb_palette.into(); 
    }
    
    /// Convert raw u16 palette indices to RgbColor using the loaded palette.
    /// 
    /// # Index Format
    /// The NES PPU outputs a 9-bit value for each pixel:
    /// - Bits 0-5 (6 bits): Color index (0-63 from the NES palette)
    /// - Bits 6-8 (3 bits): Emphasis bits (R, G, B emphasis from PPU mask register)
    /// - Bits 9-15: Unused (ignored by this function)
    /// 
    /// # Returns
    /// A vector of RgbColor values corresponding to the input indices.
    pub fn indices_to_rgb(&self, indices: &[u16]) -> Vec<RgbColor> {
        indices
            .iter()
            .map(|&index| self.palette.palette[(index as usize) & ((8 * 64) - 1)])
            .collect()
    }
    
    /// Convert a single index to RgbColor
    #[inline]
    pub fn index_to_rgb(&self, index: u16) -> RgbColor {
        self.palette.palette[(index as usize) & ((8 * 64) - 1)]
    }
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