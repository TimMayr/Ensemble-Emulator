use std::collections::HashMap;
use std::time::Instant;

use egui::{ColorImage, Context, TextureHandle, TextureOptions};

use crate::emulation::messages::{NAMETABLE_HEIGHT, NAMETABLE_WIDTH, PaletteData, PatternTableViewerData, SpriteData, TOTAL_OUTPUT_HEIGHT, TOTAL_OUTPUT_WIDTH, TileData, NametableData};
use crate::emulation::ppu::TILE_SIZE;

/// Texture storage and management for the emulator display
#[derive(Eq, PartialEq, Clone)]
pub struct EmuTextures {
    pub current_frame: Option<Vec<u32>>,
    pub emulator_texture: Option<TextureHandle>,
    pub pattern_table_data: Option<Box<PatternTableViewerData>>,
    pub nametable_data: Option<Box<NametableData>>,
    pub sprite_viewer_data: Option<Vec<SpriteData>>,
    pub sprite_viewer_textures: Option<HashMap<u8, TextureHandle>>,
    pub last_pattern_table_request: Instant,
    pub last_nametable_request: Instant,
    pub last_frame_request: Instant,
    pub last_sprite_viewer_request: Instant,
    pub sprite_height: usize,
    pub tile_textures: Vec<Vec<TextureHandle>>,
}

impl Default for EmuTextures {
    fn default() -> Self {
        Self {
            current_frame: Default::default(),
            emulator_texture: Default::default(),
            pattern_table_data: Default::default(),
            nametable_data: Default::default(),
            sprite_viewer_data: Default::default(),
            sprite_viewer_textures: Default::default(),
            last_pattern_table_request: Instant::now(),
            last_nametable_request: Instant::now(),
            last_frame_request: Instant::now(),
            last_sprite_viewer_request: Instant::now(),
            sprite_height: 8,
            tile_textures: vec![vec![], vec![]],
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
            self.emulator_texture = Some(texture);
        }
    }

    /// Create a texture for a single tile
    pub fn get_texture_for_tile(
        tile: &TileData,
        palette: &PaletteData,
        ctx: &Context,
    ) -> TextureHandle {
        let mut data = [0u32; 64];

        for (i, color) in data.iter_mut().enumerate() {
            let bit = 63 - i;

            let lo = ((tile.plane_0 >> bit) & 1) as usize;
            let hi = ((tile.plane_1 >> bit) & 1) as usize;

            let color_index = lo | (hi << 1);
            *color = palette.colors[color_index];
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
    pub fn update_tile_textures(&mut self, ctx: &Context) {
        if let Some(ref data) = self.pattern_table_data {
            let palette = data.palette;

            self.tile_textures[0].clear();
            self.tile_textures[1].clear();
            for x in data.left.tiles {
                let texture = Self::get_texture_for_tile(&x, &palette, ctx);
                self.tile_textures[0].push(texture);
            }

            for x in data.right.tiles {
                let texture = Self::get_texture_for_tile(&x, &palette, ctx);
                self.tile_textures[1].push(texture);
            }
        }
    }
}
