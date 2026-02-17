use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::util::{Hashable, ToBytes, compute_hash};

/// Trait for rendering palette indices to RGB colors.
///
/// Implementations must be serializable to enable persistence of renderer state.
pub trait ScreenRenderer: Debug {
    /// Convert a buffer of palette indices to RGB colors.
    fn buffer_to_image(&mut self, buffer: &[u16]) -> &[RgbColor];

    /// Set the palette to use for rendering.
    ///
    /// Called when the user loads a new palette file.
    fn set_palette(&mut self, palette: RgbPalette);

    fn get_name(&self) -> &str;

    fn get_width(&self) -> usize;
    fn get_height(&self) -> usize;
}

pub struct NoneRenderer {
    image: [RgbColor; 1],
}

impl NoneRenderer {
    pub fn new() -> Self {
        NoneRenderer {
            image: [RgbColor::default()],
        }
    }
}

impl Debug for NoneRenderer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { f.write_str("None") }
}

impl ScreenRenderer for NoneRenderer {
    fn buffer_to_image(&mut self, _: &[u16]) -> &[RgbColor] { &self.image }

    fn set_palette(&mut self, _: RgbPalette) {}

    fn get_name(&self) -> &str { "None" }

    fn get_width(&self) -> usize { 1 }

    fn get_height(&self) -> usize { 1 }
}

inventory::submit! {
    RendererRegistration {

    name: "None",
        factory: || Box::new(NoneRenderer::new())}
}

pub struct RendererRegistration {
    pub name: &'static str,
    pub factory: fn() -> Box<dyn ScreenRenderer>,
}

inventory::collect!(RendererRegistration);

pub fn create_renderer(name: Option<&str>) -> Box<dyn ScreenRenderer> {
    if let Some(name) = name {
        inventory::iter::<RendererRegistration>
            .into_iter()
            .find(|r| r.name == name)
            .map(|r| (r.factory)())
            .unwrap_or_else(|| Box::new(NoneRenderer::new()))
    } else {
        Box::new(NoneRenderer::new())
    }
}

pub fn get_all_renderers() -> Vec<&'static str> {
    let mut renderers: Vec<&'static str> = inventory::iter::<RendererRegistration>
        .into_iter()
        .map(|r| r.name)
        .collect::<Vec<&'static str>>();

    renderers.sort();

    renderers
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Serialize, Deserialize, Default)]
pub struct RgbColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RgbColor {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self {
            r,
            g,
            b,
        }
    }

    pub fn to_tuple(self) -> (u8, u8, u8) { (self.r, self.g, self.b) }
}

impl From<(u8, u8, u8)> for RgbColor {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        Self {
            r,
            g,
            b,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct RgbPalette {
    pub colors: [[RgbColor; 64]; 8],
}

impl Default for RgbPalette {
    fn default() -> Self { parse_palette_from_file(None, None) }
}

static DEFAULT_PALETTE: &[u8] = include_bytes!("../../../core/assets/2C02G_wiki.pal");
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

    let mut colors: [[RgbColor; 64]; 8] = [[RgbColor::default(); 64]; 8];

    for (palette_index, palette) in colors.iter_mut().enumerate() {
        for (color_index, color) in palette.iter_mut().enumerate() {
            let base_index = ((palette_index * 64) + color_index) * 3;
            let read_color: RgbColor = if let (Some(r), Some(g), Some(b)) = (
                data.get(base_index),
                data.get(base_index + 1),
                data.get(base_index + 2),
            ) {
                RgbColor {
                    r: *r,
                    g: *g,
                    b: *b,
                }
            } else {
                RgbColor {
                    r: DEFAULT_PALETTE[base_index],
                    g: DEFAULT_PALETTE[base_index + 1],
                    b: DEFAULT_PALETTE[base_index + 2],
                }
            };

            *color = read_color;
        }
    }

    RgbPalette {
        colors,
    }
}

impl ToBytes for RgbPalette {
    fn to_bytes(&self, _: Option<String>) -> Vec<u8> {
        self.colors
            .iter()
            .flatten()
            .flat_map(|c| [c.r, c.g, c.b])
            .collect()
    }
}

impl Hashable for RgbPalette {
    /// Compute a fast hash of the given data for change detection.
    /// Uses FNV-1a algorithm which is fast and has good distribution.
    #[inline]
    fn hash(&self) -> u64 {
        let bytes = self.to_bytes(None);
        compute_hash(&bytes[..])
    }
}
