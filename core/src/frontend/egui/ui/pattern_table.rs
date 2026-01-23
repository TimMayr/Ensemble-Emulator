use crossbeam_channel::Sender;
use eframe::epaint::TextureHandle;
use egui::Ui;

use crate::emulation::messages::{EmulatorFetchable, FrontendMessage, TileData};
use crate::frontend::egui::config::AppConfig;
use crate::frontend::util::FromU32;

/// Draw a pattern table (left or right) in the UI
pub fn draw_pattern_table(
    ui: &mut Ui,
    width: f32,
    emu_textures: &[TextureHandle],
    palette: [u32; 4],
    pattern_data: &[TileData],
    to_emu: &Sender<FrontendMessage>,
    config: &mut AppConfig,
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

        let painter = ui.ctx().layer_painter(egui::LayerId::new(
            egui::Order::Foreground,
            egui::Id::new("popup_painter"),
        ));

        egui::Popup::context_menu(&response)
            .close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside)
            .show(|ui| {
                ui.heading("Edit Pattern");

                let tile_data = pattern_data[i];
                ui.label(format!("Rom address: ${:0X}", tile_data.address));

                ui.horizontal(|ui| {
                    color_radio(
                        ui,
                        &mut config.user_config.pattern_edit_color,
                        0,
                        egui::Color32::from_u32(palette[0]),
                    );
                    color_radio(
                        ui,
                        &mut config.user_config.pattern_edit_color,
                        1,
                        egui::Color32::from_u32(palette[1]),
                    );
                    color_radio(
                        ui,
                        &mut config.user_config.pattern_edit_color,
                        2,
                        egui::Color32::from_u32(palette[2]),
                    );
                    color_radio(
                        ui,
                        &mut config.user_config.pattern_edit_color,
                        3,
                        egui::Color32::from_u32(palette[3]),
                    );
                });

                ui.label("Pattern:");

                let height = 4.0 * 8.0;
                let (parent, _) = ui.allocate_exact_size(
                    egui::vec2(height * 8.0, height * 8.0),
                    egui::Sense::click(),
                );

                for index in 0..64 {
                    let row = index / 8;
                    let col = index % 8;

                    let lo = (tile_data.plane_0 >> (63 - index) & 1) as u8;
                    let hi = (tile_data.plane_1 >> (63 - index) & 1) as u8;
                    let color_id = hi << 1 | lo;

                    let color = palette[color_id as usize];

                    let rect = egui::Rect::from_min_size(
                        parent.min + egui::vec2(height * col as f32, height * row as f32),
                        egui::vec2(height, height),
                    );

                    let response = ui.interact(
                        rect,
                        ui.id().with(format!("{}", rect)),
                        egui::Sense::click(),
                    );

                    painter.rect_filled(rect, 0.0, egui::Color32::from_u32(color));

                    if response.clicked() {
                        let new_pattern = (color_id + 1) % 4;
                        let new_lo = new_pattern & 1;
                        let new_hi = (new_pattern >> 1) & 1;

                        let u64_bit_pos = 63usize.wrapping_sub(index) & 63;

                        let ppu_row = 7 - (u64_bit_pos / 8);
                        let ppu_bit = u64_bit_pos % 8;

                        let byte_shift = (7 - ppu_row) * 8;
                        let old_byte_0 = ((tile_data.plane_0 >> byte_shift) & 0xFF) as u8;
                        let old_byte_1 = ((tile_data.plane_1 >> byte_shift) & 0xFF) as u8;

                        let new_byte_0 = (old_byte_0 & !(1 << ppu_bit)) | (new_lo << ppu_bit);
                        let new_byte_1 = (old_byte_1 & !(1 << ppu_bit)) | (new_hi << ppu_bit);

                        let addr_0 = tile_data.address + ppu_row as u16;
                        let addr_1 = tile_data.address + ppu_row as u16 + 8;

                        let _ = to_emu.send(FrontendMessage::WritePpu(addr_0, new_byte_0));
                        let _ = to_emu.send(FrontendMessage::WritePpu(addr_1, new_byte_1));

                        let _ = to_emu.send(FrontendMessage::RequestDebugData(
                            EmulatorFetchable::Tiles(None),
                        ));
                    }

                    if response.hovered() {
                        painter.rect_stroke(
                            rect,
                            0.0,
                            egui::Stroke::new(3.0, egui::Color32::WHITE),
                            egui::StrokeKind::Inside,
                        );
                    }
                }
            });
    }
}

fn color_radio<'a, Value: PartialEq>(
    ui: &mut Ui,
    current: &mut Value,
    alternative: Value,
    color32: egui::Color32,
) {
    let selected = *current == alternative;

    let frame = egui::Frame::NONE
        .stroke(if selected {
            egui::Stroke::new(2.0, ui.visuals().selection.stroke.color)
        } else {
            egui::Stroke::NONE
        })
        .fill(if selected {
            ui.visuals().selection.bg_fill
        } else {
            egui::Color32::TRANSPARENT
        })
        .corner_radius(6.0)
        .inner_margin(6.0);

    let min = ui.cursor().min;
    let response = frame
        .show(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.painter().rect_filled(
                    egui::Rect::from_min_size(min, egui::vec2(10.0, 10.0)),
                    0.0,
                    color32,
                );
            });
        })
        .response;

    if response.clicked() {
        *current = alternative;
    }
}
