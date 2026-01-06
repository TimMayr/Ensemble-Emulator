use egui::Context;

use crate::emulation::messages::{
    NAMETABLE_HEIGHT, NAMETABLE_WIDTH, TOTAL_OUTPUT_HEIGHT, TOTAL_OUTPUT_WIDTH,
};
use crate::frontend::egui::config::ViewConfig;
use crate::frontend::egui::textures::EmuTextures;
use crate::frontend::egui::ui::draw_pattern_table;

/// Add all emulator view windows
pub fn add_emulator_views(ctx: &Context, view_config: &mut ViewConfig, emu_textures: &EmuTextures) {
    add_main_output_window(ctx, emu_textures);

    if view_config.show_pattern_table {
        add_pattern_table_window(ctx, emu_textures);
    }

    if view_config.show_nametable {
        add_nametable_window(ctx, view_config, emu_textures);
    }

    if view_config.show_sprite_viewer {
        add_sprite_viewer_window(ctx, view_config);
    }
}

/// Add the main emulator output window
fn add_main_output_window(ctx: &Context, emu_textures: &EmuTextures) {
    egui::Window::new("Emulator Output")
        .default_size([512.0, 480.0])
        .default_pos([50.0, 50.0])
        .show(ctx, |ui| {
            if let Some(ref texture) = emu_textures.emulator_texture {
                // Get available content region
                let available = ui.available_size();

                // Calculate display size (scale to fit window while maintaining aspect ratio)
                let scale = (available.x / TOTAL_OUTPUT_WIDTH as f32)
                    .min(available.y / TOTAL_OUTPUT_HEIGHT as f32);

                let display_width = TOTAL_OUTPUT_WIDTH as f32 * scale;
                let display_height = TOTAL_OUTPUT_HEIGHT as f32 * scale;

                ui.label(format!(
                    "{}x{} at {:.1}x scale",
                    TOTAL_OUTPUT_WIDTH, TOTAL_OUTPUT_HEIGHT, scale
                ));

                ui.image((texture.id(), egui::vec2(display_width, display_height)));
            } else {
                ui.label("Waiting for first frame...");
            }
        });
}

/// Add the pattern table viewer window
fn add_pattern_table_window(ctx: &Context, emu_textures: &EmuTextures) {
    egui::Window::new("Pattern Table Viewer")
        .default_size([580.0, 300.0])
        .default_pos([700.0, 50.0])
        .open(&mut true)
        .resizable(true)
        .max_height(0.0)
        .show(ctx, |ui| {
            if !emu_textures.tile_textures.is_empty() {
                let full_width = ui.available_width();
                let half_width = (full_width - ui.spacing().item_spacing.x * 3.0) * 0.5;

                ui.label(format!(
                    "Pattern Tables (128x128x2 at {:.1}x scale)",
                    (half_width / 256.0) - 0.1
                ));

                ui.horizontal_top(|ui| {
                    ui.allocate_ui(egui::vec2(half_width, half_width), |ui| {
                        draw_pattern_table(ui, 0, emu_textures);
                    });

                    ui.separator();
                    ui.allocate_ui(egui::vec2(half_width, half_width), |ui| {
                        draw_pattern_table(ui, 1, emu_textures);
                    });
                });
            } else {
                ui.label("Waiting for pattern table data...");
            }
        });
}

/// Add the nametable viewer window
fn add_nametable_window(ctx: &Context, view_config: &mut ViewConfig, emu_textures: &EmuTextures) {
    egui::Window::new("Nametable Viewer")
        .default_size([600.0, 560.0])
        .default_pos([700.0, 370.0])
        .open(&mut view_config.show_nametable)
        .show(ctx, |ui| {
            if let Some(ref texture) = emu_textures.nametable_texture {
                let available = ui.available_size();
                let scale = (available.x / NAMETABLE_WIDTH as f32)
                    .min(available.y / NAMETABLE_HEIGHT as f32);

                let display_width = NAMETABLE_WIDTH as f32 * scale;
                let display_height = NAMETABLE_HEIGHT as f32 * scale;

                ui.label(format!(
                    "Nametables ({}x{} at {:.1}x)",
                    NAMETABLE_WIDTH, NAMETABLE_HEIGHT, scale
                ));

                ui.image((texture.id(), egui::vec2(display_width, display_height)));
            } else {
                ui.label("Waiting for nametable data...");
            }
        });
}

/// Add the sprite viewer window
fn add_sprite_viewer_window(ctx: &Context, view_config: &mut ViewConfig) {
    egui::Window::new("Sprite Viewer")
        .default_size([600.0, 560.0])
        .default_pos([700.0, 370.0])
        .open(&mut view_config.show_sprite_viewer)
        .show(ctx, |ui| {
            ui.label("Waiting for Sprite data...");
        });
}
