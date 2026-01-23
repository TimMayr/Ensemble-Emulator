use std::time::Instant;

use crossbeam_channel::Sender;
use egui::Context;

use crate::emulation::messages::{ControllerEvent, FrontendMessage};
use crate::frontend::egui::config::AppConfig;

/// Handle keyboard input from the user.
///
/// # Arguments
/// * `ctx` - The egui context
/// * `to_emulator` - Channel to send messages to the emulator
/// * `config` - Application configuration (modified for speed/view settings)
/// * `last_frame_request` - Last frame request time (reset when pausing)
pub fn handle_keyboard_input(
    ctx: &Context,
    to_emulator: &Sender<FrontendMessage>,
    config: &mut AppConfig,
    last_frame_request: &mut Instant,
) {
    ctx.input(|i| {
        // Debug controls
        if i.key_pressed(egui::Key::N) {
            config.view_config.debug_active_palette += 1;
            config.view_config.debug_active_palette &= 7;
        }

        // Emulation controls
        if i.key_pressed(egui::Key::Period) {
            config.speed_config.is_paused = !config.speed_config.is_paused;
            *last_frame_request = Instant::now();
        }
        if i.key_pressed(egui::Key::R) {
            let _ = to_emulator.send(FrontendMessage::Reset);
        }

        // NES controller input
        handle_controller_input(i, to_emulator);
    });
}

/// Handle NES controller input mapping from keyboard
fn handle_controller_input(input: &egui::InputState, to_emulator: &Sender<FrontendMessage>) {
    // D-pad
    if input.key_pressed(egui::Key::ArrowLeft) {
        let _ = to_emulator.send(FrontendMessage::ControllerInput(ControllerEvent::Left));
    }
    if input.key_pressed(egui::Key::ArrowRight) {
        let _ = to_emulator.send(FrontendMessage::ControllerInput(ControllerEvent::Right));
    }
    if input.key_pressed(egui::Key::ArrowUp) {
        let _ = to_emulator.send(FrontendMessage::ControllerInput(ControllerEvent::Up));
    }
    if input.key_pressed(egui::Key::ArrowDown) {
        let _ = to_emulator.send(FrontendMessage::ControllerInput(ControllerEvent::Down));
    }

    // Buttons
    if input.key_pressed(egui::Key::S) {
        let _ = to_emulator.send(FrontendMessage::ControllerInput(ControllerEvent::Start));
    }
    if input.key_pressed(egui::Key::Tab) {
        let _ = to_emulator.send(FrontendMessage::ControllerInput(ControllerEvent::Select));
    }
    if input.key_pressed(egui::Key::Space) {
        let _ = to_emulator.send(FrontendMessage::ControllerInput(ControllerEvent::A));
    }
    if input.modifiers.shift {
        let _ = to_emulator.send(FrontendMessage::ControllerInput(ControllerEvent::B));
    }
}
