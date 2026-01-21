use eframe::epaint::TextureHandle;
use egui::Ui;

use crate::emulation::messages::TileData;
use crate::frontend::util::{Contrastable, FromU32};

/// Draw a pattern table (left or right) in the UI
pub fn draw_pattern_table(
    ui: &mut Ui,
    width: f32,
    emu_textures: &[TextureHandle],
    palette: [u32; 4],
    pattern_data: &[TileData],
) {
    let tile_size = width / 16.0;

    let (parent, _) = ui.allocate_exact_size(egui::vec2(width, width), egui::Sense::hover());

    for (i, tex) in emu_textures.iter().enumerate() {
        let row = i / 16;
        let col = i % 16;
        let min = parent.min + egui::vec2(col as f32 * tile_size, row as f32 * tile_size);

        let rect = egui::Rect::from_min_size(min, egui::vec2(tile_size, tile_size));

        let response = ui.interact(rect, ui.id().with(format!("{}", rect)), egui::Sense::all());

        let painter = ui.painter();
        painter.image(
            tex.id(),
            rect,
            egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
            egui::Color32::WHITE,
        );

        if response.hovered() {
            painter.rect_stroke(
                rect,
                0.0,
                egui::Stroke::new(3.0, egui::Color32::WHITE),
                egui::StrokeKind::Inside,
            );
        }

        response.on_hover_ui(|ui| {
            let tile_data = pattern_data[i];
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

                    let color = palette[color_id as usize];

                    let galley = ui.fonts_mut(|f| {
                        f.layout_no_wrap(
                            ch.to_string(),
                            font_id.clone(),
                            egui::Color32::from_u32(color).get_contrast(),
                        )
                    });
                    let rect = egui::Rect::from_min_size(egui::pos2(x, y), galley.size());

                    painter.rect_filled(rect, 0.0, egui::Color32::from_u32(color));
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
        });
    }
}
