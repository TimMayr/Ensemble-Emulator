use crate::frontend::egui::config::AppConfig;
use crate::frontend::egui::textures::EmuTextures;
use crate::frontend::util::FromU32;

pub fn render_palettes(ui: &mut egui::Ui, config: &AppConfig, emu_textures: &EmuTextures) {
    if let Some(palette_data) = &emu_textures.palette_data {
        let full_width = ui.available_width();
        let single_color_width = full_width / 4.0;
        let single_color_height = 20.0;

        for (i, palette) in palette_data.colors.iter().enumerate() {
            if i < 4 {
                ui.label(format!("Background Palette {}", i + 1));
            } else {
                ui.label(format!("Sprite Palette {}", i - 3));
            };

            let painter = ui.painter();
            let start_pos = ui.cursor().min;

            for (j, color) in palette.iter().enumerate() {
                let rect = egui::Rect::from_min_size(
                    egui::pos2((j as f32) * single_color_width + start_pos.x, start_pos.y),
                    egui::vec2(single_color_width, single_color_height),
                );
                painter.rect_filled(
                    rect,
                    0.0,
                    egui::Color32::from_u32(
                        config.view_config.palette_rgb_data.colors[0][*color as usize],
                    ),
                );
            }
            ui.allocate_space(egui::vec2(4.0 * single_color_width, single_color_height));
        }
    }
}
