//! Pattern table viewer pane rendering

use crate::frontend::egui::config::AppConfig;
use crate::frontend::egui::textures::EmuTextures;
use crate::frontend::egui::ui::{calculate_integer_scale, draw_pattern_table};

/// Pattern table native dimensions: 128x128 pixels each
const PATTERN_TABLE_WIDTH: f32 = 128.0;
const PATTERN_TABLE_HEIGHT: f32 = 128.0;

/// Render both pattern tables side by side
pub fn render_pattern_table(
    ui: &mut egui::Ui,
    config: &AppConfig,
    emu_textures: &EmuTextures,
) {
    if emu_textures.tile_textures.is_some()
        && let Some(palettes) = &emu_textures.palette_data
    {
        let available = ui.available_size();
        // Account for separator and spacing between tables
        let spacing = ui.spacing().item_spacing.x * 3.0;
        // Available width for each table (two tables side by side)
        let available_per_table_width = (available.x - spacing) / 2.0;

        // Calculate integer scale for one pattern table
        let scale = calculate_integer_scale(
            PATTERN_TABLE_WIDTH,
            PATTERN_TABLE_HEIGHT,
            available_per_table_width,
            available.y,
        );

        let table_size = PATTERN_TABLE_WIDTH * scale as f32;

        ui.label(format!("Pattern Tables (128x128x2 at {}x scale)", scale));

        let selected_palette = palettes.colors[config.view_config.debug_active_palette];
        let transformed_palette = selected_palette
            .map(|color_index| config.view_config.palette_rgb_data[color_index as usize]);

        ui.horizontal_top(|ui| {
            ui.allocate_ui(egui::vec2(table_size, table_size), |ui| {
                draw_pattern_table(
                    ui,
                    0,
                    emu_textures,
                    config.view_config.debug_active_palette,
                    transformed_palette,
                );
            });

            ui.separator();
            ui.allocate_ui(egui::vec2(table_size, table_size), |ui| {
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
