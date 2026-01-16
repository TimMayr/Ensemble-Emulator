//! Nametable viewer pane rendering

use crate::frontend::egui::textures::EmuTextures;

/// Render all 4 nametables in a 2x2 grid layout
pub fn render_nametable(ui: &mut egui::Ui, emu_textures: &EmuTextures) {
    if let Some(ref data) = emu_textures.nametable_data
        && let Some(ref textures) = emu_textures.tile_textures
    {
        let available = ui.available_width();
        // Each nametable is 32x30 tiles, we show 2 side by side
        let base_size = 8.0;
        let tile_cols = 32;
        let logical_width = (tile_cols as f32 * base_size) * 2.0; // 2 nametables side by side
        let scale = available / logical_width;
        let tex_size = egui::vec2(base_size, base_size) * scale;

        ui.label(format!(
            "Nametables (256x240 x4 at {:.1}x scale)",
            scale
        ));

        // Render 4 nametables in a 2x2 grid
        egui::Grid::new("nametables_container")
            .num_columns(2)
            .spacing(egui::vec2(4.0, 4.0))
            .show(ui, |ui| {
                for nametable_index in 0..4 {
                    if nametable_index < data.tiles.len() {
                        let nametable = &data.tiles[nametable_index];
                        
                        egui::Grid::new(format!("nametable_{}", nametable_index))
                            .num_columns(32)
                            .min_row_height(scale * base_size)
                            .min_col_width(scale * base_size)
                            .max_col_width(scale * base_size)
                            .spacing(egui::vec2(0.0, 0.0))
                            .show(ui, |ui| {
                                for (i, tile) in nametable.iter().enumerate() {
                                    let texture = &textures[0][*tile as usize];
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
