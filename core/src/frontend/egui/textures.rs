use std::time::Instant;

use egui::{ColorImage, Context, TextureHandle, TextureOptions};

use crate::emulation::messages::{
    NametableData, PALETTE_COUNT, PaletteData, RgbColor, RgbPalette, TILE_COUNT,
    TOTAL_OUTPUT_HEIGHT, TOTAL_OUTPUT_WIDTH, TileData,
};
use crate::emulation::ppu::TILE_SIZE;

/// Texture storage and management for the emulator display
#[derive(Eq, PartialEq, Clone)]
pub struct EmuTextures {
    pub current_frame: Option<Vec<RgbColor>>,
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
    /// Convert RgbColor pixel data to egui ColorImage
    pub fn rgb_to_color_image(data: &[RgbColor], width: usize, height: usize) -> ColorImage {
        let mut pixels = Vec::with_capacity(width * height);
        for &(r, g, b) in data {
            pixels.push(egui::Color32::from_rgb(r, g, b));
        }
        ColorImage {
            size: [width, height],
            source_size: Default::default(),
            pixels,
        }
    }

    /// Convert u32 ARGB pixel data to egui ColorImage (RGBA)
    /// Kept for backward compatibility with tile rendering
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
                Self::rgb_to_color_image(frame.as_ref(), TOTAL_OUTPUT_WIDTH, TOTAL_OUTPUT_HEIGHT);

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
        let mut data: [RgbColor; 64] = [(0, 0, 0); 64];

        for (i, color) in data.iter_mut().enumerate() {
            let bit = 63 - i;

            let lo = ((tile.plane_0 >> bit) & 1) as usize;
            let hi = ((tile.plane_1 >> bit) & 1) as usize;

            let color_index = lo | (hi << 1);
            *color = rgb_palette_map.colors[0][palette[color_index] as usize];
        }

        let image = Self::rgb_to_color_image(data.as_ref(), TILE_SIZE, TILE_SIZE);

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
    /// If `palette_index` is Some, only update textures for that specific palette
    /// If `palette_index` is None, update all palette variations
    /// If `tile_index` is Some, only update that specific tile (across all requested palettes)
    pub fn update_tile_textures(
        &mut self,
        ctx: &Context,
        rgb_palette_map: &RgbPalette,
        palette_index: Option<usize>,
        tile_index: Option<usize>,
    ) {
        if let Some(ref palettes) = self.palette_data
            && let Some(ref tiles) = self.tile_data
        {
            // Get or create the tile textures array
            let tile_textures = self.tile_textures.get_or_insert_with(|| {
                // Initialize with empty placeholders - they'll be filled below
                std::array::from_fn(|_| {
                    std::array::from_fn(|_| {
                        ctx.load_texture(
                            "placeholder",
                            ColorImage::new([1, 1], vec![egui::Color32::TRANSPARENT]),
                            TextureOptions::default(),
                        )
                    })
                })
            });

            // Determine which palettes to update
            let palette_range: Box<dyn Iterator<Item = usize>> = match palette_index {
                Some(idx) if idx < palettes.colors.len() => Box::new(std::iter::once(idx)),
                _ => Box::new(0..palettes.colors.len()),
            };

            for palette_idx in palette_range {
                let palette = palettes.colors[palette_idx];

                // Determine which tiles to update
                let tile_range: Box<dyn Iterator<Item = usize>> = match tile_index {
                    Some(idx) if idx < tiles.len() => Box::new(std::iter::once(idx)),
                    _ => Box::new(0..tiles.len()),
                };

                for tile_idx in tile_range {
                    let texture = Self::get_texture_for_tile(
                        &tiles[tile_idx],
                        &palette,
                        rgb_palette_map,
                        ctx,
                    );
                    tile_textures[palette_idx][tile_idx] = texture;
                }
            }
        }
    }

    /// Force rebuild all tile textures - call when pattern tables viewer becomes visible
    pub fn force_rebuild_all_tiles(&mut self, ctx: &Context, rgb_palette_map: &RgbPalette) {
        // Clear existing textures to force full rebuild
        self.tile_textures = None;
        self.update_tile_textures(ctx, rgb_palette_map, None, None);
    }
}
