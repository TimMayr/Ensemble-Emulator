//! Debug viewer export functionality for CLI.
//!
//! This module provides rendering and export capabilities for debug viewers:
//! - Pattern tables (2 tables of 16x16 tiles)
//! - Nametables (4 nametables of 32x30 tiles)
//! - Sprites (64 sprites from OAM)
//!
//! These are typically used for debugging and TAS development.

use std::path::Path;

use image::{ImageBuffer, Rgb, RgbImage};

use crate::emulation::messages::{
    EmulatorFetchable, NametableData, PaletteData, RgbColor, RgbPalette, TileData,
    NAMETABLE_COLS, NAMETABLE_COUNT, NAMETABLE_ROWS, TILE_COUNT,
};
use crate::emulation::nes::Nes;
use crate::emulation::ppu::TILE_SIZE;

// =============================================================================
// Constants
// =============================================================================

/// Pattern table display dimensions
const PATTERN_TABLE_TILE_COLS: usize = 16;
const PATTERN_TABLE_TILE_ROWS: usize = 16;
/// Gap between left and right pattern tables in pixels
const PATTERN_TABLE_GAP: usize = 16;
/// Single pattern table width in pixels (16 tiles × 8 pixels)
const SINGLE_PATTERN_TABLE_WIDTH: usize = PATTERN_TABLE_TILE_COLS * TILE_SIZE;
/// Single pattern table height in pixels (16 tiles × 8 pixels)
const SINGLE_PATTERN_TABLE_HEIGHT: usize = PATTERN_TABLE_TILE_ROWS * TILE_SIZE;
/// Total pattern table viewer width (2 tables + gap)
const PATTERN_TABLES_WIDTH: usize = SINGLE_PATTERN_TABLE_WIDTH * 2 + PATTERN_TABLE_GAP;
/// Total pattern table viewer height
const PATTERN_TABLES_HEIGHT: usize = SINGLE_PATTERN_TABLE_HEIGHT;

/// Single nametable width in pixels (32 tiles × 8 pixels)
const SINGLE_NAMETABLE_WIDTH: usize = NAMETABLE_COLS * TILE_SIZE;
/// Single nametable height in pixels (30 tiles × 8 pixels)
const SINGLE_NAMETABLE_HEIGHT: usize = NAMETABLE_ROWS * TILE_SIZE;
/// Nametable viewer width (2 nametables side by side)
const NAMETABLES_WIDTH: usize = SINGLE_NAMETABLE_WIDTH * 2;
/// Nametable viewer height (2 nametables stacked)
const NAMETABLES_HEIGHT: usize = SINGLE_NAMETABLE_HEIGHT * 2;

/// Sprite viewer dimensions (8 sprites per row × 8 rows = 64 sprites)
const SPRITE_VIEWER_COLS: usize = 8;
const SPRITE_VIEWER_ROWS: usize = 8;
/// Sprite viewer pixel dimensions (each sprite is 8×8 or 8×16 depending on mode)
const SPRITE_VIEWER_WIDTH: usize = SPRITE_VIEWER_COLS * TILE_SIZE;
/// Max height assuming 8×16 sprites
const SPRITE_VIEWER_HEIGHT_8X16: usize = SPRITE_VIEWER_ROWS * 16;

// =============================================================================
// Pattern Table Export
// =============================================================================

