//! Nametable viewer pane rendering

use crate::emulation::messages::{NAMETABLE_HEIGHT, NAMETABLE_WIDTH};
use crate::frontend::egui::textures::EmuTextures;

/// Render the nametable viewer
pub fn render_nametable(ui: &mut egui::Ui, emu_textures: &EmuTextures) {
    if let Some(ref data) = emu_textures.nametable_data
        && let Some(ref textures) = emu_textures.tile_textures
    {
        let available = ui.available_width();
        let base_size = 4.0;
        let logical_width = 16.0 * base_size;
        let scale = available / logical_width;
        let tex_size = egui::vec2(base_size, base_size) * scale;

        ui.label(format!(
            "Nametables ({}x{} at {:.1}x)",
            NAMETABLE_WIDTH, NAMETABLE_HEIGHT, scale
        ));

        egui::Grid::new("nametables")
            .num_columns(2)
            .spacing(egui::vec2(0.0, 0.0))
            .show(ui, |ui| {
                for (nametable_id, nametable) in data.tiles.iter().enumerate() {
                    egui::Grid::new(format!("nametable_{}", nametable_id))
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
                    if (nametable_id + 1) % 2 == 0 {
                        ui.end_row()
                    }
                }
            });
    } else {
        ui.label("Waiting for nametable data...");
    }
}
