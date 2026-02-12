//! Dynamic renderer dispatch using an enum.
//!
//! This module provides `RendererKind`, an enum that wraps all available renderer
//! implementations and allows runtime switching between them while maintaining
//! serializability for configuration persistence.

use std::fmt::{Debug, Formatter};
use serde::{Deserialize, Serialize};
use ensemble_lockstep::emulation::screen_renderer::{RgbColor, RgbPalette, ScreenRenderer};

use crate::LookupPaletteRenderer;

/// An enum wrapping all available renderer implementations.
/// 
/// This allows runtime switching between renderer types while maintaining
/// serializability. Each variant holds the complete state of its renderer,
/// including any loaded palettes.
/// 
/// # Example
/// 
/// ```ignore
/// use ensemble_gown::RendererKind;
/// 
/// let mut renderer = RendererKind::default();
/// renderer.set_palette(my_palette);
/// let rgb_colors = renderer.buffer_to_image(&pixel_indices);
/// 
/// // Switch to a different renderer at runtime
/// renderer = RendererKind::LookupPalette(LookupPaletteRenderer::default());
/// ```
#[derive(Clone, Serialize, Deserialize)]
pub enum RendererKind {
    /// Lookup table-based palette renderer (fast, default).
    LookupPalette(LookupPaletteRenderer),
    // Future renderer variants can be added here, e.g.:
    // CrtFilter(CrtFilterRenderer),
    // CompositeDecoder(CompositeDecoderRenderer),
}

impl Default for RendererKind {
    fn default() -> Self {
        Self::LookupPalette(LookupPaletteRenderer::default())
    }
}

impl Debug for RendererKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LookupPalette(_) => f.write_str("LookupPaletteRenderer"),
        }
    }
}

impl RendererKind {
    /// Returns a human-readable display name for the current renderer type.
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::LookupPalette(_) => "Lookup Palette Table",
        }
    }

    /// Returns a brief description of the current renderer.
    pub fn description(&self) -> &'static str {
        match self {
            Self::LookupPalette(_) => "Fast lookup table-based renderer using the selected palette file",
        }
    }

    /// Returns a list of all available renderer kinds with their default configurations.
    pub fn all_variants() -> Vec<RendererKind> {
        vec![
            Self::LookupPalette(LookupPaletteRenderer::default()),
        ]
    }

    /// Returns an identifier string for the renderer type (used for persistence).
    pub fn type_id(&self) -> &'static str {
        match self {
            Self::LookupPalette(_) => "lookup_palette",
        }
    }

    /// Convert a buffer of palette indices to RGB colors.
    /// 
    /// Delegates to the wrapped renderer implementation.
    pub fn buffer_to_image(&mut self, buffer: &[u16]) -> &[RgbColor] {
        match self {
            Self::LookupPalette(r) => r.buffer_to_image(buffer),
        }
    }

    /// Set the palette to use for rendering.
    /// 
    /// Delegates to the wrapped renderer implementation.
    pub fn set_palette(&mut self, palette: RgbPalette) {
        match self {
            Self::LookupPalette(r) => r.set_palette(palette),
        }
    }
}