/// Render pattern tables to RGB pixel data.
///
/// Renders both pattern tables (left and right) side by side with a gap.
/// Uses palette 0 for rendering since pattern tables are palette-independent.
///
/// # Returns
/// A vector of RGB colors representing the rendered pattern tables.
pub fn render_pattern_tables(emu: &Nes) -> Vec<RgbColor> {
    let ppu = emu.ppu.borrow();

    // Get tile and palette data
    let tiles = match ppu.get_tiles_debug() {
        EmulatorFetchable::Tiles(Some(tiles)) => tiles,
        _ => return vec![(0, 0, 0); PATTERN_TABLES_WIDTH * PATTERN_TABLES_HEIGHT],
    };

    let palettes = match ppu.get_palettes_debug() {
        EmulatorFetchable::Palettes(Some(palettes)) => palettes,
        _ => return vec![(0, 0, 0); PATTERN_TABLES_WIDTH * PATTERN_TABLES_HEIGHT],
    };

    // Get RGB palette map from the emulator
    let rgb_palette = emu.get_rgb_palette();

    let mut pixels = vec![(0, 0, 0); PATTERN_TABLES_WIDTH * PATTERN_TABLES_HEIGHT];

    // Use palette 0 (first background palette) for pattern table display
    let palette_colors = &palettes.colors[0];

    // Render left pattern table (tiles 0-255)
    render_pattern_table_to_buffer(
        &mut pixels,
        &tiles[0..256],
        palette_colors,
        &rgb_palette,
        0, // x offset
        PATTERN_TABLES_WIDTH,
    );

    // Render right pattern table (tiles 256-511)
    render_pattern_table_to_buffer(
        &mut pixels,
        &tiles[256..512],
        palette_colors,
        &rgb_palette,
        SINGLE_PATTERN_TABLE_WIDTH + PATTERN_TABLE_GAP, // x offset after left table + gap
        PATTERN_TABLES_WIDTH,
    );

    pixels
}

/// Render a single pattern table (256 tiles) to a pixel buffer.
fn render_pattern_table_to_buffer(
    pixels: &mut [RgbColor],
    tiles: &[TileData],
    palette: &[u8; 4],
    rgb_palette: &RgbPalette,
    x_offset: usize,
    stride: usize,
) {
    for (tile_idx, tile) in tiles.iter().enumerate() {
        let tile_col = tile_idx % PATTERN_TABLE_TILE_COLS;
        let tile_row = tile_idx / PATTERN_TABLE_TILE_COLS;

        // Render each pixel of the 8×8 tile
        for py in 0..TILE_SIZE {
            for px in 0..TILE_SIZE {
                let bit = 63 - (py * TILE_SIZE + px);

                let lo = ((tile.plane_0 >> bit) & 1) as usize;
                let hi = ((tile.plane_1 >> bit) & 1) as usize;
                let color_index = lo | (hi << 1);

                let color = rgb_palette.colors[0][palette[color_index] as usize];

                let pixel_x = x_offset + tile_col * TILE_SIZE + px;
                let pixel_y = tile_row * TILE_SIZE + py;
                pixels[pixel_y * stride + pixel_x] = color;
            }
        }
    }
}

/// Export pattern tables to a PNG file.
pub fn export_pattern_tables(emu: &Nes, path: &Path) -> Result<(), String> {
    let pixels = render_pattern_tables(emu);

    let img: RgbImage = ImageBuffer::from_fn(
        PATTERN_TABLES_WIDTH as u32,
        PATTERN_TABLES_HEIGHT as u32,
        |x, y| {
            let (r, g, b) = pixels[(y as usize) * PATTERN_TABLES_WIDTH + (x as usize)];
            Rgb([r, g, b])
        },
    );

    img.save(path)
        .map_err(|e| format!("Failed to save pattern tables image: {}", e))
}

// =============================================================================
// Nametable Export
// =============================================================================

