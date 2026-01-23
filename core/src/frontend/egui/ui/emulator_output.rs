//! Emulator output pane rendering

use crossbeam_channel::Sender;

use crate::emulation::messages::{TOTAL_OUTPUT_HEIGHT, TOTAL_OUTPUT_WIDTH};
use crate::frontend::egui::textures::EmuTextures;
use crate::frontend::messages::AsyncFrontendMessage;

/// Render the main emulator output with drag-and-drop support for ROM files
pub fn render_emulator_output(
    ui: &mut egui::Ui,
    emu_textures: &EmuTextures,
    async_sender: &Sender<AsyncFrontendMessage>,
) {
    // Handle drag and drop for ROM files
    let response = ui.allocate_response(ui.available_size(), egui::Sense::hover());
    let pane_rect = response.rect;
    
    // Check if mouse is over this pane
    let mouse_over_pane = ui.ctx().input(|i| {
        i.pointer.hover_pos().map_or(false, |pos| pane_rect.contains(pos))
    });
    
    // Preview dropped files only when hovering over this pane
    if mouse_over_pane {
        preview_files_being_dropped(ui, pane_rect, &["nes"], "Drop .nes ROM file here");
    }

    // Handle dropped files only when mouse is over this pane
    if mouse_over_pane {
        ui.ctx().input(|i| {
            if !i.raw.dropped_files.is_empty() {
                for file in &i.raw.dropped_files {
                    if let Some(path) = &file.path {
                        if path.extension().map_or(false, |ext| ext.eq_ignore_ascii_case("nes")) {
                            let _ = async_sender.send(AsyncFrontendMessage::LoadRom(Some(path.clone())));
                        }
                    }
                }
            }
        });
    }

    // Render the emulator output
    let rect = response.rect;
    if let Some(ref texture) = emu_textures.frame_texture {
        let available = rect.size();

        let scale =
            (available.x / TOTAL_OUTPUT_WIDTH as f32).min(available.y / TOTAL_OUTPUT_HEIGHT as f32);

        let display_width = TOTAL_OUTPUT_WIDTH as f32 * scale;
        let display_height = TOTAL_OUTPUT_HEIGHT as f32 * scale;

        // Center the image in the available space
        let offset_x = (available.x - display_width) / 2.0;
        let offset_y = (available.y - display_height) / 2.0;
        let image_rect = egui::Rect::from_min_size(
            rect.min + egui::vec2(offset_x, offset_y),
            egui::vec2(display_width, display_height),
        );

        ui.painter().image(
            texture.id(),
            image_rect,
            egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
            egui::Color32::WHITE,
        );

        // Draw scale info at top
        ui.painter().text(
            rect.min + egui::vec2(5.0, 5.0),
            egui::Align2::LEFT_TOP,
            format!("{}x{} at {:.1}x scale", TOTAL_OUTPUT_WIDTH, TOTAL_OUTPUT_HEIGHT, scale),
            egui::FontId::default(),
            ui.visuals().text_color(),
        );
    } else {
        ui.painter().text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            "Waiting for first frame...\nDrop a .nes ROM file here",
            egui::FontId::default(),
            ui.visuals().text_color(),
        );
    }
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
