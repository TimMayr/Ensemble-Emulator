use std::time::Instant;

use crossbeam_channel::Sender;
use egui::Context;

use crate::emulation::messages::ControllerEvent;
use crate::frontend::egui::config::AppConfig;
use crate::frontend::messages::AsyncFrontendMessage;

/// Handle keyboard input from the user.
///
/// # Arguments
/// * `ctx` - The egui context
/// * `async_sender` - Channel to send async messages
/// * `config` - Application configuration (modified for speed/view settings)
/// * `last_frame_request` - Last frame request time (reset when pausing)
pub fn handle_keyboard_input(
    ctx: &Context,
    async_sender: &Sender<AsyncFrontendMessage>,
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
            let _ = async_sender.send(AsyncFrontendMessage::Reset);
        }

        if i.key_pressed(egui::Key::F5) {
            let _ = async_sender.send(AsyncFrontendMessage::Quicksave);
        }

        if i.key_pressed(egui::Key::F8) {
            let _ = async_sender.send(AsyncFrontendMessage::Quickload);
        }

        // NES controller input
        handle_controller_input(i, async_sender);
    });
}

/// Handle NES controller input mapping from keyboard
fn handle_controller_input(input: &egui::InputState, async_sender: &Sender<AsyncFrontendMessage>) {
    // D-pad
    if input.key_pressed(egui::Key::ArrowLeft) {
        let _ = async_sender.send(AsyncFrontendMessage::ControllerInput(ControllerEvent::Left));
    }
    if input.key_pressed(egui::Key::ArrowRight) {
        let _ = async_sender.send(AsyncFrontendMessage::ControllerInput(ControllerEvent::Right));
    }
    if input.key_pressed(egui::Key::ArrowUp) {
        let _ = async_sender.send(AsyncFrontendMessage::ControllerInput(ControllerEvent::Up));
    }
    if input.key_pressed(egui::Key::ArrowDown) {
        let _ = async_sender.send(AsyncFrontendMessage::ControllerInput(ControllerEvent::Down));
    }

    // Buttons
    if input.key_pressed(egui::Key::S) {
        let _ = async_sender.send(AsyncFrontendMessage::ControllerInput(ControllerEvent::Start));
    }
    if input.key_pressed(egui::Key::Tab) {
        let _ = async_sender.send(AsyncFrontendMessage::ControllerInput(ControllerEvent::Select));
    }
    if input.key_pressed(egui::Key::Space) {
        let _ = async_sender.send(AsyncFrontendMessage::ControllerInput(ControllerEvent::A));
    }
    if input.modifiers.shift {
        let _ = async_sender.send(AsyncFrontendMessage::ControllerInput(ControllerEvent::B));
    }
}