/// Render nametables to RGB pixel data.
///
/// Renders all 4 nametables in a 2×2 grid arrangement:
/// ```text
/// +--------+--------+
/// |  NT 0  |  NT 1  |
/// +--------+--------+
/// |  NT 2  |  NT 3  |
/// +--------+--------+
/// ```
pub fn render_nametables(emu: &Nes) -> Vec<RgbColor> {
    let ppu = emu.ppu.borrow();

    // Get tile data (pattern tables)
    let tiles = match ppu.get_tiles_debug() {
        EmulatorFetchable::Tiles(Some(tiles)) => tiles,
        _ => return vec![(0, 0, 0); NAMETABLES_WIDTH * NAMETABLES_HEIGHT],
    };

    // Get nametable data
    let nametables = match ppu.get_nametable_debug() {
        EmulatorFetchable::Nametables(Some(nt)) => nt,
        _ => return vec![(0, 0, 0); NAMETABLES_WIDTH * NAMETABLES_HEIGHT],
    };

    // Get palette data
    let palettes = match ppu.get_palettes_debug() {
        EmulatorFetchable::Palettes(Some(palettes)) => palettes,
        _ => return vec![(0, 0, 0); NAMETABLES_WIDTH * NAMETABLES_HEIGHT],
    };

    let rgb_palette = emu.get_rgb_palette();

    let mut pixels = vec![(0, 0, 0); NAMETABLES_WIDTH * NAMETABLES_HEIGHT];

    // Render each of the 4 nametables
    for nt_idx in 0..NAMETABLE_COUNT {
        let nt_col = nt_idx % 2;
        let nt_row = nt_idx / 2;
        let x_offset = nt_col * SINGLE_NAMETABLE_WIDTH;
        let y_offset = nt_row * SINGLE_NAMETABLE_HEIGHT;

        render_nametable_to_buffer(
            &mut pixels,
            &nametables,
            nt_idx,
            &tiles,
            &palettes,
            &rgb_palette,
            x_offset,
            y_offset,
            NAMETABLES_WIDTH,
        );
    }

    pixels
}

/// Render a single nametable to a pixel buffer.
fn render_nametable_to_buffer(
    pixels: &mut [RgbColor],
    nametables: &NametableData,
    nt_idx: usize,
    tiles: &[TileData; TILE_COUNT],
    palettes: &PaletteData,
    rgb_palette: &RgbPalette,
    x_offset: usize,
    y_offset: usize,
    stride: usize,
) {
    for row in 0..NAMETABLE_ROWS {
        for col in 0..NAMETABLE_COLS {
            // Get tile index from nametable
            let tile_id = nametables.tiles[nt_idx][row * NAMETABLE_COLS + col] as usize;

            // Get palette from attribute table
            let attr_byte_idx = (row / 4) * 8 + (col / 4);
            let attr_byte = nametables.palettes[nt_idx][attr_byte_idx];
            let attr_shift = ((row & 2) << 1) | (col & 2);
            let palette_idx = ((attr_byte >> attr_shift) & 0b11) as usize;

            // Get the palette colors (background palettes are 0-3)
            let palette = &palettes.colors[palette_idx];
            let tile = &tiles[tile_id];

            // Render each pixel of the tile
            for py in 0..TILE_SIZE {
                for px in 0..TILE_SIZE {
                    let bit = 63 - (py * TILE_SIZE + px);

                    let lo = ((tile.plane_0 >> bit) & 1) as usize;
                    let hi = ((tile.plane_1 >> bit) & 1) as usize;
                    let color_index = lo | (hi << 1);

                    let color = rgb_palette.colors[0][palette[color_index] as usize];

                    let pixel_x = x_offset + col * TILE_SIZE + px;
                    let pixel_y = y_offset + row * TILE_SIZE + py;
                    pixels[pixel_y * stride + pixel_x] = color;
                }
            }
        }
    }
}

/// Export nametables to a PNG file.
pub fn export_nametables(emu: &Nes, path: &Path) -> Result<(), String> {
    let pixels = render_nametables(emu);

    let img: RgbImage = ImageBuffer::from_fn(
        NAMETABLES_WIDTH as u32,
        NAMETABLES_HEIGHT as u32,
        |x, y| {
            let (r, g, b) = pixels[(y as usize) * NAMETABLES_WIDTH + (x as usize)];
            Rgb([r, g, b])
        },
    );

    img.save(path)
        .map_err(|e| format!("Failed to save nametables image: {}", e))
}

