//! Pattern table viewer pane rendering

use crossbeam_channel::Sender;
use crate::emulation::messages::FrontendMessage;
use crate::frontend::egui::config::AppConfig;
use crate::frontend::egui::textures::EmuTextures;
use crate::frontend::egui::ui::draw_pattern_table;

/// Render both pattern tables side by side
pub fn render_pattern_table(ui: &mut egui::Ui, config: &AppConfig, emu_textures: &EmuTextures, to_emu: &Sender<FrontendMessage>) {
    if let Some(tile_textures) = &emu_textures.tile_textures
        && let Some(palettes) = &emu_textures.palette_data
        && let Some(pattern_data) = &emu_textures.tile_data
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
            draw_pattern_table(
                ui,
                half_width,
                &tile_textures[config.view_config.debug_active_palette][..256],
                transformed_palette,
                &pattern_data[..256],
                to_emu
            );

            ui.separator();
            draw_pattern_table(
                ui,
                half_width,
                &tile_textures[config.view_config.debug_active_palette][256..],
                transformed_palette,
                &pattern_data[256..],
                to_emu
            );
        });
    } else {
        ui.label("Waiting for pattern table data...");
    }
}
