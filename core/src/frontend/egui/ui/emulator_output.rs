//! Emulator output pane rendering

use crate::emulation::messages::{TOTAL_OUTPUT_HEIGHT, TOTAL_OUTPUT_WIDTH};
use crate::frontend::egui::textures::EmuTextures;
use crate::frontend::egui::ui::calculate_integer_scale;

/// Render the main emulator output
pub fn render_emulator_output(ui: &mut egui::Ui, emu_textures: &EmuTextures) {
    if let Some(ref texture) = emu_textures.frame_texture {
        let available = ui.available_size();

        // Use integer scaling to snap to exact pixel multiples
        let scale = calculate_integer_scale(
            TOTAL_OUTPUT_WIDTH as f32,
            TOTAL_OUTPUT_HEIGHT as f32,
            available.x,
            available.y,
        );

        let display_width = TOTAL_OUTPUT_WIDTH as f32 * scale as f32;
        let display_height = TOTAL_OUTPUT_HEIGHT as f32 * scale as f32;

        ui.label(format!(
            "{}x{} at {}x scale",
            TOTAL_OUTPUT_WIDTH, TOTAL_OUTPUT_HEIGHT, scale
        ));

        ui.image((texture.id(), egui::vec2(display_width, display_height)));
    } else {
        ui.label("Waiting for first frame...");
    }
}