// =============================================================================
// Sprite Export
// =============================================================================

/// Render sprites (OAM) to RGB pixel data.
///
/// Displays all 64 sprites in an 8×8 grid. Each cell shows the sprite tile
/// with its assigned palette. Sprites are rendered in OAM order (not by priority).
///
/// For 8×16 sprite mode, only the first tile is shown.
pub fn render_sprites(emu: &Nes) -> Vec<RgbColor> {
    let ppu = emu.ppu.borrow();

    // Get sprite height from PPU control register
    let sprite_height = if (ppu.ctrl_register & 0x20) != 0 {
        16
    } else {
        8
    };

    let viewer_height = if sprite_height == 16 {
        SPRITE_VIEWER_HEIGHT_8X16
    } else {
        SPRITE_VIEWER_ROWS * TILE_SIZE
    };

    // Get tile data
    let tiles = match ppu.get_tiles_debug() {
        EmulatorFetchable::Tiles(Some(tiles)) => tiles,
        _ => return vec![(0, 0, 0); SPRITE_VIEWER_WIDTH * viewer_height],
    };

    // Get palette data
    let palettes = match ppu.get_palettes_debug() {
        EmulatorFetchable::Palettes(Some(palettes)) => palettes,
        _ => return vec![(0, 0, 0); SPRITE_VIEWER_WIDTH * viewer_height],
    };

    let rgb_palette = emu.get_rgb_palette();

    // Get OAM data
    let oam = ppu.oam.get_memory_debug(None);

    let mut pixels = vec![(0, 0, 0); SPRITE_VIEWER_WIDTH * viewer_height];

    // Sprite pattern table base (bit 3 of CTRL register for 8×8 sprites)
    let sprite_pattern_table = if sprite_height == 8 {
        if (ppu.ctrl_register & 0x08) != 0 {
            256
        } else {
            0
        }
    } else {
        0 // For 8×16 sprites, the pattern table is determined per-sprite
    };

    // Render each of the 64 sprites
    for sprite_idx in 0..64 {
        let oam_offset = sprite_idx * 4;
        if oam_offset + 3 >= oam.len() {
            break;
        }

        let tile_idx_raw = oam[oam_offset + 1] as usize;
        let attributes = oam[oam_offset + 2];

        // Sprite palette (palettes 4-7, bits 0-1 of attributes)
        let palette_idx = 4 + (attributes & 0b11) as usize;
        let palette = &palettes.colors[palette_idx];

        // Horizontal/vertical flip (bits 6-7)
        let flip_h = (attributes & 0x40) != 0;
        let flip_v = (attributes & 0x80) != 0;

        // Calculate tile index
        let tile_idx = if sprite_height == 16 {
            // For 8×16 sprites, bit 0 of tile index selects pattern table
            let bank = (tile_idx_raw & 1) * 256;
            bank + (tile_idx_raw & 0xFE)
        } else {
            sprite_pattern_table + tile_idx_raw
        };

        let tile = &tiles[tile_idx.min(TILE_COUNT - 1)];

        // Calculate position in the viewer grid
        let grid_col = sprite_idx % SPRITE_VIEWER_COLS;
        let grid_row = sprite_idx / SPRITE_VIEWER_COLS;
        let x_offset = grid_col * TILE_SIZE;
        let y_offset = grid_row * sprite_height;

        // Render the sprite tile
        render_sprite_tile(
            &mut pixels,
            tile,
            palette,
            &rgb_palette,
            x_offset,
            y_offset,
            SPRITE_VIEWER_WIDTH,
            flip_h,
            flip_v,
        );

        // For 8×16 sprites, render the second tile
        if sprite_height == 16 && tile_idx + 1 < TILE_COUNT {
            let tile2 = &tiles[tile_idx + 1];
            render_sprite_tile(
                &mut pixels,
                tile2,
                palette,
                &rgb_palette,
                x_offset,
                y_offset + TILE_SIZE,
                SPRITE_VIEWER_WIDTH,
                flip_h,
                flip_v,
            );
        }
    }

    pixels
}

