use std::path::PathBuf;

use crossbeam_channel::Sender;

use crate::emulation::messages::{EmulatorFetchable, FrontendMessage};
use crate::emulation::ppu::PALETTE_RAM_START_ADDRESS;
use crate::frontend::egui::config::AppConfig;
use crate::frontend::egui::textures::EmuTextures;
use crate::frontend::messages::AsyncFrontendMessage;
use crate::frontend::util::{pick_palette, FromU32};

pub fn render_palettes(
    ui: &mut egui::Ui,
    config: &mut AppConfig,
    emu_textures: &EmuTextures,
    to_emu: &Sender<FrontendMessage>,
    to_frontend: &Sender<AsyncFrontendMessage>,
) {
    let full_width = ui.available_width();
    let single_color_width = 80f32.min(full_width / 4.0);
    let single_color_height = 20.0;

    if let Some(palette_data) = &emu_textures.palette_data {
        for (i, palette) in palette_data.colors.iter().enumerate() {
            if i < 4 {
                ui.label(format!("Background Palette {}", i + 1));
            } else {
                ui.label(format!("Sprite Palette {}", i - 3));
            };

            let (parent, _) = ui.allocate_exact_size(
                egui::vec2(single_color_width * 4.0, single_color_height),
                egui::Sense::hover(),
            );

            for (j, color) in palette.iter().enumerate() {
                let rgb_color = config.view_config.palette_rgb_data.colors[0][*color as usize];

                let min = parent.min + egui::vec2(j as f32 * single_color_width, 0.0);
                let rect = egui::Rect::from_min_size(
                    min,
                    egui::vec2(single_color_width, single_color_height),
                );

                let response =
                    ui.interact(rect, ui.id().with(format!("{}", rect)), egui::Sense::all());

                let painter = ui.painter();
                painter.rect_filled(rect, 0.0, egui::Color32::from_u32(rgb_color));

                if response.hovered() {
                    painter.rect_stroke(
                        rect,
                        0.0,
                        egui::Stroke::new(3.0, egui::Color32::WHITE),
                        egui::StrokeKind::Inside,
                    );
                }

                let address = PALETTE_RAM_START_ADDRESS as usize | (j + (i * 4));

                if response.clicked() {
                    let _ = to_emu.send(FrontendMessage::WritePpu(address as u16, color + 1));
                    let _ = to_emu.send(FrontendMessage::RequestDebugData(
                        EmulatorFetchable::Palettes(None),
                    ));
                }

                response.on_hover_ui(|ui| {
                    ui.label(format!("Global palette index: ${color}"));
                    ui.label(format!("Address: ${:0X}", address));
                    ui.label(format!(
                        "Palette rgb mapping: #{:06X}",
                        rgb_color & 0x00FFFFFF
                    ));
                });
            }
        }
    }

    ui.separator();
    egui::MenuBar::new().ui(ui, |ui| {
        ui.menu_button("File", |ui| {
            if ui.button("Load palette file").clicked() {
                std::thread::spawn({
                    let sender = to_frontend.clone();
                    let prev_path = config.user_config.previous_palette_path.clone();
                    let prev_dir = if let Some(prev_path) = prev_path.parent() {
                        prev_path.to_path_buf()
                    } else {
                        PathBuf::default()
                    };

                    move || {
                        let path = pick_palette(prev_dir);
                        sender.send(AsyncFrontendMessage::LoadPalette(path))
                    }
                });
            }
        })
    });

    let color_width = 40f32.min(full_width / 8.0);

    let (parent, _) = ui.allocate_exact_size(
        egui::vec2(color_width * 8.0, color_width * 8.0),
        egui::Sense::hover(),
    );

    for (i, color) in config.view_config.palette_rgb_data.colors[0]
        .iter()
        .enumerate()
    {
        let row = (i / 8) as f32;
        let col = (i % 8) as f32;
        let min = parent.min + egui::vec2(col * color_width, row * color_width);
        let rect = egui::Rect::from_min_size(min, egui::vec2(color_width, color_width));

        let response = ui.interact(rect, ui.id().with(format!("{}", rect)), egui::Sense::all());

        let painter = ui.painter();
        painter.rect_filled(rect, 0.0, egui::Color32::from_u32(*color));

        if response.hovered() {
            painter.rect_stroke(
                rect,
                0.0,
                egui::Stroke::new(3.0, egui::Color32::WHITE),
                egui::StrokeKind::Inside,
            );
        }

        response.on_hover_ui(|ui| {
            ui.label(format!("Index: {i}"));
            ui.label(format!("Hex: 0x{:06X}", color & 0x00FFFFFF));
        });
    }
}
