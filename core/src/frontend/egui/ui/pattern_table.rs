use crossbeam_channel::Sender;
use eframe::epaint::TextureHandle;
use egui::Ui;

use crate::emulation::messages::{EmulatorFetchable, FrontendMessage, TileData};
use crate::frontend::util::FromU32;

/// Draw a pattern table (left or right) in the UI
pub fn draw_pattern_table(
    ui: &mut Ui,
    width: f32,
    emu_textures: &[TextureHandle],
    palette: [u32; 4],
    pattern_data: &[TileData],
    to_emu: &Sender<FrontendMessage>
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

        egui::Popup::menu(&response)
            .close_behavior(egui::PopupCloseBehavior::IgnoreClicks)
            .show(|ui| {
                ui.heading("Edit Pattern");

                let tile_data = pattern_data[i];
                ui.label(format!("Rom address: ${:0X}", tile_data.address));

                let plane_0_string = format!("{:064b}", tile_data.plane_0);
                let plane_1_string = format!("{:064b}", tile_data.plane_1);

                let mut res_vec = Vec::new();

                for (i, c) in plane_0_string.chars().enumerate() {
                    let p0 = c;
                    let p1 = plane_1_string.chars().nth(i).unwrap();
                    res_vec.push(format!("{p1}{p0}"));
                }

                ui.label("Pattern:");

                let height = 4.0 * 8.0;
                let (parent, _) = ui.allocate_exact_size(
                    egui::vec2(height * 8.0, height * 8.0),
                    egui::Sense::click(),
                );

                for (index, string) in res_vec.iter().enumerate() {
                    let row = index / 8;
                    let col = index % 8;

                    let lo = (tile_data.plane_0 >> (64 - index) & 1) as u8;
                    let hi = (tile_data.plane_1 >> (64 - index) & 1) as u8;
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

                        // Match the bit position calculation from the read code (64 - index)
                        // to ensure we modify the same pixel that is displayed.
                        // When index=0, 64-0=64 which wraps to 0 for shift operations.
                        let u64_bit_pos = 64usize.wrapping_sub(index) & 63;
                        // Convert u64 bit position to PPU byte row and bit within byte
                        let ppu_row = 7 - (u64_bit_pos / 8);
                        let ppu_bit = u64_bit_pos % 8;
                        // Extract current bytes for this row from plane_0 and plane_1
                        let byte_shift = (7 - ppu_row) * 8;
                        let old_byte_0 = ((tile_data.plane_0 >> byte_shift) & 0xFF) as u8;
                        let old_byte_1 = ((tile_data.plane_1 >> byte_shift) & 0xFF) as u8;
                        // Clear the old bit and set the new bit
                        let new_byte_0 = (old_byte_0 & !(1 << ppu_bit)) | (new_lo << ppu_bit);
                        let new_byte_1 = (old_byte_1 & !(1 << ppu_bit)) | (new_hi << ppu_bit);
                        // Calculate PPU addresses for the bytes
                        let addr_0 = tile_data.address + ppu_row as u16;
                        let addr_1 = tile_data.address + ppu_row as u16 + 8;
                        // Send the updated bytes to the emulator
                        let _ = to_emu.send(FrontendMessage::WritePpu(addr_0, new_byte_0));
                        let _ = to_emu.send(FrontendMessage::WritePpu(addr_1, new_byte_1));

                        let _ = to_emu.send(FrontendMessage::RequestDebugData(EmulatorFetchable::Tiles(None)));
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