/// Render a single sprite tile to a pixel buffer with flip support.
fn render_sprite_tile(
    pixels: &mut [RgbColor],
    tile: &TileData,
    palette: &[u8; 4],
    rgb_palette: &RgbPalette,
    x_offset: usize,
    y_offset: usize,
    stride: usize,
    flip_h: bool,
    flip_v: bool,
) {
    for py in 0..TILE_SIZE {
        for px in 0..TILE_SIZE {
            let src_px = if flip_h { TILE_SIZE - 1 - px } else { px };
            let src_py = if flip_v { TILE_SIZE - 1 - py } else { py };

            let bit = 63 - (src_py * TILE_SIZE + src_px);

            let lo = ((tile.plane_0 >> bit) & 1) as usize;
            let hi = ((tile.plane_1 >> bit) & 1) as usize;
            let color_index = lo | (hi << 1);

            // Color index 0 is transparent for sprites, but we'll show it as the background color
            let color = rgb_palette.colors[0][palette[color_index] as usize];

            let pixel_x = x_offset + px;
            let pixel_y = y_offset + py;
            if pixel_y < pixels.len() / stride {
                pixels[pixel_y * stride + pixel_x] = color;
            }
        }
    }
}

/// Export sprites to a PNG file.
pub fn export_sprites(emu: &Nes, path: &Path) -> Result<(), String> {
    let ppu = emu.ppu.borrow();
    let sprite_height = if (ppu.ctrl_register & 0x20) != 0 {
        16
    } else {
        8
    };
    drop(ppu);

    let pixels = render_sprites(emu);

    let viewer_height = if sprite_height == 16 {
        SPRITE_VIEWER_HEIGHT_8X16
    } else {
        SPRITE_VIEWER_ROWS * TILE_SIZE
    };

    let img: RgbImage = ImageBuffer::from_fn(
        SPRITE_VIEWER_WIDTH as u32,
        viewer_height as u32,
        |x, y| {
            let (r, g, b) = pixels[(y as usize) * SPRITE_VIEWER_WIDTH + (x as usize)];
            Rgb([r, g, b])
        },
    );

    img.save(path)
        .map_err(|e| format!("Failed to save sprites image: {}", e))
}

// =============================================================================
// Video Export Helpers
// =============================================================================

/// Get the dimensions for pattern table export.
pub fn pattern_tables_dimensions() -> (u32, u32) {
    (PATTERN_TABLES_WIDTH as u32, PATTERN_TABLES_HEIGHT as u32)
}

/// Get the dimensions for nametables export.
pub fn nametables_dimensions() -> (u32, u32) {
    (NAMETABLES_WIDTH as u32, NAMETABLES_HEIGHT as u32)
}

/// Get the dimensions for sprite export (based on sprite height mode).
pub fn sprites_dimensions(emu: &Nes) -> (u32, u32) {
    let ppu = emu.ppu.borrow();
    let sprite_height = if (ppu.ctrl_register & 0x20) != 0 {
        16
    } else {
        8
    };
    let viewer_height = if sprite_height == 16 {
        SPRITE_VIEWER_HEIGHT_8X16
    } else {
        SPRITE_VIEWER_ROWS * TILE_SIZE
    };
    (SPRITE_VIEWER_WIDTH as u32, viewer_height as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern_table_dimensions() {
        let (w, h) = pattern_tables_dimensions();
        assert_eq!(w, 272); // 128 + 16 + 128
        assert_eq!(h, 128); // 16 * 8
    }

    #[test]
    fn test_nametable_dimensions() {
        let (w, h) = nametables_dimensions();
        assert_eq!(w, 512); // 256 * 2
        assert_eq!(h, 480); // 240 * 2
    }
}
