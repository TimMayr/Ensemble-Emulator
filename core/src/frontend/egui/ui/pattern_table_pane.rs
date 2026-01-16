//! Pattern table viewer pane rendering

use crate::frontend::egui::config::AppConfig;
use crate::frontend::egui::textures::EmuTextures;
use crate::frontend::egui::ui::draw_pattern_table;

/// Render a single pattern table viewer
/// 
/// # Arguments
/// * `ui` - The egui Ui context
/// * `config` - Application configuration
/// * `emu_textures` - Emulator texture data
/// * `table_index` - Which pattern table to render (0 for left/CHR $0000-$0FFF, 1 for right/CHR $1000-$1FFF)
pub fn render_pattern_table(
    ui: &mut egui::Ui,
    config: &AppConfig,
    emu_textures: &EmuTextures,
    table_index: usize,
) {
    if emu_textures.tile_textures.is_some()
        && let Some(palettes) = &emu_textures.palette_data
    {
        let available_width = ui.available_width();
        let scale = available_width / 128.0;

        let table_name = if table_index == 0 {
            "Left ($0000-$0FFF)"
        } else {
            "Right ($1000-$1FFF)"
        };

        ui.label(format!(
            "Pattern Table {} (128x128 at {:.1}x scale)",
            table_name, scale
        ));

        let selected_palette = palettes.colors[config.view_config.debug_active_palette];
        let transformed_palette = selected_palette
            .map(|color_index| config.view_config.palette_rgb_data[color_index as usize]);

        draw_pattern_table(
            ui,
            table_index,
            emu_textures,
            config.view_config.debug_active_palette,
            transformed_palette,
        );
    } else {
        ui.label("Waiting for pattern table data...");
    }
}
