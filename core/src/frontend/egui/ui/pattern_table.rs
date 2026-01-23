use crossbeam_channel::Sender;
use eframe::epaint::TextureHandle;
use egui::Ui;

use crate::emulation::messages::{EmulatorFetchable, FrontendMessage, TileData};
use crate::frontend::egui::config::AppConfig;
use crate::frontend::egui::ui::widgets::{image_cell, PainterGridConfig};
use crate::frontend::util::{color_radio, FromU32};

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
    let grid_config = PainterGridConfig::square(width, 16);
    let (parent, _) = ui.allocate_exact_size(grid_config.total_size(), egui::Sense::hover());

    for (i, tex) in emu_textures.iter().enumerate() {
        let rect = grid_config.cell_rect(parent.min, i);
        let response = image_cell(ui, rect, tex.id(), egui::Sense::all(), ("tile", i, parent.to_string()));

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

                ui.label("Pattern:");

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

                // Tile pixel editor grid (8x8 pixels, each 4x4 in size = 32x32 total)
                let pixel_size = 4.0 * 8.0;
                let pixel_grid = PainterGridConfig::square(pixel_size * 8.0, 8);
                let (pixel_parent, _) =
                    ui.allocate_exact_size(pixel_grid.total_size(), egui::Sense::click_and_drag());

                for index in 0..64 {
                    let lo = (tile_data.plane_0 >> (63 - index) & 1) as u8;
                    let hi = (tile_data.plane_1 >> (63 - index) & 1) as u8;
                    let color_id = hi << 1 | lo;
                    let color = palette[color_id as usize];

                    let pixel_rect = pixel_grid.cell_rect(pixel_parent.min, index);
                    let response = ui.interact(
                        pixel_rect,
                        ui.id().with(("pixel", index)),
                        egui::Sense::click_and_drag(),
                    );

                    painter.rect_filled(pixel_rect, 0.0, egui::Color32::from_u32(color));

                    // Support both click and drag for editing pixels
                    let should_edit = response.clicked() || (response.dragged() && ui.input(|i| i.pointer.primary_down()));
                    if should_edit {
                        handle_pixel_edit(&tile_data, index, config, to_emu);
                    }

                    if response.hovered() {
                        painter.rect_stroke(
                            pixel_rect,
                            0.0,
                            egui::Stroke::new(3.0, egui::Color32::WHITE),
                            egui::StrokeKind::Inside,
                        );
                    }
                }
            });
    }
}

/// Handle editing a single pixel in a tile pattern
fn handle_pixel_edit(
    tile_data: &TileData,
    index: usize,
    config: &AppConfig,
    to_emu: &Sender<FrontendMessage>,
) {
    let new_pattern = config.user_config.pattern_edit_color;
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
    let _ = to_emu.send(FrontendMessage::RequestDebugData(EmulatorFetchable::Tiles(
        None,
    )));
}
