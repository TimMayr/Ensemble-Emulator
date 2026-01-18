//! Nametable viewer pane rendering

use crate::frontend::egui::textures::EmuTextures;
use crate::frontend::egui::ui::calculate_integer_scale;

/// Single nametable native dimensions: 32x30 tiles at 8px each = 256x240 pixels
/// Full nametable grid: 2x2 nametables = 512x480 pixels (without spacing)
const NAMETABLE_GRID_WIDTH: f32 = 512.0;
const NAMETABLE_GRID_HEIGHT: f32 = 480.0;

/// Render all 4 nametables in a 2x2 grid layout
pub fn render_nametable(ui: &mut egui::Ui, emu_textures: &EmuTextures) {
    if let Some(ref data) = emu_textures.nametable_data
        && let Some(ref textures) = emu_textures.tile_textures
    {
        let available = ui.available_size();
        // Account for spacing between nametables (4px spacing in 2x2 grid = 4px horizontal, 4px vertical)
        let spacing = 4.0;
        let available_width = available.x - spacing;
        let available_height = available.y - spacing;

        // Calculate integer scale for the full 2x2 nametable grid
        let scale = calculate_integer_scale(
            NAMETABLE_GRID_WIDTH,
            NAMETABLE_GRID_HEIGHT,
            available_width,
            available_height,
        );
        let scale_f32 = scale as f32;

        // Each tile is 8x8 pixels, scaled by the integer factor
        let tile_size = 8.0 * scale_f32;
        let tex_size = egui::vec2(tile_size, tile_size);

        ui.label(format!("Nametables (256x240 x4 at {}x scale)", scale));

        // Render 4 nametables in a 2x2 grid
        egui::Grid::new("nametables_container")
            .num_columns(2)
            .spacing(egui::vec2(spacing, spacing))
            .show(ui, |ui| {
                for nametable_index in 0..4 {
                    if nametable_index < data.tiles.len() {
                        let nametable = &data.tiles[nametable_index];

                        egui::Grid::new(format!("nametable_{}", nametable_index))
                            .num_columns(32)
                            .min_row_height(tile_size)
                            .min_col_width(tile_size)
                            .max_col_width(tile_size)
                            .spacing(egui::vec2(0.0, 0.0))
                            .show(ui, |ui| {
                                for (i, tile) in nametable.iter().enumerate() {
                                    // Calculate tile position in the 32x30 nametable
                                    let tile_col = i % 32;
                                    let tile_row = i / 32;

                                    // Each attribute byte covers a 4x4 tile area (32x32 pixels)
                                    // Attribute table is 8 bytes wide (8 * 4 = 32 tiles)
                                    let attr_col = tile_col / 4;
                                    let attr_row = tile_row / 4;
                                    let attr_table_byte =
                                        data.palettes[nametable_index][attr_row * 8 + attr_col];

                                    // Each attribute byte contains 4 palette indices (2 bits each)
                                    // for the four 2x2 tile quadrants within the 4x4 area:
                                    //   bits 0-1: top-left     bits 2-3: top-right
                                    //   bits 4-5: bottom-left  bits 6-7: bottom-right
                                    // Use bit 1 of tile position to determine quadrant
                                    let shift = ((tile_row & 2) << 1) | (tile_col & 2);

                                    let palette = (attr_table_byte >> shift) & 0b11;
                                    let texture = &textures[palette as usize][*tile as usize];
                                    ui.image((texture.id(), tex_size));

                                    if (i + 1) % 32 == 0 {
                                        ui.end_row();
                                    }
                                }
                            });
                    }

                    // End row after every 2 nametables (0,1 on first row, 2,3 on second)
                    if nametable_index % 2 == 1 {
                        ui.end_row();
                    }
                }
            });
    } else {
        ui.label("Waiting for nametable data...");
    }
}
