//! Nametable viewer pane rendering

use crate::frontend::egui::textures::EmuTextures;

/// Render a single nametable viewer
///
/// # Arguments
/// * `ui` - The egui Ui context
/// * `emu_textures` - Emulator texture data
/// * `nametable_index` - Which nametable to render (0-3)
pub fn render_nametable(ui: &mut egui::Ui, emu_textures: &EmuTextures, nametable_index: usize) {
    if let Some(ref data) = emu_textures.nametable_data
        && let Some(ref textures) = emu_textures.tile_textures
    {
        if nametable_index >= data.tiles.len() {
            ui.label(format!("Nametable {} not available", nametable_index));
            return;
        }

        let nametable = &data.tiles[nametable_index];
        let available = ui.available_width();
        let base_size = 8.0;
        let tile_cols = 32;
        let logical_width = tile_cols as f32 * base_size;
        let scale = available / logical_width;
        let tex_size = egui::vec2(base_size, base_size) * scale;

        ui.label(format!(
            "Nametable {} (256x240 at {:.1}x scale)",
            nametable_index, scale
        ));

        egui::Grid::new(format!("nametable_{}", nametable_index))
            .num_columns(32)
            .min_row_height(scale * base_size)
            .min_col_width(scale * base_size)
            .max_col_width(scale * base_size)
            .spacing(egui::vec2(0.0, 0.0))
            .show(ui, |ui| {
                for (i, tile) in nametable.iter().enumerate() {
                    let col = (i % 32) / 4;
                    let row = (i / 32) / 4;
                    let attr_table_byte = data.palettes[nametable_index][row * 8 + col];

                    let shift = match (((i % 32) / 2) % 2, ((i / 32) / 2) % 2) {
                        (0, 0) => 0,
                        (1, 0) => 2,
                        (0, 1) => 4,
                        (1, 1) => 6,
                        _ => unreachable!(),
                    };


                    let palette = (attr_table_byte >> shift) & 0b0000_0011;
                    let texture = &textures[palette as usize][*tile as usize];
                    ui.image((texture.id(), tex_size));

                    if (i + 1) % 32 == 0 {
                        ui.end_row();
                    }
                }
            });
    } else {
        ui.label("Waiting for nametable data...");
    }
}
