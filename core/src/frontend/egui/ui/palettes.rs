use crossbeam_channel::Sender;

use crate::emulation::messages::{EmulatorFetchable, FrontendMessage};
use crate::emulation::ppu::PALETTE_RAM_START_ADDRESS;
use crate::frontend::egui::config::AppConfig;
use crate::frontend::egui::textures::EmuTextures;
use crate::frontend::egui::ui::widgets::{color_cell, PainterGridConfig};
use crate::frontend::messages::AsyncFrontendMessage;
use crate::frontend::util::{
    spawn_palette_picker, spawn_palette_save, AsU32, FromU32, Hashable, ToBytes,
};

pub fn render_palettes(
    ui: &mut egui::Ui,
    config: &mut AppConfig,
    emu_textures: &EmuTextures,
    to_emu: &Sender<FrontendMessage>,
    to_frontend: &Sender<AsyncFrontendMessage>,
) {
    let old_palette = config.view_config.palette_rgb_data;
    let full_width = ui.available_width();
    let single_color_width = 80f32.min(full_width / 4.0);
    let single_color_height = 20.0;

    // Get the pane rect for drag and drop scoping
    let pane_rect = ui.max_rect();
    
    // Check if mouse is over this pane
    let mouse_over_pane = ui.ctx().input(|i| {
        i.pointer.hover_pos().map_or(false, |pos| pane_rect.contains(pos))
    });

    // Handle drag and drop for palette files only when hovering over this pane
    if mouse_over_pane {
        handle_palette_drag_drop(ui, pane_rect, to_frontend);
    }

    // Render the 8 active palettes (4 background + 4 sprite)
    if let Some(palette_data) = &emu_textures.palette_data {
        for (i, palette) in palette_data.colors.iter().enumerate() {
            if i < 4 {
                ui.label(format!("Background Palette {}", i + 1));
            } else {
                ui.label(format!("Sprite Palette {}", i - 3));
            };

            let grid_config = PainterGridConfig::rect(
                single_color_width * 4.0,
                single_color_height,
                4,
                1,
            );
            let (parent, _) = ui.allocate_exact_size(grid_config.total_size(), egui::Sense::hover());

            for (j, color) in palette.iter().enumerate() {
                let rgb_color = config.view_config.palette_rgb_data.colors[0][*color as usize];
                let rect = grid_config.cell_rect(parent.min, j);
                let response = color_cell(ui, rect, rgb_color, egui::Sense::all(), ("palette", i, j));

                let address = PALETTE_RAM_START_ADDRESS as usize | (j + (i * 4));
                let mut new_color = *color;

                egui::Popup::context_menu(&response)
                    .close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside)
                    .show(|ui| {
                        ui.add(egui::Slider::new(&mut new_color, 1..=64).text("Palette Index"));
                    });

                if new_color != *color {
                    let _ = to_emu.send(FrontendMessage::WritePpu(address as u16, new_color));
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

    // File operations menu
    ui.separator();
    egui::MenuBar::new().ui(ui, |ui| {
        ui.menu_button("File", |ui| {
            if ui.button("Load Palette").clicked() {
                spawn_palette_picker(
                    to_frontend,
                    config.user_config.previous_palette_path.as_ref(),
                );
            }

            if ui.button("Save Palette").clicked() {
                spawn_palette_save(
                    config.user_config.previous_palette_path.as_ref(),
                    config.view_config.palette_rgb_data.to_bytes(),
                );
            }

            if ui.button("Reset Palette").clicked() {
                let _ = to_frontend.send(AsyncFrontendMessage::LoadPalette(None));
            }
        })
    });

    ui.label("Drag and drop .pal files here to load");

    // Render the full 64-color palette editor
    let color_width = 40f32.min(full_width / 8.0);
    let grid_config = PainterGridConfig::square(color_width * 8.0, 8);
    let (parent, _) = ui.allocate_exact_size(grid_config.total_size(), egui::Sense::hover());

    for (i, color) in config.view_config.palette_rgb_data.colors[0]
        .clone()
        .iter()
        .enumerate()
    {
        let rect = grid_config.cell_rect(parent.min, i);
        let response = color_cell(ui, rect, *color, egui::Sense::all(), ("rgb_palette", i));

        let mut picked_color = egui::Color32::from_u32(*color);
        egui::Popup::context_menu(&response)
            .close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside)
            .show(|ui| {
                egui::color_picker::color_picker_color32(
                    ui,
                    &mut picked_color,
                    egui::color_picker::Alpha::Opaque,
                );
            });

        config.view_config.palette_rgb_data.colors[0][i] = picked_color.as_u32();

        response.on_hover_ui(|ui| {
            ui.label(format!("Index: {i}"));
            ui.label(format!("Hex: 0x{:06X}", color & 0x00FFFFFF));
        });
    }

    // Send palette update if changed
    if old_palette.hash() != config.view_config.palette_rgb_data.hash() {
        let _ = to_emu.send(FrontendMessage::SetPalette(Box::new(
            config.view_config.palette_rgb_data,
        )));
        let _ = to_frontend.send(AsyncFrontendMessage::RefreshPalette);
    }
}

/// Handle drag and drop for palette files
fn handle_palette_drag_drop(ui: &egui::Ui, pane_rect: egui::Rect, to_frontend: &Sender<AsyncFrontendMessage>) {
    // Show drop preview when hovering with files over this pane
    preview_files_being_dropped(ui, pane_rect, &["pal"], "Drop .pal palette file here");

    // Handle dropped files
    ui.ctx().input(|i| {
        if !i.raw.dropped_files.is_empty() {
            for file in &i.raw.dropped_files {
                if let Some(path) = &file.path {
                    if path.extension().map_or(false, |ext| ext.eq_ignore_ascii_case("pal")) {
                        let _ = to_frontend.send(AsyncFrontendMessage::LoadPalette(Some(path.clone())));
                    }
                }
            }
        }
    });
}

/// Show a visual indicator when files are being dragged over the pane
fn preview_files_being_dropped(ui: &egui::Ui, pane_rect: egui::Rect, valid_extensions: &[&str], hint_text: &str) {
    use egui::*;

    if !ui.ctx().input(|i| i.raw.hovered_files.is_empty()) {
        let has_valid_file = ui.ctx().input(|i| {
            i.raw.hovered_files.iter().any(|f| {
                f.path.as_ref().map_or(false, |p| {
                    p.extension().map_or(false, |ext| {
                        valid_extensions.iter().any(|valid_ext| ext.eq_ignore_ascii_case(valid_ext))
                    })
                })
            })
        });

        let painter = ui.painter();
        
        let color = if has_valid_file {
            Color32::from_rgba_unmultiplied(0, 100, 0, 180)
        } else {
            Color32::from_rgba_unmultiplied(100, 0, 0, 180)
        };
        
        painter.rect_filled(pane_rect, 0.0, color);
        
        let text = if has_valid_file {
            hint_text.to_string()
        } else {
            "Invalid file type".to_string()
        };
        
        painter.text(
            pane_rect.center(),
            Align2::CENTER_CENTER,
            text,
            FontId::proportional(24.0),
            Color32::WHITE,
        );
    }
}
