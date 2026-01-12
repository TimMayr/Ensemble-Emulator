use egui::Ui;

use crate::frontend::egui::textures::EmuTextures;

/// Calculate foreground color (black or white) based on background luminance
fn foreground_for_background(bg: u32) -> egui::Color32 {
    let r = (bg >> 16) & 0xFF;
    let g = (bg >> 8) & 0xFF;
    let b = bg & 0xFF;

    let luminance = 0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32;

    if luminance > 128.0 {
        egui::Color32::BLACK
    } else {
        egui::Color32::WHITE
    }
}

/// Draw a pattern table (left or right) in the UI
pub fn draw_pattern_table(ui: &mut Ui, pattern_table: usize, emu_textures: &EmuTextures) {
    let available = ui.available_width();
    let base_size = 16.0;
    let logical_width = 16.0 * base_size;
    let scale = available / logical_width;
    let tex_size = egui::vec2(base_size, base_size) * scale;
    let pattern_data = &emu_textures.pattern_table_data;

    egui::Grid::new("pattern_table")
        .num_columns(16)
        .min_row_height(scale * base_size)
        .min_col_width(scale * base_size)
        .max_col_width(scale * base_size)
        .spacing(egui::vec2(0.0, 0.0))
        .show(ui, |ui| {
            for (i, texture) in emu_textures.tile_textures[pattern_table].iter().enumerate() {
                let tile_data = if pattern_table == 0 {
                    pattern_data
                        .as_ref()
                        .map(|pattern_data| pattern_data.left.tiles[i])
                } else {
                    pattern_data
                        .as_ref()
                        .map(|pattern_data| pattern_data.right.tiles[i])
                };

                let palette_data = pattern_data
                    .as_ref()
                    .map(|pattern_data| pattern_data.palette);

                ui.image((texture.id(), tex_size)).on_hover_ui(|ui| {
                    if let Some(tile_data) = tile_data
                        && let Some(palette_data) = palette_data
                    {
                        ui.label(format!("Rom address: ${:0X}", tile_data.address));

                        let plane_0_string = format!("{:064b}", tile_data.plane_0);
                        let plane_1_string = format!("{:064b}", tile_data.plane_1);

                        let mut res = String::new();
                        let mut res_line = String::new();
                        let mut res_vec = Vec::new();

                        let mut char_width = 0.0;
                        let mut line_height = 0.0;
                        for (i, c) in plane_0_string.chars().enumerate() {
                            res_line.push(c);
                            res_line.push(plane_1_string.chars().nth(i).unwrap());

                            if (i + 1) % 8 == 0 {
                                res += res_line.as_str();
                                res.push('\n');
                                res_vec.push(res_line);
                                res_line = String::new();
                            }
                        }

                        ui.label("Pattern:");

                        let painter = ui.painter();
                        let font_id = egui::FontId::monospace(14.0);

                        let start_pos = ui.cursor().min;
                        let mut y = start_pos.y;
                        for (row, string) in res_vec.iter().enumerate() {
                            let mut x = start_pos.x;
                            let mut current_line_height: f32 = 0.0;

                            for (col, ch) in string.chars().enumerate() {
                                let bitmap_col = col / 2;
                                let bit = 63 - (row * 8 + bitmap_col);
                                let lo = (tile_data.plane_0 >> bit & 1) as u8;
                                let hi = (tile_data.plane_1 >> bit & 1) as u8;
                                let color_id = hi << 1 | lo;

                                let color = palette_data.colors[color_id as usize];

                                let galley = ui.fonts_mut(|f| {
                                    f.layout_no_wrap(
                                        ch.to_string(),
                                        font_id.clone(),
                                        foreground_for_background(color),
                                    )
                                });
                                let rect =
                                    egui::Rect::from_min_size(egui::pos2(x, y), galley.size());

                                painter.rect_filled(
                                    rect,
                                    0.0,
                                    egui::Color32::from_rgb(
                                        (color >> 16) as u8,
                                        (color >> 8) as u8,
                                        color as u8,
                                    ),
                                );
                                painter.galley(rect.min, galley, egui::Color32::WHITE);

                                x += rect.width();
                                char_width = rect.width();
                                current_line_height = current_line_height.max(rect.height());
                                line_height = current_line_height;
                            }

                            y += current_line_height;
                        }

                        let total_height = res_vec.len() as f32 * line_height;
                        let total_width = res_vec[0].len() as f32 * char_width;
                        ui.allocate_space(egui::vec2(total_width, total_height));
                    }
                });

                if (i + 1) % 16 == 0 {
                    ui.end_row();
                }
            }
        });
}
