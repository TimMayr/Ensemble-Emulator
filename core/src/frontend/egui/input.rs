use std::time::Instant;

use crossbeam_channel::Sender;
use egui::Context;

use crate::emulation::messages::{ControllerEvent, FrontendMessage};
use crate::frontend::egui::config::{SpeedConfig, ViewConfig};

/// Handle keyboard input from the user
pub fn handle_keyboard_input(
    ctx: &Context,
    to_emulator: &Sender<FrontendMessage>,
    speed_config: &mut SpeedConfig,
    view_config: &mut ViewConfig,
    last_frame_request: &mut Instant,
) {
    ctx.input(|i| {
        // Emulator controls
        if i.key_pressed(egui::Key::N) {
            view_config.debug_active_palette += 1;
            view_config.debug_active_palette &= 7;
        }
        if i.key_pressed(egui::Key::Period) {
            speed_config.is_paused = !speed_config.is_paused;
            *last_frame_request = Instant::now();
        }
        if i.key_pressed(egui::Key::R) {
            let _ = to_emulator.send(FrontendMessage::Reset);
        }

        // Controller input
        if i.key_pressed(egui::Key::ArrowLeft) {
            let _ = to_emulator.send(FrontendMessage::ControllerInput(ControllerEvent::Left));
        }
        if i.key_pressed(egui::Key::ArrowRight) {
            let _ = to_emulator.send(FrontendMessage::ControllerInput(ControllerEvent::Right));
        }
        if i.key_pressed(egui::Key::ArrowUp) {
            let _ = to_emulator.send(FrontendMessage::ControllerInput(ControllerEvent::Up));
        }
        if i.key_pressed(egui::Key::ArrowDown) {
            let _ = to_emulator.send(FrontendMessage::ControllerInput(ControllerEvent::Down));
        }
        if i.key_pressed(egui::Key::S) {
            let _ = to_emulator.send(FrontendMessage::ControllerInput(ControllerEvent::Start));
        }
        if i.key_pressed(egui::Key::Tab) {
            let _ = to_emulator.send(FrontendMessage::ControllerInput(ControllerEvent::Select));
        }
        if i.key_pressed(egui::Key::Space) {
            let _ = to_emulator.send(FrontendMessage::ControllerInput(ControllerEvent::A));
        }

        if i.modifiers.shift {
            let _ = to_emulator.send(FrontendMessage::ControllerInput(ControllerEvent::B));
        }
    });
}
