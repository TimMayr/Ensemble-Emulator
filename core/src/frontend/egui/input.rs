use std::time::Instant;

use crossbeam_channel::Sender;
use egui::Context;

use crate::emulation::messages::ControllerEvent;
use crate::frontend::egui::config::AppConfig;
use crate::frontend::egui::keybindings::Keybind;
use crate::frontend::messages::AsyncFrontendMessage;

/// Check if a keybind is pressed in the given input state.
fn is_keybind_pressed(input: &egui::InputState, keybind: &Keybind) -> bool {
    match &keybind.0 {
        Some(shortcut) => input.key_pressed(shortcut.logical_key) && input.modifiers == shortcut.modifiers,
        None => false,
    }
}

/// Consume a keybind shortcut from the input state.
fn consume_keybind(input: &mut egui::InputState, keybind: &Keybind) -> bool {
    match &keybind.0 {
        Some(shortcut) => input.consume_shortcut(shortcut),
        None => false,
    }
}

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
    ctx.input_mut(|i| {
        // Debug controls
        if consume_keybind(i, &config.keybindings.debug.cycle_palette) {
            config.view_config.debug_active_palette += 1;
            config.view_config.debug_active_palette &= 7;
        }

        // Emulation controls
        if consume_keybind(i, &config.keybindings.emulation.pause) {
            config.speed_config.is_paused = !config.speed_config.is_paused;
            *last_frame_request = Instant::now();
        }

        if consume_keybind(i, &config.keybindings.emulation.reset) {
            let _ = async_sender.send(AsyncFrontendMessage::Reset);
        }

        if consume_keybind(i, &config.keybindings.emulation.quicksave) {
            let _ = async_sender.send(AsyncFrontendMessage::Quicksave);
        }

        if consume_keybind(i, &config.keybindings.emulation.quickload) {
            let _ = async_sender.send(AsyncFrontendMessage::Quickload);
        }

        // NES controller input
        handle_controller_input(i, async_sender, config);
    });
}

/// Handle NES controller input mapping from keyboard
fn handle_controller_input(
    input: &mut egui::InputState,
    async_sender: &Sender<AsyncFrontendMessage>,
    config: &AppConfig,
) {
    // D-pad
    if is_keybind_pressed(input, &config.keybindings.controller.left) {
        let _ = async_sender.send(AsyncFrontendMessage::ControllerInput(ControllerEvent::Left));
    }
    if is_keybind_pressed(input, &config.keybindings.controller.right) {
        let _ = async_sender.send(AsyncFrontendMessage::ControllerInput(
            ControllerEvent::Right,
        ));
    }
    if is_keybind_pressed(input, &config.keybindings.controller.up) {
        let _ = async_sender.send(AsyncFrontendMessage::ControllerInput(ControllerEvent::Up));
    }
    if is_keybind_pressed(input, &config.keybindings.controller.down) {
        let _ = async_sender.send(AsyncFrontendMessage::ControllerInput(ControllerEvent::Down));
    }

    // Buttons
    if is_keybind_pressed(input, &config.keybindings.controller.start) {
        let _ = async_sender.send(AsyncFrontendMessage::ControllerInput(
            ControllerEvent::Start,
        ));
    }
    if is_keybind_pressed(input, &config.keybindings.controller.select) {
        let _ = async_sender.send(AsyncFrontendMessage::ControllerInput(
            ControllerEvent::Select,
        ));
    }
    if is_keybind_pressed(input, &config.keybindings.controller.a) {
        let _ = async_sender.send(AsyncFrontendMessage::ControllerInput(ControllerEvent::A));
    }
    if is_keybind_pressed(input, &config.keybindings.controller.b) {
        let _ = async_sender.send(AsyncFrontendMessage::ControllerInput(ControllerEvent::B));
    }
}
