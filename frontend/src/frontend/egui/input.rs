use std::time::Instant;

use crossbeam_channel::Sender;
use egui::Context;
use ensemble_lockstep::emulation::screen_renderer::ScreenRenderer;

use crate::frontend::egui::config::AppConfig;
use crate::frontend::egui::keybindings::Binding;
use crate::frontend::messages::AsyncFrontendMessage;
use crate::messages::ControllerEvent;

/// Check if a keybind is pressed in the given input state.
fn is_binding_pressed(input: &egui::InputState, binding: &Option<Binding>) -> bool {
    match binding {
        Some(b) => b.pressed(input),
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
pub fn handle_keyboard_input<R: ScreenRenderer>(
    ctx: &Context,
    async_sender: &Sender<AsyncFrontendMessage>,
    config: &mut AppConfig<R>,
    last_frame_request: &mut Instant,
) {
    ctx.input(|i| {
        // Debug controls
        if is_binding_pressed(i, &config.keybindings.debug.cycle_palette) {
            config.view_config.debug_active_palette += 1;
            config.view_config.debug_active_palette &= 7;
        }

        // Emulation controls
        if is_binding_pressed(i, &config.keybindings.emulation.pause) {
            config.speed_config.is_paused = !config.speed_config.is_paused;
            *last_frame_request = Instant::now();
        }

        if is_binding_pressed(i, &config.keybindings.emulation.reset) {
            let _ = async_sender.send(AsyncFrontendMessage::Reset);
        }

        if is_binding_pressed(i, &config.keybindings.emulation.quicksave) {
            let _ = async_sender.send(AsyncFrontendMessage::Quicksave);
        }

        if is_binding_pressed(i, &config.keybindings.emulation.quickload) {
            let _ = async_sender.send(AsyncFrontendMessage::Quickload);
        }

        // NES controller input
        handle_controller_input(i, async_sender, config);
    });
}

/// Handle NES controller input mapping from keyboard
fn handle_controller_input<R: ScreenRenderer>(
    input: &egui::InputState,
    async_sender: &Sender<AsyncFrontendMessage>,
    config: &AppConfig<R>,
) {
    // D-pad
    if is_binding_pressed(input, &config.keybindings.controller.left) {
        let _ = async_sender.send(AsyncFrontendMessage::ControllerInput(ControllerEvent::Left));
    }
    if is_binding_pressed(input, &config.keybindings.controller.right) {
        let _ = async_sender.send(AsyncFrontendMessage::ControllerInput(
            ControllerEvent::Right,
        ));
    }
    if is_binding_pressed(input, &config.keybindings.controller.up) {
        let _ = async_sender.send(AsyncFrontendMessage::ControllerInput(ControllerEvent::Up));
    }
    if is_binding_pressed(input, &config.keybindings.controller.down) {
        let _ = async_sender.send(AsyncFrontendMessage::ControllerInput(ControllerEvent::Down));
    }

    // Buttons
    if is_binding_pressed(input, &config.keybindings.controller.start) {
        let _ = async_sender.send(AsyncFrontendMessage::ControllerInput(
            ControllerEvent::Start,
        ));
    }
    if is_binding_pressed(input, &config.keybindings.controller.select) {
        let _ = async_sender.send(AsyncFrontendMessage::ControllerInput(
            ControllerEvent::Select,
        ));
    }
    if is_binding_pressed(input, &config.keybindings.controller.a) {
        let _ = async_sender.send(AsyncFrontendMessage::ControllerInput(ControllerEvent::A));
    }
    if is_binding_pressed(input, &config.keybindings.controller.b) {
        let _ = async_sender.send(AsyncFrontendMessage::ControllerInput(ControllerEvent::B));
    }
}
