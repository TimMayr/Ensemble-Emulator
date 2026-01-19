//! Pattern table viewer pane rendering

use crate::frontend::egui::config::AppConfig;
use crate::frontend::egui::textures::EmuTextures;
use crate::frontend::egui::ui::draw_pattern_table;

/// Render both pattern tables side by side
pub fn render_pattern_table(ui: &mut egui::Ui, config: &AppConfig, emu_textures: &EmuTextures) {
    if emu_textures.tile_textures.is_some()
        && let Some(palettes) = &emu_textures.palette_data
    {
        let full_width = ui.available_width();
        let half_width = (full_width - ui.spacing().item_spacing.x * 3.0) * 0.5;

        ui.label(format!(
            "Pattern Tables (128x128x2 at {:.1}x scale)",
            half_width / 128.0
        ));

        let selected_palette = palettes.colors[config.view_config.debug_active_palette];
        let transformed_palette = selected_palette
            .map(|color_index| config.view_config.palette_rgb_data.colors[0][color_index as usize]);

        ui.horizontal_top(|ui| {
            ui.allocate_ui(egui::vec2(half_width, half_width), |ui| {
                draw_pattern_table(
                    ui,
                    0,
                    emu_textures,
                    config.view_config.debug_active_palette,
                    transformed_palette,
                );
            });

            ui.separator();
            ui.allocate_ui(egui::vec2(half_width, half_width), |ui| {
                draw_pattern_table(
                    ui,
                    1,
                    emu_textures,
                    config.view_config.debug_active_palette,
                    transformed_palette,
                );
            });
        });
    } else {
        ui.label("Waiting for pattern table data...");
    }
}
