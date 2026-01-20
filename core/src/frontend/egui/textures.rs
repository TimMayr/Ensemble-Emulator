use std::time::Instant;

use egui::{ColorImage, Context, TextureHandle, TextureOptions};

use crate::emulation::messages::{
    NametableData, PALETTE_COUNT, PaletteData, RgbPalette, TILE_COUNT, TOTAL_OUTPUT_HEIGHT,
    TOTAL_OUTPUT_WIDTH, TileData,
};
use crate::emulation::ppu::TILE_SIZE;

/// Texture storage and management for the emulator display
#[derive(Eq, PartialEq, Clone)]
pub struct EmuTextures {
    pub current_frame: Option<Vec<u32>>,
    pub frame_texture: Option<TextureHandle>,
    pub last_debug_request: Instant,
    pub last_frame_request: Instant,
    pub sprite_height: usize,
    pub tile_textures: Option<[[TextureHandle; TILE_COUNT]; PALETTE_COUNT]>,
    pub palette_data: Option<Box<PaletteData>>,
    pub tile_data: Option<Box<[TileData; TILE_COUNT]>>,
    pub nametable_data: Option<Box<NametableData>>,
}

impl Default for EmuTextures {
    fn default() -> Self {
        Self {
            current_frame: Default::default(),
            frame_texture: Default::default(),
            last_debug_request: Instant::now(),
            last_frame_request: Instant::now(),
            sprite_height: 8,
            tile_textures: None,
            palette_data: None,
            tile_data: None,
            nametable_data: None,
        }
    }
}

impl EmuTextures {
    /// Convert u32 ARGB pixel data to egui ColorImage (RGBA)
    pub fn u32_to_color_image(data: &[u32], width: usize, height: usize) -> ColorImage {
        let mut pixels = Vec::with_capacity(width * height);
        for &pixel in data {
            // Input is ARGB (0xAARRGGBB), we need RGBA for egui
            let a = ((pixel >> 24) & 0xFF) as u8;
            let r = ((pixel >> 16) & 0xFF) as u8;
            let g = ((pixel >> 8) & 0xFF) as u8;
            let b = (pixel & 0xFF) as u8;
            pixels.push(egui::Color32::from_rgba_unmultiplied(r, g, b, a));
        }
        ColorImage {
            size: [width, height],
            source_size: Default::default(),
            pixels,
        }
    }

    /// Update the main emulator display texture
    pub fn update_emulator_texture(&mut self, ctx: &Context) {
        if let Some(ref frame) = self.current_frame {
            let image =
                Self::u32_to_color_image(frame.as_ref(), TOTAL_OUTPUT_WIDTH, TOTAL_OUTPUT_HEIGHT);

            let texture = ctx.load_texture(
                "emulator_output",
                image,
                TextureOptions {
                    magnification: egui::TextureFilter::Nearest,
                    minification: egui::TextureFilter::Nearest,
                    ..Default::default()
                },
            );
            self.frame_texture = Some(texture);
        }
    }

    /// Create a texture for a single tile
    pub fn get_texture_for_tile(
        tile: &TileData,
        palette: &[u8; 4],
        rgb_palette_map: &RgbPalette,
        ctx: &Context,
    ) -> TextureHandle {
        let mut data = [0u32; 64];

        for (i, color) in data.iter_mut().enumerate() {
            let bit = 63 - i;

            let lo = ((tile.plane_0 >> bit) & 1) as usize;
            let hi = ((tile.plane_1 >> bit) & 1) as usize;

            let color_index = lo | (hi << 1);
            *color = rgb_palette_map.colors[0][palette[color_index] as usize];
        }

        let image = Self::u32_to_color_image(data.as_ref(), TILE_SIZE, TILE_SIZE);

        ctx.load_texture(
            format!("tile_{:16X}", tile.address),
            image,
            TextureOptions {
                magnification: egui::TextureFilter::Nearest,
                minification: egui::TextureFilter::Nearest,
                ..Default::default()
            },
        )
    }

    /// Update the pattern table textures
    pub fn update_tile_textures(&mut self, ctx: &Context, rgb_palette_map: &RgbPalette) {
        if let Some(ref palettes) = self.palette_data
            && let Some(ref tiles) = self.tile_data
        {
            let mut tile_textures: Vec<[TextureHandle; TILE_COUNT]> =
                Vec::with_capacity(palettes.colors.len());

            for palette in palettes.colors {
                let mut palette_tile_textures = Vec::with_capacity(tiles.len());
                for tile in tiles.iter() {
                    let texture = Self::get_texture_for_tile(tile, &palette, rgb_palette_map, ctx);
                    palette_tile_textures.push(texture);
                }
                tile_textures.push(palette_tile_textures.try_into().unwrap_or_else(|_| {
                    panic!("Fatal internal error encountered while trying to render debug views")
                }))
            }

            self.tile_textures = Some(tile_textures.try_into().unwrap_or_else(|_| {
                panic!("Fatal internal error encountered while trying to render debug views")
            }));
        }
    }
}
